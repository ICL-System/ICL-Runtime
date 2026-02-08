// Conformance: valid/type-array-of-strings.icl
// Tests: Array<String> collection type in state
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v17-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "1700170017001700170017001700170017001700170017001700170017001700"
  }

  PurposeStatement {
    narrative: "Tests Array of String collection type in state",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      tags: Array<String>,
      labels: Array<String>,
      categories: Array<String>
    },
    invariants: [
      "tags has no duplicates"
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
