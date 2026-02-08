// Conformance: valid/hmc-commitments-only.icl
// Tests: HumanMachineContract with only system_commitments populated
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v44-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "4400440044004400440044004400440044004400440044004400440044004400"
  }

  PurposeStatement {
    narrative: "Tests HumanMachineContract with only system_commitments",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      value: String
    },
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
    system_commitments: [
      "Data is encrypted at rest",
      "All operations are logged",
      "State changes are atomic",
      "Deterministic execution guaranteed"
    ],
    system_refusals: [],
    user_obligations: []
  }
}
