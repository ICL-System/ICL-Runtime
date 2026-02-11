/* @ts-self-types="./icl_runtime.d.ts" */

import * as wasm from "./icl_runtime_bg.wasm";
import { __wbg_set_wasm } from "./icl_runtime_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    execute, normalize, parseContract, semanticHash, verify
} from "./icl_runtime_bg.js";
