# ICL Getting Started Guide

**Purpose:** Help developers get started with Intent Contract Language quickly.

---

## What is ICL?

Intent Contract Language (ICL) is a formal specification language for declaring intent contracts in a machine-readable, deterministic, and verifiable format.

**Key properties:**
- **Deterministic**: Same input always produces identical output
- **Portable**: Works across Python, JavaScript, Go, Rust, etc.
- **Verifiable**: Formal proofs of execution correctness
- **Canonical**: Single normalized form for all contracts

---

## Installation

### Python

```bash
pip install icl-runtime
```

```python
from icl import Contract, parse_contract, normalize, verify, execute

# Load contract
contract_text = open("my_contract.icl").read()
contract = parse_contract(contract_text)

# Verify before execution
errors = verify(contract)
if errors:
    print(f"Contract invalid: {errors}")

# Normalize to canonical form
canonical = normalize(contract)

# Execute contract
result = execute(canonical, inputs={"action": "log_intake", "volume_ml": 250})
print(result)
```

### JavaScript/Node.js

```bash
npm install icl-runtime
```

```javascript
const { parseContract, normalize, verify, execute } = require('icl-runtime');

// Load contract
const contractText = require('fs').readFileSync('my_contract.icl', 'utf-8');
const contract = parseContract(contractText);

// Verify
const errors = verify(contract);
if (errors.length > 0) {
  console.error(`Contract invalid: ${errors}`);
}

// Normalize
const canonical = normalize(contract);

// Execute
const result = execute(canonical, { action: 'log_intake', volume_ml: 250 });
console.log(result);
```

### Go

```bash
go get icl-runtime
```

```go
package main

import (
  icl "icl-runtime"
)

func main() {
  // Load contract
  contractText, _ := ioutil.ReadFile("my_contract.icl")
  contract, _ := icl.ParseContract(string(contractText))
  
  // Verify
  errors := icl.Verify(contract)
  if len(errors) > 0 {
    log.Fatalf("Contract invalid: %v", errors)
  }
  
  // Normalize
  canonical, _ := icl.Normalize(contract)
  
  // Execute
  result, _ := icl.Execute(canonical, map[string]interface{}{
    "action": "log_intake",
    "volume_ml": 250,
  })
  fmt.Println(result)
}
```

### Rust

```toml
[dependencies]
icl-runtime = "0.1"
```

```rust
use icl_runtime::{parse_contract, normalize, verify, execute};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Load contract
  let contract_text = std::fs::read_to_string("my_contract.icl")?;
  let contract = parse_contract(&contract_text)?;
  
  // Verify
  verify(&contract)?;
  
  // Normalize
  let canonical = normalize(&contract)?;
  
  // Execute
  let result = execute(&canonical, &inputs)?;
  println!("{:?}", result);
  
  Ok(())
}
```

---

## Writing Your First Contract

### Step 1: Define Purpose

```icl
Contract {
  Identity {
    stable_id: "ic-hello-001",
    version: 1,
    created_timestamp: 2026-02-01T10:00:00Z,
    owner: "developer"
  }

  PurposeStatement {
    narrative: "Simple contract that echoes input",
    intent_source: "tutorial",
    confidence_level: 1.0
  }
```

### Step 2: Define Data

```icl
  DataSemantics {
    state: {
      message: String,
      count: Integer = 0
    },
    invariants: [
      "message is not empty",
      "count >= 0"
    ]
  }
```

### Step 3: Define Operations

```icl
  BehavioralSemantics {
    operations: [
      {
        name: "echo",
        precondition: "input_provided",
        parameters: { message: String },
        postcondition: "state_updated_with_message",
        side_effects: ["log_operation"],
        idempotence: "idempotent"
      }
    ]
  }
```

### Step 4: Define Constraints & Commitments

```icl
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
      "All messages are echoed",
      "Count increments correctly",
      "No messages lost"
    ],
    system_refusals: [
      "Will not modify past messages",
      "Will not lose data"
    ],
    user_obligations: [
      "May provide new messages",
      "May reset count"
    ]
  }
}
```

