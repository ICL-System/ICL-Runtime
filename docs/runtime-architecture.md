# ICL Runtime Architecture

**Purpose:** Describe how the Intent Contract Language runtime executes contracts deterministically across all language bindings.

---

## Architecture Principles

The ICL runtime architecture enforces:

1. **One Canonical Implementation** (Rust) — Single source of truth for semantics
2. **Thin Language Bindings** — No reimplementation in other languages
3. **Deterministic Execution** — Same input always produces identical output
4. **Bounded Resources** — All execution bounded in memory and time
5. **Transparent Verification** — All operations verifiable and auditable

---

## Core Components

### 1. Parser

Converts ICL text → Abstract Syntax Tree (AST)

**Responsibility:**
- Tokenize input (BNF grammar from spec)
- Parse tokens into structured AST
- Report syntax errors with line/column

**Determinism:** Deterministic (no random choices)

**Language:** Rust (canonical)

### 2. Canonical Normalizer

Transforms AST → canonical form (deterministic, unique)

**Responsibility:**
- Sort all declarations
- Apply consistent formatting
- Compute semantic hash (SHA-256)
- Idempotent normalization

**Determinism:** Deterministic (idempotent)

**Language:** Rust (canonical)

**Property:** `normalize(normalize(x)) == normalize(x)`

### 3. Type Checker

Verifies type correctness

**Responsibility:**
- Check operation parameters match declared types
- Verify postcondition types valid
- Detect type mismatches
- Enforce no implicit coercion

**Determinism:** Deterministic (no random choices)

**Language:** Rust (canonical)

### 4. Coherence Verifier

Checks invariants and logical consistency

**Responsibility:**
- Verify all invariants can hold simultaneously
- Check operation logic (preconditions reasonable)
- Detect circular dependencies
- Verify determinism assumptions

**Determinism:** Deterministic (property checking)

**Language:** Rust (canonical)

### 5. Executor

Runs contract operations in sandbox

**Responsibility:**
- Load contract definition
- Evaluate preconditions
- Execute operation in isolated environment
- Verify postconditions
- Log all effects to provenance

**Determinism:** Deterministic (WASM + WASI)

**Language:** Rust (canonical)

**Sandbox:** WebAssembly (WASM + WASI) or V8 Isolates

### 6. Language Bindings

Wrap Rust core in language-specific APIs

**Responsibility:**
- Convert language-native input → Rust types
- Call Rust core (FFI)
- Convert Rust output → language-native types
- Provide idiomatic API per language

**Determinism:** Deterministic (thin wrapper, no logic)

**Languages:**
- Python (ctypes FFI)
- JavaScript (Node.js native binding)
- Go (cgo FFI)
- Rust (direct use of core)

---

## Execution Flow

```
Input Contract (ICL text)
    ↓
Parser (Rust)
    → BNF tokenization
    → AST construction
    ↓ (AST)
Canonical Normalizer (Rust)
    → Sort declarations
    → Consistent formatting
    → Compute hash
    ↓ (Canonical Form)
Type Checker (Rust)
    → Verify types
    → Check operations
    ↓ (Type-Correct)
Coherence Verifier (Rust)
    → Check invariants
    → Verify logic
    ↓ (Verified)
Executor (Rust + WASM)
    → Evaluate preconditions
    → Execute in sandbox
    → Verify postconditions
    → Log effects
    ↓ (Output + Effects)
User Application
```

---

## Determinism Guarantee

**Guarantee:** Same input always produces identical output.

**Enforced by:**

1. **No Random Sources** — Forbidden in all components
2. **No External I/O** — All I/O parameterized
3. **No Time Access** — Timestamps passed as parameters
4. **Deterministic Sorting** — All collections sorted
5. **IEEE 754 Compliance** — Floats handled precisely

**Verified by:**

```rust
#[test]
fn determinism_proof_100_iterations() {
  let input = load_test_contract();
  let mut outputs = Vec::new();
  
  for _ in 0..100 {
    outputs.push(execute(&input)?);
  }
  
  // All outputs must be byte-identical
  for output in &outputs[1..] {
    assert_eq!(output, &outputs[0]);
  }
}
```

---

## Multi-Language Support

### Architecture: One Core, Many Bindings

```
┌─────────────────────────────────────────┐
│     Rust Core (Canonical Runtime)       │
│  - Parser                               │
│  - Normalizer                           │
│  - Verifier                             │
│  - Executor                             │
│  - Determinism Guarantee                │
└──────────────┬──────────────────────────┘
               │
        FFI (Foreign Function Interface)
        
    ┌──────────┬──────────┬─────────────┐
    ↓          ↓          ↓             ↓
Python         JS         Go          Rust
Binding      Binding    Binding      (Direct)
(ctypes)     (napi)     (cgo)
```

### Language Binding Contract

Each binding must:

1. **Preserve Semantics** — No changes to execution logic
2. **Maintain Determinism** — Pass-through to Rust core
3. **Match Rust Output** — Byte-for-byte identical results
4. **Pass All Tests** — Include determinism tests

**Key Rule:** Bindings are thin wrappers. No reimplementation.

---

## Resource Limits

All execution bounded:

| Resource | Limit | Reason |
|----------|-------|--------|
| **Memory** | 256MB | Prevent memory exhaustion |
| **CPU Time** | 100ms | Prevent infinite loops |
| **Stack Depth** | 1000 | Prevent stack overflow |
| **State Size** | 100MB | Prevent runaway state |

Exceeding limits → graceful halt (no corruption).

---

## Verification & Proof

### Before Execution

1. **Syntax Verification** — Valid BNF
2. **Type Verification** — Correct types
3. **Coherence Check** — Invariants consistent
4. **Determinism Proof** — No non-deterministic components

### During Execution

1. **Precondition Check** — Can operation proceed?
2. **Resource Monitoring** — Within bounds?
3. **Postcondition Verification** — Did operation succeed?
4. **Invariant Monitoring** — Still satisfied?

### After Execution

1. **Contract Enforcement** — Commitments honored?
2. **Provenance Logging** — All effects recorded?
3. **State Consistency** — Valid end state?

---

## Implementation Status

| Component | Status | Language |
|-----------|--------|----------|
| **Parser** | TODO | Rust |
| **Normalizer** | TODO | Rust |
| **Type Checker** | TODO | Rust |
| **Verifier** | TODO | Rust |
| **Executor** | TODO | Rust + WASM |
| **Python Binding** | TODO | Python/ctypes |
| **JS Binding** | TODO | JavaScript/napi |
| **Go Binding** | TODO | Go/cgo |

See `/icl/runtime/` for implementation scaffolding.

---

## Next Steps

1. Implement Rust core components (parser, normalizer, verifier, executor)
2. Write determinism tests (100+ iterations)
3. Create Python binding with ctypes FFI
4. Create JavaScript binding with Node native module
5. Create Go binding with cgo FFI
6. Verify all bindings produce identical results
7. Publish to package registries (PyPI, npm, crates.io, Go modules)
