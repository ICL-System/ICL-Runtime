# icl-core

> Canonical Rust implementation of the [Intent Contract Language (ICL)](https://github.com/ICL-System/ICL-Spec) specification.

`icl-core` is the core library that implements the full ICL processing pipeline: parsing, normalization, verification, and execution. All logic is written once in Rust and compiled to every target (native, Python/PyO3, JavaScript/WASM, Go/cgo).

## Features

- **Parser** — Tokenizer + recursive descent parser producing a typed AST
- **Normalizer** — Canonical form generation with SHA-256 content hashing
- **Verifier** — Type checking, invariant validation, determinism proof, coherence checks
- **Executor** — Sandboxed execution with provenance tracking
- **Deterministic** — Same input always produces identical output, guaranteed

## Usage

```rust
use icl_core::parser;
use icl_core::normalizer;
use icl_core::verifier;
use icl_core::executor;

let source = std::fs::read_to_string("contract.icl").unwrap();

// Parse ICL source into AST
let ast = parser::parse(&source).unwrap();

// Normalize to canonical form
let canonical = normalizer::normalize(&source).unwrap();

// Compute semantic hash
let hash = normalizer::compute_semantic_hash(&ast);

// Verify all properties (types, invariants, determinism, coherence)
let result = verifier::verify(&ast);
assert!(result.is_valid());

// Parse into high-level Contract and execute
let contract = parser::parse_contract(&source).unwrap();
let output = executor::execute_contract(
    &contract,
    r#"{"operation":"echo","inputs":{"message":"Hello"}}"#,
).unwrap();
```

## Architecture

```
ICL Text → Parser → AST → Normalizer → Canonical Form
                           ↓
                        Verifier → Type Check + Invariants + Determinism
                           ↓
                        Executor → Sandboxed Execution
```

## Testing

203 tests covering every pipeline stage plus determinism proofs:

```bash
cargo test -p icl-core
```

## License

MIT — See [LICENSE](../../LICENSE) for details.
