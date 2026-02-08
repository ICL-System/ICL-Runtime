// Conformance: valid/type-uuid-field.icl
// Tests: UUID typed state fields
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v16-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "1600160016001600160016001600160016001600160016001600160016001600"
  }

  PurposeStatement {
    narrative: "Tests UUID typed state fields",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      user_id: UUID,
      session_id: UUID,
      correlation_id: UUID
    },
    invariants: [
      "user_id is not null",
      "session_id is not null"
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
