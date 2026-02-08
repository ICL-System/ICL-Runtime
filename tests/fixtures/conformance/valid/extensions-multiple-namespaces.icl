// Conformance: valid/extensions-multiple-namespaces.icl
// Tests: Extensions section with multiple distinct namespaces
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v48-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "4800480048004800480048004800480048004800480048004800480048004800"
  }

  PurposeStatement {
    narrative: "Tests Extensions section with multiple namespaces",
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
  logging_system {
    log_level: "debug",
    log_format: "json",
    log_destination: "stdout"
  }

  caching_layer {
    enabled: "true",
    ttl_seconds: "300",
    max_entries: "10000",
    eviction_policy: "lru"
  }

  rate_limiter {
    requests_per_second: "100",
    burst_size: "50",
    penalty_seconds: "60"
  }
}
