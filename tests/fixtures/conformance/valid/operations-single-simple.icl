// Conformance: valid/operations-single-simple.icl
// Tests: Single simple operation with all required fields
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v31-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "3100310031003100310031003100310031003100310031003100310031003100"
  }

  PurposeStatement {
    narrative: "Tests a single simple operation definition",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      message: String
    },
    invariants: []
  }

  BehavioralSemantics {
    operations: [
      {
        name: "set_message",
        precondition: "true",
        parameters: {
          new_message: String
        },
        postcondition: "message equals new_message",
        side_effects: [],
        idempotence: "idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["manual"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }
}
