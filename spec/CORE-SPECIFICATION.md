# ICL Formal Specification

**Purpose:** Define the complete formal specification of Intent Contract Language (ICL) for universal, language-agnostic contract validation.

> ICL is a specification language for intent contracts. It is deterministic, verifiable, and platform-independent.

---

## Philosophy

ICL is:
- **Deterministic**: Same input produces same output (always)
- **Diffable**: Versions can be compared meaningfully
- **Verifiable**: Properties can be proven machine-checkable
- **Canonical**: One true form for each contract (via normalizer)
- **Closed-World**: Anything not declared does not exist

ICL is NOT:
- A programming language (no loops, conditionals, or side effects)
- A configuration language (not key-value pairs)
- A query language (not for retrieving data)
- Turing-complete (intentionally bounded)

---

## Core vs Extensions

**Core ICL** (this specification) is:
- Implementation-agnostic
- Platform-independent
- Reproducible across any system
- Extensible without modification

**Extensions** (optional, system-specific):
- Custom namespaces (e.g., `YourSystem { ... }`)
- System-specific optimizations
- Domain-specific features
- Future capabilities

Any system can adopt Core ICL and add its own Extensions without changing Core semantics.

---

## 1. Formal Grammar (BNF Notation)

### 1.1 Top-Level Structure

```bnf
<ContractDefinition> ::= 
  <CoreContract>
  (<Extensions>)?

<CoreContract> ::= 
  "Contract" "{" 
    <Identity>
    <PurposeStatement>
    <DataSemantics>
    <BehavioralSemantics>
    <ExecutionConstraints>
    <HumanMachineContract>
  "}"

<Extensions> ::= 
  "Extensions" "{" 
    (<SystemExtension>)*
  "}"

<SystemExtension> ::= 
  <SystemName> "{" 
    (<CustomField>)*
  "}"

<SystemName> ::= "Custom" | [a-z_][a-z0-9_-]*

<Identifier> ::= [a-z_][a-z0-9_]*

<StringLiteral> ::= '"' (printable | escape_sequence)* '"'

<IntegerLiteral> ::= [0-9]+

<FloatLiteral> ::= [0-9]+ '.' [0-9]+

<ISO8601> ::= YYYY-MM-DDTHH:MM:SSZ

<UUIDv4> ::= [0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}
```

### 1.2 Identity Section

```bnf
<Identity> ::= 
  "Identity" "{" 
    "stable_id:" <StringLiteral> ","
    "version:" <IntegerLiteral> ","
    "created_timestamp:" <ISO8601> ","
    "owner:" <StringLiteral> ","
    "semantic_hash:" <StringLiteral>
  "}"
```

**Constraints:**
- `stable_id` must match pattern: `[a-z0-9][a-z0-9\-]{0,30}[a-z0-9]`
- `version` is non-negative integer (increment on each change)
- `semantic_hash` is SHA-256 hex digest
- `owner` is opaque string (system-dependent)

### 1.3 Purpose Statement

```bnf
<PurposeStatement> ::= 
  "PurposeStatement" "{" 
    "narrative:" <StringLiteral> ","
    "intent_source:" <StringLiteral> ","
    "confidence_level:" <FloatLiteral>
  "}"
```

**Constraints:**
- `narrative` concisely describes the contract's purpose (< 500 chars)
- `intent_source` describes origin (e.g., `"user_natural_language:2026-01-31T09:00:00Z"`)
- `confidence_level` is float in range [0.0, 1.0]

### 1.4 Data Semantics Section

```bnf
<DataSemantics> ::= 
  "DataSemantics" "{" 
    "state:" <StateDefinition> ","
    "invariants:" <InvariantList>
  "}"

<StateDefinition> ::= 
  "{" (<StateField>)* "}"

<StateField> ::= 
  <Identifier> ":" <TypeExpression> ("=" <DefaultValue>)? ","

<TypeExpression> ::=
  <PrimitiveType> |
  <CompositeType> |
  <CollectionType>

<PrimitiveType> ::= 
  "Integer" | "Float" | "String" | "Boolean" | "ISO8601" | "UUID"

<CompositeType> ::= 
  "Object" "{" (<StateField>)* "}" |
  "Enum" "[" <StringLiteral> ("," <StringLiteral>)* "]"

<CollectionType> ::= 
  "Array" "<" <TypeExpression> ">" |
  "Map" "<" <TypeExpression> "," <TypeExpression> ">"

<InvariantList> ::= 
  "[" (<StringLiteral>)* "]"
```

**Semantics:**
- `state` defines all persistent data (what can be stored)
- Each field has a type and optional default value
- `invariants` are immutable rules that must never be violated
- Types are strict (no implicit coercion)

### 1.5 Behavioral Semantics Section

