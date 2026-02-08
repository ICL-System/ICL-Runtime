// Package icl provides Go bindings for the ICL (Intent Contract Language) runtime.
//
// All functions are thin wrappers around the canonical Rust implementation via cgo FFI.
// Deterministic: same input always produces identical output.
//
// # Memory Safety
//
// All C strings returned by the Rust FFI are automatically freed by these Go wrappers.
// Users of this package do not need to manage C memory.
package icl

/*
#cgo CFLAGS: -I${SRCDIR}
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -licl_ffi -lm -ldl -lpthread
#include "icl.h"
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"unsafe"
)

// handleResult converts an IclResult to a Go string and error,
// freeing the C strings.
func handleResult(r C.struct_IclResult) (string, error) {
	defer func() {
		if r.result != nil {
			C.icl_free_string(r.result)
		}
		if r.error != nil {
			C.icl_free_string(r.error)
		}
	}()

	if r.error != nil {
		return "", fmt.Errorf("%s", C.GoString(r.error))
	}
	return C.GoString(r.result), nil
}

// ParseContract parses ICL contract text and returns a JSON string of the parsed Contract.
//
// Returns an error if the contract text has syntax or semantic errors.
func ParseContract(text string) (string, error) {
	cText := C.CString(text)
	defer C.free(unsafe.Pointer(cText))

	return handleResult(C.icl_parse_contract(cText))
}

// Normalize normalizes ICL contract text to canonical form.
//
// Guarantees:
//   - Deterministic: same input → same output
//   - Idempotent: Normalize(Normalize(x)) == Normalize(x)
//   - Semantic preserving: meaning is unchanged
func Normalize(text string) (string, error) {
	cText := C.CString(text)
	defer C.free(unsafe.Pointer(cText))

	return handleResult(C.icl_normalize(cText))
}

// Verify verifies an ICL contract for correctness.
//
// Returns JSON: { "valid": bool, "errors": [...], "warnings": [...] }
func Verify(text string) (string, error) {
	cText := C.CString(text)
	defer C.free(unsafe.Pointer(cText))

	return handleResult(C.icl_verify(cText))
}

// Execute executes an ICL contract with the given inputs.
//
// The inputs parameter should be a JSON string:
//
//	Single request: {"operation": "name", "inputs": {...}}
//	Multiple: [{"operation": "name", "inputs": {...}}, ...]
//
// Returns JSON with execution result including provenance log.
func Execute(text string, inputs string) (string, error) {
	cText := C.CString(text)
	defer C.free(unsafe.Pointer(cText))
	cInputs := C.CString(inputs)
	defer C.free(unsafe.Pointer(cInputs))

	return handleResult(C.icl_execute(cText, cInputs))
}

// SemanticHash computes the SHA-256 semantic hash of a contract.
//
// The hash is computed from the normalized (canonical) form,
// so semantically equivalent contracts produce the same hash.
func SemanticHash(text string) (string, error) {
	cText := C.CString(text)
	defer C.free(unsafe.Pointer(cText))

	return handleResult(C.icl_semantic_hash(cText))
}
