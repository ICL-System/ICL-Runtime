// Conformance: valid/invariants-complex-expressions.icl
// Tests: Complex invariant expression strings
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v29-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "2900290029002900290029002900290029002900290029002900290029002900"
  }

  PurposeStatement {
    narrative: "Tests complex invariant expression strings",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      balance: Float = 0.0,
      items: Array<String>,
      count: Integer = 0,
      max_items: Integer = 100
    },
    invariants: [
      "balance >= 0.0 AND balance <= 1000000.0",
      "count == length(items)",
      "count >= 0 AND count <= max_items",
      "IF active THEN count > 0",
      "FOR ALL item IN items: item IS NOT empty",
      "max_items > 0 AND max_items <= 10000"
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
