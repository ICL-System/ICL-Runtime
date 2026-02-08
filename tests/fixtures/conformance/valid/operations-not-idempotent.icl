// Conformance: valid/operations-not-idempotent.icl
// Tests: Operations marked as not_idempotent
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v35-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "3500350035003500350035003500350035003500350035003500350035003500"
  }

  PurposeStatement {
    narrative: "Tests operations marked as not_idempotent",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      counter: Integer = 0,
      log_entries: Array<String>
    },
    invariants: [
      "counter >= 0"
    ]
  }

  BehavioralSemantics {
    operations: [
      {
        name: "increment",
        precondition: "counter < max_int",
        parameters: {},
        postcondition: "counter incremented by 1",
        side_effects: ["emit_event"],
        idempotence: "not_idempotent"
      },
      {
        name: "append_log",
        precondition: "true",
        parameters: {
          entry: String
        },
        postcondition: "entry appended to log_entries",
        side_effects: ["write_to_disk"],
        idempotence: "not_idempotent"
      },
      {
        name: "generate_id",
        precondition: "true",
        parameters: {},
        postcondition: "new unique id generated",
        side_effects: ["id_allocated"],
        idempotence: "not_idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["api_call"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: ["Operations are not safe to retry blindly"],
    system_refusals: [],
    user_obligations: ["Must handle duplicate execution"]
  }
}
