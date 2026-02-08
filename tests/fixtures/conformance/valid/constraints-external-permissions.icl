// Conformance: valid/constraints-external-permissions.icl
// Tests: Non-empty external_permissions array in ExecutionConstraints
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v42-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "4200420042004200420042004200420042004200420042004200420042004200"
  }

  PurposeStatement {
    narrative: "Tests external_permissions with multiple permission entries",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      data: String
    },
    invariants: []
  }

  BehavioralSemantics {
    operations: [
      {
        name: "fetch_data",
        precondition: "true",
        parameters: {},
        postcondition: "data is populated",
        side_effects: ["network_request"],
        idempotence: "idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["api_call"],
    resource_limits: {
      max_memory_bytes: 4194304,
      computation_timeout_ms: 10000,
      max_state_size_bytes: 2097152
    },
    external_permissions: [
      "network_access",
      "database_read",
      "database_write",
      "file_system_read",
      "email_send",
      "webhook_call"
    ],
    sandbox_mode: "network_allowed"
  }

  HumanMachineContract {
    system_commitments: ["Only accesses explicitly permitted resources"],
    system_refusals: ["Will not access unpermitted resources"],
    user_obligations: ["Must grant required permissions"]
  }
}
