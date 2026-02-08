// Conformance: valid/comments-everywhere.icl
// Tests: Comments placed before every section and inline
// Expected: MUST parse successfully

// This is a top-level comment before the Contract
// Another top-level comment
Contract {
  // Comment before Identity section
  Identity {
    // Comment for stable_id
    stable_id: "ic-conf-v50-001",
    // Comment for version
    version: 1,
    // Comment for timestamp
    created_timestamp: 2026-02-08T00:00:00Z,
    // Comment for owner
    owner: "conformance-suite",
    // Comment for semantic_hash
    semantic_hash: "5000500050005000500050005000500050005000500050005000500050005000"
  }

  // Comment before PurposeStatement
  PurposeStatement {
    // Narrative describes the purpose
    narrative: "Tests that comments can appear everywhere in a contract",
    // Source of the intent
    intent_source: "conformance_test",
    // Confidence in the range [0.0, 1.0]
    confidence_level: 1.0
  }

  // Comment before DataSemantics
  DataSemantics {
    // State definition
    state: {
      // A simple string field
      value: String,
      // An integer with default
      count: Integer = 0
    },
    // Invariant list
    invariants: [
      // First invariant
      "count >= 0"
    ]
  }

  // Comment before BehavioralSemantics
  BehavioralSemantics {
    // Operations list
    operations: [
      // A simple operation
      {
        name: "update",
        precondition: "true",
        parameters: {
          // Input parameter
          new_value: String
        },
        postcondition: "value updated",
        side_effects: [],
        idempotence: "idempotent"
      }
    ]
  }

  // Comment before ExecutionConstraints
  ExecutionConstraints {
    // Supported trigger types
    trigger_types: ["manual"],
    // Resource limits block
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    // Permissions
    external_permissions: [],
    // Sandbox configuration
    sandbox_mode: "full_isolation"
  }

  // Comment before HumanMachineContract
  HumanMachineContract {
    // System promises
    system_commitments: [],
    // What the system will not do
    system_refusals: [],
    // What users must do
    user_obligations: []
  }
}
// Comment after the contract
