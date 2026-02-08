// Conformance: valid/identity-hex-hash.icl
// Tests: semantic_hash as a realistic 64-character SHA-256 hex digest
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v05-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  }

  PurposeStatement {
    narrative: "Tests that a realistic SHA-256 hex hash is accepted as semantic_hash",
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
