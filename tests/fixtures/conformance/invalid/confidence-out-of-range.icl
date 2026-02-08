// Conformance: invalid/confidence-out-of-range.icl
// Tests: confidence_level must be in [0.0, 1.0]
// Expected: MUST fail with ValidationError("confidence_level out of range")

Contract {
  Identity {
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }

  PurposeStatement {
    narrative: "Contract with invalid confidence",
    intent_source: "test",
    confidence_level: 2.5
  }

  DataSemantics {
    state: {},
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
