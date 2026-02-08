# ICL Project — Roadmap & Progress Tracker

**Started:** 2026-02-08
**Status:** Phase 9 — Conformance Suite ✅

> Check boxes as each step is completed. Each phase must be finished before starting the next.

---

## Phase 0: Restructure & Clean

### 0.1 — Create ICL-Spec Repository

- [x] Create `ICL-System/ICL-Spec` repo on GitHub
- [x] Add LICENSE (MIT OR Apache-2.0)
- [x] Create `spec/` directory
- [x] Move `CORE-SPECIFICATION.md` from ICL-Runtime → ICL-Spec/spec/
- [x] Create `grammar/icl.bnf` — extract formal BNF from spec into standalone file
- [x] Create `examples/` directory
- [x] Extract example contracts from `example-contracts.md` into individual `.icl` files
  - [x] `examples/db-write-validation.icl`
  - [x] `examples/api-rate-limiting.icl`
  - [x] `examples/agent-action-verification.icl`
  - [x] `examples/hello-world.icl`
- [x] Create `conformance/` directory structure
  - [x] `conformance/valid/` — contracts that must parse
  - [x] `conformance/invalid/` — contracts that must fail
  - [x] `conformance/normalization/` — input→expected pairs
- [x] Move `standardization-roadmap.md` → `ICL-Spec/roadmap/`
- [x] Write ICL-Spec `README.md`
- [x] Add initial conformance test fixtures (at least 5 valid, 5 invalid)
- [x] Commit and push

### 0.2 — Restructure ICL-Runtime

- [x] Create workspace `Cargo.toml` at repo root
- [x] Create `crates/icl-core/` directory
- [x] Move `runtime/src/*.rs` → `crates/icl-core/src/`
- [x] Move `runtime/Cargo.toml` → `crates/icl-core/Cargo.toml` (update paths)
- [x] Create `crates/icl-cli/` with stub `main.rs` and `Cargo.toml`
- [x] Refactor `crates/icl-core/src/` into proper module structure:
  - [x] Create `parser/` module (mod.rs, tokenizer.rs, ast.rs)
  - [x] Move normalizer, verifier, executor, error into place
  - [x] Update `lib.rs` to reflect new module paths
- [x] Delete old `runtime/` directory
- [x] Delete empty `bindings/python/`, `bindings/javascript/`, `bindings/go/`
- [x] Delete empty `examples/`
- [x] Delete empty `tests/`
- [x] Delete `docs/` directory (content moved to ICL-Docs and ICL-Spec)
- [x] Delete `spec/` directory (moved to ICL-Spec)
- [x] Create `tests/` directory with proper structure
  - [x] `tests/integration/`
  - [x] `tests/conformance/`
  - [x] `tests/determinism/`
- [x] Create `benches/` directory
- [x] Verify `cargo build` succeeds
- [x] Verify `cargo test` succeeds
- [x] Update `README.md` — honest status, correct structure, no false install commands
- [x] Update `CONTRIBUTING.md` — reflect new file structure
- [x] Add `CHANGELOG.md`
- [x] Commit and push

### 0.3 — Restructure ICL-Docs

- [x] Install mdBook (`cargo install mdbook`)
- [x] Initialize mdBook in ICL-Docs (`mdbook init`)
- [x] Create `book.toml` configuration
- [x] Create `src/SUMMARY.md` (table of contents)
- [x] Move docs from ICL-Runtime to ICL-Docs/src/:
  - [x] `getting-started.md`
  - [x] `api-reference.md`
  - [x] `implementation-guide.md`
  - [x] `runtime-architecture.md`
  - [x] `testing.md`
- [x] Create new pages:
  - [x] `src/introduction.md`
  - [x] `src/writing-contracts.md`
  - [x] `src/cli-reference.md` (stub for now)
  - [x] `src/integrations/overview.md`
- [x] Update all docs to reflect actual project state (remove false claims)
- [x] Verify `mdbook build` succeeds
- [x] Write ICL-Docs `README.md`
- [x] Commit and push

### 0.4 — Cross-Repo Linking

- [x] ICL-Spec README links to ICL-Runtime and ICL-Docs
- [x] ICL-Runtime README links to ICL-Spec and ICL-Docs
- [x] ICL-Docs README links to ICL-Spec and ICL-Runtime
- [x] All repos have consistent LICENSE
- [x] Verify all links work (repos are private, verified in browser)

---

## Phase 1: Parser

### 1.1 — Tokenizer

