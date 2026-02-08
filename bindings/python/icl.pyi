"""
ICL (Intent Contract Language) — Python bindings for the canonical Rust runtime.

All functions are thin wrappers around the Rust implementation.
Deterministic: same input always produces identical output.
"""

def parse_contract(text: str) -> str:
    """Parse ICL contract text and return a JSON string of the parsed Contract.

    Args:
        text: ICL contract source text

    Returns:
        JSON string representation of the parsed Contract

    Raises:
        ValueError: If the contract text has syntax or semantic errors
    """
    ...

def normalize(text: str) -> str:
    """Normalize ICL contract text to canonical form.

    Deterministic and idempotent: normalize(normalize(x)) == normalize(x)

    Args:
        text: ICL contract source text

    Returns:
        Canonical normalized ICL text

    Raises:
        ValueError: If the contract text cannot be parsed
    """
    ...

def verify(text: str) -> str:
    """Verify an ICL contract for correctness.

    Returns JSON with verification result including errors and warnings.

    Args:
        text: ICL contract source text

    Returns:
        JSON string: {"valid": bool, "errors": [...], "warnings": [...]}

    Raises:
        ValueError: If the contract text cannot be parsed
    """
    ...

def execute(text: str, inputs: str) -> str:
    """Execute an ICL contract with the given inputs.

    Args:
        text: ICL contract source text
        inputs: JSON string with execution inputs

    Returns:
        JSON string with execution result including provenance log

    Raises:
        ValueError: If the contract cannot be parsed, verified, or executed
    """
    ...

def semantic_hash(text: str) -> str:
    """Compute the SHA-256 semantic hash of a contract.

    Args:
        text: ICL contract source text

    Returns:
        Hex-encoded SHA-256 hash string

    Raises:
        ValueError: If the contract text cannot be parsed
    """
    ...
