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
use icl_core::{parse, normalize, verify, execute};

let source = r#"
contract HelloWorld {
  version: "1.0"
  identity { name: "hello" }
  intent { action: "greet" }
}
"#;

// Parse ICL source into AST
let ast = parse(source).unwrap();

// Normalize to canonical form
let canonical = normalize(&ast).unwrap();

// Verify all properties
let report = verify(&ast).unwrap();

// Execute in sandbox
let result = execute(&ast).unwrap();
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
