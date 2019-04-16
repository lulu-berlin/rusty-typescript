import {exec} from 'child_process';

const WASM_PACK_DEST = 'wasm';

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

export async function wasmPack() {
  await execCommand(`wasm-pack build --target nodejs --out-dir ${WASM_PACK_DEST}`);
}
