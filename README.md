# ICL Runtime

> **Status: Phases 0–9 Complete**
>
> This is the canonical Rust implementation of the [Intent Contract Language (ICL)](https://github.com/ICL-System/ICL-Spec) specification. All core logic is written once in Rust and compiled to every target: native binary, Python (PyO3), JavaScript/WASM (wasm-pack), and Go (cgo).

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

## Current Status

| Component | Status | Tests |
|-----------|--------|-------|
| Tokenizer | **Complete** | 26 tests + determinism proof |
| Parser (recursive descent) | **Complete** | 50+ tests |
| Normalizer (canonical + SHA-256) | **Complete** | 30+ tests |
| Verifier (types, invariants, determinism, coherence) | **Complete** | 40+ tests |
| Executor (sandbox + provenance) | **Complete** | 20+ tests |
| CLI (9 commands) | **Complete** | 28 tests |
| Python binding (PyO3) | **Complete** | 18 tests |
| JavaScript binding (WASM) | **Complete** | 31 tests |
| Go binding (cgo) | **Complete** | 16 tests |

**Total: 203 Rust tests + 65 binding tests = 268 tests passing**

## Development

### Prerequisites
- Rust 1.93+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- For Python: Python 3.8+, maturin
- For JavaScript: wasm-pack, Node.js 16+
- For Go: Go 1.21+

### Building

```bash
cargo build              # Build all crates
cargo test --workspace   # Run all tests (203 Rust tests)
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
│   └── icl-cli/            # Binary: `icl-cli validate`, `icl-cli verify`, etc.
│       └── src/main.rs
├── bindings/
│   ├── python/             # PyO3 binding (pip: icl-runtime)
│   ├── javascript/         # WASM binding (npm: icl-runtime)
│   └── go/                 # cgo binding
├── tests/
│   ├── integration/
│   ├── conformance/
│   └── determinism/
├── benches/
└── .github/workflows/      # CI/CD (ci, bindings, publish, deploy)
```

## Related Repositories

| Repo | Purpose |
|------|---------|
| [ICL-Spec](https://github.com/ICL-System/ICL-Spec) | The standard: BNF grammar, specification, conformance tests |
| [ICL-Docs](https://github.com/ICL-System/ICL-Docs) | Documentation website (mdBook) |

## License

MIT — See [LICENSE](LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.
