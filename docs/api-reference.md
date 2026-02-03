# Intent Contract Language — API Reference

**Version:** 0.1.0
**Status:** Early Development
**Last Updated:** 2026-02-01

---

## Core Functions

### `parse_contract(text: String) → Result<Contract, ParseError>`

Parse text representation into Contract object.

**Parameters:**
- `text`: Valid ICL syntax (as per CORE-SPECIFICATION.md)

**Returns:**
- `Ok(contract)`: Parsed and validated structure
- `Err(ParseError)`: Syntax or structure violation

**Example:**

```python
from icl import parse_contract

contract_text = """
Contract {
  Identity {
    stable_id: "ic-water-001",
    version: 1,
    created_timestamp: 2026-02-01T10:00:00Z,
    owner: "alice"
  }
}
"""

contract = parse_contract(contract_text)
print(contract.identity.stable_id)  # "ic-water-001"
```

**Errors:**
- `ParseError::SyntaxError` — Invalid token at line:col
- `ParseError::UnknownField` — Unknown property name
- `ParseError::TypeMismatch` — Wrong value type for field
- `ParseError::MissingRequired` — Required field absent

---

### `verify(contract: Contract) → Result<(), Vec<VerificationError>>`

Verify contract correctness without modifying it.

**Parameters:**
- `contract`: Contract object to check

**Returns:**
- `Ok(())`: Contract is valid
- `Err(errors)`: List of verification errors

**Checks performed:**
- Type consistency (all types match declared)
- Invariant satisfaction (can all invariants be true?)
- Determinism requirements (no randomness)
- Precondition/postcondition consistency
- Resource limit feasibility
- Cycle detection (no circular dependencies)

**Example:**

```rust
use icl_runtime::{parse_contract, verify};

let contract = parse_contract(&text)?;
match verify(&contract) {
  Ok(()) => println!("✓ Contract is valid"),
  Err(errors) => {
    for error in errors {
      eprintln!("{}: {}", error.kind, error.message);
    }
  }
}
```

**Error Types:**
- `VerificationError::TypeMismatch` — Type inconsistency
- `VerificationError::Unsatisfiable` — Invariant can't be satisfied
- `VerificationError::NonDeterministic` — Non-determinism detected
- `VerificationError::LogicError` — Precondition contradicts postcondition
- `VerificationError::Cycle` — Circular dependency detected
- `VerificationError::InfeasibleConstraint` — Resource limits impossible

---

### `normalize(contract: Contract) → Result<Contract, NormalizationError>`

Transform contract to canonical form (deterministic, idempotent).

**Parameters:**
- `contract`: Contract object to normalize

**Returns:**
- `Ok(canonical)`: Normalized contract
- `Err(error)`: Normalization failed

**Guarantees:**
- `normalize(contract)` applied twice produces same result (idempotent)
- All whitespace normalized
- All semantically equivalent forms converted to one form
- All comments removed (canonicalization step)
- Order of fields standardized

**Example:**

```javascript
const { parseContract, normalize } = require('icl-runtime');

const contract = parseContract(contractText);
const canonical = normalize(contract);

// Save canonical form
fs.writeFileSync('contract.canonical.json', JSON.stringify(canonical));
```

**Errors:**
- `NormalizationError::CannotCanonical` — Contract has no canonical form

---

### `execute(contract: Contract, inputs: InputMap) → Result<ExecutionResult, ExecutionError>`

Execute contract with given inputs in sandbox.

**Parameters:**
- `contract`: Contract to execute (should be normalized)
- `inputs`: Map of input names → values

**Returns:**
- `Ok(result)`: Execution succeeded
- `Err(error)`: Execution failed

**Result fields:**
- `success: Boolean` — Did execution complete?
- `output: OutputMap` — Results of operations
- `state_updated: Boolean` — Was state modified?
- `time_elapsed_ms: Integer` — Execution time
- `memory_used_bytes: Integer` — Peak memory
- `contract_satisfied: Boolean` — Did contract hold?

**Example:**

```python
from icl import execute

result = execute(contract, {
  'action': 'log_intake',
  'volume_ml': 250
})

if result.success:
  print(f"Output: {result.output}")
  print(f"State updated: {result.state_updated}")
else:
  print(f"Failed: {result.error}")
```

**Errors:**
- `ExecutionError::PreconditionFailed` — Input doesn't satisfy precondition
- `ExecutionError::Timeout` — Exceeded computation limit
- `ExecutionError::OutOfMemory` — Exceeded memory limit
- `ExecutionError::ContractViolation` — Postcondition not satisfied
- `ExecutionError::DeterminismViolation` — Non-deterministic behavior detected
- `ExecutionError::StateCorruption` — State invalid after operation

---

## Type System

### Contract

Root object representing an ICL contract.

```typescript
interface Contract {
  Identity: IdentityBlock
  PurposeStatement: PurposeStatementBlock
  DataSemantics: DataSemanticsBlock
  BehavioralSemantics: BehavioralSemanticsBlock
  ExecutionConstraints: ExecutionConstraintsBlock
  HumanMachineContract: HumanMachineContractBlock
}
```

### Identity

Machine-readable contract identifier.

