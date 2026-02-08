// Conformance: valid/type-array-of-objects.icl
// Tests: Array of Object composite type in state
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v23-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "2300230023002300230023002300230023002300230023002300230023002300"
  }

  PurposeStatement {
    narrative: "Tests Array of Object composite type in state",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      items: Array<Object {
        name: String,
        quantity: Integer,
        price: Float
      }>,
      logs: Array<Object {
        timestamp: ISO8601,
        message: String
      }>
    },
    invariants: [
      "all item quantities >= 0",
      "all item prices >= 0.0"
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
