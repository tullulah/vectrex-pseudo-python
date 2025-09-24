// Implementación adaptadora que reutiliza la clase existente EmulatorService.
import { IEmulatorCore, MetricsSnapshot, RegistersSnapshot, Segment } from './emulatorCore';
// Reutilizamos el servicio actual con import relativo para no duplicar lógica.
import { EmulatorService } from './emulatorWasm';

export class RustWasmEmulatorCore implements IEmulatorCore {
  /** Devuelve true si el envelope PSG acaba de finalizar (evento one-shot, se limpia tras leer). */
  psgEnvJustFinished(): boolean { return this.svc.psgEnvJustFinished(); }
  private svc = new EmulatorService();
  init(wasmUrl?: string){ return this.svc.init(wasmUrl); }
  ensureBios(opts?: { bytes?: Uint8Array; urlCandidates?: string[] }){ return (this.svc as any).ensureBios?.(opts); }
  loadBios(bytes: Uint8Array){ this.svc.loadBios(bytes); }
  isBiosLoaded(){ return this.svc.isBiosLoaded(); }
  reset(){ this.svc.reset(); }
  resetStats(){ this.svc.resetStats(); }
  loadProgram(bytes: Uint8Array, base?: number){ this.svc.loadProgram(bytes, base); }
  runFrame(maxInstr?: number){ this.svc.runFrame(maxInstr); }
  metrics(): MetricsSnapshot | null { return this.svc.metrics() as any; }
  registers(): RegistersSnapshot | null { return this.svc.registers() as any; }
  biosCalls(){ return this.svc.biosCalls(); }
  clearBiosCalls(){ this.svc.clearBiosCalls(); }
  enableTraceCapture(on:boolean, limit?:number){ this.svc.enableTraceCapture(on, limit); }
  clearTrace(){ this.svc.clearTrace(); }
  traceLog(){ return this.svc.traceLog(); }
  loopWatch(){ return this.svc.loopWatch(); }
  getSegmentsShared(): Segment[] { return this.svc.getSegmentsShared(); }
  drainSegmentsJson(){ return this.svc.drainSegmentsJson?.(); }
  peekSegmentsJson(){ return this.svc.peekSegmentsJson?.(); }
  demoTriangle(){ return this.svc.demoTriangle?.(); }
  snapshotMemory(){ return this.svc.snapshotMemory?.(); }
  invalidateMemoryView(){ this.svc.invalidateMemoryView?.(); }
  setInput(x:number,y:number,buttons:number){ this.svc.setInput?.(x,y,buttons); }
  audioPrepareDelta?(): Int16Array {
    const raw: any = (this.svc as any).emu; if (!raw) return new Int16Array();
    if (typeof raw.psg_prepare_delta_pcm !== 'function') return new Int16Array();
    try {
      const count = raw.psg_prepare_delta_pcm() >>> 0;
      if (count === 0) return new Int16Array();
      const ptr = raw.psg_delta_pcm_ptr();
      const len = raw.psg_delta_pcm_len() >>> 0;
      if (!ptr || len === 0) return new Int16Array();
      // memory discovery similar to segments
      const mem: WebAssembly.Memory | undefined = (raw.memory) || (raw.constructor?.memory) || ((globalThis as any).wasmMemory);
      if (!mem) return new Int16Array();
      const bytes = new Int16Array(mem.buffer, ptr, len);
      // Copiamos para no retener view que se invalidará con próximas escrituras
      return new Int16Array(bytes);
    } catch { return new Int16Array(); }
  }
  audioSampleRate?(): number {
    const raw: any = (this.svc as any).emu; if (!raw) return 0;
    if (typeof raw.psg_sample_rate === 'function') { try { return raw.psg_sample_rate()|0; } catch {} }
    return 0;
  }
  audioHasOverflow?(): boolean {
    const raw: any = (this.svc as any).emu; if (!raw) return false;
    if (typeof raw.psg_delta_overflow === 'function') { try { return !!raw.psg_delta_overflow(); } catch {} }
    return false;
  }
  // Exponer acceso directo por si debugging desea llegar al wasm interno.
  get raw(){ return (this.svc as any).emu; }
}
