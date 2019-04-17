import {exec} from 'child_process';
import del = require('del');
import {readFileSync} from 'fs';
import {dest, series, src} from 'gulp';
import replace = require('gulp-replace');
import path = require('path');

const WASM_PACK_DEST = 'wasm';
const PROJ_NAME = 'rusty-typescript';
const TMP = 'tmp';
const DIST = 'dist';
const BG_JS_FILENAME = `${PROJ_NAME.replace('-', '_')}_bg.js`;
const BG_WASM_FILENAME = `${PROJ_NAME.replace('-', '_')}_bg.wasm`;

function cleanWasmPackDest() {
  return del(WASM_PACK_DEST);
}

function cleanTmp() {
  return del(TMP);
};

export const clean = series(cleanWasmPackDest, cleanTmp);

const execCommand = (command: string) => new Promise(
  (resolve, reject) => {
    exec(command, (error, stdout) => {
      if (error) {
        reject(error);
        return;
      }
      resolve(stdout);
    });
  }
);

async function installWasmPack() {
  try {
    // Check whether wasm-pack is available
    await execCommand('wasm-pack --version');
  } catch {
    // If not, install it
    await execCommand('cargo install wasm-pack');
  }
}

async function wasmPack() {
  await execCommand(`wasm-pack build --target nodejs --out-dir ${WASM_PACK_DEST}`);
}

/**
 * Inline the file rusty_typescript_bg.wasm into rusty_typescript_bg.js
 * serialized as base64 and copy it into the tmp/ folder.
 */
function inlineWasm() {
  const wasm = readFileSync(path.resolve(WASM_PACK_DEST, BG_WASM_FILENAME));
  const base64 = wasm.toString('base64');
  const bytesLines = [
    `const atob = typeof window !== 'undefined' ? window.atob : b64 => Buffer.from(b64, 'base64').toString('binary');`,
    `const binaryString = atob('${base64}');`,
    `const bytes = new Uint8Array(${wasm.length});`,
    `for (let i = 0; i < ${wasm.length}; i++) {`,
    `  bytes[i] = binaryString.charCodeAt(i);`,
    `}\n`
  ].join('\n');

  return src([path.resolve(WASM_PACK_DEST, BG_JS_FILENAME)])
    // remove the import of 'path'
    .pipe(replace(/const path = .*\n/g, ''))
    // replace the loading of the wasm bytes to use the inlined base64 version
    .pipe(replace(/const bytes = .*\n/g, bytesLines))
    .pipe(dest(TMP));
}

function copyToTmp() {
  return src([
    path.resolve(WASM_PACK_DEST, '**'),
    `!${path.resolve(WASM_PACK_DEST, BG_WASM_FILENAME)}`,
    `!${path.resolve(WASM_PACK_DEST, BG_JS_FILENAME)}`
  ])
    .pipe(dest(TMP));
}

export const build = series(installWasmPack, clean, wasmPack, inlineWasm, copyToTmp);
