// Conformance: valid/composite-types.icl
// Tests: Object, Enum, Array, Map composite types
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-test-composite-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "2222222222222222"
  }

  PurposeStatement {
    narrative: "Contract testing composite and collection types",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      status: Enum["pending", "active", "completed"],
      metadata: Object {
        key: String,
        value: String
      },
      tags: Array<String>,
      scores: Map<String, Integer>
    },
    invariants: [
      "status is valid enum value"
    ]
  }

  BehavioralSemantics {
    operations: []
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