---

## Testing Your Contract

### Determinism Test

```python
from icl import execute

# Execute 100 times, verify identical results
inputs = {"action": "echo", "message": "hello"}
results = [execute(contract, inputs) for _ in range(100)]

assert all(r == results[0] for r in results), "Non-determinism detected!"
print("✓ Determinism verified")
```

### Precondition Test

```python
from icl import execute

# Valid: precondition met
valid_result = execute(contract, {"message": "hello"})
assert valid_result.success

# Invalid: precondition not met
invalid_result = execute(contract, {})  # No message
assert not invalid_result.success
assert "precondition" in invalid_result.error
```

### Invariant Test

```python
from icl import verify

# Verify invariants can hold
errors = verify(contract)
assert len(errors) == 0, f"Invariants violated: {errors}"
```

---

## Common Patterns

### Pattern 1: Append-Only Log

```icl
DataSemantics {
  state: {
    entries: Array<{
      timestamp: ISO8601,
      value: String,
      immutable: Boolean = true
    }>
  },
  invariants: [
    "all_entries_immutable_after_creation",
    "timestamps_ordered_ascending"
  ]
}

BehavioralSemantics {
  operations: [
    {
      name: "log_entry",
      precondition: "user_action_triggered",
      postcondition: "entry_appended_and_immutable"
    }
  ]
}
```

### Pattern 2: Computed State

```icl
DataSemantics {
  state: {
    entries: Array<{ value: Integer }>,
    total: Integer  // Computed, not stored
  },
  invariants: [
    "total_equals_sum_of_entries"
  ]
}

BehavioralSemantics {
  operations: [
    {
      name: "compute_total",
      trigger: "on_demand",
      computation: "sum(entries[].value)",
      postcondition: "total_matches_sum"
    }
  ]
}
```

### Pattern 3: Time-Based Trigger

```icl
BehavioralSemantics {
  operations: [
    {
      name: "daily_reminder",
      trigger: "time_based",
      schedule: "09:00_daily",
      precondition: "today_is_weekday",
      action: "send_notification"
    }
  ]
}
```

---

## Debugging

### Parse Errors

```python
from icl import parse_contract

try:
  contract = parse_contract(text)
except ParseError as e:
  print(f"Line {e.line}, Column {e.column}: {e.message}")
  print(f"  {e.context}")
```

### Verification Errors

```python
from icl import verify

errors = verify(contract)
for error in errors:
  print(f"{error.type}: {error.message}")
  print(f"  Location: {error.section}")
```

### Execution Errors

```python
from icl import execute

result = execute(contract, inputs)
if not result.success:
  print(f"Execution failed: {result.error}")
  print(f"Precondition met: {result.precondition_satisfied}")
  print(f"Postcondition met: {result.postcondition_satisfied}")
```

---

## Next Steps

1. **Read the [Core Specification](./spec/CORE-SPECIFICATION.md)** — Understand formal grammar and semantics
2. **Study [Example Contracts](./example-contracts.md)** — See working examples
3. **Review [API Reference](./api-reference.md)** — Details on all functions
4. **Join the community** — Contribute to ICL standardization

---

## FAQ

**Q: Can I use ICL in production?**
A: ICL is in early development (v0.1.0). Test thoroughly before production use. Report bugs on GitHub.

**Q: How is ICL different from JSON Schema?**
A: JSON Schema validates data format. ICL specifies execution semantics and contracts. They complement each other.

**Q: Can I extend ICL?**
A: Yes! Use the Extensions mechanism to add system-specific features without modifying Core ICL.

**Q: Is ICL Turing-complete?**
A: No, intentionally. This ensures contracts terminate and are verifiable.

**Q: Can multiple systems use the same ICL contract?**
A: Yes! That's the goal. Core ICL is universal. Different systems can adopt it independently.
