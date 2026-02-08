# Changelog

All notable changes to ICL Runtime will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Cargo workspace with `icl-core` (library) and `icl-cli` (binary) crates
- Parser module structure: `parser/mod.rs`, `tokenizer.rs`, `ast.rs`
- **Tokenizer** — full character-by-character scanner for all ICL syntax (26 tests, 100-iteration determinism proof)
- **AST types** — complete node definitions matching BNF grammar: ContractNode, IdentityNode, TypeExpression, OperationNode, etc. (8 tests)
- Normalizer, verifier, and executor stubs
- Error types: ParseError, TypeError, DeterminismViolation, ContractViolation, ValidationError, ExecutionError, NormalizationError
- CLI scaffolding with subcommands: validate, normalize, verify, fmt, hash, diff, init, execute, version
- Test directory structure: integration/, conformance/, determinism/
- Honest README reflecting actual project state

### Changed
- Restructured from flat `runtime/src/` to Cargo workspace under `crates/`
- Moved specification to separate ICL-Spec repository

### Removed
- Removed empty `bindings/` directory stubs (will be recreated with actual code in Phase 6)

## [0.0.0] - 2025-02-01

### Added
- Initial project structure with stub implementations
- Core types: Contract, Identity, PurposeStatement, DataSemantics, BehavioralSemantics, ExecutionConstraints, HumanMachineContract
- CONTRIBUTING.md with development guidelines
