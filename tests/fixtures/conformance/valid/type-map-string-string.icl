// Conformance: valid/type-map-string-string.icl
// Tests: Map<String, String> collection type in state
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v20-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "2000200020002000200020002000200020002000200020002000200020002000"
  }

  PurposeStatement {
    narrative: "Tests Map with String keys and String values in state",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      config: Map<String, String>,
      headers: Map<String, String>,
      environment: Map<String, String>
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
