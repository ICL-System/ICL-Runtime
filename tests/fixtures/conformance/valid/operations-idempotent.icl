// Conformance: valid/operations-idempotent.icl
// Tests: All operations marked as idempotent
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v34-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "3400340034003400340034003400340034003400340034003400340034003400"
  }

  PurposeStatement {
    narrative: "Tests that all operations can be marked as idempotent",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      config_key: String,
      config_value: String
    },
    invariants: []
  }

  BehavioralSemantics {
    operations: [
      {
        name: "set_config",
        precondition: "true",
        parameters: {
          key: String,
          value: String
        },
        postcondition: "config_key and config_value updated",
        side_effects: [],
        idempotence: "idempotent"
      },
      {
        name: "get_config",
        precondition: "config_key is set",
        parameters: {},
        postcondition: "state unchanged",
        side_effects: [],
        idempotence: "idempotent"
      },
      {
        name: "reset_config",
        precondition: "true",
        parameters: {},
        postcondition: "config_key and config_value are empty",
        side_effects: [],
        idempotence: "idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["api_call"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: ["All operations are safe to retry"],
    system_refusals: [],
    user_obligations: []
  }
}
