# Intent Contract Language — Implementation Guide

**Purpose:** Guidance for implementing ICL runtime in Rust (or porting to other languages).

---

## Architecture Overview

```
Input (ICL Text)
    ↓
Parser (Tokenizer → Parser → AST)
    ↓
Type Checker (Verification)
    ↓
Normalizer (Canonical Form)
    ↓
Optimizer (Optional)
    ↓
Executor (WASM VM)
    ↓
Output (Result)
```

---

## Phase 1: Parser

### Tokenizer

**Input:** ICL text string
**Output:** Token stream

**Tokens:**
```rust
enum Token {
  Contract,
  Identity,
  Lbrace,
  Rbrace,
  Colon,
  Comma,
  String(String),
  Integer(i64),
  Float(f64),
  Identifier(String),
  // ... etc
}
```

**Implementation:**
```rust
pub struct Tokenizer {
  input: Vec<char>,
  position: usize,
}

impl Tokenizer {
  pub fn new(text: &str) -> Self { /* ... */ }
  pub fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> { /* ... */ }
}
```

### Parser

**Input:** Token stream
**Output:** Abstract Syntax Tree (Contract)

**Implementation:**
```rust
pub struct Parser {
  tokens: Vec<Token>,
  position: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self { /* ... */ }
  pub fn parse(&mut self) -> Result<Contract, ParseError> { /* ... */ }
  
  fn parse_identity(&mut self) -> Result<Identity, ParseError> { /* ... */ }
  fn parse_purpose(&mut self) -> Result<PurposeStatement, ParseError> { /* ... */ }
  // ... etc
}
```

**Error handling:**
- Provide line/column in errors
- Include context (show line of code)
- Suggest fix if obvious

---

## Phase 2: Verification (Type Checking)

### Type System

```rust
pub enum Type {
  String,
  Integer,
  Float,
  Boolean,
  Timestamp,
  Array(Box<Type>),
  Map(Box<Type>, Box<Type>),
  Enum(Vec<String>),
  Custom(String),
}

pub struct TypeChecker {
  symbols: HashMap<String, Type>,
  errors: Vec<TypeError>,
}

impl TypeChecker {
  pub fn check(&mut self, contract: &Contract) -> Result<(), Vec<TypeError>> {
    // Check all type assignments
    // Verify all operations use correct types
    // Validate state transitions preserve types
  }
}
```

### Invariant Verification

```rust
pub struct InvariantVerifier {
  contract: Contract,
}

impl InvariantVerifier {
  pub fn verify(&self) -> Result<(), Vec<String>> {
    // For each invariant:
    // 1. Can initial state satisfy it?
    // 2. Can all operations preserve it?
    // 3. Is it consistent with postconditions?
  }
}
```

### Determinism Check

```rust
pub struct DeterminismChecker;

impl DeterminismChecker {
  pub fn check(contract: &Contract) -> Result<(), Vec<String>> {
    // Check for:
    // - Randomness functions (forbidden)
    // - System time access (bounded)
    // - External I/O (logged)
    // - Floating point operations (documented)
    // - Any non-determinism is explicit
  }
}
```

---

## Phase 3: Normalization

### Canonical Form

**Goal:** Same contract → same normalized output (idempotent, deterministic)

```rust
pub struct Normalizer;

impl Normalizer {
  pub fn normalize(contract: Contract) -> Result<Contract, String> {
    let mut normalized = contract;
    
    // 1. Sort all fields alphabetically
    normalized.sort_all();
    
    // 2. Normalize all whitespace (remove comments)
    normalized.remove_comments();
    
    // 3. Normalize type representations
    // e.g., "int" → "Integer", "str" → "String"
    normalized.normalize_types();
    
    // 4. Expand all shorthand
    // e.g., "count: Integer = 0" expands fully
    normalized.expand_shorthand();
    
    // 5. Verify idempotence
    let double_normalized = Self::normalize(normalized.clone())?;
    if normalized != double_normalized {
      return Err("Normalization not idempotent".to_string());
    }
    
    Ok(normalized)
  }
}
```

**Idempotence test:**

```rust
#[test]
fn test_normalization_is_idempotent() {
  let contract = parse_contract(TEST_CONTRACT).unwrap();
  
  let norm1 = normalize(&contract).unwrap();
  let norm2 = normalize(&norm1).unwrap();
  
  assert_eq!(norm1, norm2, "Normalization not idempotent");
}
```

---

## Phase 4: Execution

### Execution Environment

```rust
pub struct ExecutionContext {
  contract: Contract,
  state: Map<String, Value>,
  inputs: Map<String, Value>,
  outputs: Map<String, Value>,
  memory_used: usize,
  time_elapsed: Duration,
  operations_executed: Vec<String>,
}

pub struct Executor {
  context: ExecutionContext,
  limits: ResourceLimits,
}

impl Executor {
  pub fn execute(&mut self) -> Result<ExecutionResult, ExecutionError> {
    // 1. Verify preconditions
    self.check_preconditions()?;
    
    // 2. Update state
    self.apply_operations()?;
    
    // 3. Verify postconditions
    self.check_postconditions()?;
    
    // 4. Verify invariants still hold
    self.verify_invariants()?;
    
    // 5. Return result
    Ok(ExecutionResult {
      success: true,
      output: self.context.outputs.clone(),
      state_updated: true,
      // ...
    })
  }
}
```

