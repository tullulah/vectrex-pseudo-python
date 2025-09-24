// Type declarations for generated wasm-bindgen (minimal subset)
export interface WasmEmu {
  load_bios(data: Uint8Array): boolean;
  load_bin(base: number, data: Uint8Array): void;
  reset(): void;
  step(count: number): number;
  run_until_wait_recal(max_instr: number): number;
  registers_json(): string;
  memory_ptr(): number;
  vector_events_json(): string;
  metrics_json(): string;
}
export function init(input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module): Promise<any>;
