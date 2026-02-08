# ICL Runtime

> **Status: Phase 1 — Parser**
>
> This is the canonical Rust implementation of the [Intent Contract Language (ICL)](https://github.com/ICL-System/ICL-Spec) specification. All core logic is written once in Rust and will be compiled to every target: native binary, Python (PyO3), JavaScript/WASM (wasm-pack), and Go (cgo).

## What This Is

ICL Runtime is the **single implementation** of the ICL specification:
- **One codebase** — all logic lives in Rust
- **Every platform** — compiles to native, Python, JS/WASM, Go
- **Deterministic** — same input always produces identical output
- **Verifiable** — all properties machine-checkable
- **Bounded** — all execution bounded in memory and time

## Architecture

```
ICL Text → Parser → AST → Normalizer → Canonical Form
                           ↓
                        Verifier → Type Check + Invariants + Determinism
                           ↓
                        Executor → Sandboxed Execution
```

## Current State (Honest)

| Component | Status |
|-----------|--------|
| Tokenizer | **Done** — full scanner, 26 tests, 100-iteration determinism proof |
| AST Types | **Done** — all node types matching BNF grammar, Display impls, 8 tests |
| Parser | In progress — recursive descent (Phase 1.3) |
| Normalizer | Stub — passes input through unchanged |
| Verifier | Stub — returns `Ok(())` for anything |
| Executor | Stub — returns empty string |
| CLI | Scaffolded — all subcommands print "not yet implemented" |
| Bindings | Planned for Phase 6 (Python/PyO3, JS/WASM, Go/cgo) |

## Development

### Prerequisites
- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

### Building

```bash
cargo build              # Build all crates
cargo test               # Run all tests (49 tests pass)
cargo build -p icl-cli   # Build CLI only
```

### Project Structure

```
ICL-Runtime/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── icl-core/           # Library: parser, normalizer, verifier, executor
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── error.rs
│   │       ├── parser/     # Tokenizer + AST + recursive descent
│   │       │   ├── mod.rs
│   │       │   ├── tokenizer.rs
│   │       │   └── ast.rs
│   │       ├── normalizer.rs
│   │       ├── verifier.rs
│   │       └── executor.rs
│   └── icl-cli/            # Binary: `icl validate`, `icl verify`, etc.
│       └── src/main.rs
├── tests/
│   ├── integration/
│   ├── conformance/
│   └── determinism/
└── benches/
```

## Related Repositories

| Repo | Purpose |
|------|---------|
| [ICL-Spec](https://github.com/ICL-System/ICL-Spec) | The standard: BNF grammar, specification, conformance tests |
| [ICL-Docs](https://github.com/ICL-System/ICL-Docs) | Documentation website (mdBook) |

## License

Apache License 2.0 — See [LICENSE](LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.
