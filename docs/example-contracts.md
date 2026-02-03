# ICL — Example Contracts

**Purpose:** Show how to write valid, portable ICL contracts that any system can execute.

> These examples demonstrate Core ICL (Sections 1-6). They are implementation-agnostic.

---

## Example 1: Database Write Validation

A contract that specifies constraints for database operations.

```icl
Contract {
  Identity {
    stable_id: "ic-db-write-001",
    version: 1,
    created_timestamp: 2026-01-31T10:00:00Z,
    owner: "developer-team"
  }

  PurposeStatement {
    narrative: "Validate database writes before execution",
    intent_source: "developer_specification",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      table_name: String,
      column_name: String,
      write_type: Enum["insert", "update"],
      value_constraint: Object {
        type: String,
        min_length: Integer,
        max_length: Integer,
        pattern: String
      },
      permission_required: Boolean
    },
    invariants: [
      "table_name is not empty",
      "column_type is valid SQL type",
      "min_length <= max_length",
      "permission_required is boolean"
    ]
  }

  BehavioralSemantics {
    operations: [
      {
        name: "validate_write",
        precondition: "write_request_received",
        parameters: {
          table: String,
          column: String,
          value: String,
          user_id: UUID
        },
        postcondition: "validation_result_returned AND decision_logged",
        side_effects: ["log_validation_attempt"],
        idempotence: "idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["event_based"],
    resource_limits: {
      max_memory_bytes: 10485760,
      computation_timeout_ms: 50,
      max_state_size_bytes: 1048576
    },
    external_permissions: ["database_query"],
    sandbox_mode: "restricted"
  }

  HumanMachineContract {
    system_commitments: [
      "Every write is validated before execution",
      "Validation result is deterministic",
      "Failed validations prevent writes",
      "All decisions are logged immutably"
    ],
    system_refusals: [
      "Will not allow invalid writes",
      "Will not silently drop writes",
      "Will not modify validation rules without explicit update"
    ],
    user_obligations: [
      "May update validation constraints",
      "May review validation logs",
      "Must provide user_id for audit trail"
    ]
  }
}
```

---

## Example 2: API Rate Limiting

A contract that enforces rate limits on API endpoints.

```icl
Contract {
  Identity {
    stable_id: "ic-ratelimit-001",
    version: 1,
    created_timestamp: 2026-01-31T10:15:00Z,
    owner: "platform-team"
  }

  PurposeStatement {
    narrative: "Enforce rate limits on API endpoints",
    intent_source: "platform_specification",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      endpoint: String,
      requests_per_minute: Integer = 60,
      requests_per_hour: Integer = 1000,
      burst_limit: Integer = 10,
      current_window: Object {
        minute_count: Integer,
        hour_count: Integer,
        last_reset: ISO8601
      }
    },
    invariants: [
      "requests_per_minute > 0",
      "requests_per_hour > requests_per_minute",
      "burst_limit <= requests_per_minute",
      "minute_count <= requests_per_minute",
      "hour_count <= requests_per_hour"
    ]
  }

  BehavioralSemantics {
    operations: [
      {
        name: "check_rate_limit",
        precondition: "api_request_received",
        parameters: {
          endpoint: String,
          user_id: UUID,
          timestamp: ISO8601
        },
        postcondition: "limit_decision_returned AND state_updated",
        side_effects: ["increment_counters", "reset_if_window_expired"],
        idempotence: "idempotent"
      },
      {
        name: "reset_window",
        trigger: "time_based",
        schedule: "every_minute",
        computation: "check_if_minute_window_expired_and_reset",
        postcondition: "minute_counter_reset_or_preserved"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["event_based", "time_based"],
    resource_limits: {
      max_memory_bytes: 5242880,
      computation_timeout_ms: 10,
      max_state_size_bytes: 524288
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: [
      "Rate limits enforced deterministically",
      "Windows reset at precise boundaries",
      "Burst limits prevent spike attacks",
      "All decisions are logged"
    ],
    system_refusals: [
      "Will not allow above-limit requests",
      "Will not drift window boundaries",
      "Will not lose count state"
    ],
    user_obligations: [
      "May configure rate limits",
      "May whitelist trusted endpoints",
      "Must respect 429 responses"
    ]
  }
}
```

---

## Example 3: Agent Action Verification

A contract that verifies AI agent actions before execution.