```bnf
<BehavioralSemantics> ::= 
  "BehavioralSemantics" "{" 
    "operations:" <OperationList>
  "}"

<OperationList> ::= 
  "[" (<Operation>)* "]"

<Operation> ::= 
  "{" 
    "name:" <StringLiteral> ","
    "precondition:" <StringLiteral> ","
    "parameters:" <ParameterList> ","
    "postcondition:" <StringLiteral> ","
    "side_effects:" <SideEffectList> ","
    "idempotence:" <StringLiteral>
  "}"

<ParameterList> ::= 
  "{" (<Parameter>)* "}"

<Parameter> ::= 
  <Identifier> ":" <TypeExpression> ("=" <DefaultValue>)? ","

<SideEffectList> ::= 
  "[" (<StringLiteral>)* "]"
```

**Semantics:**
- `precondition` must be true before operation
- `parameters` define inputs (with types)
- `postcondition` must be true after operation
- `side_effects` are visible state changes (logged in provenance)
- `idempotence` describes repeated execution behavior

### 1.6 Execution Constraints Section

```bnf
<ExecutionConstraints> ::= 
  "ExecutionConstraints" "{" 
    "trigger_types:" <TriggerTypeList> ","
    "resource_limits:" <ResourceLimits> ","
    "external_permissions:" <PermissionList> ","
    "sandbox_mode:" <StringLiteral>
  "}"

<TriggerTypeList> ::= 
  "[" (<StringLiteral>)* "]"

<ResourceLimits> ::= 
  "{" 
    "max_memory_bytes:" <IntegerLiteral> ","
    "computation_timeout_ms:" <IntegerLiteral> ","
    "max_state_size_bytes:" <IntegerLiteral>
  "}"

<PermissionList> ::= 
  "[" (<StringLiteral>)* "]"
```

**Semantics:**
- `trigger_types` specifies how operation can be initiated (manual, time_based, event_based)
- `resource_limits` enforces bounded execution (prevents denial-of-service)
- `external_permissions` lists required system access (file, network, etc.)
- `sandbox_mode` specifies isolation level (full_isolation, restricted, none)

### 1.7 Human-Machine Contract Section

```bnf
<HumanMachineContract> ::= 
  "HumanMachineContract" "{" 
    "system_commitments:" <CommitmentList> ","
    "system_refusals:" <RefusalList> ","
    "user_obligations:" <ObligationList>
  "}"

<CommitmentList> ::= 
  "[" (<StringLiteral>)* "]"

<RefusalList> ::= 
  "[" (<StringLiteral>)* "]"

<ObligationList> ::= 
  "[" (<StringLiteral>)* "]"
```

**Semantics:**
- `system_commitments` are guarantees the system makes (e.g., "All state changes logged")
- `system_refusals` are explicit bounds (e.g., "Will never delete archived data")
- `user_obligations` are user responsibilities (e.g., "May modify goal before execution")

---

## 2. Type System

### 2.1 Primitive Types

| Type | Representation | Example |
|------|----------------|---------|
| `Integer` | 64-bit signed | `42`, `-100` |
| `Float` | 64-bit IEEE 754 | `3.14`, `2.71828` |
| `String` | UTF-8 text | `"hello"` |
| `Boolean` | True/False | `true`, `false` |
| `ISO8601` | RFC 3339 timestamp | `2026-01-31T09:00:00Z` |
| `UUID` | RFC 4122 UUIDv4 | `550e8400-e29b-41d4-a716-446655440000` |

### 2.2 Composite Types

**Object:**
```
Object {
  field1: Integer,
  field2: String = "default"
}
```

**Enum:**
```
Enum ["pending", "active", "completed", "archived"]
```

### 2.3 Collection Types

**Array:**
```
Array<Integer>         // Ordered list of integers
Array<Object { ... }>  // Ordered list of objects
```

**Map:**
```
Map<String, Integer>   // Key-value mapping
```

### 2.4 Type System Rules

1. **Strict Typing** — No implicit coercion between types
2. **Nullability** — Values are non-null by default (no null/undefined)
3. **Immutability** — Types are immutable unless explicitly mutable
4. **Composability** — Types can be nested arbitrarily

---

## 3. Semantics & Execution Model

### 3.1 Deterministic Execution

**Requirement:** Same input always produces identical output.

**Implications:**
- No random number generation (use seeded generators if needed)
- No current time access (pass timestamp as parameter)
- No external system calls that vary (all must be mocked or parameterized)
- No hash table iteration order dependencies (sort keys)
- No floating-point operations without specified rounding

### 3.2 Canonical Form

Each contract has a **canonical form** — a deterministic normalization.

Properties:
- **Idempotent**: `normalize(normalize(x)) == normalize(x)`
- **Unique**: Each distinct contract has one canonical form
- **Comparable**: Canonical forms can be diff'd and versioned

