//! Error types for ICL runtime

use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    ParseError(String),
    TypeError { expected: String, found: String },
    DeterminismViolation(String),
    ContractViolation { commitment: String, violation: String },
    ValidationError(String),
    ExecutionError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Error::TypeError { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            Error::DeterminismViolation(msg) => write!(f, "Determinism violation: {}", msg),
            Error::ContractViolation { commitment, violation } => {
                write!(f, "Contract violation - {}: {}", commitment, violation)
            }
            Error::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Error::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
