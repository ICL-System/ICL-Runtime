// Conformance: invalid/sandbox-mode-integer.icl
// Tests: sandbox_mode field with integer value instead of string
// Expected: MUST fail — TypeError("sandbox_mode", expected: String, got: Integer)

Contract {
  Identity {
    stable_id: "ic-inv-21-001",
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
    sandbox_mode: 1
  }

  HumanMachineContract {
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }
}
