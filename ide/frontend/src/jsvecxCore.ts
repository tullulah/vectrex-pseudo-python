// Stub de integración futura con jsvecx (u otro emulador JS). Provee estructura y errores claros.
import { IEmulatorCore, MetricsSnapshot, RegistersSnapshot, Segment } from './emulatorCore';

// Integración mínima con jsvecx: suponemos presencia del árbol `jsvecx/src/preprocess` accesible vía import dinámico
// en entorno de desarrollo (Vite). Para build producción podría requerir copia/alias en assets.
// Política: no mutar código original, sólo importar y usar su API constructiva (VecX, Globals).

interface JsVecxInstance {
  vecx_reset: () => void;
  vecx_emu: (cycles:number, ahead:number)=>void;
  rom: number[];
  cart: number[];
  vectors_draw: any[];
  vector_draw_cnt: number;
  fcycles: number;
  alg_vectoring: number;
  alg_vector_x0: number; alg_vector_y0: number; alg_vector_x1: number; alg_vector_y1: number;
  e6809: {
    reg_a: number; reg_b: number; reg_dp: number; reg_pc: number; reg_cc: number;
    reg_x: { value: number }; reg_y: { value: number }; reg_u: { value: number }; reg_s: { value: number };
  }
}

type JsVecxModule = { VecX: new()=>JsVecxInstance; Globals:any };

export class JsVecxEmulatorCore implements IEmulatorCore {
  private biosOk = false;
  private mod: JsVecxModule | null = null;
  private inst: JsVecxInstance | null = null;
  private cartLoaded = false;
  private lastFrameSegments: Segment[] = [];
  private frameCounter = 0;
  // Contador de ciclos aproximado (sumamos los ciclos solicitados a vecx_emu). No exacto pero útil para paridad básica.
  private totalCycles = 0;
  private lastFcycles: number | null = null; // para detectar rollover de frame en jsvecx (fcycles reinicia sumando FCYCLES_INIT)
  private fcInitCached: number | null = null;
  private memScratch: Uint8Array | null = null;

