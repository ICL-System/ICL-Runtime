// Conformance: valid/identity-timestamp-variants.icl
// Tests: Various valid ISO8601 timestamp formats in created_timestamp
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v06-001",
    version: 1,
    created_timestamp: 2025-12-31T23:59:59Z,
    owner: "conformance-suite",
    semantic_hash: "f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1"
  }

  PurposeStatement {
    narrative: "Tests that different valid ISO8601 timestamps are accepted",
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
