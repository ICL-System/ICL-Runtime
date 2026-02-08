// Conformance: valid/purpose-zero-confidence.icl
// Tests: confidence_level set to 0.0 (minimum boundary)
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v07-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0"
  }

  PurposeStatement {
    narrative: "Tests that confidence_level of 0.0 is valid",
    intent_source: "conformance_test",
    confidence_level: 0.0
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
