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

pub mod error;
pub mod executor;
pub mod normalizer;
pub mod parser;
pub mod verifier;

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
    use super::*;

    fn test_contract() -> Contract {
        Contract {
            identity: Identity {
                stable_id: "ic-lib-test-001".into(),
                version: 1,
                created_timestamp: "2026-02-01T10:00:00Z".into(),
                owner: "test".into(),
                semantic_hash: "abc123".into(),
            },
            purpose_statement: PurposeStatement {
                narrative: "Lib test contract".into(),
                intent_source: "test".into(),
                confidence_level: 1.0,
            },
            data_semantics: DataSemantics {
                state: serde_json::json!({"message": "String", "count": "Integer"}),
                invariants: vec!["count >= 0".into()],
            },
            behavioral_semantics: BehavioralSemantics {
                operations: vec![Operation {
                    name: "echo".into(),
                    precondition: "input_provided".into(),
                    parameters: serde_json::json!({"message": "String"}),
                    postcondition: "state_updated".into(),
                    side_effects: vec!["log".into()],
                    idempotence: "idempotent".into(),
                }],
            },
            execution_constraints: ExecutionConstraints {
                trigger_types: vec!["manual".into()],
                resource_limits: ResourceLimits {
                    max_memory_bytes: 1_048_576,
                    computation_timeout_ms: 1000,
                    max_state_size_bytes: 1_048_576,
                },
                external_permissions: vec![],
                sandbox_mode: "full_isolation".into(),
            },
            human_machine_contract: HumanMachineContract {
                system_commitments: vec!["Echoes messages".into()],
                system_refusals: vec![],
                user_obligations: vec![],
            },
        }
    }

    #[test]
    fn test_contract_serialization() {
        let contract = test_contract();
        let json = serde_json::to_string(&contract).unwrap();
        let deserialized: Contract = serde_json::from_str(&json).unwrap();
        assert_eq!(contract, deserialized);
    }

    #[test]
    fn test_determinism_100_iterations() {
        let contract = test_contract();
        let input = r#"{"operation": "echo", "inputs": {"message": "determinism"}}"#;
        let first = executor::execute_contract(&contract, input).unwrap();
        for i in 0..100 {
            let result = executor::execute_contract(&contract, input).unwrap();
            assert_eq!(first, result, "Non-determinism at iteration {}", i);
        }
    }
}
