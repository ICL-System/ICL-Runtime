/**
 * Tests for ICL JavaScript/WASM bindings.
 *
 * Verifies that JS bindings produce identical results to the Rust implementation.
 * Run with: node tests/test_icl.mjs (after wasm-pack build)
 */

import { parseContract, normalize, verify, execute, semanticHash } from '../dist/nodejs/icl_runtime.mjs';

const HELLO_WORLD = `Contract {
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
}`;

let passed = 0;
let failed = 0;

function assert(condition, message) {
  if (!condition) {
    console.error(`  FAIL: ${message}`);
    failed++;
  } else {
    console.log(`  PASS: ${message}`);
    passed++;
  }
}

function assertThrows(fn, message) {
  try {
    fn();
    console.error(`  FAIL: ${message} (expected error, got none)`);
    failed++;
  } catch (e) {
    console.log(`  PASS: ${message}`);
    passed++;
  }
}

// ── parseContract tests ─────────────────────────────────
console.log('\n=== parseContract ===');

const parsed = JSON.parse(parseContract(HELLO_WORLD));
assert(parsed.identity.stable_id === 'ic-hello-001', 'parse: stable_id');
assert(parsed.identity.version === 1, 'parse: version');
assert(parsed.purpose_statement.confidence_level === 0.95, 'parse: confidence');
assert(parsed.behavioral_semantics.operations.length === 1, 'parse: operations count');
assert(parsed.behavioral_semantics.operations[0].name === 'greet', 'parse: operation name');
assertThrows(() => parseContract('invalid'), 'parse: invalid input throws');
assertThrows(() => parseContract(''), 'parse: empty input throws');

// ── normalize tests ──────────────────────────────────────
console.log('\n=== normalize ===');

const normalized = normalize(HELLO_WORLD);
assert(typeof normalized === 'string', 'normalize: returns string');
assert(normalized.includes('Contract {'), 'normalize: contains Contract');
const norm2 = normalize(normalized);
assert(normalized === norm2, 'normalize: idempotent');
assertThrows(() => normalize('not valid icl'), 'normalize: invalid throws');

// determinism check
let allSame = true;
const first = normalize(HELLO_WORLD);
for (let i = 0; i < 100; i++) {
  if (normalize(HELLO_WORLD) !== first) { allSame = false; break; }
}
assert(allSame, 'normalize: deterministic (100 iterations)');

// ── verify tests ─────────────────────────────────────────
console.log('\n=== verify ===');

const verifyResult = JSON.parse(verify(HELLO_WORLD));
assert(verifyResult.valid === true, 'verify: valid contract');
assert(Array.isArray(verifyResult.errors), 'verify: errors is array');
assert(Array.isArray(verifyResult.warnings), 'verify: warnings is array');
assert(verifyResult.errors.length === 0, 'verify: no errors');
assertThrows(() => verify('not valid icl'), 'verify: invalid throws');

// ── execute tests ────────────────────────────────────────
console.log('\n=== execute ===');

const inputs = JSON.stringify({ operation: 'greet', inputs: { name: 'World' } });
const execResult = JSON.parse(execute(HELLO_WORLD, inputs));
assert(execResult.success === true, 'execute: success');
assertThrows(() => execute('not valid', '{}'), 'execute: invalid contract throws');
assertThrows(() => execute(HELLO_WORLD, 'not json'), 'execute: invalid json throws');

// determinism check
allSame = true;
const firstExec = execute(HELLO_WORLD, inputs);
for (let i = 0; i < 100; i++) {
  if (execute(HELLO_WORLD, inputs) !== firstExec) { allSame = false; break; }
}
assert(allSame, 'execute: deterministic (100 iterations)');

// ── semanticHash tests ───────────────────────────────────
console.log('\n=== semanticHash ===');

const hash = semanticHash(HELLO_WORLD);
assert(typeof hash === 'string', 'hash: returns string');
assert(hash.length === 64, 'hash: SHA-256 (64 hex chars)');
assert(/^[0-9a-f]{64}$/.test(hash), 'hash: valid hex');
assertThrows(() => semanticHash('not valid icl'), 'hash: invalid throws');

// determinism
allSame = true;
for (let i = 0; i < 100; i++) {
  if (semanticHash(HELLO_WORLD) !== hash) { allSame = false; break; }
}
assert(allSame, 'hash: deterministic (100 iterations)');

// ── Full pipeline ────────────────────────────────────────
console.log('\n=== Full Pipeline ===');

const p = JSON.parse(parseContract(HELLO_WORLD));
assert(p.identity.stable_id === 'ic-hello-001', 'pipeline: parse');
const n = normalize(HELLO_WORLD);
assert(n.length > 0, 'pipeline: normalize');
const v = JSON.parse(verify(HELLO_WORLD));
assert(v.valid === true, 'pipeline: verify');
const e = JSON.parse(execute(HELLO_WORLD, inputs));
assert(e.success === true, 'pipeline: execute');
const h = semanticHash(HELLO_WORLD);
assert(h.length === 64, 'pipeline: hash');

// ── Summary ──────────────────────────────────────────────
console.log(`\n${passed} passed, ${failed} failed`);
process.exit(failed > 0 ? 1 : 0);
