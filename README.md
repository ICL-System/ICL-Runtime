# Intent Contract Language (ICL)

A universal, **language-agnostic** specification language for intent contracts — like OpenAPI but for human intent and AI agent constraints.

## What is ICL?

ICL is a formal specification language that allows you to:
- **Define intent contracts** in a machine-readable, deterministic format
- **Validate operations** against declared constraints
- **Prove determinism** (same input → same output)
- **Share contracts** across different AI systems and programming languages
- **Verify guarantees** through formal methods

## Quick Start

```bash
# Install Python binding
pip install icl-runtime

# Install JavaScript binding
npm install icl-runtime

# Install Rust core
cargo add icl-runtime
```

## Core Features

- **Deterministic Execution**: Same input always produces identical output
- **Language-Agnostic**: One specification, bindings for Python, JavaScript, Go, Rust
- **Portable**: Run contracts in any language, on any platform
- **Verifiable**: Formal proof of execution correctness
- **Canonical**: Single normalized form for all contracts

## Project Structure

```
/icl/
├── spec/                    # Single source of truth (Core ICL specification)
├── runtime/                 # Canonical runtime implementation (Rust)
├── bindings/               # Language wrappers (Python, JavaScript, Go)
├── docs/                   # Comprehensive documentation
├── examples/               # Real-world examples
├── tests/                  # Test suite and fixtures
└── CONTRIBUTING.md         # Development guidelines
```

## Use Cases

ICL is used to:
- **Validate database operations** before execution
- **Constrain AI agent actions** deterministically
- **Enforce API contracts** in development tools
- **Verify robotics commands** before hardware execution
- **Test contract-based systems** at scale

## Architecture

**One Canonical Runtime:**
- Core implementation in **Rust** (deterministic, verifiable, performant)
- Thin language bindings wrap the core (no reimplementation)
- All runtimes execute identical semantics

**Multi-Language Support:**
- Python: `pip install icl-runtime`
- JavaScript/Node: `npm install icl-runtime`
- Go: `go get icl-runtime`
- Rust: `cargo add icl-runtime`
- Docker: `docker pull icl-runtime`

## License

MIT or Apache 2.0 (open-source)

## Roadmap

- **Q1 2026**: Open-source Core validator + reference parser
- **Q2 2026**: RFC for Extension standards, governance model
- **Q3 2026**: First alternative implementation (Python/JavaScript)
- **Q4 2026**: Standardization proposal (`iclstandard.org`)
- **2027+**: Community adoption, industry standard

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development guidelines.

## Documentation

- [Getting Started](./docs/getting-started.md)
- [Language Specification](./spec/grammar.md)
- [Type System](./spec/types.md)
- [API Reference](./docs/api-reference.md)
- [Examples](./examples/)
