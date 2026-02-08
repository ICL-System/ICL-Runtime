// Conformance: invalid/wrong-version-type.icl
// Tests: Type error — version should be Integer, not String
// Expected: MUST fail with TypeError("version", expected: Integer, found: String)

Contract {
  Identity {
    stable_id: "ic-test-001",
    version: "not_an_integer",
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }

  PurposeStatement {
    narrative: "Contract with wrong type",
    intent_source: "test",
    confidence_level: 1.0
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
