#!/usr/bin/env node

import { promises as fs } from "node:fs";

const packageObject = JSON.parse(await fs.readFile('package.json', 'utf-8'));
const entryPath = packageObject.module ?? packageObject.main;
const wasmPath = packageObject.files.find(file => file.endsWith('.wasm'));

if (!entryPath) {
  throw new Error('package.json must define either "module" or "main"');
}

if (!wasmPath) {
  throw new Error('package.json does not include a .wasm file entry');
}

const wasm = await fs.readFile(wasmPath);
const wasmBASE64 = wasm.toString('base64');

const initializeScript = `
  const wasmBASE64 = '${wasmBASE64}';

  let bytes;

  if (typeof atob === 'function') {
    const binary = atob(wasmBASE64);
    bytes = new Uint8Array(binary.length);
    
    for (let i = 0; i < binary.length; i++) {
      bytes[i] = binary.charCodeAt(i);
    }  
  }else if (typeof Buffer === 'function') {
    bytes = Buffer.from(wasmBASE64, 'base64');
  }else {
    throw new Error('Unsupported platform');
  }

  const initFn = typeof init === 'function' ? init : __wbg_init;
  await initFn({ module_or_path: bytes });
`

await fs.appendFile(entryPath, initializeScript);

packageObject.files = packageObject.files.filter(file => !file.endsWith('.wasm'));
packageObject.main = entryPath;
if (packageObject.module) {
  packageObject.module = entryPath;
}
packageObject.type = "module";

await fs.writeFile('package.json', JSON.stringify(packageObject, null, 2));
