// Conformance: valid/hmc-long-strings.icl
// Tests: HumanMachineContract with long commitment/refusal/obligation strings
// Expected: MUST parse successfully

Contract {
  Identity {
    stable_id: "ic-conf-v46-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "conformance-suite",
    semantic_hash: "4600460046004600460046004600460046004600460046004600460046004600"
  }

  PurposeStatement {
    narrative: "Tests HumanMachineContract with long string values",
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
    system_commitments: [
      "The system commits to maintaining full data integrity across all state transitions, ensuring that no data loss occurs during any operation execution, and that all intermediate states are recoverable in the event of a failure or interruption",
      "The system guarantees deterministic behavior such that given identical inputs and initial state, the output and final state will always be exactly the same regardless of execution environment or timing"
    ],
    system_refusals: [
      "The system will refuse to execute any operation that would result in a state that violates any declared invariant, and will return a detailed error message explaining which invariant would be violated and why the operation cannot proceed",
      "The system will not process any request that exceeds the declared resource limits including memory, computation time, and state size, and will terminate such requests gracefully"
    ],
    user_obligations: [
      "The user is obligated to ensure all input data conforms to the declared type specifications before submitting operations, including proper formatting of dates, valid enumeration values, and adherence to all stated constraints on numeric ranges and string lengths",
      "The user must maintain awareness of the contract version and ensure compatibility between their client implementation and the contract version they are interacting with to prevent undefined behavior"
    ]
  }
}
