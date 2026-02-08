// Conformance: invalid/missing-identity.icl
// Tests: Contract without required Identity section
// Expected: MUST fail with MissingRequired("Identity")

Contract {
  PurposeStatement {
    narrative: "Contract missing identity",
    intent_source: "test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {},
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
