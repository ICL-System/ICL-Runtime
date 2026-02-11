/* tslint:disable */
/* eslint-disable */

/**
 * Execute an ICL contract with the given inputs.
 *
 * @param text - ICL contract source text
 * @param inputs - JSON string with execution inputs
 * @returns JSON string with execution result including provenance log
 * @throws Error if the contract cannot be parsed, verified, or executed
 */
export function execute(text: string, inputs: string): string;

/**
 * Normalize ICL contract text to canonical form.
 *
 * Guarantees:
 *   - Deterministic: same input → same output
 *   - Idempotent: normalize(normalize(x)) === normalize(x)
 *   - Semantic preserving: meaning is unchanged
 *
 * @param text - ICL contract source text
 * @returns Canonical normalized ICL text
 * @throws Error if the contract text cannot be parsed
 */
export function normalize(text: string): string;

/**
 * Parse ICL contract text and return a JSON string of the parsed Contract.
 *
 * @param text - ICL contract source text
 * @returns JSON string representation of the parsed Contract
 * @throws Error if the contract text has syntax or semantic errors
 */
export function parseContract(text: string): string;

/**
 * Compute the SHA-256 semantic hash of a contract.
 *
 * @param text - ICL contract source text
 * @returns Hex-encoded SHA-256 hash string
 * @throws Error if the contract text cannot be parsed
 */
export function semanticHash(text: string): string;

/**
 * Verify an ICL contract for correctness.
 *
 * Runs all verification phases:
 *   - Type checking
 *   - Invariant verification
 *   - Determinism checking
 *   - Coherence verification
 *
 * @param text - ICL contract source text
 * @returns JSON string: { valid: boolean, errors: [...], warnings: [...] }
 * @throws Error if the contract text cannot be parsed
 */
export function verify(text: string): string;
