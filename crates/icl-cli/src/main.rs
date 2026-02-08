use clap::{Parser, Subcommand};
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

    /// Full verification (invariants, determinism, coherence)
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

fn main() {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Commands::Validate { file, json: _ } => {
            eprintln!("icl validate: not yet implemented (Phase 4)");
            eprintln!("  file: {}", file.display());
            1
        }
        Commands::Normalize { file } => {
            eprintln!("icl normalize: not yet implemented (Phase 4)");
            eprintln!("  file: {}", file.display());
            1
        }
        Commands::Verify { file, json: _ } => {
            eprintln!("icl verify: not yet implemented (Phase 4)");
            eprintln!("  file: {}", file.display());
            1
        }
        Commands::Fmt { file } => {
            eprintln!("icl fmt: not yet implemented (Phase 4)");
            eprintln!("  file: {}", file.display());
            1
        }
        Commands::Hash { file } => {
            eprintln!("icl hash: not yet implemented (Phase 4)");
            eprintln!("  file: {}", file.display());
            1
        }
        Commands::Diff { file_a, file_b } => {
            eprintln!("icl diff: not yet implemented (Phase 4)");
            eprintln!("  a: {}", file_a.display());
            eprintln!("  b: {}", file_b.display());
            1
        }
        Commands::Init { name } => {
            eprintln!("icl init: not yet implemented (Phase 4)");
            if let Some(n) = name {
                eprintln!("  name: {}", n);
            }
            1
        }
        Commands::Execute { file, input, json: _ } => {
            eprintln!("icl execute: not yet implemented (Phase 5)");
            eprintln!("  file: {}", file.display());
            eprintln!("  input: {}", input);
            1
        }
        Commands::Version => {
            println!("icl {} (icl-core {})", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_VERSION"));
            println!("Status: Early Development — Phase 0");
            0
        }
    };

    process::exit(exit_code);
}
