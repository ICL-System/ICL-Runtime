// Conformance: invalid/timestamp-string.icl
// Tests: Completely invalid token where timestamp expected
// Expected: MUST fail — SyntaxError(invalid timestamp token)

Contract {
  Identity {
    stable_id: "ic-inv-18-001",
    version: 1,
    created_timestamp: @not_a_timestamp,
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
