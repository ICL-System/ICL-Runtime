// Conformance: valid/extensions-single-namespace.icl
// Tests: Extensions section with a single namespace
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v47-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "4700470047004700470047004700470047004700470047004700470047004700"
  }

  PurposeStatement {
    narrative: "Tests Extensions section with a single namespace",
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

Extensions {
  monitoring_system {
    enabled: "true",
    endpoint: "https://metrics.example.com/api/v1",
    interval_seconds: "30",
    alert_threshold: "0.95"
  }
}
