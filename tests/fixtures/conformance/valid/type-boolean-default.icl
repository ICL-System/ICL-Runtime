// Conformance: valid/type-boolean-default.icl
// Tests: Boolean state fields with default values (true and false)
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v14-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "1400140014001400140014001400140014001400140014001400140014001400"
  }

  PurposeStatement {
    narrative: "Tests Boolean state fields with default values",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      active: Boolean = true,
      archived: Boolean = false,
      visible: Boolean
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
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }
}
