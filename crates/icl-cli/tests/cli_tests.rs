//! Integration tests for the ICL CLI
//!
//! These tests invoke the actual icl-cli binary and verify:
//! - Exit codes (0 = success, 1 = validation failure, 2 = error)
//! - stdout/stderr output
//! - JSON output format
//! - All commands work end-to-end

use std::path::PathBuf;
use std::process::Command;

// ── Helpers ───────────────────────────────────────────────

fn icl_bin() -> PathBuf {
    // cargo test puts test binaries alongside the main binary
    let mut path = PathBuf::from(env!("CARGO_BIN_EXE_icl-cli"));
    if !path.exists() {
        // Fallback: try debug directory
        path = PathBuf::from("target/debug/icl-cli");
    }
    path
}

fn fixture_valid(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../tests/fixtures/conformance/valid/{}", name))
}

fn fixture_invalid(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../tests/fixtures/conformance/invalid/{}", name))
}

fn run_icl(args: &[&str]) -> std::process::Output {
    Command::new(icl_bin())
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to execute icl-cli")
}

// ── Version ───────────────────────────────────────────────

#[test]
fn test_version_command() {
    let output = run_icl(&["version"]);
    assert!(output.status.success(), "version should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("icl"), "should contain 'icl'");
    assert!(
        stdout.contains(env!("CARGO_PKG_VERSION")),
        "should contain version"
    );
}

#[test]
fn test_version_flag() {
    let output = run_icl(&["--version"]);
    assert!(output.status.success(), "--version should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(env!("CARGO_PKG_VERSION")),
        "should contain version"
    );
}

// ── Validate ──────────────────────────────────────────────

#[test]
fn test_validate_valid_contract() {
    let output = run_icl(&[
        "validate",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
    ]);
    assert!(output.status.success(), "valid contract should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("valid"), "should mention valid");
}

#[test]
fn test_validate_invalid_contract() {
    let output = run_icl(&[
        "validate",
        fixture_invalid("missing-identity.icl").to_str().unwrap(),
    ]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "invalid contract should exit 1"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error"), "should mention error");
}

#[test]
fn test_validate_nonexistent_file() {
    let output = run_icl(&["validate", "nonexistent.icl"]);
    assert_eq!(output.status.code(), Some(2), "missing file should exit 2");
}

#[test]
fn test_validate_json_output() {
    let output = run_icl(&[
        "validate",
        "--json",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "valid contract --json should exit 0"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["valid"], true);
    assert_eq!(json["errors"], 0);
}

#[test]
fn test_validate_json_invalid() {
    let output = run_icl(&[
        "validate",
        "--json",
        fixture_invalid("missing-identity.icl").to_str().unwrap(),
    ]);
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["valid"], false);
}

#[test]
fn test_validate_quiet_valid() {
    let output = run_icl(&[
        "--quiet",
        "validate",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
    ]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.is_empty(), "quiet mode should produce no stdout");
}

// ── Normalize ─────────────────────────────────────────────

#[test]
fn test_normalize_valid_contract() {
    let output = run_icl(&[
        "normalize",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
    ]);
    assert!(output.status.success(), "normalize should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Contract {"), "should output contract");
    assert!(stdout.contains("Identity {"), "should contain Identity");
}

#[test]
fn test_normalize_invalid_contract() {
    let output = run_icl(&[
        "normalize",
        fixture_invalid("missing-identity.icl").to_str().unwrap(),
    ]);
    assert_eq!(
        output.status.code(),
        Some(2),
        "normalize of invalid should exit 2"
    );
}

#[test]
fn test_normalize_idempotent() {
    // normalize(normalize(x)) should equal normalize(x)
    let output1 = run_icl(&[
        "normalize",
        fixture_valid("all-primitive-types.icl").to_str().unwrap(),
    ]);
    assert!(output1.status.success());
    let canonical1 = String::from_utf8_lossy(&output1.stdout).to_string();

    // Write canonical to temp file and normalize again
    let temp = std::env::temp_dir().join("icl_test_idempotent.icl");
    std::fs::write(&temp, &canonical1).expect("write temp");

    let output2 = run_icl(&["normalize", temp.to_str().unwrap()]);
    assert!(output2.status.success());
    let canonical2 = String::from_utf8_lossy(&output2.stdout).to_string();

    assert_eq!(canonical1, canonical2, "normalize must be idempotent");
    let _ = std::fs::remove_file(&temp);
}

// ── Verify ────────────────────────────────────────────────

#[test]
fn test_verify_valid_contract() {
    let output = run_icl(&[
        "verify",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
    ]);
    assert!(output.status.success(), "verify valid should exit 0");
}

#[test]
fn test_verify_json_output() {
    let output = run_icl(&[
        "verify",
        "--json",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
    ]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["verified"], true);
}

// ── Hash ──────────────────────────────────────────────────

#[test]
fn test_hash_valid_contract() {
    let output = run_icl(&[
        "hash",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
    ]);
    assert!(output.status.success(), "hash should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert_eq!(stdout.len(), 64, "SHA-256 hash should be 64 hex chars");
    assert!(
        stdout.chars().all(|c| c.is_ascii_hexdigit()),
        "hash should be hex"
    );
}

#[test]
fn test_hash_determinism() {
    let fixture = fixture_valid("all-primitive-types.icl");
    let path = fixture.to_str().unwrap();

    let first = run_icl(&["hash", path]);
    let first_hash = String::from_utf8_lossy(&first.stdout).trim().to_string();

    for _ in 0..10 {
        let output = run_icl(&["hash", path]);
        let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(first_hash, hash, "hash must be deterministic");
    }
}

// ── Fmt ───────────────────────────────────────────────────

#[test]
fn test_fmt_outputs_to_stdout() {
    let output = run_icl(&[
        "fmt",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
    ]);
    assert!(output.status.success(), "fmt should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Contract {"),
        "should output formatted contract"
    );
}

#[test]
fn test_fmt_write_flag() {
    // Copy fixture to temp, fmt --write, verify it changed
    let temp = std::env::temp_dir().join("icl_test_fmt_write.icl");
    let fixture = fixture_valid("minimal-contract.icl");
    let source = std::fs::read_to_string(&fixture).expect("read fixture");
    std::fs::write(&temp, &source).expect("write temp");

    let output = run_icl(&["fmt", "--write", temp.to_str().unwrap()]);
    assert!(output.status.success(), "fmt --write should exit 0");

    let formatted = std::fs::read_to_string(&temp).expect("read formatted");
    assert!(formatted.contains("Contract {"), "should be valid contract");

    let _ = std::fs::remove_file(&temp);
}

// ── Diff ──────────────────────────────────────────────────

#[test]
fn test_diff_identical_files() {
    let path = fixture_valid("minimal-contract.icl")
        .to_str()
        .unwrap()
        .to_string();
    let output = run_icl(&["diff", &path, &path]);
    assert!(output.status.success(), "diff of same file should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("identical"),
        "should say semantically identical"
    );
}

#[test]
fn test_diff_different_files() {
    let a = fixture_valid("minimal-contract.icl")
        .to_str()
        .unwrap()
        .to_string();
    let b = fixture_valid("all-primitive-types.icl")
        .to_str()
        .unwrap()
        .to_string();
    let output = run_icl(&["diff", &a, &b]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "diff of different files should exit 1"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("---"), "should contain diff markers");
    assert!(stdout.contains("+++"), "should contain diff markers");
}

// ── Init ──────────────────────────────────────────────────

#[test]
fn test_init_creates_file() {
    let temp_dir = std::env::temp_dir().join("icl_test_init");
    let _ = std::fs::create_dir_all(&temp_dir);

    let output = Command::new(icl_bin())
        .args(["init", "test-contract"])
        .current_dir(&temp_dir)
        .output()
        .expect("run init");

    assert!(output.status.success(), "init should exit 0");

    let file = temp_dir.join("test-contract.icl");
    assert!(file.exists(), "should create .icl file");

    let content = std::fs::read_to_string(&file).expect("read");
    assert!(content.contains("Contract {"), "should be a contract");
    assert!(
        content.contains("test-contract"),
        "should contain contract name"
    );

    // Validate the generated contract parses
    let validate_output = Command::new(icl_bin())
        .args(["validate", file.to_str().unwrap()])
        .output()
        .expect("run validate");
    assert!(
        validate_output.status.success(),
        "generated contract should be valid"
    );

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_init_default_name() {
    let temp_dir = std::env::temp_dir().join("icl_test_init_default");
    let _ = std::fs::create_dir_all(&temp_dir);

    let output = Command::new(icl_bin())
        .args(["init"])
        .current_dir(&temp_dir)
        .output()
        .expect("run init");

    assert!(output.status.success(), "init without name should exit 0");

    let file = temp_dir.join("my-contract.icl");
    assert!(file.exists(), "should create my-contract.icl");

    let _ = std::fs::remove_dir_all(&temp_dir);
}

// ── Execute ───────────────────────────────────────────────

#[test]
fn test_execute_no_operations() {
    let output = run_icl(&[
        "execute",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
        "--input",
        "[]",
    ]);
    assert!(
        output.status.success(),
        "execute with empty operations should exit 0"
    );
}

#[test]
fn test_execute_with_operation() {
    let output = run_icl(&[
        "execute",
        fixture_valid("all-primitive-types.icl").to_str().unwrap(),
        "--input",
        r#"{"operation": "update_count", "inputs": {"new_count": 42, "label": "test"}}"#,
    ]);
    assert!(
        output.status.success(),
        "execute with valid operation should exit 0: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_execute_json_output() {
    let output = run_icl(&[
        "execute",
        fixture_valid("all-primitive-types.icl").to_str().unwrap(),
        "--input",
        r#"{"operation": "update_count", "inputs": {"new_count": 10, "label": "test"}}"#,
        "--json",
    ]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["success"], true);
    assert!(!json["provenance"]["entries"].as_array().unwrap().is_empty());
}

#[test]
fn test_execute_unknown_operation() {
    let output = run_icl(&[
        "execute",
        fixture_valid("minimal-contract.icl").to_str().unwrap(),
        "--input",
        r#"{"operation": "nonexistent", "inputs": {}}"#,
    ]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "unknown operation should exit 1 (execution failure)"
    );
}

// ── All valid conformance fixtures ────────────────────────

#[test]
fn test_all_valid_conformance_fixtures_validate() {
    let valid_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/conformance/valid");

    if valid_dir.exists() {
        for entry in std::fs::read_dir(&valid_dir).expect("read dir") {
            let entry = entry.expect("entry");
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "icl") {
                let output = run_icl(&["validate", path.to_str().unwrap()]);
                assert!(
                    output.status.success(),
                    "conformance fixture {:?} should validate",
                    path.file_name()
                );
            }
        }
    }
}

#[test]
fn test_all_invalid_conformance_fixtures_fail() {
    let invalid_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/conformance/invalid");

    if invalid_dir.exists() {
        for entry in std::fs::read_dir(&invalid_dir).expect("read dir") {
            let entry = entry.expect("entry");
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "icl") {
                let output = run_icl(&["validate", path.to_str().unwrap()]);
                assert!(
                    !output.status.success(),
                    "conformance fixture {:?} should fail validation",
                    path.file_name()
                );
            }
        }
    }
}

// ── Determinism: CLI output ───────────────────────────────

#[test]
fn test_cli_validate_determinism_100_iterations() {
    let path = fixture_valid("all-primitive-types.icl")
        .to_str()
        .unwrap()
        .to_string();

    let first = run_icl(&["validate", "--json", &path]);
    let first_stdout = String::from_utf8_lossy(&first.stdout).to_string();

    for i in 0..100 {
        let output = run_icl(&["validate", "--json", &path]);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        assert_eq!(
            first_stdout, stdout,
            "validate --json determinism failure at iteration {}",
            i
        );
    }
}
