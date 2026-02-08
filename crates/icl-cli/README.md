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
| `icl-cli parse <file>` | Parse an ICL file and display the AST |
| `icl-cli validate <file>` | Validate syntax and structure |
| `icl-cli verify <file>` | Run full verification (types, invariants, determinism) |
| `icl-cli execute <file>` | Execute a contract in the sandbox |
| `icl-cli normalize <file>` | Output canonical form |
| `icl-cli hash <file>` | Compute SHA-256 content hash |
| `icl-cli inspect <file>` | Show detailed contract metadata |
| `icl-cli pipeline <file>` | Run the full pipeline (parse → normalize → verify → execute) |
| `icl-cli completions <shell>` | Generate shell completions (bash, zsh, fish, powershell) |

## Options

All commands support:
- `--json` — Output as JSON
- `--verbose` / `-v` — Verbose output

## Example

```bash
$ icl-cli validate contract.icl
✓ contract.icl is valid

$ icl-cli verify contract.icl --json
{"status":"verified","checks":{"type_check":"pass","invariants":"pass","determinism":"pass","coherence":"pass"}}

$ icl-cli hash contract.icl
sha256:a1b2c3d4...
```

## License

MIT — See [LICENSE](../../LICENSE) for details.
