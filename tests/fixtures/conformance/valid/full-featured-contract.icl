// Conformance: valid/full-featured-contract.icl
// Tests: Uses every feature — all types, operations, extensions, comments
// Expected: MUST parse successfully

// A comprehensive contract exercising all ICL features
Contract {
  // === Identity ===
  Identity {
    stable_id: "ic-conf-v52-001",
    version: 42,
    created_timestamp: 2026-02-08T12:30:45Z,
    owner: "Full Feature Test Suite",
    semantic_hash: "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
  }

  // === Purpose ===
  PurposeStatement {
    narrative: "A comprehensive contract that exercises every single feature of the ICL specification including all primitive types, composite types, collection types, operations with parameters, extensions, and all sections",
    intent_source: "comprehensive_conformance_test",
    confidence_level: 0.95
  }

  // === Data ===
  DataSemantics {
    state: {
      // Primitive types
      name: String = "default",
      count: Integer = 0,
      ratio: Float = 1.0,
      active: Boolean = true,
      created_at: ISO8601,
      session_id: UUID,
      // Enum type
      status: Enum["draft", "pending", "active", "paused", "completed", "archived"],
      priority: Enum["low", "medium", "high"],
      // Object type
      profile: Object {
        display_name: String,
        email: String,
        age: Integer,
        verified: Boolean
      },
      // Array types
      tags: Array<String>,
      scores: Array<Integer>,
      thresholds: Array<Float>,
      // Map types
      metadata: Map<String, String>,
      counters: Map<String, Integer>,
      // Complex nested types
      entries: Array<Object {
        id: UUID,
        label: String,
        value: Float,
        active: Boolean
      }>,
      category_scores: Map<String, Array<Integer>>
    },
    invariants: [
      "count >= 0",
      "ratio > 0.0",
      "ratio <= 100.0",
      "name is not empty",
      "profile.age >= 0",
      "all scores >= 0",
      "status is valid enum value"
    ]
  }

  // === Behavior ===
  BehavioralSemantics {
    operations: [
      {
        name: "create_entry",
        precondition: "active is true AND status is not archived",
        parameters: {
          label: String,
          value: Float,
          entry_tags: Array<String>
        },
        postcondition: "new entry added to entries AND count incremented",
        side_effects: ["log_creation", "notify_subscribers", "update_index"],
        idempotence: "not_idempotent"
      },
      {
        name: "update_profile",
        precondition: "true",
        parameters: {
          new_name: String,
          new_email: String,
          new_age: Integer
        },
        postcondition: "profile fields updated",
        side_effects: ["audit_log"],
        idempotence: "idempotent"
      },
      {
        name: "change_status",
        precondition: "new_status is valid transition from current status",
        parameters: {
          new_status: String
        },
        postcondition: "status equals new_status",
        side_effects: ["log_transition", "notify_status_change"],
        idempotence: "idempotent"
      },
      {
        name: "reset_counters",
        precondition: "true",
        parameters: {},
        postcondition: "all counters are zero AND count is zero",
        side_effects: ["audit_log"],
        idempotence: "idempotent"
      },
      {
        name: "bulk_import",
        precondition: "active is true",
        parameters: {
          import_data: Array<Object {
            label: String,
            value: Float
          }>
        },
        postcondition: "all import items added to entries",
        side_effects: ["log_import", "update_index", "send_summary_report"],
        idempotence: "not_idempotent"
      }
    ]
  }

  // === Constraints ===
  ExecutionConstraints {
    trigger_types: ["manual", "api_call", "scheduled", "event_based"],
    resource_limits: {
      max_memory_bytes: 8388608,
      computation_timeout_ms: 10000,
      max_state_size_bytes: 4194304
    },
    external_permissions: [
      "database_read",
      "database_write",
      "email_service",
      "notification_service",
      "search_index"
    ],
    sandbox_mode: "network_allowed"
  }

  // === Human-Machine Contract ===
  HumanMachineContract {
    system_commitments: [
      "All operations are atomic — partial state changes never persist",
      "State transitions respect all declared invariants",
      "Deterministic execution is guaranteed for identical inputs",
      "Audit trail is maintained for every state-changing operation",
      "Resource limits are enforced strictly"
    ],
    system_refusals: [
      "Will not execute operations that would violate invariants",
      "Will not exceed declared resource limits",
      "Will not process input that fails type validation",
      "Will not allow invalid status transitions"
    ],
    user_obligations: [
      "Must provide correctly typed parameters for all operations",
      "Must authenticate before invoking operations",
      "Must respect rate limits",
      "Must handle error responses appropriately"
    ]
  }
}

// Extensions for system-specific configuration
Extensions {
  // Deployment configuration
  deployment {
    environment: "production",
    region: "us-west-2",
    replicas: "3",
    auto_scaling: "enabled"
  }

  // Observability configuration
  observability {
    tracing_enabled: "true",
    metrics_endpoint: "https://metrics.example.com/v2",
    log_level: "info",
    sampling_rate: "0.1"
  }

  // Feature management
  feature_flags {
    new_import_engine: "true",
    experimental_search: "false",
    beta_users: ["alice", "bob", "charlie"]
  }
}
