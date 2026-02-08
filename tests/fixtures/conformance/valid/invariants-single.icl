// Conformance: valid/invariants-single.icl
// Tests: Single invariant in the invariants array
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v27-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "2700270027002700270027002700270027002700270027002700270027002700"
  }

  PurposeStatement {
    narrative: "Tests a single invariant in the invariants array",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      balance: Float = 0.0
    },
    invariants: [
      "balance >= 0.0"
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
