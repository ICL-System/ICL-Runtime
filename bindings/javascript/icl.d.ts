/**
 * ICL (Intent Contract Language) — JavaScript/TypeScript bindings
 *
 * All functions are thin wrappers around the canonical Rust implementation
 * compiled to WebAssembly. Deterministic: same input always produces identical output.
 */

/**
 * Parse ICL contract text and return a JSON string of the parsed Contract.
 *
 * @param text - ICL contract source text
 * @returns JSON string representation of the parsed Contract
 * @throws Error if the contract text has syntax or semantic errors
 */
export function parseContract(text: string): string;

/**
 * Normalize ICL contract text to canonical form.
 *
 * Deterministic and idempotent: normalize(normalize(x)) === normalize(x)
 *
 * @param text - ICL contract source text
 * @returns Canonical normalized ICL text
 * @throws Error if the contract text cannot be parsed
 */
export function normalize(text: string): string;

/**
 * Verify an ICL contract for correctness.
 *
 * Returns JSON with verification result including errors and warnings.
 *
 * @param text - ICL contract source text
 * @returns JSON string: { valid: boolean, errors: Array, warnings: Array }
 * @throws Error if the contract text cannot be parsed
 */
export function verify(text: string): string;

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
 * Compute the SHA-256 semantic hash of a contract.
 *
 * @param text - ICL contract source text
 * @returns Hex-encoded SHA-256 hash string
 * @throws Error if the contract text cannot be parsed
 */
export function semanticHash(text: string): string;
