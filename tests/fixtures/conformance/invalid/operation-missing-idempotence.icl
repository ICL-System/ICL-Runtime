// Conformance: invalid/operation-missing-idempotence.icl
// Tests: Operation definition without required idempotence field
// Expected: MUST fail — MissingField("Operation", "idempotence")

Contract {
  Identity {
    stable_id: "ic-inv-45-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }

  PurposeStatement {
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: { value: String },
    invariants: []
  }

  BehavioralSemantics {
    operations: [
      {
        name: "do_something",
        precondition: "true",
        postcondition: "true",
        parameters: [],
        side_effects: []
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
