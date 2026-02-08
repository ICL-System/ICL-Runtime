// Conformance: valid/whitespace-flexible.icl
// Tests: Extra whitespace and blank lines between sections
// Expected: MUST parse successfully


Contract {

  Identity {

    stable_id: "ic-conf-v51-001",

    version: 1,

    created_timestamp: 2026-02-08T00:00:00Z,

    owner: "conformance-suite",

    semantic_hash: "5100510051005100510051005100510051005100510051005100510051005100"

  }


  PurposeStatement {

    narrative: "Tests that extra whitespace and blank lines are accepted",

    intent_source: "conformance_test",

    confidence_level: 1.0

  }


  DataSemantics {

    state: {

      value: String

    },

    invariants: []

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
