//! ICL Core - Canonical implementation of Intent Contract Language
//!
//! This is the single source of truth for ICL semantics.
//! All language bindings (Python, JavaScript, Go) compile this same core.
//!
//! # Architecture
//!
//! ```text
//! ICL Text → Parser → AST → Normalizer → Canonical Form
//!                              ↓
//!                           Verifier → Type Check + Invariants + Determinism
//!                              ↓
//!                           Executor → Sandboxed Execution
//! ```
//!
//! # Guarantees
//!
//! - **Deterministic**: Same input always produces identical output
//! - **Verifiable**: All properties machine-checkable
//! - **Bounded**: All execution bounded in memory and time
//! - **Canonical**: One normalized form per contract

pub mod parser;
pub mod normalizer;
pub mod verifier;
pub mod executor;
pub mod error;

pub use error::{Error, Result};
pub use parser::ast::*;

/// Core contract definition
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    pub identity: Identity,
    pub purpose_statement: PurposeStatement,
    pub data_semantics: DataSemantics,
    pub behavioral_semantics: BehavioralSemantics,
    pub execution_constraints: ExecutionConstraints,
    pub human_machine_contract: HumanMachineContract,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Identity {
    pub stable_id: String,
    pub version: u32,
    pub created_timestamp: String, // ISO8601
    pub owner: String,
    pub semantic_hash: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PurposeStatement {
    pub narrative: String,
    pub intent_source: String,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataSemantics {
    pub state: serde_json::Value,
    pub invariants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BehavioralSemantics {
    pub operations: Vec<Operation>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Operation {
    pub name: String,
    pub precondition: String,
    pub parameters: serde_json::Value,
    pub postcondition: String,
    pub side_effects: Vec<String>,
    pub idempotence: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecutionConstraints {
    pub trigger_types: Vec<String>,
    pub resource_limits: ResourceLimits,
    pub external_permissions: Vec<String>,
    pub sandbox_mode: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceLimits {
    pub max_memory_bytes: u64,
    pub computation_timeout_ms: u64,
    pub max_state_size_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HumanMachineContract {
    pub system_commitments: Vec<String>,
    pub system_refusals: Vec<String>,
    pub user_obligations: Vec<String>,
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_contract_serialization() {
        // TODO: Test that contract can be serialized and deserialized
    }

    #[test]
    fn test_determinism_100_iterations() {
        // TODO: Implement 100-iteration determinism test
        // Same contract → 100x execution → all outputs identical
    }
}
