// Conformance: invalid/extra-closing-brace.icl
// Tests: Premature closing of Contract body, leaving sections outside
// Expected: MUST fail — SyntaxError(missing required sections)

Contract {
  Identity {
    stable_id: "ic-inv-28-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }
}