Normalization includes:
- Sorted declarations
- Explicit defaults
- Consistent whitespace
- Deterministic hashing

### 3.3 State Transitions

State can only change through explicit operations:

```
Current State + Operation → New State
     ↓                         ↓
  (verified            (verified against
   invariants)         postconditions)
```

All state transitions are:
- **Atomic**: All-or-nothing (no partial updates)
- **Logged**: Every transition recorded in provenance
- **Justified**: Every transition has causal reason

### 3.4 Execution Boundaries

Execution is bounded in:
- **Memory**: `max_state_size_bytes` limit
- **Time**: `computation_timeout_ms` limit
- **Scope**: Only declared operations permitted

Exceeding bounds → immediate halt (never corruption).

---

## 4. Verification & Guarantees

### 4.1 Contract Verification

Before execution, contracts must verify:

1. **Syntax** — Valid BNF
2. **Type Correctness** — All operations respect types
3. **Invariant Consistency** — Invariants can logically hold
4. **Determinism** — No non-deterministic components
5. **Boundedness** — No unbounded loops or recursion

### 4.2 Execution Verification

During execution, system must verify:

1. **Preconditions** — Operation can proceed
2. **Postconditions** — Operation completed correctly
3. **Invariants** — No invariant violated
4. **Resource Limits** — Within bounds

### 4.3 Determinism Guarantee

**Guarantee**: Same input always produces identical output.

**Verification Method**:
1. Run contract 100+ times with identical inputs
2. Compare all outputs byte-for-byte
3. Zero variance required

**Test Example**:
```rust
#[test]
fn determinism_test() {
  let input = create_test_input();
  let outputs: Vec<Output> = (0..100)
    .map(|_| execute_contract(&input))
    .collect();
  
  let first = &outputs[0];
  for (i, output) in outputs.iter().enumerate() {
    assert_eq!(output, first, "Non-determinism at iteration {}", i);
  }
}
```

### 4.4 Transparency Guarantee

**Guarantee**: All state changes are visible and explainable.

**Verification Method**:
- Maintain immutable provenance log
- Each state change has causal explanation
- Provenance includes: timestamp, actor, operation, reason, result

---

## 5. Extension Mechanism

Core ICL can be extended without modification.

### 5.1 Extension Syntax

```bnf
<Extensions> ::= 
  "Extensions" "{" 
    (<SystemExtension>)*
  "}"

<SystemExtension> ::= 
  <SystemName> "{" 
    (<CustomField>)*
  "}"
```

### 5.2 Extension Rules

1. **No Core Modification** — Extensions never change Core syntax or semantics
2. **Namespace Isolation** — Each extension in its own namespace
3. **Compatibility** — Extensions are optional (systems ignore unknown extensions)
4. **Documentation** — Each extension documents its purpose and semantics

### 5.3 Creating an Extension

To create extension `YourSystem`:

```
Contract {
  // Core ICL (Sections 1-7) — unchanged
  Identity { ... }
  PurposeStatement { ... }
  // ... rest of core ...
}

Extensions {
  YourSystem {
    CustomField1: String,
    CustomField2: Array<Object { ... }>
  }
}
```

**Key Point**: Core sections are immutable and portable. Extensions are optional and system-specific.

---

## 6. Canonical Normalizer Algorithm

The normalizer transforms ICL into canonical form.

### 6.1 Normalization Steps

1. **Parsing** — Parse input into AST
2. **Validation** — Verify all constraints
3. **Sorting** — Sort sections and fields by name
4. **Formatting** — Apply consistent whitespace
5. **Hashing** — Compute SHA-256 of normalized form

### 6.2 Idempotence Property

```rust
assert_eq!(
  normalize(x),
  normalize(normalize(x))
)
```

Running normalizer twice produces identical output (proof of determinism).

### 6.3 Semantic Preservation

Normalization preserves all meaning:
- No information loss
- All valid constraints preserved
- Behavior identical before/after

---

## Guarantees Summary

| Guarantee | Enforced By | Verified By |
|-----------|------------|------------|
| **Determinism** | No random, time, external I/O | 100+ iteration tests |
| **Transparency** | Immutable provenance log | Full audit trail |
| **Boundedness** | Resource limits enforced | Execution monitoring |
| **Correctness** | Pre/postcondition checking | Contract verification |

---

## Next Steps

This specification defines Core ICL. To implement:

1. **Build Parser** — Tokenize and parse BNF grammar
2. **Build Canonicalizer** — Implement normalization algorithm
3. **Build Verifier** — Check invariants and determinism
4. **Build Executor** — Execute operations deterministically
5. **Build Bindings** — Wrap core in language-specific APIs

See `/icl/runtime/` for canonical Rust implementation.
See `/icl/bindings/` for language bindings.
