// Conformance: valid/all-primitive-types.icl
// Tests: All primitive types (Integer, Float, String, Boolean, ISO8601, UUID) in state
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-test-types-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "1111111111111111"
  }

  PurposeStatement {
    narrative: "Contract testing all primitive types",
    intent_source: "conformance_test",
    confidence_level: 0.95
  }

  DataSemantics {
    state: {
      count: Integer = 0,
      ratio: Float = 1.0,
      label: String,
      active: Boolean,
      created_at: ISO8601,
      user_id: UUID
    },
    invariants: [
      "count >= 0",
      "ratio > 0.0",
      "label is not empty"
    ]
  }

  BehavioralSemantics {
    operations: [
      {
        name: "update_count",
        precondition: "count >= 0",
        parameters: {
          new_count: Integer
        },
        postcondition: "count updated",
        side_effects: ["log_update"],
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
    system_commitments: ["State is type-safe"],
    system_refusals: ["Will not accept wrong types"],
    user_obligations: ["Must provide correct types"]
  }
}
