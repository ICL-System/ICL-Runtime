// Conformance: invalid/unknown-section.icl
// Tests: Unknown top-level section inside Contract
// Expected: MUST fail with UnknownField("FakeSection")

Contract {
  Identity {
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }

  FakeSection {
    something: "invalid"
  }

  PurposeStatement {
    narrative: "Contract with unknown section",
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
