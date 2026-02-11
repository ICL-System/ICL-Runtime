# ICL JavaScript/TypeScript Bindings

JavaScript and TypeScript bindings for the [ICL (Intent Contract Language)](https://github.com/ICL-System/ICL-Runtime) runtime.

Built with [wasm-pack](https://rustwasm.github.io/wasm-pack/) — the canonical Rust implementation compiled to WebAssembly. Works out of the box in **Node.js**, **bundlers** (Vite, Webpack, Rollup), and **plain browsers**.

## Status: Alpha

This package is in early development. The API may change.

## Install

```bash
npm install icl-runtime
```

Published on [npm](https://www.npmjs.com/package/icl-runtime).

## Usage

### Node.js (CommonJS)

```javascript
const { parseContract, normalize, verify, execute, semanticHash } = require('icl-runtime');

const contractText = fs.readFileSync('my-contract.icl', 'utf-8');

// Parse → JSON AST
const ast = JSON.parse(parseContract(contractText));

// Normalize to deterministic canonical form
const canonical = normalize(contractText);

// Verify (type checking, invariants, determinism)
const result = JSON.parse(verify(contractText));
console.log(result.valid, result.errors, result.warnings);

// Execute with inputs
const output = JSON.parse(execute(contractText, '{"operation": "echo", "inputs": {"message": "Hi"}}'));

// Semantic hash (SHA-256 of normalized form)
const hash = semanticHash(contractText);
```

### Node.js (ES Modules)

```javascript
import { parseContract, normalize, verify, execute, semanticHash } from 'icl-runtime';

const ast = JSON.parse(parseContract(contractText));
```

### Bundlers (Vite, Webpack, Rollup)

```javascript
import { parseContract, normalize, verify, execute, semanticHash } from 'icl-runtime';

// Works with Vite (requires vite-plugin-wasm), Webpack 5, Rollup, etc.
const ast = JSON.parse(parseContract(contractText));
```

> **Vite users**: Add `vite-plugin-wasm` and `vite-plugin-top-level-await` to your Vite config, or use `optimizeDeps.exclude: ['icl-runtime']`.

### Browser (Script Tag)

```html
<script type="module">
  import init, { parseContract } from 'icl-runtime/web';

  // Must call init() first to load the WASM binary
  await init();

  const ast = JSON.parse(parseContract(contractText));
</script>
```

## TypeScript

Full TypeScript definitions are included — no `@types` package needed.

```typescript
import { parseContract, normalize, verify, execute, semanticHash } from 'icl-runtime';
// All functions: (text: string) => string  (execute takes two string args)
```

## API

All functions take ICL contract text as a string and return a string (JSON or plain text).

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `parseContract(text)` | ICL source | JSON AST | Parse contract into AST |
| `normalize(text)` | ICL source | Canonical ICL | Deterministic canonical form |
| `verify(text)` | ICL source | JSON `{valid, errors, warnings}` | Type check + invariant verification |
| `execute(text, inputs)` | ICL source + JSON inputs | JSON result | Sandboxed execution with provenance |
| `semanticHash(text)` | ICL source | Hex string | SHA-256 of normalized form |

## Guarantees

- **Deterministic**: Same input always produces identical output
- **Identical to Rust**: All results match the canonical Rust implementation exactly
- **Zero logic in bindings**: All behavior comes from `icl-core` via WASM

## Package Structure

```
icl-runtime/
├── dist/
│   ├── nodejs/     # CJS + ESM wrapper (Node.js require/import)
│   ├── bundler/    # ES modules (Vite, Webpack, Rollup)
│   └── web/        # Browser with async init()
├── package.json    # Conditional exports auto-select the right target
└── README.md
```

The correct target is selected automatically via [conditional exports](https://nodejs.org/api/packages.html#conditional-exports):
- `require('icl-runtime')` → `dist/nodejs/` (CJS)
- `import from 'icl-runtime'` in Node.js → `dist/nodejs/` (ESM wrapper)
- `import from 'icl-runtime'` in bundlers → `dist/bundler/`
- `import from 'icl-runtime/web'` → `dist/web/` (browser)

## Development

Requires: Rust toolchain + [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
# Build all 3 targets (cross-platform)
node build.mjs

# Run tests
node tests/test_icl.mjs
```

## License

MIT