### Determinism Enforcement

```rust
pub struct DeterminismEnforcer {
  random_seed: u64,
}

impl DeterminismEnforcer {
  pub fn enforce(&self, operation: &Operation) -> Result<(), String> {
    // Rule 1: No randomness functions
    if operation.uses_randomness() {
      return Err("Randomness forbidden".to_string());
    }
    
    // Rule 2: Time operations must use seeded clock
    if operation.accesses_system_time() {
      // Provide seeded time instead
    }
    
    // Rule 3: All floating point must be documented
    if operation.uses_float_operations() {
      // Log for determinism validation
    }
    
    Ok(())
  }
}
```

---

## Phase 5: Result Validation

### Contract Satisfaction

```rust
pub struct ContractValidator;

impl ContractValidator {
  pub fn validate(
    result: &ExecutionResult,
    contract: &Contract,
  ) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    // 1. Did postcondition hold?
    if !result.postcondition_satisfied {
      errors.push("Postcondition violated".to_string());
    }
    
    // 2. Did all invariants remain true?
    if !result.invariants_satisfied {
      errors.push("Invariant violated".to_string());
    }
    
    // 3. Were resource limits respected?
    if result.memory_used_bytes > contract.execution_constraints.max_memory_bytes {
      errors.push("Memory limit exceeded".to_string());
    }
    
    if !errors.is_empty() {
      return Err(errors);
    }
    
    Ok(())
  }
}
```

---

## Error Handling Strategy

```rust
pub enum ICLError {
  Parse(ParseError),
  Type(TypeError),
  Verification(VerificationError),
  Execution(ExecutionError),
  Determinism(DeterminismError),
}

impl ICLError {
  pub fn explanation(&self) -> String {
    // Human-readable explanation
  }
  
  pub fn suggestion(&self) -> Option<String> {
    // How to fix
  }
}
```

**For all errors:**
- ✅ Include location (line, column)
- ✅ Show context (code snippet)
- ✅ Provide suggestion (how to fix)
- ✅ Log full error with timestamp

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_parse_simple_contract() {
    let text = r#"Contract { ... }"#;
    let contract = parse_contract(text);
    assert!(contract.is_ok());
  }
  
  #[test]
  fn test_invalid_syntax_error() {
    let text = "Contract {";  // Missing closing brace
    let result = parse_contract(text);
    assert!(result.is_err());
  }
  
  #[test]
  fn test_type_mismatch() {
    let text = r#"
      Contract {
        DataSemantics {
          state: { count: Integer }
          invariants: [ "count is string" ]  // Type mismatch!
        }
      }
    "#;
    let contract = parse_contract(text).unwrap();
    assert!(verify(&contract).is_err());
  }
}
```

### Integration Tests

```rust
#[test]
fn test_full_workflow() {
  // Parse
  let contract = parse_contract(CONTRACT_TEXT).unwrap();
  
  // Verify
  verify(&contract).unwrap();
  
  // Normalize
  let canonical = normalize(&contract).unwrap();
  
  // Execute 100 times
  for i in 0..100 {
    let result = execute(&canonical, &INPUTS).unwrap();
    assert_eq!(result, EXPECTED_RESULT,
      "Non-determinism at iteration {}", i);
  }
}
```

### Determinism Tests

```rust
#[test]
fn test_execution_is_deterministic() {
  let contract = parse_contract(CONTRACT_TEXT).unwrap();
  let normalized = normalize(&contract).unwrap();
  
  let mut results = Vec::new();
  for _ in 0..100 {
    let result = execute(&normalized, &INPUTS).unwrap();
    results.push(result);
  }
  
  // All results must be identical
  for (i, result) in results.iter().enumerate().skip(1) {
    assert_eq!(&results[0], result,
      "Non-determinism at iteration {}", i);
  }
}
```

---

## Performance Optimization

After correctness is verified:

1. **Memoization** — Cache repeated computations
2. **Lazy evaluation** — Only compute what's needed
3. **Parallel execution** — Parallelize independent operations (deterministically!)
4. **JIT compilation** — Compile hot paths
5. **Memory pooling** — Reuse allocations

**Constraint:** Optimizations must maintain determinism and correctness.

---

## Porting to Other Languages

When porting ICL to Python/JavaScript/Go:

1. **Implement canonical runtime in Rust** (source of truth)
2. **Create thin FFI wrapper** (don't reimplement)
3. **Call Rust core via FFI** (for parse, verify, normalize, execute)
4. **Pass/fail identical conformance tests**
5. **Zero byte variance requirement** (same input → identical output)

Example binding structure:

```python
# Python binding (thin wrapper)
from ctypes import cdll

_icl_core = cdll.LoadLibrary('libicl_runtime.so')

def parse_contract(text):
  return _icl_core.parse_contract(text.encode())

def execute(contract, inputs):
  return _icl_core.execute(contract, inputs)
```

---

## Maintenance Checklist

- ✅ All tests pass before commit
- ✅ No unsafe code without justification
- ✅ Error messages are actionable
- ✅ Determinism tests pass (100+ iterations)
- ✅ Code is well-commented
- ✅ Performance regressions tracked
- ✅ No silent failures (all errors logged)
