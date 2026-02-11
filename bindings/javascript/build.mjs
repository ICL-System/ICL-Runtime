#!/usr/bin/env node
/**
 * Build all 3 WASM targets for the icl-runtime npm package.
 * Cross-platform (Linux, macOS, Windows).
 *
 * Usage: node build.mjs
 */
import { execSync } from 'node:child_process';
import { cpSync, mkdirSync, rmSync, readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { statSync } from 'node:fs';

const __dirname = dirname(fileURLToPath(import.meta.url));
process.chdir(__dirname);

function run(cmd) {
  console.log(`  $ ${cmd}`);
  execSync(cmd, { stdio: 'inherit' });
}

function sha256(filePath) {
  const data = readFileSync(filePath);
  return createHash('sha256').update(data).digest('hex');
}

// ── 1. Build 3 targets ──────────────────────────────────────────────
console.log('\n=== Building WASM targets ===\n');

console.log('[1/3] nodejs...');
run('wasm-pack build --target nodejs --out-dir pkg-nodejs --release');

console.log('\n[2/3] bundler...');
run('wasm-pack build --target bundler --out-dir pkg-bundler --release');

console.log('\n[3/3] web...');
run('wasm-pack build --target web --out-dir pkg-web --release');

// ── 2. Assemble dist/ ───────────────────────────────────────────────
console.log('\n=== Assembling dist/ ===\n');

rmSync('dist', { recursive: true, force: true });
mkdirSync('dist/nodejs', { recursive: true });
mkdirSync('dist/bundler', { recursive: true });
mkdirSync('dist/web', { recursive: true });

// nodejs
for (const f of ['icl_runtime.js', 'icl_runtime.d.ts', 'icl_runtime_bg.wasm', 'icl_runtime_bg.wasm.d.ts']) {
  cpSync(`pkg-nodejs/${f}`, `dist/nodejs/${f}`);
}

// ESM wrapper for Node.js
writeFileSync('dist/nodejs/icl_runtime.mjs', [
  '// ESM wrapper for Node.js — re-exports the CJS wasm-pack output as named exports.',
  "import cjs from './icl_runtime.js';",
  'export const { parseContract, normalize, verify, semanticHash, execute } = cjs;',
  'export default cjs;',
  '',
].join('\n'));

// bundler
for (const f of ['icl_runtime.js', 'icl_runtime.d.ts', 'icl_runtime_bg.js', 'icl_runtime_bg.wasm', 'icl_runtime_bg.wasm.d.ts']) {
  cpSync(`pkg-bundler/${f}`, `dist/bundler/${f}`);
}

// web
for (const f of ['icl_runtime.js', 'icl_runtime.d.ts', 'icl_runtime_bg.wasm', 'icl_runtime_bg.wasm.d.ts']) {
  cpSync(`pkg-web/${f}`, `dist/web/${f}`);
}

// ── 3. Verify ────────────────────────────────────────────────────────
console.log('=== Verifying ===\n');

const hashNode    = sha256('dist/nodejs/icl_runtime_bg.wasm');
const hashBundler = sha256('dist/bundler/icl_runtime_bg.wasm');
const hashWeb     = sha256('dist/web/icl_runtime_bg.wasm');

if (hashNode === hashBundler && hashBundler === hashWeb) {
  console.log(`✓ All 3 WASM binaries are identical (${hashNode})`);
} else {
  console.error('✗ WASM binaries differ!');
  console.error(`  nodejs:  ${hashNode}`);
  console.error(`  bundler: ${hashBundler}`);
  console.error(`  web:     ${hashWeb}`);
  process.exit(1);
}

const size = statSync('dist/nodejs/icl_runtime_bg.wasm').size;
console.log(`✓ WASM size: ${size} bytes`);
console.log('✓ dist/ ready to publish');