```typescript
interface Identity {
  stable_id: string              // Unique, immutable ID (e.g., "ic-water-001")
  version: integer               // Semantic version (major.minor.patch)
  created_timestamp: ISO8601     // When contract created
  owner: string                  // Contract owner (e.g., "alice" or "system")
}
```

### PurposeStatement

Human intent and confidence.

```typescript
interface PurposeStatement {
  narrative: string              // Human-readable purpose
  intent_source: string          // "user", "system", "derived"
  confidence_level: float        // 0.0-1.0, above 0.8 = confident
  domain: string?                // Optional: "finance", "health", etc.
}
```

### DataSemantics

State and invariants.

```typescript
interface DataSemantics {
  state: Map<string, Type>       // State variables
  invariants: string[]           // Conditions always true
  constraints: Constraint[]      // Typed constraints
}
```

**Type system:**
- `String` — UTF-8 text
- `Integer` — Unbounded integer
- `Float` — IEEE 754 double
- `Boolean` — True/false
- `ISO8601` — Timestamp
- `Array<T>` — Homogeneous array
- `Map<K, V>` — Key-value map
- `Enum { A, B, C }` — Disjoint union

### BehavioralSemantics

Operations and state transitions.

```typescript
interface BehavioralSemantics {
  operations: Operation[]        // Available operations
  initial_state: StateValue      // Starting state
  invariant_preservation: bool   // Always maintain invariants?
}

interface Operation {
  name: string
  trigger: "manual" | "time_based" | "event_based"
  precondition: string           // Condition that must be true
  postcondition: string          // Condition after operation
  parameters: Map<string, Type>
  computation: string?           // Optional computation expression
  side_effects: string[]         // List of effects
  idempotence: "idempotent" | "non_idempotent"
}
```

### ExecutionConstraints

Resource limits and permissions.

```typescript
interface ExecutionConstraints {
  trigger_types: string[]        // Allowed triggers ("manual", "automatic", etc.)
  resource_limits: {
    max_memory_bytes: integer
    computation_timeout_ms: integer
    max_state_size_bytes: integer
  }
  external_permissions: string[] // Permissions needed (e.g., ["filesystem_read"])
  sandbox_mode: "full_isolation" | "limited" | "none"
}
```

### HumanMachineContract

Promises and obligations.

```typescript
interface HumanMachineContract {
  system_commitments: string[]   // What system guarantees
  system_refusals: string[]      // What system refuses to do
  user_obligations: string[]     // What user must do
  user_entitlements: string[]    // What user can expect
}
```

---

## Error Handling

All errors follow this pattern:

```typescript
interface ICLError {
  kind: string              // Error type
  message: string           // Human-readable message
  location?: {
    file: string?
    line: integer?
    column: integer?
  }
  context?: string          // Code context
  suggestion?: string       // How to fix
}
```

**Error categories:**

| Category | Examples |
|----------|----------|
| **ParseError** | SyntaxError, UnknownField, TypeMismatch |
| **TypeError** | TypeMismatch, UndefinedVariable |
| **VerificationError** | Unsatisfiable, NonDeterministic |
| **ExecutionError** | PreconditionFailed, ContractViolation |
| **DeterminismError** | RandomnessDetected, TimingVariance |

---

## Language Bindings

### Python

```bash
pip install icl-runtime
```

```python
from icl import (
  parse_contract, normalize, verify, execute,
  Contract, Identity, PurposeStatement
)

contract = parse_contract(text)
verify(contract)
canonical = normalize(contract)
result = execute(canonical, inputs)
```

### JavaScript/Node.js

```bash
npm install icl-runtime
```

```javascript
const {
  parseContract, normalize, verify, execute,
  Contract, Identity
} = require('icl-runtime');

const contract = parseContract(text);
verify(contract);
const canonical = normalize(contract);
const result = execute(canonical, inputs);
```

### Go

```bash
go get icl-runtime
```

```go
import "icl-runtime"

contract, _ := icl.ParseContract(text)
icl.Verify(contract)
canonical, _ := icl.Normalize(contract)
result, _ := icl.Execute(canonical, inputs)
```

### Rust

```toml
[dependencies]
icl-runtime = "0.1"
```

```rust
use icl_runtime::{parse_contract, normalize, verify, execute};

let contract = parse_contract(&text)?;
verify(&contract)?;
let canonical = normalize(&contract)?;
let result = execute(&canonical, &inputs)?;
```

---

## FAQ

**Q: What's the difference between `verify()` and `execute()`?**

A: `verify()` checks that contract is *possible* (types match, invariants are satisfiable). `execute()` checks that contract actually *works* (preconditions met, postconditions satisfy contract).

**Q: What does `normalize()` do?**

A: Transforms contract to canonical form. Used to:
- Compare contracts for equivalence
- Produce deterministic output for signing
- Store in normalized form in databases
- Detect if two contracts are semantically identical

**Q: Can I modify a contract after creation?**

A: Create new version. Old version preserved. See Contract versioning for details.

**Q: Is ICL Turing-complete?**

A: No, intentionally. ICL is bounded (must terminate, memory-limited). This ensures contracts are verifiable.

**Q: How do I handle errors in contracts?**

A: Errors must be handled explicitly. No silent failures. All error paths tested.
