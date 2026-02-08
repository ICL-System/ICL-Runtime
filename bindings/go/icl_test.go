package icl

import (
	"encoding/json"
	"strings"
	"testing"
)

const helloWorld = `Contract {
  Identity {
    stable_id: "ic-hello-001",
    version: 1,
    created_timestamp: 2026-02-08T00:00:00Z,
    owner: "test",
    semantic_hash: "abc123"
  }

  PurposeStatement {
    narrative: "Hello world test",
    intent_source: "test",
    confidence_level: 0.95
  }

  DataSemantics {
    state: {
      message: String = "hello"
    },
    invariants: [
      "message is not empty"
    ]
  }

  BehavioralSemantics {
    operations: [
      {
        name: "greet",
        precondition: "input_provided",
        parameters: {
          name: String
        },
        postcondition: "state_updated",
        side_effects: ["log"],
        idempotence: "idempotent"
      }
    ]
  }

  ExecutionConstraints {
    trigger_types: ["manual"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 1000,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: ["Greets users"],
    system_refusals: [],
    user_obligations: []
  }
}`

// ── ParseContract tests ─────────────────────────────────

func TestParseContractValid(t *testing.T) {
	result, err := ParseContract(helloWorld)
	if err != nil {
		t.Fatalf("ParseContract failed: %v", err)
	}

	var parsed map[string]interface{}
	if err := json.Unmarshal([]byte(result), &parsed); err != nil {
		t.Fatalf("Failed to parse JSON: %v", err)
	}

	identity := parsed["identity"].(map[string]interface{})
	if identity["stable_id"] != "ic-hello-001" {
		t.Errorf("Expected stable_id ic-hello-001, got %v", identity["stable_id"])
	}
}

func TestParseContractInvalid(t *testing.T) {
	_, err := ParseContract("invalid contract")
	if err == nil {
		t.Fatal("Expected error for invalid contract")
	}
}

func TestParseContractEmpty(t *testing.T) {
	_, err := ParseContract("")
	if err == nil {
		t.Fatal("Expected error for empty input")
	}
}

// ── Normalize tests ──────────────────────────────────────

func TestNormalizeValid(t *testing.T) {
	result, err := Normalize(helloWorld)
	if err != nil {
		t.Fatalf("Normalize failed: %v", err)
	}
	if !strings.Contains(result, "Contract {") {
		t.Error("Normalized result should contain 'Contract {'")
	}
}

func TestNormalizeIdempotent(t *testing.T) {
	first, err := Normalize(helloWorld)
	if err != nil {
		t.Fatalf("First normalize failed: %v", err)
	}
	second, err := Normalize(first)
	if err != nil {
		t.Fatalf("Second normalize failed: %v", err)
	}
	if first != second {
		t.Error("Normalize is not idempotent")
	}
}

func TestNormalizeDeterministic(t *testing.T) {
	first, _ := Normalize(helloWorld)
	for i := 0; i < 100; i++ {
		result, _ := Normalize(helloWorld)
		if result != first {
			t.Fatalf("Non-deterministic at iteration %d", i)
		}
	}
}

func TestNormalizeInvalid(t *testing.T) {
	_, err := Normalize("not valid icl")
	if err == nil {
		t.Fatal("Expected error for invalid input")
	}
}

// ── Verify tests ─────────────────────────────────────────

func TestVerifyValid(t *testing.T) {
	result, err := Verify(helloWorld)
	if err != nil {
		t.Fatalf("Verify failed: %v", err)
	}

	var parsed map[string]interface{}
	if err := json.Unmarshal([]byte(result), &parsed); err != nil {
		t.Fatalf("Failed to parse JSON: %v", err)
	}
	if parsed["valid"] != true {
		t.Error("Expected valid=true")
	}
}

func TestVerifyInvalid(t *testing.T) {
	_, err := Verify("not valid icl")
	if err == nil {
		t.Fatal("Expected error for invalid input")
	}
}

// ── Execute tests ────────────────────────────────────────

func TestExecuteOperation(t *testing.T) {
	inputs := `{"operation": "greet", "inputs": {"name": "World"}}`
	result, err := Execute(helloWorld, inputs)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	var parsed map[string]interface{}
	if err := json.Unmarshal([]byte(result), &parsed); err != nil {
		t.Fatalf("Failed to parse JSON: %v", err)
	}
	if parsed["success"] != true {
		t.Error("Expected success=true")
	}
}

func TestExecuteDeterministic(t *testing.T) {
	inputs := `{"operation": "greet", "inputs": {"name": "World"}}`
	first, _ := Execute(helloWorld, inputs)
	for i := 0; i < 100; i++ {
		result, _ := Execute(helloWorld, inputs)
		if result != first {
			t.Fatalf("Non-deterministic at iteration %d", i)
		}
	}
}

func TestExecuteInvalidContract(t *testing.T) {
	_, err := Execute("not valid", `{"operation": "test"}`)
	if err == nil {
		t.Fatal("Expected error for invalid contract")
	}
}

// ── SemanticHash tests ───────────────────────────────────

func TestSemanticHash(t *testing.T) {
	hash, err := SemanticHash(helloWorld)
	if err != nil {
		t.Fatalf("SemanticHash failed: %v", err)
	}
	if len(hash) != 64 {
		t.Errorf("Expected 64-char hex hash, got %d chars", len(hash))
	}
}

func TestSemanticHashDeterministic(t *testing.T) {
	first, _ := SemanticHash(helloWorld)
	for i := 0; i < 100; i++ {
		result, _ := SemanticHash(helloWorld)
		if result != first {
			t.Fatalf("Non-deterministic at iteration %d", i)
		}
	}
}

func TestSemanticHashInvalid(t *testing.T) {
	_, err := SemanticHash("not valid icl")
	if err == nil {
		t.Fatal("Expected error for invalid input")
	}
}

// ── Full pipeline ────────────────────────────────────────

func TestFullPipeline(t *testing.T) {
	// Parse
	parsed, err := ParseContract(helloWorld)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}
	if !strings.Contains(parsed, "ic-hello-001") {
		t.Error("Parse result missing stable_id")
	}

	// Normalize
	normalized, err := Normalize(helloWorld)
	if err != nil {
		t.Fatalf("Normalize failed: %v", err)
	}
	if len(normalized) == 0 {
		t.Error("Normalized result is empty")
	}

	// Verify
	verified, err := Verify(helloWorld)
	if err != nil {
		t.Fatalf("Verify failed: %v", err)
	}
	var verifyResult map[string]interface{}
	json.Unmarshal([]byte(verified), &verifyResult)
	if verifyResult["valid"] != true {
		t.Error("Verify: expected valid=true")
	}

	// Execute
	inputs := `{"operation": "greet", "inputs": {"name": "ICL"}}`
	executed, err := Execute(helloWorld, inputs)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	var execResult map[string]interface{}
	json.Unmarshal([]byte(executed), &execResult)
	if execResult["success"] != true {
		t.Error("Execute: expected success=true")
	}

	// Hash
	hash, err := SemanticHash(helloWorld)
	if err != nil {
		t.Fatalf("SemanticHash failed: %v", err)
	}
	if len(hash) != 64 {
		t.Error("Hash should be 64 hex chars")
	}
}
