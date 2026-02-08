// Conformance: valid/operations-empty-parameters.icl
// Tests: Operations with empty parameter lists
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v37-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "3700370037003700370037003700370037003700370037003700370037003700"
  }

  PurposeStatement {
    narrative: "Tests operations with empty parameter lists",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      counter: Integer = 0,
      active: Boolean = true
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
        side_effects: [],
        idempotence: "not_idempotent"
      },
      {
        name: "decrement",
        precondition: "counter > 0",
        parameters: {},
        postcondition: "counter decremented by 1",
        side_effects: [],
        idempotence: "not_idempotent"
      },
      {
        name: "toggle_active",
        precondition: "true",
        parameters: {},
        postcondition: "active is toggled",
        side_effects: [],
        idempotence: "not_idempotent"
      }
    ]
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
