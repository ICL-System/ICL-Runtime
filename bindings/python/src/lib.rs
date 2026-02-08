//! Python bindings for ICL (Intent Contract Language)
//!
//! Thin wrapper around `icl-core` — ZERO logic here.
//! All behavior comes from the canonical Rust implementation.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Parse ICL contract text and return a JSON string of the parsed Contract.
///
/// Args:
///     text: ICL contract source text
///
/// Returns:
///     JSON string representation of the parsed Contract
///
/// Raises:
///     ValueError: If the contract text has syntax or semantic errors
#[pyfunction]
fn parse_contract(text: &str) -> PyResult<String> {
    let contract =
        icl_core::parser::parse_contract(text).map_err(|e| PyValueError::new_err(e.to_string()))?;

    serde_json::to_string_pretty(&contract)
        .map_err(|e| PyValueError::new_err(format!("Serialization error: {}", e)))
}

/// Normalize ICL contract text to canonical form.
///
/// Guarantees:
///   - Deterministic: same input → same output
///   - Idempotent: normalize(normalize(x)) == normalize(x)
///   - Semantic preserving: meaning is unchanged
///
/// Args:
///     text: ICL contract source text
///
/// Returns:
///     Canonical normalized ICL text
///
/// Raises:
///     ValueError: If the contract text cannot be parsed
#[pyfunction]
fn normalize(text: &str) -> PyResult<String> {
    icl_core::normalizer::normalize(text).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Verify an ICL contract for correctness.
///
/// Runs all verification phases:
///   - Type checking
///   - Invariant verification
///   - Determinism checking
///   - Coherence verification
///
/// Args:
///     text: ICL contract source text
///
/// Returns:
///     JSON string with verification result:
///     {
///         "valid": bool,
///         "errors": [{"severity": "error", "kind": "...", "message": "..."}],
///         "warnings": [{"severity": "warning", "kind": "...", "message": "..."}]
///     }
///
/// Raises:
///     ValueError: If the contract text cannot be parsed
#[pyfunction]
fn verify(text: &str) -> PyResult<String> {
    let ast = icl_core::parser::parse(text).map_err(|e| PyValueError::new_err(e.to_string()))?;

    let result = icl_core::verifier::verify(&ast);

    // Convert to JSON-serializable structure
    let errors: Vec<serde_json::Value> = result
        .errors()
        .iter()
        .map(|d| {
            serde_json::json!({
                "severity": "error",
                "kind": d.kind.to_string(),
                "message": d.message,
            })
        })
        .collect();

    let warnings: Vec<serde_json::Value> = result
        .warnings()
        .iter()
        .map(|d| {
            serde_json::json!({
                "severity": "warning",
                "kind": d.kind.to_string(),
                "message": d.message,
            })
        })
        .collect();

    let output = serde_json::json!({
        "valid": result.is_valid(),
        "errors": errors,
        "warnings": warnings,
    });

    serde_json::to_string_pretty(&output)
        .map_err(|e| PyValueError::new_err(format!("Serialization error: {}", e)))
}

/// Execute an ICL contract with the given inputs.
///
/// Runs the contract in a sandboxed environment with:
///   - Precondition evaluation
///   - Resource limit enforcement
///   - Postcondition verification
///   - Full provenance logging
///
/// Args:
///     text: ICL contract source text
///     inputs: JSON string with execution inputs
///         Single request: {"operation": "name", "inputs": {...}}
///         Multiple: [{"operation": "name", "inputs": {...}}, ...]
///
/// Returns:
///     JSON string with execution result including provenance log
///
/// Raises:
///     ValueError: If the contract cannot be parsed, verified, or executed
#[pyfunction]
fn execute(text: &str, inputs: &str) -> PyResult<String> {
    let contract = icl_core::parser::parse_contract(text)
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;

    icl_core::executor::execute_contract(&contract, inputs)
        .map_err(|e| PyValueError::new_err(format!("Execution error: {}", e)))
}

/// Compute the SHA-256 semantic hash of a contract.
///
/// The hash is computed from the normalized (canonical) form,
/// so semantically equivalent contracts produce the same hash.
///
/// Args:
///     text: ICL contract source text
///
/// Returns:
///     Hex-encoded SHA-256 hash string
///
/// Raises:
///     ValueError: If the contract text cannot be parsed
#[pyfunction]
fn semantic_hash(text: &str) -> PyResult<String> {
    let ast = icl_core::parser::parse(text).map_err(|e| PyValueError::new_err(e.to_string()))?;

    let normalized = icl_core::normalizer::normalize_ast(ast);
    Ok(icl_core::normalizer::compute_semantic_hash(&normalized))
}

/// ICL Python module — deterministic intent contract runtime
#[pymodule]
fn icl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_contract, m)?)?;
    m.add_function(wrap_pyfunction!(normalize, m)?)?;
    m.add_function(wrap_pyfunction!(verify, m)?)?;
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    m.add_function(wrap_pyfunction!(semantic_hash, m)?)?;
    Ok(())
}
