use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use std::process;

/// ICL — Intent Contract Language CLI
///
/// Validate, normalize, verify, and execute ICL contracts.
#[derive(Parser)]
#[command(name = "icl", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Suppress non-error output (for CI usage)
    #[arg(long, global = true)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate an ICL contract (syntax + types)
    Validate {
        /// Path to .icl file
        file: PathBuf,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Normalize a contract to canonical form
    Normalize {
        /// Path to .icl file
        file: PathBuf,
    },

    /// Full verification (types, invariants, determinism, coherence)
    Verify {
        /// Path to .icl file
        file: PathBuf,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Format a contract to standard style
    Fmt {
        /// Path to .icl file
        file: PathBuf,
        /// Write formatted output back to file (in-place)
        #[arg(long, short)]
        write: bool,
    },

    /// Compute semantic hash (SHA-256) of a contract
    Hash {
        /// Path to .icl file
        file: PathBuf,
    },

    /// Semantic diff between two contracts
    Diff {
        /// First .icl file
        file_a: PathBuf,
        /// Second .icl file
        file_b: PathBuf,
    },

    /// Scaffold a new ICL contract
    Init {
        /// Contract name (used for stable_id)
        name: Option<String>,
    },

    /// Execute a contract with inputs
    Execute {
        /// Path to .icl file
        file: PathBuf,
        /// JSON input string
        #[arg(long)]
        input: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show version information
    Version,
}

// ── Exit codes ────────────────────────────────────────────

const EXIT_SUCCESS: i32 = 0;
const EXIT_VALIDATION_FAILURE: i32 = 1;
const EXIT_ERROR: i32 = 2;

// ── Main ──────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();
    let quiet = cli.quiet;

    let exit_code = match cli.command {
        Commands::Validate { file, json } => cmd_validate(&file, json, quiet),
        Commands::Normalize { file } => cmd_normalize(&file, quiet),
        Commands::Verify { file, json } => cmd_verify(&file, json, quiet),
        Commands::Fmt { file, write } => cmd_fmt(&file, write, quiet),
        Commands::Hash { file } => cmd_hash(&file, quiet),
        Commands::Diff { file_a, file_b } => cmd_diff(&file_a, &file_b, quiet),
        Commands::Init { name } => cmd_init(name.as_deref(), quiet),
        Commands::Execute { file, input, json } => cmd_execute(&file, &input, json, quiet),
        Commands::Version => cmd_version(),
    };

    process::exit(exit_code);
}

// ── Command Implementations ──────────────────────────────

/// `icl validate <file>` — parse + verify (types, invariants, determinism, coherence)
fn cmd_validate(file: &PathBuf, json: bool, quiet: bool) -> i32 {
    let source = match read_icl_file(file) {
        Ok(s) => s,
        Err(code) => return code,
    };

    // Parse
    let ast = match icl_core::parser::parse(&source) {
        Ok(ast) => ast,
        Err(e) => {
            if json {
                let output = serde_json::json!({
                    "valid": false,
                    "error": format!("{}", e),
                    "phase": "parse"
                });
                println!(
                    "{}",
                    serde_json::to_string_pretty(&output).unwrap_or_default()
                );
            } else {
                eprintln!("{} {}", "error:".red().bold(), e);
            }
            return EXIT_VALIDATION_FAILURE;
        }
    };

    // Verify
    let result = icl_core::verifier::verify(&ast);

    if json {
        let diagnostics: Vec<serde_json::Value> = result
            .diagnostics
            .iter()
            .map(|d| {
                serde_json::json!({
                    "severity": format!("{:?}", d.severity),
                    "kind": format!("{}", d.kind),
                    "message": d.message,
                })
            })
            .collect();

        let output = serde_json::json!({
            "valid": result.is_valid(),
            "file": file.display().to_string(),
            "errors": result.errors().len(),
            "warnings": result.warnings().len(),
            "diagnostics": diagnostics,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&output).unwrap_or_default()
        );
    } else if result.is_valid() {
        if !quiet {
            println!("{} {} is valid", "✓".green().bold(), file.display());
            let warning_count = result.warnings().len();
            if warning_count > 0 {
                for w in result.warnings() {
                    eprintln!("  {} {}", "warning:".yellow(), w.message);
                }
            }
        }
    } else {
        for e in result.errors() {
            eprintln!("  {} {}", "error:".red().bold(), e.message);
        }
        for w in result.warnings() {
            eprintln!("  {} {}", "warning:".yellow(), w.message);
        }
        eprintln!(
            "{} {} ({} error(s), {} warning(s))",
            "✗".red().bold(),
            file.display(),
            result.errors().len(),
            result.warnings().len(),
        );
    }

    if result.is_valid() {
        EXIT_SUCCESS
    } else {
        EXIT_VALIDATION_FAILURE
    }
}

/// `icl normalize <file>` — output canonical form
fn cmd_normalize(file: &PathBuf, _quiet: bool) -> i32 {
    let source = match read_icl_file(file) {
        Ok(s) => s,
        Err(code) => return code,
    };

    match icl_core::normalizer::normalize(&source) {
        Ok(canonical) => {
            print!("{}", canonical);
            EXIT_SUCCESS
        }
        Err(e) => {
            eprintln!("{} {}", "error:".red().bold(), e);
            EXIT_ERROR
        }
    }
}

/// `icl verify <file>` — full verification with detailed output
fn cmd_verify(file: &PathBuf, json: bool, quiet: bool) -> i32 {
    let source = match read_icl_file(file) {
        Ok(s) => s,
        Err(code) => return code,
    };

    let ast = match icl_core::parser::parse(&source) {
        Ok(ast) => ast,
        Err(e) => {
            if json {
                let output = serde_json::json!({
                    "verified": false,
                    "error": format!("{}", e),
                    "phase": "parse"
                });
                println!(
                    "{}",
                    serde_json::to_string_pretty(&output).unwrap_or_default()
                );
            } else {
                eprintln!("{} {}", "error:".red().bold(), e);
            }
            return EXIT_VALIDATION_FAILURE;
        }
    };

    let result = icl_core::verifier::verify(&ast);

    if json {
        let diagnostics: Vec<serde_json::Value> = result
            .diagnostics
            .iter()
            .map(|d| {
                serde_json::json!({
                    "severity": format!("{:?}", d.severity),
                    "kind": format!("{}", d.kind),
                    "message": d.message,
                })
            })
            .collect();

        let output = serde_json::json!({
            "verified": result.is_valid(),
            "file": file.display().to_string(),
            "errors": result.errors().len(),
            "warnings": result.warnings().len(),
            "diagnostics": diagnostics,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&output).unwrap_or_default()
        );
    } else if result.is_valid() {
        if !quiet {
            println!(
                "{} {} verified successfully",
                "✓".green().bold(),
                file.display()
            );
            let warning_count = result.warnings().len();
            if warning_count > 0 {
                for w in result.warnings() {
                    eprintln!("  {} {}", "warning:".yellow(), w.message);
                }
            }
        }
    } else {
        eprintln!(
            "{} Verification failed for {}",
            "✗".red().bold(),
            file.display()
        );
        for e in result.errors() {
            eprintln!("  {} [{}] {}", "error:".red().bold(), e.kind, e.message);
        }
        for w in result.warnings() {
            eprintln!("  {} [{}] {}", "warning:".yellow(), w.kind, w.message);
        }
    }

    if result.is_valid() {
        EXIT_SUCCESS
    } else {
        EXIT_VALIDATION_FAILURE
    }
}

/// `icl fmt <file>` — format to standard style (normalize without hash update)
fn cmd_fmt(file: &PathBuf, write: bool, quiet: bool) -> i32 {
    let source = match read_icl_file(file) {
        Ok(s) => s,
        Err(code) => return code,
    };

    match icl_core::normalizer::normalize(&source) {
        Ok(formatted) => {
            if write {
                match std::fs::write(file, &formatted) {
                    Ok(_) => {
                        if !quiet {
                            println!("{} formatted {}", "✓".green().bold(), file.display());
                        }
                        EXIT_SUCCESS
                    }
                    Err(e) => {
                        eprintln!(
                            "{} failed to write {}: {}",
                            "error:".red().bold(),
                            file.display(),
                            e
                        );
                        EXIT_ERROR
                    }
                }
            } else {
                print!("{}", formatted);
                EXIT_SUCCESS
            }
        }
        Err(e) => {
            eprintln!("{} {}", "error:".red().bold(), e);
            EXIT_ERROR
        }
    }
}

/// `icl hash <file>` — compute and print semantic hash
fn cmd_hash(file: &PathBuf, _quiet: bool) -> i32 {
    let source = match read_icl_file(file) {
        Ok(s) => s,
        Err(code) => return code,
    };

    match icl_core::parser::parse(&source) {
        Ok(ast) => {
            let normalized = icl_core::normalizer::normalize_ast(ast);
            let hash = icl_core::normalizer::compute_semantic_hash(&normalized);
            println!("{}", hash);
            EXIT_SUCCESS
        }
        Err(e) => {
            eprintln!("{} {}", "error:".red().bold(), e);
            EXIT_ERROR
        }
    }
}

/// `icl diff <a> <b>` — semantic diff between two contracts
fn cmd_diff(file_a: &PathBuf, file_b: &PathBuf, _quiet: bool) -> i32 {
    let source_a = match read_icl_file(file_a) {
        Ok(s) => s,
        Err(code) => return code,
    };
    let source_b = match read_icl_file(file_b) {
        Ok(s) => s,
        Err(code) => return code,
    };

    let canonical_a = match icl_core::normalizer::normalize(&source_a) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {} — {}", "error:".red().bold(), file_a.display(), e);
            return EXIT_ERROR;
        }
    };
    let canonical_b = match icl_core::normalizer::normalize(&source_b) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {} — {}", "error:".red().bold(), file_b.display(), e);
            return EXIT_ERROR;
        }
    };

    if canonical_a == canonical_b {
        println!(
            "{} contracts are semantically identical",
            "=".green().bold()
        );
        EXIT_SUCCESS
    } else {
        // Line-by-line diff of canonical forms
        let lines_a: Vec<&str> = canonical_a.lines().collect();
        let lines_b: Vec<&str> = canonical_b.lines().collect();
        let max_lines = lines_a.len().max(lines_b.len());

        println!("--- {} (canonical)", file_a.display().to_string().red());
        println!("+++ {} (canonical)", file_b.display().to_string().green());

        for i in 0..max_lines {
            let la = lines_a.get(i).copied().unwrap_or("");
            let lb = lines_b.get(i).copied().unwrap_or("");
            if la != lb {
                if !la.is_empty() {
                    println!("{}", format!("- {}", la).red());
                }
                if !lb.is_empty() {
                    println!("{}", format!("+ {}", lb).green());
                }
            } else {
                println!("  {}", la);
            }
        }

        EXIT_VALIDATION_FAILURE
    }
}