- [x] Define `Token` enum (all ICL token types)
- [x] Implement `Tokenizer` struct
- [x] Handle: keywords, identifiers, strings, integers, floats, ISO8601, UUID
- [x] Handle: braces, colons, commas, brackets
- [x] Handle: comments (skip them)
- [x] Error reporting with line/column numbers
- [x] Unit tests: valid tokens
- [x] Unit tests: invalid tokens
- [x] Unit tests: edge cases (empty input, unicode, max lengths)

### 1.2 — AST Types

- [x] Define AST node types matching BNF grammar
- [x] `ContractNode`, `IdentityNode`, `PurposeStatementNode`, etc.
- [x] Type expression nodes (`PrimitiveType`, `CompositeType`, `CollectionType`)
- [x] Operation node with all fields
- [x] Extension nodes
- [x] Implement `Display` for all AST nodes (for pretty printing)

### 1.3 — Parser

- [x] Implement recursive descent parser
- [x] Parse top-level `Contract { ... }`
- [x] Parse `Identity` section
- [x] Parse `PurposeStatement` section
- [x] Parse `DataSemantics` section (state + invariants)
- [x] Parse `BehavioralSemantics` section (operations)
- [x] Parse `ExecutionConstraints` section
- [x] Parse `HumanMachineContract` section
- [x] Parse `Extensions` section (optional)
- [x] Parse type expressions (primitives, composites, collections)
- [x] Error recovery (don't stop at first error)
- [x] Unit tests against ICL-Spec conformance/valid/
- [x] Unit tests against ICL-Spec conformance/invalid/
- [x] Determinism test: 100 iterations identical output

---

## Phase 2: Normalizer

- [x] Implement section sorting (alphabetical)
- [x] Implement field sorting within sections
- [x] Implement whitespace normalization
- [x] Implement comment removal
- [x] Implement type normalization (shorthand → full form)
- [x] Implement default expansion
- [x] Implement SHA-256 semantic hash computation
- [x] Implement canonical serialization (AST → string)
- [x] Idempotence proof: `normalize(normalize(x)) == normalize(x)`
- [x] Determinism test: 100 iterations identical output
- [x] Test against ICL-Spec conformance/normalization/ pairs
- [x] Semantic preservation test: parse(normalize(x)) == parse(x)

---

## Phase 3: Verifier

### 3.1 — Type Checker

- [x] Implement type inference for state fields
- [x] Implement type checking for operation parameters
- [x] Verify postcondition type consistency
- [x] Detect type mismatches
- [x] Enforce no implicit coercion
- [x] Unit tests for each type (Integer, Float, String, Boolean, ISO8601, UUID)
- [x] Unit tests for composite types (Object, Enum)
- [x] Unit tests for collection types (Array, Map)

### 3.2 — Invariant Verifier

- [x] Check invariants can hold for initial state
- [x] Check operations preserve invariants
- [x] Check invariant consistency (no contradictions)
- [x] Detect unsatisfiable invariants

### 3.3 — Determinism Checker

- [x] Detect randomness usage
- [x] Detect system time access
- [x] Detect external I/O
- [x] Detect floating-point non-determinism
- [x] Detect hash iteration order dependencies

### 3.4 — Coherence Verifier

- [x] Check precondition/postcondition consistency
- [x] Detect circular dependencies
- [x] Verify resource limits are feasible
- [x] Verify extension namespace isolation

---

## Phase 4: CLI

- [x] Set up `clap` in `icl-cli`
- [x] Implement `icl validate <file.icl>`
- [x] Implement `icl normalize <file.icl>`
- [x] Implement `icl verify <file.icl>`
- [x] Implement `icl fmt <file.icl>`
- [x] Implement `icl hash <file.icl>`
- [x] Implement `icl diff <a.icl> <b.icl>`
- [x] Implement `icl init` (scaffold new contract)
- [x] Implement `icl version`
- [x] Colored output (errors in red, success in green)
- [x] Exit codes (0 = success, 1 = validation failure, 2 = error)
- [x] `--json` flag for machine-readable output
- [x] `--quiet` flag for CI usage
- [x] Man page / help text
- [x] Integration tests for all commands
- [x] Verify `cargo install icl-cli` works

---

## Phase 5: Executor

- [x] Research WASM runtime options (wasmtime vs wasmer)
  - Decision: Pure-Rust executor (no WASM needed — ICL is declarative, not executable code)
- [x] Implement sandbox environment
- [x] Implement precondition evaluation
- [x] Implement operation execution in sandbox
- [x] Implement postcondition verification
- [x] Implement resource limit enforcement (memory, time)
- [x] Implement provenance logging (every state change recorded)
- [x] Implement `icl execute <file.icl> --input '{}'` CLI command
- [x] Determinism test: 100 iterations identical output
- [x] Benchmark execution performance

---

## Phase 6: Language Bindings

### 6.1 — Python Binding (PyO3 + maturin)

- [x] Set up `bindings/python/` with PyO3
- [x] Create `pyproject.toml` with maturin build backend
- [x] Expose `parse_contract()` to Python
- [x] Expose `normalize()` to Python
- [x] Expose `verify()` to Python
- [x] Expose `execute()` to Python
- [x] Python type stubs (`.pyi` files)
- [x] Test: Python produces identical results to Rust
- [x] Test: `pip install` from local build works
- [x] Publish to PyPI (test.pypi.org first)

### 6.2 — JavaScript Binding (wasm-pack)

- [x] Set up `bindings/javascript/` with wasm-pack
- [x] Create `package.json`
- [x] Expose `parseContract()` to JS
- [x] Expose `normalize()` to JS
- [x] Expose `verify()` to JS
- [x] Expose `execute()` to JS
- [x] TypeScript type definitions (`.d.ts` files)
- [x] Test: JS produces identical results to Rust
- [x] Test: `npm install` from local build works
- [x] Publish to npm (--dry-run first)

### 6.3 — Go Binding (cgo)

- [x] Set up `bindings/go/` with cgo FFI
- [x] Generate C headers from Rust (`cbindgen`)
- [x] Create Go wrapper functions
- [x] Expose `ParseContract()` to Go
- [x] Expose `Normalize()` to Go
- [x] Expose `Verify()` to Go
- [x] Expose `Execute()` to Go
- [x] Test: Go produces identical results to Rust
- [x] Test: `go get` from local works

---

## Phase 7: Documentation Site

- [x] Configure mdBook theme and styling
- [x] Write comprehensive introduction page
- [x] Write getting-started tutorial (step by step with working examples)
- [x] Write contract authoring guide
- [x] Write CLI reference (all commands documented)
- [x] Write API reference (all public functions documented)
- [x] Write architecture explanation
- [x] Write testing guide
- [x] Add code examples that actually run
- [x] Set up GitHub Pages deployment
- [ ] Configure custom domain (`iclstandard.org` when ready)
- [x] Add search functionality

---

## Phase 8: CI/CD

- [x] GitHub Actions: Rust build + test on push
- [x] GitHub Actions: Clippy linting
- [x] GitHub Actions: Cargo fmt check
- [x] GitHub Actions: Conformance tests against ICL-Spec
- [x] GitHub Actions: Determinism tests (100 iterations)
- [x] GitHub Actions: Build Python wheel (maturin)
- [x] GitHub Actions: Build WASM package (wasm-pack)
- [x] GitHub Actions: Build Go shared library
- [x] GitHub Actions: Publish to crates.io on tag
- [x] GitHub Actions: Publish to PyPI on tag
- [x] GitHub Actions: Publish to npm on tag
- [x] GitHub Actions: Deploy docs to GitHub Pages on merge
- [x] GitHub Actions: Cross-platform tests (Linux, macOS, Windows)
- [x] Branch protection rules on main

---

## Phase 9: Conformance Suite

- [x] 50+ valid contract test fixtures
- [x] 50+ invalid contract test fixtures (each targets specific error)
- [x] 20+ normalization input→expected pairs
- [x] Determinism proof suite (contracts run 100+ times)
- [x] Cross-implementation test runner script
- [x] Document how alternative implementations run conformance tests
- [x] Version the conformance suite (matching spec version)

---

## Phase 10: Standardization

- [ ] Write RFC document
- [ ] Set up advisory board structure
- [ ] Define governance model
- [ ] Collect vendor interest (3+ vendors minimum)
- [ ] Register `iclstandard.org` domain
- [ ] Build standards org website
- [ ] Write research paper on intent languages
- [ ] Submit to standards body (IETF/W3C/equivalent)
- [ ] Collect 5+ vendor support letters
- [ ] Present at conferences / publish blog posts

---

## Future — Post-Standardization

- [ ] VS Code extension (`icl-vscode`) — syntax highlighting + validation
- [ ] GitHub Action (`icl-action`) — CI contract validation
- [ ] VibeTensor integration exploration
- [ ] Community extension registry
- [ ] Alternative implementations (Python-native, etc.)
- [ ] Academic partnerships
- [ ] 1000+ GitHub stars milestone
