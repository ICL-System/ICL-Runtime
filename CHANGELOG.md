# Changelog

All notable changes to ICL Runtime will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Phase 0 — Restructure
- Cargo workspace with `icl-core` (library) and `icl-cli` (binary) crates
- Parser module structure: `parser/mod.rs`, `tokenizer.rs`, `ast.rs`
- Error types: ParseError, TypeError, DeterminismViolation, ContractViolation, ValidationError, ExecutionError, NormalizationError
- Test directory structure: integration/, conformance/, determinism/
- CONTRIBUTING.md, CHANGELOG.md, README.md

#### Phase 1 — Parser
- **Tokenizer** — full character-by-character scanner for all ICL syntax (26 tests, 100-iteration determinism proof)
- **AST types** — complete node definitions matching BNF grammar: ContractNode, IdentityNode, TypeExpression, OperationNode, etc. (8 tests)
- **Recursive descent parser** — full parser for all 7 contract sections plus Extensions
- Type expression parsing (primitives, composites, collections)
- Error recovery (reports multiple errors per parse)
- Conformance tests against ICL-Spec valid/ and invalid/ fixtures

#### Phase 2 — Normalizer
- Section sorting (canonical order)
- Field sorting within sections (alphabetical)
- Whitespace normalization (2-space indent, trailing commas)
- Comment removal
- Type normalization and canonical serialization
- SHA-256 semantic hash computation
- Idempotence proof: `normalize(normalize(x)) == normalize(x)`
- Determinism test: 100 iterations identical output

#### Phase 3 — Verifier
- Type checker: inference, checking, composite types, no implicit coercion
- Invariant verifier: initial state, operation preservation, consistency, unsatisfiable detection
- Determinism checker: randomness, time access, I/O, floating-point, hash order
- Coherence verifier: pre/postcondition consistency, circular dependencies, resource limits, namespace isolation

#### Phase 4 — CLI
- 9 commands: `icl-cli validate`, `normalize`, `verify`, `fmt`, `hash`, `diff`, `init`, `execute`, `version`
- Colored output, `--json` and `--quiet` flags
- Exit codes: 0 = success, 1 = validation failure, 2 = error
- 28 CLI integration tests

#### Phase 5 — Executor
- Pure-Rust sandbox environment (no WASM — ICL is declarative)
- Precondition evaluation and postcondition verification
- Resource limit enforcement (memory, time)
- Provenance logging (every state change recorded)
- Determinism test: 100 iterations identical output

#### Phase 6 — Language Bindings
- **Python binding** (`bindings/python/`) — PyO3 + maturin, exposes parse/normalize/verify/execute, 18 tests
- **JavaScript binding** (`bindings/javascript/`) — wasm-pack, npm package `icl-runtime`, TypeScript definitions, 31 tests
- **Go binding** (`bindings/go/`) — cgo FFI with cbindgen, exposes all core functions, 16 tests
- All bindings produce identical results to Rust core

#### Phase 7 — Documentation Site
- mdBook documentation at https://icl-system.github.io/ICL-Docs/
- Getting-started tutorial, contract authoring guide, CLI reference
- API reference, architecture docs, implementation guide
- Testing guide, integrations overview
- GitHub Pages deployment

#### Phase 8 — CI/CD
- GitHub Actions: `ci.yml` (fmt, clippy, cross-platform test, conformance, determinism)
- GitHub Actions: `bindings.yml` (Python, JavaScript, Go builds + tests)
- GitHub Actions: `publish.yml` (crates.io, PyPI, npm on `v*` tag)
- GitHub Actions: `deploy.yml` (mdBook → GitHub Pages)
- Branch protection rules on main (5 required checks)

#### Phase 9 — Conformance Suite
- 57 valid contract fixtures (ICL-Spec)
- 55 invalid contract fixtures targeting specific errors (ICL-Spec)
- 25 normalization input→expected pairs (ICL-Spec)
- Cross-implementation test runner (`run-conformance.sh`)
- Determinism proof suite (100+ iterations)
- Conformance suite versioned at 0.1.0

### Changed
- Restructured from flat `runtime/src/` to Cargo workspace under `crates/`
- Moved specification to separate ICL-Spec repository
- Renamed npm package from `icl-wasm` to `icl-runtime`
- CLI binary is `icl-cli` (not `icl`)

### Removed
- Removed empty `bindings/` directory stubs (recreated with actual code in Phase 6)

## [0.0.0] - 2025-02-01

### Added
- Initial project structure with stub implementations
- Core types: Contract, Identity, PurposeStatement, DataSemantics, BehavioralSemantics, ExecutionConstraints, HumanMachineContract
- CONTRIBUTING.md with development guidelines
