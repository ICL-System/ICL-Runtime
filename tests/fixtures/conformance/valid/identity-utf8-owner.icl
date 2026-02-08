// Conformance: valid/identity-utf8-owner.icl
// Tests: owner field containing unicode/UTF-8 characters
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v04-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "José García-López — 日本語テスト",
    semantic_hash: "d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5"
  }

  PurposeStatement {
    narrative: "Tests that owner with UTF-8 characters is accepted",
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