  async init(){
    if (this.mod) return;
    try {
  // Simplificado: sólo intentamos cargar el bundle estático servido desde /public -> /jsvecx/vecx_full.js
  // Razón: Vite emite warning si se importan directamente assets crudos dentro de /public (no pasan por transform).
  // El script jsvecx:bundle garantiza que el archivo exporte VecX y Globals.
  let bundle: any = null;
  try {
    // Nueva ubicación: bundle generado en src/generated/jsvecx/vecx_full.js (incluido en pipeline de TS/Vite)
    // Usamos import relativo explícito para permitir tree-shaking futuro si se particiona.
    const bundlePath = './generated/jsvecx/vecx_full.js';
    bundle = await import(/* @vite-ignore */ bundlePath);
  } catch (e){
    console.warn('[JsVecxCore] no se pudo importar bundle interno generated/jsvecx/vecx_full.js; backend jsvecx inerte', e);
    return;
  }
      const VecX = (bundle as any).VecX || (bundle as any).default?.VecX;
      const Globals = (bundle as any).Globals || (bundle as any).default?.Globals;
      if (!VecX) throw new Error('VecX constructor not found');
      this.mod = { VecX, Globals } as any;
      this.inst = new VecX();
      // Inicializa vectores internos si el código original requiere llamada implícita (vecx_reset luego)
  this.inst?.vecx_reset();
    } catch (e){
      console.warn('[JsVecxCore] init failed', e);
    }
  }
  loadBios(bytes: Uint8Array){
    // jsvecx espera 8K ROM en this.rom; copiamos mínimo (clamp a 0x2000)
    if (!this.inst) return; const len = Math.min(0x2000, bytes.length);
    for (let i=0;i<len;i++) this.inst.rom[i] = bytes[i];
    this.biosOk = true;
  }
  isBiosLoaded(){ return this.biosOk; }
  reset(){ if (this.inst) { try { this.inst.vecx_reset(); } catch{} this.frameCounter = 0; } }
  loadProgram(bytes: Uint8Array, _base?: number){
    if (!this.inst) return; // Cartridge array size 0x8000
    const len = Math.min(0x8000, bytes.length);
    for (let i=0;i<len;i++) this.inst.cart[i] = bytes[i];
    this.cartLoaded = true;
  }
  runFrame(_maxInstr?: number){
    if (!this.inst) return;
    // Derivar ciclos por frame usando constante real FCYCLES_INIT (approx VECTREX_MHZ / PDECAY) si está disponible.
    const fcInit = (this.mod as any)?.Globals?.FCYCLES_INIT || 25000;
    if (this.fcInitCached == null) this.fcInitCached = fcInit;
    // Observación: FCYCLES_INIT en jsvecx está relacionado con decaimiento fosforo, no frame completo 60Hz exacto.
    // Usamos un factor heurístico  *  (aquí 1) y dejamos refino posterior (posible acumulador fcycles).
    const cycles = fcInit;
    const beforeF = (this.inst as any).fcycles;
    try { this.inst.vecx_emu(cycles, 0); } catch(e){ /* silencioso */ }
    const afterF = (this.inst as any).fcycles;
    // jsvecx decrementa fcycles y cuando <0 lo incrementa sumando FCYCLES_INIT -> rollover detectado si after > before
    if (typeof beforeF === 'number' && typeof afterF === 'number') {
      if (this.lastFcycles == null) this.lastFcycles = beforeF;
      if (afterF > beforeF) { // rollover
        this.frameCounter++;
      }
      this.lastFcycles = afterF;
    } else {
      // fallback: incrementar siempre
      this.frameCounter++;
    }
    this.totalCycles += cycles; // aproximación
    // Extraer segmentos (vectors_draw[0..vector_draw_cnt]) mapeándolos a rango normalizado [-1,1]
    const out: Segment[] = [];
    const drawCnt = (this.inst as any).vector_draw_cnt as number;
    const arr = (this.inst as any).vectors_draw as any[];
    const maxX = (this.mod as any)?.Globals?.ALG_MAX_X || 33000;
    const maxY = (this.mod as any)?.Globals?.ALG_MAX_Y || 41000;
    const norm = (v:number, max:number) => (v / (max/2));
    for (let i=0; i<drawCnt && i<arr.length; i++) {
      const d = arr[i]; if (!d || d.color === undefined) continue;
      // Color==VECTREX_COLORS => inválido; ignorar
      if (d.color === (this.mod as any)?.Globals?.VECTREX_COLORS) continue;
      out.push({ x0: norm(d.x0, maxX), y0: norm(d.y0, maxY), x1: norm(d.x1, maxX), y1: norm(d.y1, maxY), intensity: (d.color ?? 0) * 2, frame: this.frameCounter });
      if (out.length > 8192) break; // límite defensivo
    }
    this.lastFrameSegments = out;
  }
  metrics(): MetricsSnapshot | null {
    const fcInit = this.fcInitCached || (this.mod as any)?.Globals?.FCYCLES_INIT || 25000;
    const fcycles = (this.inst as any)?.fcycles;
    let cycle_frame: number | undefined = undefined;
    if (typeof fcycles === 'number') {
      // Ciclo dentro del frame: invertimos resto (cuánto se ha consumido)
      cycle_frame = (fcInit - fcycles) >>> 0;
      if (cycle_frame < 0) cycle_frame = 0;
    }
    return {
      total: 0,
      unimplemented: 0,
      frames: this.frameCounter,
      cycle_frame,
      draw_vl: this.lastFrameSegments.length,
      last_intensity: this.lastFrameSegments.length ? this.lastFrameSegments[this.lastFrameSegments.length-1].intensity : 0,
      unique_unimplemented: [],
      cycles: this.totalCycles,
      avg_cycles_per_frame: this.frameCounter ? (this.totalCycles / this.frameCounter) : 0,
      top_opcodes: [] as any,
    } as any;
  }
  registers(): RegistersSnapshot | null {
    if (!this.inst) return null;
    const c = this.inst as any;
    const cpu = c.e6809;
    if (!cpu) return null;
    // Algunos registros index/stack están envueltos en objetos fptr -> usar .value
    const x = cpu.reg_x?.value ?? 0;
    const y = cpu.reg_y?.value ?? 0;
    const u = cpu.reg_u?.value ?? 0;
    const s = cpu.reg_s?.value ?? 0;
    const snap: RegistersSnapshot = {
      a: cpu.reg_a|0,
      b: cpu.reg_b|0,
      dp: cpu.reg_dp|0,
      x: x|0,
      y: y|0,
      u: u|0,
      s: s|0,
      pc: cpu.reg_pc|0,
      cycles: this.totalCycles,
      frame_count: this.frameCounter,
      last_intensity: this.lastFrameSegments.length ? this.lastFrameSegments[this.lastFrameSegments.length-1].intensity : 0,
      draw_vl_count: this.lastFrameSegments.length
    };
    // Añadir ciclo dentro de frame si disponible
    const fcycles = (this.inst as any)?.fcycles;
    const fcInit = this.fcInitCached || (this.mod as any)?.Globals?.FCYCLES_INIT;
    if (typeof fcycles === 'number' && fcInit) {
      (snap as any).cycle_frame = (fcInit - fcycles) >>> 0;
    }
    return snap;
  }
  getSegmentsShared(): Segment[] { return this.lastFrameSegments; }
  // Opcionales
  resetStats(){ /* noop */ }
  biosCalls(){ return []; }
  clearBiosCalls(){ /* noop */ }
  enableTraceCapture(){ /* noop */ }
  clearTrace(){ /* noop */ }
  traceLog(){ return []; }
  loopWatch(){ return []; }
  drainSegmentsJson(){ return []; }
  peekSegmentsJson(){ return this.lastFrameSegments; }
  demoTriangle(){ return this.lastFrameSegments; }
  snapshotMemory(){
    if (!this.inst) return new Uint8Array();
    const snap = new Uint8Array(65536);
    // Relleno por defecto (gap/illegal) 0xFF
    snap.fill(0xFF);
    // Cartridge 0000-BFFF (32K) desde cart (0x8000) – si un bin menor, resto queda 0xFF
    const cart = (this.inst as any).cart as number[];
    for (let i=0; i<0x8000 && i<cart.length; i++) snap[i] = cart[i] & 0xFF;
    // RAM shadow C800-CFFF (0x800) replicando 1K real dos veces
    const ram = (this.inst as any).ram as number[];
    for (let i=0; i<0x800; i++) snap[0xC800 + i] = ram[i & 0x3FF] & 0xFF;
    // BIOS E000-FFFF (8K)
    const rom = (this.inst as any).rom as number[];
    for (let i=0; i<0x2000 && i<rom.length; i++) snap[0xE000 + i] = rom[i] & 0xFF;
    // VIA región D000-D7FF simplificada: escribir algunos registros base repetidos cada 0x10
    const viaBase = 0xD000;
    const viaVals: Record<string,number> = {
      ora: (this.inst as any).via_ora|0,
      orb: (this.inst as any).via_orb|0,
      ddra: (this.inst as any).via_ddra|0,
      ddrb: (this.inst as any).via_ddrb|0,
      t1c_lo: (this.inst as any).via_t1c & 0xFF,
      t1c_hi: ((this.inst as any).via_t1c >> 8) & 0xFF,
      t2c_lo: (this.inst as any).via_t2c & 0xFF,
      t2c_hi: ((this.inst as any).via_t2c >> 8) & 0xFF,
      acr: (this.inst as any).via_acr|0,
      pcr: (this.inst as any).via_pcr|0,
      ifr: (this.inst as any).via_ifr|0,
      ier: (this.inst as any).via_ier|0
    };
    for (let off=0; off<0x800; off+=0x10){
      let idx=0; for (const k of Object.keys(viaVals)) { snap[viaBase + off + (idx++)] = viaVals[k]!; if (idx>=0x10) break; }
    }
    return snap;
  }
  invalidateMemoryView(){ /* noop */ }
  setInput(x:number=0,y:number=0,buttons:number=0){
    // Mapear rango esperado (-1..1) a 0..255 centrado en 128.
    if (!this.inst) return;
    const clamp = (v:number)=> Math.max(-1, Math.min(1, v));
    const toByte = (v:number)=> (128 + Math.round(clamp(v)*127)) & 0xFF;
    // Canal 0 = X, canal 1 = Y (convención interna jsvecx: alg_jch0/1)
    (this.inst as any).alg_jch0 = toByte(x);
    (this.inst as any).alg_jch1 = toByte(y);
    // Botones: placeholder – se podrían mapear a un registro VIA más adelante; guardamos copia por si métricas futuras.
    (this.inst as any)._buttonsSnapshot = buttons & 0x0F;
  }
}
