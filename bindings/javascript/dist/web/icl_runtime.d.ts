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

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly execute: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly normalize: (a: number, b: number) => [number, number, number, number];
    readonly parseContract: (a: number, b: number) => [number, number, number, number];
    readonly semanticHash: (a: number, b: number) => [number, number, number, number];
    readonly verify: (a: number, b: number) => [number, number, number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