/// `icl init [name]` — scaffold a new contract
fn cmd_init(name: Option<&str>, quiet: bool) -> i32 {
    let contract_name = name.unwrap_or("my-contract");

    // Validate name looks like a stable_id
    if contract_name.len() < 2
        || contract_name.len() > 32
        || !contract_name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        eprintln!(
            "{} contract name must be 2-32 chars, lowercase alphanumeric + hyphens",
            "error:".red().bold()
        );
        return EXIT_ERROR;
    }

    let filename = format!("{}.icl", contract_name);

    if std::path::Path::new(&filename).exists() {
        eprintln!("{} {} already exists", "error:".red().bold(), filename);
        return EXIT_ERROR;
    }

    let template = format!(
        r#"Contract {{
  Identity {{
    stable_id: "{}",
    version: 1,
    created_timestamp: 2026-01-01T00:00:00Z,
    owner: "your-name",
    semantic_hash: "0000000000000000000000000000000000000000000000000000000000000000"
  }}

  PurposeStatement {{
    narrative: "Describe what this contract does",
    intent_source: "user_natural_language",
    confidence_level: 0.9
  }}

  DataSemantics {{
    state: {{
      value: String
    }},
    invariants: []
  }}

  BehavioralSemantics {{
    operations: []
  }}

  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: 10485760,
      computation_timeout_ms: 1000,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}

  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}
