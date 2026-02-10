# icl-cli

> Command-line interface for the [Intent Contract Language (ICL)](https://github.com/ICL-System/ICL-Spec).

`icl-cli` provides 9 commands for working with ICL contracts from the terminal — parsing, validation, verification, execution, normalization, and more.

## Installation

```bash
cargo install icl-cli
```

## Commands

| Command | Description |
|---------|-------------|
| `icl-cli validate <file>` | Validate syntax and structure |
| `icl-cli normalize <file>` | Output canonical form |
| `icl-cli verify <file>` | Run full verification (types, invariants, determinism) |
| `icl-cli fmt <file>` | Format a contract to standard style |
| `icl-cli hash <file>` | Compute SHA-256 semantic hash |
| `icl-cli diff <a> <b>` | Semantic diff between two contracts |
| `icl-cli init [name]` | Scaffold a new ICL contract |
| `icl-cli execute <file>` | Execute a contract in the sandbox |
| `icl-cli version` | Show version information |

## Options

All commands support:
- `--json` — Output as JSON
- `--quiet` — Suppress non-error output (for CI usage)

## Example

```bash
$ icl-cli validate contract.icl
✓ contract.icl is valid

$ icl-cli verify contract.icl --json
{"file":"contract.icl","verified":true,...}

$ icl-cli hash contract.icl
1f7dcf67d92b813f3cc0402781f023ea33c76dd7c2b6963531fe68bf9c032cb8
```

## License

MIT — See [LICENSE](../../LICENSE) for details.
