#!/usr/bin/env zx
 
const packageObject = JSON.parse(await fs.readFile('package.json', 'utf-8'));

packageObject.type = "module";
await fs.writeFile('package.json', JSON.stringify(packageObject, null, 2));

const wasmPath = packageObject.files.find(file => file.endsWith('.wasm'));
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

  await init(bytes);
`

await fs.appendFile(packageObject.module, initializeScript);
