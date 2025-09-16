// Shim glue for development when the wasm-bindgen output (vectrex_emulator.js) is not yet generated.
// After running `npm run wasm:build`, this file can optionally be replaced or left to delegate.
// We try to import the actual generated file if it exists.
let actual;
export async function init(arg) {
  if (!actual) {
    try {
      // Use dynamic computed string to avoid Vite/Rollup static resolution failure when file missing.
      const target = './vectrex_emulator.js';
      actual = await import(/* @vite-ignore */ target);
    } catch (e) {
      throw new Error('vectrex_emulator.js missing: run `npm run wasm:build` to generate wasm glue');
    }
  }
  const fn = actual.default || actual.init || actual;
  return fn(arg);
}

export const WasmEmu = new Proxy({}, {
  construct(_t, args) {
    if (!actual) throw new Error('Call init() before constructing WasmEmu');
    const Ctor = actual.WasmEmu || actual.WasmEmuClass;
    if (!Ctor) throw new Error('WasmEmu export missing in vectrex_emulator.js');
    return new Ctor(...args);
  }
});

export default init;