"#,
        contract_name
    );

    match std::fs::write(&filename, &template) {
        Ok(_) => {
            if !quiet {
                println!("{} created {}", "✓".green().bold(), filename);
            }
            EXIT_SUCCESS
        }
        Err(e) => {
            eprintln!(
                "{} failed to write {}: {}",
                "error:".red().bold(),
                filename,
                e
            );
            EXIT_ERROR
        }
    }
}

/// `icl execute <file>` — execute a contract with JSON inputs
fn cmd_execute(file: &PathBuf, input: &str, json: bool, quiet: bool) -> i32 {
    let source = match read_icl_file(file) {
        Ok(s) => s,
        Err(code) => return code,
    };

    // Parse
    let contract = match icl_core::parser::parse(&source) {
        Ok(ast) => ast,
        Err(e) => {
            if !quiet {
                eprintln!(
                    "{} {} has parse errors: {}",
                    "error:".red().bold(),
                    file.display(),
                    e
                );
            }
            return EXIT_ERROR;
        }
    };

    // Verify first
    let verification = icl_core::verifier::verify(&contract);
    if !verification.is_valid() {
        if !quiet {
            eprintln!(
                "{} {} failed verification:",
                "error:".red().bold(),
                file.display()
            );
            for e in &verification.errors() {
                eprintln!("  {}", e.message);
            }
        }
        return EXIT_VALIDATION_FAILURE;
    }

    // Convert AST to runtime Contract
    let runtime_contract = match icl_core::parser::lower_contract(&contract) {
        Ok(c) => c,
        Err(e) => {
            if !quiet {
                eprintln!("{} {}", "error:".red().bold(), e);
            }
            return EXIT_ERROR;
        }
    };

    // Execute
    match icl_core::executor::execute_contract(&runtime_contract, input) {
        Ok(result) => {
            if json {
                println!("{}", result);
            } else {
                // Pretty-print a summary
                let result_json: serde_json::Value =
                    serde_json::from_str(&result).unwrap_or_default();
                let success = result_json["success"].as_bool().unwrap_or(false);
                if success {
                    if !quiet {
                        println!(
                            "{} {} executed successfully",
                            "✓".green().bold(),
                            file.display()
                        );
                    }
                    let ops = result_json["operations"]
                        .as_array()
                        .map(|a| a.len())
                        .unwrap_or(0);
                    if !quiet {
                        println!("  Operations: {}", ops);
                        println!(
                            "  Provenance entries: {}",
                            result_json["provenance"]["entries"]
                                .as_array()
                                .map(|a| a.len())
                                .unwrap_or(0)
                        );
                    }
                } else {
                    if !quiet {
                        eprintln!(
                            "{} execution failed: {}",
                            "✗".red().bold(),
                            result_json["error"].as_str().unwrap_or("unknown error")
                        );
                    }
                    return EXIT_VALIDATION_FAILURE;
                }
            }
            EXIT_SUCCESS
        }
        Err(e) => {
            if json {
                let err_json = serde_json::json!({
                    "success": false,
                    "error": e.to_string()
                });
                println!("{}", serde_json::to_string_pretty(&err_json).unwrap());
            } else if !quiet {
                eprintln!("{} {}", "error:".red().bold(), e);
            }
            EXIT_ERROR
        }
    }
}

/// `icl version` — show version information
fn cmd_version() -> i32 {
    println!(
        "icl {} (icl-core {})",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_VERSION")
    );
    println!("Phases complete: Parser, Normalizer, Verifier, CLI");
    EXIT_SUCCESS
}

// ── Helpers ───────────────────────────────────────────────

/// Read an ICL file, printing error and returning exit code on failure
fn read_icl_file(file: &PathBuf) -> std::result::Result<String, i32> {
    match std::fs::read_to_string(file) {
        Ok(s) => Ok(s),
        Err(e) => {
            eprintln!(
                "{} cannot read '{}': {}",
                "error:".red().bold(),
                file.display(),
                e
            );
            Err(EXIT_ERROR)
        }
    }
}
