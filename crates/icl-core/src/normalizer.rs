//! Canonical normalizer - converts ICL to deterministic canonical form
//!
//! The normalizer transforms an ICL contract into its canonical representation.
//! This is the single deterministic form used for hashing, comparison, and storage.

use crate::{Contract, Result};

/// Normalize contract to canonical form
///
/// # Guarantees
/// - Idempotent: `normalize(normalize(x)) == normalize(x)`
/// - Deterministic: same input always produces same output
/// - Unique: each distinct contract has one canonical form
/// - No information loss: all semantics preserved
///
/// # Errors
/// Returns `NormalizationError` if contract cannot be canonicalized.
pub fn normalize(icl: &str) -> Result<String> {
    // TODO: Phase 2 — Implement section sorting (alphabetical)
    // TODO: Phase 2 — Implement field sorting within sections
    // TODO: Phase 2 — Implement whitespace normalization
    // TODO: Phase 2 — Implement comment removal
    // TODO: Phase 2 — Implement type normalization
    // TODO: Phase 2 — Implement default expansion
    // TODO: Phase 2 — Implement SHA-256 semantic hash
    // TODO: Phase 2 — Implement canonical serialization

    // Placeholder — pass through
    Ok(icl.to_string())
}

/// Normalize a parsed Contract struct to canonical form
pub fn normalize_contract(_contract: &Contract) -> Result<Contract> {
    // TODO: Phase 2 — Implement on Contract struct
    Err(crate::Error::ValidationError("Normalizer not yet implemented".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idempotence() {
        // TODO: Test that normalize(normalize(x)) == normalize(x)
    }

    #[test]
    fn test_determinism_100_iterations() {
        // TODO: Test same input → 100x identical outputs
    }

    #[test]
    fn test_semantic_preservation() {
        // TODO: Test that normalization doesn't lose information
    }
}
