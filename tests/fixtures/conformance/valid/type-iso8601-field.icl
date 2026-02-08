// Conformance: valid/type-iso8601-field.icl
// Tests: ISO8601 typed state fields
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v15-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "1500150015001500150015001500150015001500150015001500150015001500"
  }

  PurposeStatement {
    narrative: "Tests ISO8601 typed state fields",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      created_at: ISO8601,
      updated_at: ISO8601,
      expires_at: ISO8601
    },
    invariants: [
      "created_at is before updated_at",
      "expires_at is after created_at"
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
