// Conformance: valid/constraints-multiple-triggers.icl
// Tests: Multiple trigger types in trigger_types array
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v40-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "4000400040004000400040004000400040004000400040004000400040004000"
  }

  PurposeStatement {
    narrative: "Tests multiple trigger types in ExecutionConstraints",
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
    trigger_types: ["manual", "api_call", "scheduled", "event_based", "webhook"],
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
