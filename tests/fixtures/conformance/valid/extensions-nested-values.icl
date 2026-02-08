// Conformance: valid/extensions-nested-values.icl
// Tests: Extensions with various value types (strings, arrays, numbers as strings)
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v49-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "4900490049004900490049004900490049004900490049004900490049004900"
  }

  PurposeStatement {
    narrative: "Tests Extensions with various value types",
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
  deployment_config {
    environment: "production",
    region: "us-east-1",
    replicas: "3",
    health_check_path: "/health",
    tags: ["mission-critical", "tier-1", "auto-scaled"],
    version_constraint: ">=1.0.0"
  }

  feature_flags {
    enable_new_ui: "false",
    enable_analytics: "true",
    rollout_percentage: "25",
    allowed_users: ["admin", "beta-tester", "developer"]
  }
}
