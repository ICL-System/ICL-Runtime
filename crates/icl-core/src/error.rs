//! Error types for ICL runtime
//!
//! All fallible operations return `Result<T, Error>`.
//! Error types provide context for diagnosis.

use std::fmt;

/// ICL runtime error types
#[derive(Debug, Clone)]
pub enum Error {
    /// Syntax or structure violation during parsing
    ParseError(String),

    /// Type mismatch in contract
    TypeError { expected: String, found: String },

    /// Non-deterministic behavior detected
    DeterminismViolation(String),

    /// Contract commitment or postcondition violated
    ContractViolation {
        commitment: String,
        violation: String,
    },

    /// Invariant or constraint validation failure
    ValidationError(String),

    /// Runtime execution failure
    ExecutionError(String),

    /// Normalization failure
    NormalizationError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Error::TypeError { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            Error::DeterminismViolation(msg) => write!(f, "Determinism violation: {}", msg),
            Error::ContractViolation {
                commitment,
                violation,
            } => {
                write!(f, "Contract violation - {}: {}", commitment, violation)
            }
            Error::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Error::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            Error::NormalizationError(msg) => write!(f, "Normalization error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type alias for ICL operations
pub type Result<T> = std::result::Result<T, Error>;
