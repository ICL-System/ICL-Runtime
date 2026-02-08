// Conformance: valid/type-integer-default.icl
// Tests: Integer state fields with default values
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v11-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "1100110011001100110011001100110011001100110011001100110011001100"
  }

  PurposeStatement {
    narrative: "Tests Integer state fields with default values",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      count: Integer = 0,
      max_retries: Integer = 3,
      threshold: Integer = 100
    },
    invariants: [
      "count >= 0",
      "max_retries > 0"
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
