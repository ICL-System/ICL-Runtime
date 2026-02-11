// ESM wrapper for Node.js — re-exports the CJS wasm-pack output as named exports.
import cjs from './icl_runtime.js';
export const { parseContract, normalize, verify, semanticHash, execute } = cjs;
export default cjs;