```icl
Contract {
  Identity {
    stable_id: "ic-agent-verify-001",
    version: 1,
    created_timestamp: 2026-01-31T10:30:00Z,
    owner: "agent-framework-team"
  }

  PurposeStatement {
    narrative: "Verify AI agent actions against policy before execution",
    intent_source: "safety_specification",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      action_type: Enum["read", "write", "delete", "external_call"],
      resource_type: String,
      resource_id: String,
      agent_id: UUID,
      policy: Object {
        allowed_actions: Array<String>,
        forbidden_resources: Array<String>,
        max_batch_size: Integer,
        requires_confirmation: Boolean
      }
    },
    invariants: [
      "action_type is valid",
      "resource_type is not empty",
      "agent_id is uuid",
      "max_batch_size > 0",
      "no_resource_both_allowed_and_forbidden"
    ]
  }

  BehavioralSemantics {
    operations: [
      {
        name: "verify_action",
        precondition: "agent_action_proposed",
        parameters: {
          action: String,
          resource_type: String,
          resource_id: String,
          agent_id: UUID
        },
        postcondition: "verification_result_returned AND decision_logged",
        side_effects: ["log_verification_decision", "trigger_alert_if_denied"],
        idempotence: "idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["event_based"],
    resource_limits: {
      max_memory_bytes: 10485760,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    external_permissions: ["policy_lookup", "audit_logging"],
    sandbox_mode: "restricted"
  }

  HumanMachineContract {
    system_commitments: [
      "All agent actions verified against policy",
      "Dangerous actions blocked deterministically",
      "All decisions logged immutably",
      "Policy violations trigger alerts immediately"
    ],
    system_refusals: [
      "Will not allow policy violations",
      "Will not silently execute forbidden actions",
      "Will not lose audit trail"
    ],
    user_obligations: [
      "May update policies",
      "Must review denied action alerts",
      "May add agents to whitelist"
    ]
  }
}
```

---

## Example 4: Code Verification

A contract that verifies code changes meet standards before merge.

```icl
Contract {
  Identity {
    stable_id: "ic-code-verify-001",
    version: 1,
    created_timestamp: 2026-01-31T10:45:00Z,
    owner: "devops-team"
  }

  PurposeStatement {
    narrative: "Verify code changes meet quality and security standards",
    intent_source: "ci_cd_specification",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      pull_request: Object {
        id: String,
        from_branch: String,
        to_branch: String,
        files_changed: Array<String>
      },
      checks: Object {
        syntax_valid: Boolean,
        tests_passing: Boolean,
        coverage_above_threshold: Boolean,
        no_security_warnings: Boolean,
        style_compliant: Boolean
      }
    },
    invariants: [
      "all_checks_are_boolean",
      "pr_id_is_not_empty",
      "file_list_is_not_empty"
    ]
  }

  BehavioralSemantics {
    operations: [
      {
        name: "verify_pr",
        precondition: "pull_request_opened",
        parameters: {
          pr_id: String,
          files: Array<String>
        },
        postcondition: "verification_complete AND result_posted_to_pr",
        side_effects: ["run_linter", "run_tests", "check_coverage", "scan_security"],
        idempotence: "idempotent"
      },
      {
        name: "merge_if_passed",
        precondition: "all_checks_passed",
        postcondition: "pr_merged OR blocked_if_failed",
        side_effects: ["update_repository"]
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["event_based"],
    resource_limits: {
      max_memory_bytes: 2147483648,
      computation_timeout_ms: 600000,
      max_state_size_bytes: 104857600
    },
    external_permissions: ["git_access", "ci_cd_runner"],
    sandbox_mode: "restricted"
  }

  HumanMachineContract {
    system_commitments: [
      "All changes verified before merge",
      "Verification is deterministic",
      "Failed checks prevent merge",
      "All results are logged"
    ],
    system_refusals: [
      "Will not merge failing code",
      "Will not skip security checks",
      "Will not lose verification results"
    ],
    user_obligations: [
      "May update standards",
      "Must fix failing checks",
      "May request waiver (recorded in provenance)"
    ]
  }
}
```

---

## Common Patterns

All example contracts demonstrate:

1. **Determinism** — Same input → same output
2. **Portability** — No system-specific syntax 
3. **Clarity** — Explicit preconditions, postconditions, invariants
4. **Auditability** — All decisions logged
5. **Verification** — Testable guarantees

Use these as templates for your own contracts.

---

## Key Rules for ICL Contracts

1. **No side effects outside ExecutionConstraints** — Document all effects
2. **No randomness** — All operations deterministic
3. **No external state dependency** — All inputs explicit
4. **No infinite loops** — Computation timeout enforced
5. **No implicit type coercion** — Types strict

Violating these rules means the contract is not valid ICL.
