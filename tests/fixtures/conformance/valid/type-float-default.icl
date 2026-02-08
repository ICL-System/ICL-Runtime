// Conformance: valid/type-float-default.icl
// Tests: Float state fields with default values
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v12-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "1200120012001200120012001200120012001200120012001200120012001200"
  }

  PurposeStatement {
    narrative: "Tests Float state fields with default values",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      ratio: Float = 1.0,
      temperature: Float = 0.5,
      weight: Float = 99.99
    },
    invariants: [
      "ratio > 0.0",
      "temperature >= 0.0"
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
