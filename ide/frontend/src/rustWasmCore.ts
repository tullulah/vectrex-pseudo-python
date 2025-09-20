// Implementación adaptadora que reutiliza la clase existente EmulatorService.
import { IEmulatorCore, MetricsSnapshot, RegistersSnapshot, Segment } from './emulatorCore';
// Reutilizamos el servicio actual con import relativo para no duplicar lógica.
import { EmulatorService } from './emulatorWasm';

export class RustWasmEmulatorCore implements IEmulatorCore {
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
  // Exponer acceso directo por si debugging desea llegar al wasm interno.
  get raw(){ return (this.svc as any).emu; }
}
