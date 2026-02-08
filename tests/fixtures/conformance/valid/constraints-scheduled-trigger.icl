// Conformance: valid/constraints-scheduled-trigger.icl
// Tests: trigger_types with scheduled trigger
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v39-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "3900390039003900390039003900390039003900390039003900390039003900"
  }

  PurposeStatement {
    narrative: "Tests scheduled trigger type in ExecutionConstraints",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      last_run: ISO8601
    },
    invariants: []
  }

  BehavioralSemantics {
    operations: []
  }

  ExecutionConstraints {
    trigger_types: ["scheduled"],
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
