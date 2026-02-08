//! JavaScript/TypeScript bindings for ICL (Intent Contract Language)
//!
//! Thin wrapper around `icl-core` compiled to WebAssembly.
//! ZERO logic here — all behavior from the canonical Rust implementation.

use wasm_bindgen::prelude::*;

/// Parse ICL contract text and return a JSON string of the parsed Contract.
///
/// @param text - ICL contract source text
/// @returns JSON string representation of the parsed Contract
/// @throws Error if the contract text has syntax or semantic errors
#[wasm_bindgen(js_name = "parseContract")]
pub fn parse_contract(text: &str) -> Result<String, JsError> {
    let contract = icl_core::parser::parse_contract(text)
        .map_err(|e| JsError::new(&e.to_string()))?;

    serde_json::to_string_pretty(&contract)
        .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
}

/// Normalize ICL contract text to canonical form.
///
/// Guarantees:
///   - Deterministic: same input → same output
///   - Idempotent: normalize(normalize(x)) === normalize(x)
///   - Semantic preserving: meaning is unchanged
///
/// @param text - ICL contract source text
/// @returns Canonical normalized ICL text
/// @throws Error if the contract text cannot be parsed
#[wasm_bindgen]
pub fn normalize(text: &str) -> Result<String, JsError> {
    icl_core::normalizer::normalize(text)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Verify an ICL contract for correctness.
///
/// Runs all verification phases:
///   - Type checking
///   - Invariant verification
///   - Determinism checking
///   - Coherence verification
///
/// @param text - ICL contract source text
/// @returns JSON string: { valid: boolean, errors: [...], warnings: [...] }
/// @throws Error if the contract text cannot be parsed
#[wasm_bindgen]
pub fn verify(text: &str) -> Result<String, JsError> {
    let ast = icl_core::parser::parse(text)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let result = icl_core::verifier::verify(&ast);

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
        .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
}

/// Execute an ICL contract with the given inputs.
///
/// @param text - ICL contract source text
/// @param inputs - JSON string with execution inputs
/// @returns JSON string with execution result including provenance log
/// @throws Error if the contract cannot be parsed, verified, or executed
#[wasm_bindgen]
pub fn execute(text: &str, inputs: &str) -> Result<String, JsError> {
    let contract = icl_core::parser::parse_contract(text)
        .map_err(|e| JsError::new(&format!("Parse error: {}", e)))?;

    icl_core::executor::execute_contract(&contract, inputs)
        .map_err(|e| JsError::new(&format!("Execution error: {}", e)))
}

/// Compute the SHA-256 semantic hash of a contract.
///
/// @param text - ICL contract source text
/// @returns Hex-encoded SHA-256 hash string
/// @throws Error if the contract text cannot be parsed
#[wasm_bindgen(js_name = "semanticHash")]
pub fn semantic_hash(text: &str) -> Result<String, JsError> {
    let ast = icl_core::parser::parse(text)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let normalized = icl_core::normalizer::normalize_ast(ast);
    Ok(icl_core::normalizer::compute_semantic_hash(&normalized))
}
