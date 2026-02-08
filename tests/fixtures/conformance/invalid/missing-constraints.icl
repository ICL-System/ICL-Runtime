// Conformance: invalid/missing-constraints.icl
// Tests: Contract without required ExecutionConstraints section
// Expected: MUST fail — MissingRequired("ExecutionConstraints")

Contract {
  Identity {
    stable_id: "ic-inv-04-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }

  PurposeStatement {
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: { value: String },
    invariants: []
  }

  BehavioralSemantics {
    operations: []
  }

  HumanMachineContract {
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }
}
