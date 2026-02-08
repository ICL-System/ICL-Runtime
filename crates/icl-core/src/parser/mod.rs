//! ICL Parser — tokenizer, AST types, and recursive descent parser
//!
//! Converts ICL text into an Abstract Syntax Tree (AST).
//! Matches the BNF grammar defined in CORE-SPECIFICATION.md Section 1.

pub mod tokenizer;
pub mod ast;

use crate::{Contract, Result, Error};

/// Parse ICL text into a Contract AST
///
/// # Guarantees
/// - Deterministic: same input always produces same AST
/// - Complete: reports all errors, not just the first
///
/// # Errors
/// Returns `ParseError` with line:column for syntax violations.
///
/// # Example
/// ```ignore
/// let contract = parse_contract(icl_text)?;
/// ```
pub fn parse_contract(_input: &str) -> Result<Contract> {
    // TODO: Phase 1 — Implement tokenizer
    // TODO: Phase 1 — Implement recursive descent parser
    // TODO: Phase 1 — Error recovery (report multiple errors)
    Err(Error::ParseError("Parser not yet implemented".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_contract() {
        // TODO: Test parsing minimal valid contract
    }

    #[test]
    fn test_parse_invalid_syntax() {
        // TODO: Test that syntax errors are caught
    }

    #[test]
    fn test_parse_determinism_100_iterations() {
        // TODO: Same input → 100x parse → identical AST
    }
}
