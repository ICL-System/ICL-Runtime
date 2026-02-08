// Conformance: valid/multiple-operations.icl
// Tests: Contract with multiple operations in BehavioralSemantics
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-test-multi-ops-001",
    version: 2,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "4444444444444444"
  }

  PurposeStatement {
    narrative: "Contract with multiple operations",
    intent_source: "conformance_test",
    confidence_level: 0.99
  }

  DataSemantics {
    state: {
      items: Array<String>,
      count: Integer = 0
    },
    invariants: [
      "count >= 0",
      "count equals length of items"
    ]
  }

  BehavioralSemantics {
    operations: [
      {
        name: "add_item",
        precondition: "item_not_duplicate",
        parameters: {
          item: String
        },
        postcondition: "item_added AND count_incremented",
        side_effects: ["log_addition"],
        idempotence: "not_idempotent"
      },
      {
        name: "remove_item",
        precondition: "item_exists",
        parameters: {
          item: String
        },
        postcondition: "item_removed AND count_decremented",
        side_effects: ["log_removal"],
        idempotence: "idempotent"
      },
      {
        name: "clear_all",
        precondition: "items_not_empty",
        parameters: {},
        postcondition: "items_empty AND count_zero",
        side_effects: ["log_clear"],
        idempotence: "idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["manual", "event_based"],
    resource_limits: {
      max_memory_bytes: 2097152,
      computation_timeout_ms: 200,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: [
      "Items are managed correctly",
      "Count always matches item list length"
    ],
    system_refusals: [
      "Will not allow duplicate items",
      "Will not allow negative count"
    ],
    user_obligations: [
      "May add or remove items",
      "May clear all items"
    ]
  }
}
