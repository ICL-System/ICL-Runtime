# Intent Contract Language (ICL) - Contributing Guidelines

## Code of Conduct

Be clear. Be rigorous. Be deterministic.

## Development Principles

These apply to all ICL code:

### 1. Determinism is Non-Negotiable

All ICL code must be provably deterministic:

```rust
// Good: Pure function, deterministic
fn normalize(contract: &str) -> Result<String> { ... }

// Good: Marked non-determinism (parsing only)
fn parse_icl(input: &str) -> Result<Contract> { ... }

// BAD: Non-determinism in verification layer
fn verify_contract(c: &Contract) -> Result<()> {
  let random_check = rand::random();  // ❌ Forbidden
  ...
}
```

**Rule:** Randomness only in parsing layer. Verification and execution must be deterministic.

### 2. Single Source of Truth

All language bindings must:
1. Wrap the Rust core (no reimplementation)
2. Have identical semantics
3. Produce identical results
4. Pass identical tests

**No custom semantics in bindings.**

### 3. Testing: Determinism Proof

Every test must verify:
- **Idempotence**: `normalize(normalize(x)) == normalize(x)`
- **Determinism**: 100+ iterations with identical outputs

Example:
```rust
#[test]
fn determinism_proof() {
  let input = load_test_contract();
  let outputs: Vec<_> = (0..100)
    .map(|_| normalize(&input).unwrap())
    .collect();
  
  // All outputs must be byte-identical
  for (i, output) in outputs.iter().enumerate().skip(1) {
    assert_eq!(&outputs[0], output, "Non-determinism at iteration {}", i);
  }
}
```

### 4. Core Specification is Immutable

The [ICL-Spec](https://github.com/ICL-System/ICL-Spec) repository defines Core ICL. Rules:

- ✅ Can clarify documentation
- ✅ Can add explanations
- ❌ Cannot change grammar
- ❌ Cannot add new primitives
- ❌ Cannot change semantics

If you need to extend: Use Extensions mechanism. Never modify Core.

### 5. Version Each Component

Three versioning systems:

**ICL Specification Version**
- Changes only when Core grammar/semantics change
- Unlikely to change frequently
- Semantic versioning (e.g., `1.0.0`)

**Implementation Version**
- Changes when runtime code updates
- More frequent
- Semantic versioning (e.g., `0.1.0` → `0.2.0`)

**Language Binding Version**
- Can differ from core (e.g., Python binding 0.2.1, Rust core 0.2.0)
- Semantic versioning per binding

All versions documented in each component.

## Rust Code Standards

### File Organization

```
ICL-Runtime/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── icl-core/           # Library crate (all core logic)
│   │   └── src/
│   │       ├── lib.rs          # Public API, Contract types
│   │       ├── error.rs        # Error types + Result alias
│   │       ├── parser/
│   │       │   ├── mod.rs      # parse_contract() entry point
│   │       │   ├── tokenizer.rs # Token types + scanning
│   │       │   └── ast.rs      # AST node definitions
│   │       ├── normalizer.rs   # Canonical form normalization
│   │       ├── verifier.rs     # Type + invariant + determinism checks
│   │       └── executor.rs     # Sandboxed execution engine
│   └── icl-cli/            # Binary crate (CLI interface)
│       └── src/main.rs
├── tests/
│   ├── integration/
│   ├── conformance/
│   └── determinism/
└── benches/
```

### Error Handling

```rust
// All fallible operations return Result<T>
fn parse_contract(input: &str) -> Result<Contract> {
  // Use ? operator
  let tokens = tokenize(input)?;
  let ast = parse_tokens(tokens)?;
  Ok(ast)
}

// Custom error types
#[derive(Debug)]
enum ParseError {
  InvalidSyntax { line: usize, msg: String },
  TypeError { expected: String, found: String },
}

// Implement Display + Error
impl fmt::Display for ParseError { ... }
impl std::error::Error for ParseError { ... }
```

### Naming

- **Types**: `PascalCase` (e.g., `CanonicalNormalizer`)
- **Functions**: `snake_case` (e.g., `normalize_contract`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_CONTRACT_SIZE`)

## Testing Strategy

### Unit Tests (75% of tests)

Test individual functions:
- Parsing, normalization, verification
- Error cases
- Edge cases
- Determinism

### Integration Tests (20% of tests)

Test multiple components:
- Parser → Normalizer → Verifier
- End-to-end contract validation

### System Tests (5% of tests)

Test against real contracts:
- Language binding compatibility
- Performance benchmarks
- Determinism under load

## Commit Guidelines

Format:
```
[COMPONENT] Brief description

Longer explanation:
- What changed
- Why it changed
- How it maintains Core integrity
- Determinism proof (if relevant)
```

Example:
```
[verifier] Add type-checking for union types

Changed: Type checker now validates Union types correctly
Why: Core ICL spec Section 2.3 requires union support
How: Added recursive type checking with cycle detection
Determinism: Type checker output is deterministic (proof: test_determinism_100_iterations)
```

## Code Review Checklist

Reviewers verify:

- [ ] Is Core ICL unchanged (or only clarified)?
- [ ] Is determinism maintained?
- [ ] Are 100+ iteration tests passing?
- [ ] Do language bindings still work?
- [ ] Is error handling complete?
- [ ] Are comments clear?

## Pull Request Process

1. Create branch: `feature/short-description`
2. Make minimal, focused changes
3. Add tests (determinism mandatory)
4. Update relevant docs
5. Submit PR with clear description
6. Address review comments
7. Merge once approved + CI passes

## API Stability

**Stable APIs** (`pub` in core modules):
- Cannot change without major version bump
- Must maintain backward compatibility

**Unstable APIs** (`pub(crate)` or under `_unstable`):
- Can change in minor versions
- Must be documented as unstable

## Deprecation Policy

1. Mark function with `#[deprecated]`
2. Document replacement
3. Deprecation lasts minimum 1 minor version
4. Only remove in major version

Example:
```rust
#[deprecated(since = "0.2.0", note = "Use normalize_contract instead")]
pub fn normalize_space(space: &str) -> Result<String> { ... }
```

## Documentation

Every public item needs:
- Doc comment explaining purpose
- Example of use (if non-obvious)
- Determinism guarantees (if applicable)

Example:
```rust
/// Normalize contract to canonical form.
/// 
/// # Guarantees
/// - Idempotent: normalize(normalize(x)) == normalize(x)
/// - Deterministic: Same input always produces same output
/// - No information loss: All semantics preserved
/// 
/// # Example
/// ```rust
/// let contract = r#"Contract { ... }"#;
/// let canonical = normalize(contract)?;
/// ```
pub fn normalize(input: &str) -> Result<String> { ... }
```

## Performance

Optimization is welcome, but:
1. Never sacrifice determinism
2. Never sacrifice clarity
3. Benchmark before/after
4. Document assumptions

## Questions?

- Check [ICL-Spec/CORE-SPECIFICATION.md](https://github.com/ICL-System/ICL-Spec/blob/main/spec/CORE-SPECIFICATION.md) (authoritative)
- Check existing tests (working examples)
- Ask on GitHub issues

Welcome to ICL! 🎉
