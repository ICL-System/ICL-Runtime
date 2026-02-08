# ICL Go Bindings

Go bindings for the [ICL (Intent Contract Language)](https://github.com/ICL-System/ICL-Runtime) runtime.

Built with cgo + cbindgen — thin wrapper around the canonical Rust implementation via C FFI.

## Status: Alpha

This package is in early development. The API may change.

## Prerequisites

- Rust toolchain (for building the FFI shared library)
- Go 1.21+
- GCC (for cgo)

## Build

```bash
# Build the Rust FFI library first
cd ICL-Runtime
cargo build --release -p icl-ffi

# Run Go tests
cd bindings/go
LD_LIBRARY_PATH=../../target/release go test -v ./...
```

## Usage

```go
package main

import (
    "encoding/json"
    "fmt"
    "log"
    "os"

    icl "github.com/ICL-System/ICL-Runtime/bindings/go"
)

func main() {
    text, _ := os.ReadFile("my-contract.icl")

    // Parse
    parsed, err := icl.ParseContract(string(text))
    if err != nil { log.Fatal(err) }

    // Normalize
    normalized, err := icl.Normalize(string(text))
    if err != nil { log.Fatal(err) }

    // Verify
    verified, err := icl.Verify(string(text))
    if err != nil { log.Fatal(err) }
    var result map[string]interface{}
    json.Unmarshal([]byte(verified), &result)
    fmt.Println("Valid:", result["valid"])

    // Execute
    output, err := icl.Execute(string(text), `{"operation": "greet", "inputs": {"name": "World"}}`)
    if err != nil { log.Fatal(err) }

    // Semantic hash
    hash, err := icl.SemanticHash(string(text))
    if err != nil { log.Fatal(err) }
    fmt.Println("Hash:", hash)
}
```

## Guarantees

- **Deterministic**: Same input always produces identical output
- **Identical to Rust**: All results match the canonical Rust implementation exactly
- **Zero logic in bindings**: All behavior comes from `icl-core` via C FFI
