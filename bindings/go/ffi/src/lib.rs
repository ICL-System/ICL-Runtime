//! C-FFI layer for ICL — used by Go (cgo) and other FFI consumers.
//!
//! ZERO logic here. All calls delegate to `icl-core`.
//!
//! # Memory Contract
//!
//! All functions that return `*mut c_char` allocate via `CString`.
//! The caller MUST free the returned string by calling `icl_free_string()`.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Result from an ICL FFI call.
/// If `error` is null, the call succeeded and `result` contains the output.
/// If `error` is non-null, the call failed and `error` contains the error message.
/// The caller MUST free both `result` and `error` with `icl_free_string()`.
#[repr(C)]
pub struct IclResult {
    pub result: *mut c_char,
    pub error: *mut c_char,
}

impl IclResult {
    fn ok(value: String) -> Self {
        let c = CString::new(value).unwrap_or_else(|_| CString::new("").unwrap());
        IclResult {
            result: c.into_raw(),
            error: std::ptr::null_mut(),
        }
    }

    fn err(msg: String) -> Self {
        let c = CString::new(msg).unwrap_or_else(|_| CString::new("unknown error").unwrap());
        IclResult {
            result: std::ptr::null_mut(),
            error: c.into_raw(),
        }
    }
}

/// Helper: convert a C string pointer to a Rust &str.
/// Returns None if the pointer is null or not valid UTF-8.
unsafe fn cstr_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr).to_str().ok()
}

/// Parse ICL contract text and return a JSON string of the parsed Contract.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 C string.
/// The caller must free the returned strings with `icl_free_string()`.
#[no_mangle]
pub unsafe extern "C" fn icl_parse_contract(text: *const c_char) -> IclResult {
    let text = match cstr_to_str(text) {
        Some(s) => s,
        None => return IclResult::err("null or invalid UTF-8 input".into()),
    };

    match icl_core::parser::parse_contract(text) {
        Ok(contract) => match serde_json::to_string_pretty(&contract) {
            Ok(json) => IclResult::ok(json),
            Err(e) => IclResult::err(format!("Serialization error: {}", e)),
        },
        Err(e) => IclResult::err(e.to_string()),
    }
}

/// Normalize ICL contract text to canonical form.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 C string.
/// The caller must free the returned strings with `icl_free_string()`.
#[no_mangle]
pub unsafe extern "C" fn icl_normalize(text: *const c_char) -> IclResult {
    let text = match cstr_to_str(text) {
        Some(s) => s,
        None => return IclResult::err("null or invalid UTF-8 input".into()),
    };

    match icl_core::normalizer::normalize(text) {
        Ok(normalized) => IclResult::ok(normalized),
        Err(e) => IclResult::err(e.to_string()),
    }
}

/// Verify an ICL contract for correctness.
/// Returns JSON: { "valid": bool, "errors": [...], "warnings": [...] }
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 C string.
/// The caller must free the returned strings with `icl_free_string()`.
#[no_mangle]
pub unsafe extern "C" fn icl_verify(text: *const c_char) -> IclResult {
    let text = match cstr_to_str(text) {
        Some(s) => s,
        None => return IclResult::err("null or invalid UTF-8 input".into()),
    };

    let ast = match icl_core::parser::parse(text) {
        Ok(ast) => ast,
        Err(e) => return IclResult::err(e.to_string()),
    };

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

    match serde_json::to_string_pretty(&output) {
        Ok(json) => IclResult::ok(json),
        Err(e) => IclResult::err(format!("Serialization error: {}", e)),
    }
}

/// Execute an ICL contract with the given inputs.
///
/// # Safety
/// `text` and `inputs` must be valid null-terminated UTF-8 C strings.
/// The caller must free the returned strings with `icl_free_string()`.
#[no_mangle]
pub unsafe extern "C" fn icl_execute(text: *const c_char, inputs: *const c_char) -> IclResult {
    let text = match cstr_to_str(text) {
        Some(s) => s,
        None => return IclResult::err("null or invalid UTF-8 text".into()),
    };
    let inputs = match cstr_to_str(inputs) {
        Some(s) => s,
        None => return IclResult::err("null or invalid UTF-8 inputs".into()),
    };

    let contract = match icl_core::parser::parse_contract(text) {
        Ok(c) => c,
        Err(e) => return IclResult::err(format!("Parse error: {}", e)),
    };

    match icl_core::executor::execute_contract(&contract, inputs) {
        Ok(result) => IclResult::ok(result),
        Err(e) => IclResult::err(format!("Execution error: {}", e)),
    }
}

/// Compute the SHA-256 semantic hash of a contract.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 C string.
/// The caller must free the returned strings with `icl_free_string()`.
#[no_mangle]
pub unsafe extern "C" fn icl_semantic_hash(text: *const c_char) -> IclResult {
    let text = match cstr_to_str(text) {
        Some(s) => s,
        None => return IclResult::err("null or invalid UTF-8 input".into()),
    };

    let ast = match icl_core::parser::parse(text) {
        Ok(ast) => ast,
        Err(e) => return IclResult::err(e.to_string()),
    };

    let normalized = icl_core::normalizer::normalize_ast(ast);
    let hash = icl_core::normalizer::compute_semantic_hash(&normalized);
    IclResult::ok(hash)
}

/// Free a string previously returned by an ICL FFI function.
///
/// # Safety
/// `ptr` must be a pointer previously returned by an ICL FFI function,
/// or null (in which case this is a no-op).
#[no_mangle]
pub unsafe extern "C" fn icl_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}
