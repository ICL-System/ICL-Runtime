// Conformance: valid/hmc-full.icl
// Tests: HumanMachineContract with all three lists populated with multiple items
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v45-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "4500450045004500450045004500450045004500450045004500450045004500"
  }

  PurposeStatement {
    narrative: "Tests HumanMachineContract with all lists fully populated",
    intent_source: "conformance_test",
    confidence_level: 0.98
  }

  DataSemantics {
    state: {
      data: String,
      user_id: UUID
    },
    invariants: []
  }

  BehavioralSemantics {
    operations: []
  }

  ExecutionConstraints {
    trigger_types: ["manual", "api_call"],
    resource_limits: {
      max_memory_bytes: 2097152,
      computation_timeout_ms: 500,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: [
      "All data is validated before processing",
      "State transitions are atomic and reversible",
      "Audit trail is maintained for all operations",
      "Data is encrypted in transit and at rest",
      "Response time is within computation_timeout_ms"
    ],
    system_refusals: [
      "Will not process malformed input",
      "Will not exceed resource limits",
      "Will not bypass access controls",
      "Will not expose internal state to unauthorized users"
    ],
    user_obligations: [
      "Must authenticate before invoking operations",
      "Must provide valid input for all parameters",
      "Must not attempt to exceed rate limits",
      "Must acknowledge terms of service",
      "Must report any security vulnerabilities found"
    ]
  }
}
