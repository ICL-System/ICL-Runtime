// Conformance: valid/operations-multiple-varied.icl
// Tests: Multiple operations with different properties and characteristics
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v32-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "3200320032003200320032003200320032003200320032003200320032003200"
  }

  PurposeStatement {
    narrative: "Tests multiple operations with varied properties",
    intent_source: "conformance_test",
    confidence_level: 0.95
  }

  DataSemantics {
    state: {
      name: String,
      count: Integer = 0,
      items: Array<String>
    },
    invariants: [
      "count >= 0"
    ]
  }

  BehavioralSemantics {
    operations: [
      {
        name: "set_name",
        precondition: "name is changeable",
        parameters: {
          new_name: String
        },
        postcondition: "name equals new_name",
        side_effects: ["audit_log"],
        idempotence: "idempotent"
      },
      {
        name: "increment_count",
        precondition: "count < max_int",
        parameters: {},
        postcondition: "count incremented by 1",
        side_effects: [],
        idempotence: "not_idempotent"
      },
      {
        name: "add_item",
        precondition: "item not in items",
        parameters: {
          item: String
        },
        postcondition: "item added to items",
        side_effects: ["notify_subscribers"],
        idempotence: "not_idempotent"
      },
      {
        name: "reset",
        precondition: "true",
        parameters: {},
        postcondition: "count is 0 AND items is empty",
        side_effects: ["audit_log", "notify_subscribers"],
        idempotence: "idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["manual", "api_call"],
    resource_limits: {
      max_memory_bytes: 2097152,
      computation_timeout_ms: 500,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: ["Operations are atomic"],
    system_refusals: [],
    user_obligations: []
  }
}
