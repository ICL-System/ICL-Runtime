// Conformance: valid/purpose-long-narrative.icl
// Tests: narrative field approaching the 500-character limit
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v09-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2c2"
  }

  PurposeStatement {
    narrative: "This is a long narrative string that approaches the 500-character limit for the narrative field. It describes in detail the purpose and scope of this contract, including its relationship to the broader system architecture. The contract is designed to validate that the parser correctly handles narrative strings of significant length without truncation or error. Additional context includes description of use cases, stakeholder requirements, and operational constraints that inform the contract design decisions.",
    intent_source: "conformance_test",
    confidence_level: 0.85
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
