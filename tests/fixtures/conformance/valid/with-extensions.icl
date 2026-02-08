// Conformance: valid/with-extensions.icl
// Tests: Contract with Extensions section (optional, must be accepted)
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-test-ext-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "3333333333333333"
  }

  PurposeStatement {
    narrative: "Contract with system-specific extensions",
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
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }
}

Extensions {
  custom_system {
    priority: "high",
    tags: ["experimental", "beta"]
  }
}
