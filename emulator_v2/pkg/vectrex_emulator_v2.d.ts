/* tslint:disable */
/* eslint-disable */
export function wasm_init(): void;
/**
 * Vector structure matching JSVecx vector_t
 * JSVecx Original: function vector_t() { this.x0 = 0; this.y0 = 0; this.x1 = 0; this.y1 = 0; this.color = 0; }
 */
export class Vector {
  free(): void;
  constructor();
  x0: number;
  y0: number;
  x1: number;
  y1: number;
  color: number;
}
/**
 * Main WASM Emulator class matching JSVecx VecX API
 * JSVecx Original: function VecX()
 */
export class VectrexEmulator {
  free(): void;
  /**
   * Constructor matching JSVecx: new VecX()
   */
  constructor();
  /**
   * Initialize emulator with BIOS
   * JSVecx Pattern: init() + loadBios()
   * Auto-loads embedded BIOS ROM (8192 bytes: 4KB real + 4KB padding)
   */
  init(): boolean;
  /**
   * Load BIOS from bytes (for custom BIOS)
   * JSVecx Pattern: loadBiosFromBytes() (custom extension for WASM)
   */
  loadBiosBytes(bios_data: Uint8Array): boolean;
  /**
   * Load ROM/cartridge
   * JSVecx Pattern: loadRom(file)
   */
  loadRom(rom_path: string): boolean;
  /**
   * Reset emulator
   * JSVecx Pattern: reset()
   */
  reset(): void;
  /**
   * Start emulation loop
   * JSVecx Pattern: start()
   */
  start(): void;
  /**
   * Stop emulation loop
   * JSVecx Pattern: stop()
   */
  stop(): void;
  /**
   * Check if running
   * JSVecx Pattern: isRunning()
   */
  isRunning(): boolean;
  /**
   * Execute one frame (called by JS animation loop)
   * JSVecx Pattern: vecx_emu(cycles, 0) called in loop
   */
  runFrame(cycles: bigint): void;
  /**
   * Get vector count
   * JSVecx Pattern: this.vector_draw_cnt
   */
  getVectorCount(): number;
  /**
   * Get vector at index (returns raw values for JS to construct object)
   * JSVecx Pattern: accessing this.vectors_draw[i]
   */
  getVector(index: number): Vector | undefined;
  /**
   * Get all vectors as JSON
   * JSVecx Extension: For easier consumption from JS
   */
  getVectorsJson(): string;
  /**
   * Get metrics as JSON
   * JSVecx Pattern: getMetrics() returns { totalCycles, instructionCount, frameCount, running }
   */
  getMetrics(): string;
  /**
   * Get CPU registers as JSON
   * JSVecx Pattern: getRegisters() returns { PC, A, B, X, Y, U, S, DP, CC }
   */
  getRegisters(): string;
  /**
   * Read memory byte
   * JSVecx Pattern: read8(address)
   */
  read8(address: number): number;
  /**
   * Write memory byte
   * JSVecx Pattern: write8(address, value)
   */
  write8(address: number, value: number): void;
  /**
   * Handle key down
   * JSVecx Pattern: onkeydown(event)
   */
  onKeyDown(key_code: number): void;
  /**
   * Handle key up
   * JSVecx Pattern: onkeyup(event)
   */
  onKeyUp(key_code: number): void;
  /**
   * Set joystick position directly (-127 to 127)
   * Extension: For programmatic control
   */
  setJoystick(x: number, y: number): void;
  /**
   * Set button state
   * Extension: For programmatic control
   */
  setButton(button: number, pressed: boolean): void;
  /**
   * Get Program Counter
   */
  getPC(): number;
  /**
   * Get register A
   */
  getA(): number;
  /**
   * Get register B
   */
  getB(): number;
  /**
   * Get register D (A:B concatenated)
   */
  getD(): number;
  /**
   * Get register X
   */
  getX(): number;
  /**
   * Get register Y
   */
  getY(): number;
  /**
   * Get register U (User stack pointer)
   */
  getU(): number;
  /**
   * Get register S (System stack pointer)
   */
  getS(): number;
  /**
   * Get Direct Page register
   */
  getDP(): number;
  /**
   * Get Condition Codes register
   */
  getCC(): number;
  /**
   * Get total cycles executed
   */
  getTotalCycles(): bigint;
  /**
   * Read byte from memory (for debugging)
   */
  readMemory(address: number): number;
  /**
   * Execute single instruction (for step debugging)
   */
  step(): void;
  /**
   * Get last error message (for debugging)
   */
  getLastError(): string;
  /**
   * Get last PC (for debugging panics)
   */
  getLastPC(): number;
  /**
   * Get last opcode (for debugging panics)
   */
  getLastOpcode(): number;
  /**
   * Get PC history as JSON string (last N instructions before current)
   */
  getPCHistory(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_vector_free: (a: number, b: number) => void;
  readonly __wbg_get_vector_x0: (a: number) => number;
  readonly __wbg_set_vector_x0: (a: number, b: number) => void;
  readonly __wbg_get_vector_y0: (a: number) => number;
  readonly __wbg_set_vector_y0: (a: number, b: number) => void;
  readonly __wbg_get_vector_x1: (a: number) => number;
  readonly __wbg_set_vector_x1: (a: number, b: number) => void;
  readonly __wbg_get_vector_y1: (a: number) => number;
  readonly __wbg_set_vector_y1: (a: number, b: number) => void;
  readonly __wbg_get_vector_color: (a: number) => number;
  readonly __wbg_set_vector_color: (a: number, b: number) => void;
  readonly vector_new: () => number;
  readonly __wbg_vectrexemulator_free: (a: number, b: number) => void;
  readonly vectrexemulator_new: () => number;
  readonly vectrexemulator_init: (a: number) => number;
  readonly vectrexemulator_loadBiosBytes: (a: number, b: number, c: number) => number;
  readonly vectrexemulator_loadRom: (a: number, b: number, c: number) => number;
  readonly vectrexemulator_reset: (a: number) => void;
  readonly vectrexemulator_start: (a: number) => void;
  readonly vectrexemulator_stop: (a: number) => void;
  readonly vectrexemulator_isRunning: (a: number) => number;
  readonly vectrexemulator_runFrame: (a: number, b: bigint) => void;
  readonly vectrexemulator_getVectorCount: (a: number) => number;
  readonly vectrexemulator_getVector: (a: number, b: number) => number;
  readonly vectrexemulator_getVectorsJson: (a: number) => [number, number];
  readonly vectrexemulator_getMetrics: (a: number) => [number, number];
  readonly vectrexemulator_getRegisters: (a: number) => [number, number];
  readonly vectrexemulator_read8: (a: number, b: number) => number;
  readonly vectrexemulator_write8: (a: number, b: number, c: number) => void;
  readonly vectrexemulator_onKeyDown: (a: number, b: number) => void;
  readonly vectrexemulator_onKeyUp: (a: number, b: number) => void;
  readonly vectrexemulator_setJoystick: (a: number, b: number, c: number) => void;
  readonly vectrexemulator_setButton: (a: number, b: number, c: number) => void;
  readonly vectrexemulator_getPC: (a: number) => number;
  readonly vectrexemulator_getA: (a: number) => number;
  readonly vectrexemulator_getB: (a: number) => number;
  readonly vectrexemulator_getD: (a: number) => number;
  readonly vectrexemulator_getX: (a: number) => number;
  readonly vectrexemulator_getY: (a: number) => number;
  readonly vectrexemulator_getU: (a: number) => number;
  readonly vectrexemulator_getS: (a: number) => number;
  readonly vectrexemulator_getDP: (a: number) => number;
  readonly vectrexemulator_getCC: (a: number) => number;
  readonly vectrexemulator_getTotalCycles: (a: number) => bigint;
  readonly vectrexemulator_step: (a: number) => void;
  readonly vectrexemulator_getLastError: (a: number) => [number, number];
  readonly vectrexemulator_getLastPC: (a: number) => number;
  readonly vectrexemulator_getLastOpcode: (a: number) => number;
  readonly vectrexemulator_getPCHistory: (a: number) => [number, number];
  readonly vectrexemulator_readMemory: (a: number, b: number) => number;
  readonly wasm_init: () => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
