// Conformance: valid/purpose-various-sources.icl
// Tests: Different valid intent_source values
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v10-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3"
  }

  PurposeStatement {
    narrative: "Tests that various intent_source values are accepted",
    intent_source: "user_defined_workflow",
    confidence_level: 0.72
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
