// Conformance: valid/invariants-multiple.icl
// Tests: Many invariants (5+) in the invariants array
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v28-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "2800280028002800280028002800280028002800280028002800280028002800"
  }

  PurposeStatement {
    narrative: "Tests many invariants in the invariants array",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      balance: Float = 0.0,
      count: Integer = 0,
      name: String,
      active: Boolean = true,
      max_limit: Integer = 1000
    },
    invariants: [
      "balance >= 0.0",
      "count >= 0",
      "count <= max_limit",
      "name is not empty",
      "active is boolean",
      "max_limit > 0",
      "balance is finite"
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
