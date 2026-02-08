// Conformance: valid/type-object-nested.icl
// Tests: Object composite type with nested fields in state
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v21-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "2100210021002100210021002100210021002100210021002100210021002100"
  }

  PurposeStatement {
    narrative: "Tests Object composite type with nested fields in state",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      address: Object {
        street: String,
        city: String,
        zip_code: String,
        country: String
      },
      profile: Object {
        first_name: String,
        last_name: String,
        age: Integer
      }
    },
    invariants: [
      "address.zip_code is not empty",
      "profile.age >= 0"
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
