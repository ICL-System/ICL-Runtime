# ICL Python Bindings

Python bindings for the [ICL (Intent Contract Language)](https://github.com/ICL-System/ICL-Runtime) runtime.

Built with [PyO3](https://pyo3.rs) + [maturin](https://www.maturin.rs) — thin wrapper around the canonical Rust implementation.

## Status: Alpha

This package is in early development. The API may change.

## Install

```bash
pip install icl-runtime
```

Published on [PyPI](https://pypi.org/project/icl-runtime/).

Or build from source (requires Rust toolchain):

```bash
pip install maturin
cd bindings/python
maturin develop
```

## Usage

```python
import icl
import json

contract_text = open("my-contract.icl").read()

# Parse
parsed = json.loads(icl.parse_contract(contract_text))

# Normalize (deterministic canonical form)
normalized = icl.normalize(contract_text)

# Verify (type checking, invariants, determinism)
result = json.loads(icl.verify(contract_text))
if result["valid"]:
    print("Contract is valid!")

# Execute
output = json.loads(icl.execute(contract_text, '{"operation": "greet", "inputs": {"name": "World"}}'))

# Semantic hash (SHA-256)
hash_hex = icl.semantic_hash(contract_text)
```

## Guarantees

- **Deterministic**: Same input always produces identical output
- **Identical to Rust**: All results match the canonical Rust implementation exactly
- **Zero logic in bindings**: All behavior comes from `icl-core`

## Development

```bash
# Build and install in development mode
maturin develop

# Run tests
pytest tests/

# Build wheel
maturin build --release
```
