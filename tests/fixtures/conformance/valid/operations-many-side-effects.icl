// Conformance: valid/operations-many-side-effects.icl
// Tests: Operations with multiple side effects listed
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v36-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "3600360036003600360036003600360036003600360036003600360036003600"
  }

  PurposeStatement {
    narrative: "Tests operations with many side effects",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      order_id: UUID,
      status: Enum["created", "processing", "shipped", "delivered"]
    },
    invariants: []
  }

  BehavioralSemantics {
    operations: [
      {
        name: "process_order",
        precondition: "status equals created",
        parameters: {},
        postcondition: "status equals processing",
        side_effects: [
          "send_confirmation_email",
          "update_inventory",
          "charge_payment",
          "create_shipping_label",
          "notify_warehouse",
          "update_analytics",
          "log_state_change"
        ],
        idempotence: "not_idempotent"
      },
      {
        name: "ship_order",
        precondition: "status equals processing",
        parameters: {},
        postcondition: "status equals shipped",
        side_effects: [
          "send_shipping_notification",
          "update_tracking",
          "notify_customer",
          "update_warehouse_inventory",
          "log_state_change"
        ],
        idempotence: "not_idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["api_call", "event_based"],
    resource_limits: {
      max_memory_bytes: 4194304,
      computation_timeout_ms: 5000,
      max_state_size_bytes: 2097152
    },
    external_permissions: ["email_service", "payment_gateway", "shipping_api"],
    sandbox_mode: "network_allowed"
  }

  HumanMachineContract {
    system_commitments: ["All side effects are executed atomically"],
    system_refusals: ["Will not process order if payment fails"],
    user_obligations: ["Must provide valid payment information"]
  }
}
