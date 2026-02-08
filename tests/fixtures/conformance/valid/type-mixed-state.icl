// Conformance: valid/type-mixed-state.icl
// Tests: State with many different types combined (primitives, composites, collections)
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v25-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "2500250025002500250025002500250025002500250025002500250025002500"
  }

  PurposeStatement {
    narrative: "Tests state with a mix of all supported types",
    intent_source: "conformance_test",
    confidence_level: 0.99
  }

  DataSemantics {
    state: {
      name: String = "default",
      count: Integer = 0,
      ratio: Float = 1.0,
      active: Boolean = true,
      created_at: ISO8601,
      user_id: UUID,
      status: Enum["pending", "active", "completed"],
      tags: Array<String>,
      scores: Map<String, Integer>,
      metadata: Object {
        key: String,
        value: String,
        version: Integer
      }
    },
    invariants: [
      "count >= 0",
      "ratio > 0.0",
      "name is not empty"
    ]
  }

  BehavioralSemantics {
    operations: []
  }

  ExecutionConstraints {
    trigger_types: ["manual"],
    resource_limits: {
      max_memory_bytes: 2097152,
      computation_timeout_ms: 200,
      max_state_size_bytes: 2097152
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: ["All types are validated"],
    system_refusals: [],
    user_obligations: []
  }
}
