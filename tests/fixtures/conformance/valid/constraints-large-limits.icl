// Conformance: valid/constraints-large-limits.icl
// Tests: Large resource limit values in ExecutionConstraints
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v41-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "4100410041004100410041004100410041004100410041004100410041004100"
  }

  PurposeStatement {
    narrative: "Tests large resource limit values",
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
    trigger_types: ["api_call"],
    resource_limits: {
      max_memory_bytes: 1073741824,
      computation_timeout_ms: 60000,
      max_state_size_bytes: 536870912
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
