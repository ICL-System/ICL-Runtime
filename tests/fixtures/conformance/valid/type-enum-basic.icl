// Conformance: valid/type-enum-basic.icl
// Tests: Basic Enum composite type with multiple variants
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v22-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "2200220022002200220022002200220022002200220022002200220022002200"
  }

  PurposeStatement {
    narrative: "Tests basic Enum composite type with multiple variants",
    intent_source: "conformance_test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      status: Enum["draft", "pending", "active", "archived"],
      priority: Enum["low", "medium", "high", "critical"],
      color: Enum["red", "green", "blue"]
    },
    invariants: [
      "status is valid enum value",
      "priority is valid enum value"
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
