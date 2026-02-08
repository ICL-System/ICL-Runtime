# ICL JavaScript/TypeScript Bindings

JavaScript and TypeScript bindings for the [ICL (Intent Contract Language)](https://github.com/ICL-System/ICL-Runtime) runtime.

Built with [wasm-pack](https://rustwasm.github.io/wasm-pack/) — the canonical Rust implementation compiled to WebAssembly.

## Status: Alpha

This package is in early development. The API may change.

## Install

```bash
# From source (requires Rust toolchain + wasm-pack)
cargo install wasm-pack
cd bindings/javascript
wasm-pack build --target nodejs
```

## Usage

```javascript
import { parseContract, normalize, verify, execute, semanticHash } from './pkg/icl_runtime.js';

const contractText = fs.readFileSync('my-contract.icl', 'utf-8');

// Parse
const parsed = JSON.parse(parseContract(contractText));

// Normalize (deterministic canonical form)
const normalized = normalize(contractText);

// Verify (type checking, invariants, determinism)
const result = JSON.parse(verify(contractText));
if (result.valid) {
  console.log('Contract is valid!');
}

// Execute
const output = JSON.parse(execute(contractText, '{"operation": "greet", "inputs": {"name": "World"}}'));

// Semantic hash (SHA-256)
const hash = semanticHash(contractText);
```

## TypeScript

Full TypeScript definitions are provided in `icl.d.ts`.

## Guarantees

- **Deterministic**: Same input always produces identical output
- **Identical to Rust**: All results match the canonical Rust implementation exactly
- **Zero logic in bindings**: All behavior comes from `icl-core` via WASM

## Development

```bash
# Build for Node.js
wasm-pack build --target nodejs

# Build for browsers
wasm-pack build --target web

# Run tests
node tests/test_icl.mjs
```
