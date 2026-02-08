// Conformance: invalid/confidence-integer.icl
// Tests: confidence_level field with integer value instead of float
// Expected: MUST fail — TypeError("confidence_level", expected: Float, got: Integer)

Contract {
  Identity {
    stable_id: "ic-inv-17-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }

  PurposeStatement {
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1
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
