// Conformance: valid/operations-with-parameters.icl
// Tests: Operations with various typed parameters
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v33-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "3300330033003300330033003300330033003300330033003300330033003300"
  }

  PurposeStatement {
    narrative: "Tests operations with various typed parameters",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      name: String,
      score: Integer = 0,
      rating: Float = 0.0,
      active: Boolean = false
    },
    invariants: []
  }

  BehavioralSemantics {
    operations: [
      {
        name: "update_profile",
        precondition: "true",
        parameters: {
          new_name: String,
          new_score: Integer,
          new_rating: Float,
          is_active: Boolean
        },
        postcondition: "all fields updated",
        side_effects: ["log_update"],
        idempotence: "idempotent"
      },
      {
        name: "batch_update",
        precondition: "active is true",
        parameters: {
          items: Array<String>,
          multiplier: Float,
          tags: Map<String, String>
        },
        postcondition: "batch processing complete",
        side_effects: ["log_batch"],
        idempotence: "not_idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["api_call"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 200,
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
