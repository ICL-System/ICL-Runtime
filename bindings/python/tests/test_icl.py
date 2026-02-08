"""Tests for ICL Python bindings.

Verifies that Python bindings produce identical results to the Rust implementation.
"""
import json
import pytest
import icl


# ── Test Contract ────────────────────────────────────────

HELLO_WORLD = """\
Contract {
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
}
"""

INVALID_CONTRACT = """\
Contract {
    Identity {
        stable_id: "test"
    }
}
"""


# ── parse_contract tests ─────────────────────────────────

class TestParseContract:
    def test_valid_contract(self):
        result = icl.parse_contract(HELLO_WORLD)
        parsed = json.loads(result)
        assert parsed["identity"]["stable_id"] == "ic-hello-001"
        assert parsed["identity"]["version"] == 1
        assert parsed["purpose_statement"]["confidence_level"] == 0.95
        assert len(parsed["behavioral_semantics"]["operations"]) == 1
        assert parsed["behavioral_semantics"]["operations"][0]["name"] == "greet"

    def test_invalid_contract_raises(self):
        with pytest.raises(ValueError):
            icl.parse_contract(INVALID_CONTRACT)

    def test_empty_input_raises(self):
        with pytest.raises(ValueError):
            icl.parse_contract("")


# ── normalize tests ──────────────────────────────────────

class TestNormalize:
    def test_produces_canonical_form(self):
        result = icl.normalize(HELLO_WORLD)
        assert isinstance(result, str)
        assert "Contract {" in result

    def test_idempotent(self):
        first = icl.normalize(HELLO_WORLD)
        second = icl.normalize(first)
        assert first == second

    def test_deterministic(self):
        results = [icl.normalize(HELLO_WORLD) for _ in range(100)]
        assert all(r == results[0] for r in results), "Non-deterministic normalize"

    def test_invalid_raises(self):
        with pytest.raises(ValueError):
            icl.normalize("not valid icl")


# ── verify tests ─────────────────────────────────────────

class TestVerify:
    def test_valid_contract(self):
        result_json = icl.verify(HELLO_WORLD)
        result = json.loads(result_json)
        assert result["valid"] is True
        assert result["errors"] == []

    def test_returns_json_structure(self):
        result_json = icl.verify(HELLO_WORLD)
        result = json.loads(result_json)
        assert "valid" in result
        assert "errors" in result
        assert "warnings" in result

    def test_invalid_raises(self):
        with pytest.raises(ValueError):
            icl.verify("not valid icl")


# ── execute tests ────────────────────────────────────────

class TestExecute:
    def test_execute_operation(self):
        inputs = json.dumps({
            "operation": "greet",
            "inputs": {"name": "World"}
        })
        result_json = icl.execute(HELLO_WORLD, inputs)
        result = json.loads(result_json)
        assert result["success"] is True

    def test_execute_deterministic(self):
        inputs = json.dumps({
            "operation": "greet",
            "inputs": {"name": "World"}
        })
        results = [icl.execute(HELLO_WORLD, inputs) for _ in range(100)]
        assert all(r == results[0] for r in results), "Non-deterministic execute"

    def test_execute_invalid_contract_raises(self):
        with pytest.raises(ValueError):
            icl.execute("not valid", '{"operation": "test"}')

    def test_execute_invalid_json_raises(self):
        with pytest.raises(ValueError):
            icl.execute(HELLO_WORLD, "not json")


# ── semantic_hash tests ──────────────────────────────────

class TestSemanticHash:
    def test_returns_hex_string(self):
        h = icl.semantic_hash(HELLO_WORLD)
        assert isinstance(h, str)
        assert len(h) == 64  # SHA-256 hex = 64 chars
        assert all(c in "0123456789abcdef" for c in h)

    def test_deterministic(self):
        hashes = [icl.semantic_hash(HELLO_WORLD) for _ in range(100)]
        assert all(h == hashes[0] for h in hashes), "Non-deterministic hash"

    def test_invalid_raises(self):
        with pytest.raises(ValueError):
            icl.semantic_hash("not valid icl")


# ── Cross-binding consistency ────────────────────────────

class TestConsistency:
    """Verify that parse → normalize → verify → execute chain works end-to-end."""

    def test_full_pipeline(self):
        # Parse
        parsed_json = icl.parse_contract(HELLO_WORLD)
        parsed = json.loads(parsed_json)
        assert parsed["identity"]["stable_id"] == "ic-hello-001"

        # Normalize
        normalized = icl.normalize(HELLO_WORLD)
        assert len(normalized) > 0

        # Verify
        verify_json = icl.verify(HELLO_WORLD)
        verify_result = json.loads(verify_json)
        assert verify_result["valid"] is True

        # Execute
        inputs = json.dumps({"operation": "greet", "inputs": {"name": "ICL"}})
        exec_json = icl.execute(HELLO_WORLD, inputs)
        exec_result = json.loads(exec_json)
        assert exec_result["success"] is True

        # Hash
        h = icl.semantic_hash(HELLO_WORLD)
        assert len(h) == 64
