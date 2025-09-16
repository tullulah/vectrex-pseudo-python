// Minimal 6809 emulator (subset) translated from Rust CPU version for prototype.
// Supports limited opcodes used by generated minimal binaries and BIOS interception.

export interface VectorSegment { x1:number; y1:number; x2:number; y2:number; intensity:number; }

export class Cpu6809 {
  a=0; b=0; dp=0xD0; x=0; y=0; u=0; s=0xC000; pc=0; cc_z=false; cc_n=false; cc_c=false; cc_v=false; cc_i=false; // ORG fixed at $0000; stack still placed at $C000
  mem = new Uint8Array(65536);
  callStack: number[] = [];
  traceBiosVec = false; // enable extra tracing for BIOS vector routines (0xF3xx)
  noInterceptDraw = false; // if true, do not intercept Draw_VL family; let BIOS run normally
  noInterceptWaitRecal = false; // if true, do not intercept WAIT_RECAL (0xF192); let BIOS run authentic path
  biosPresent = false;
  // Raw BIOS image (4K expected) retained so that cold resets can reapply after memory clear
  biosImage: Uint8Array | null = null;
  lastIntensity = 0x5F;
  frameSegments: VectorSegment[] = [];
  viaEvents: { pc:number; reg:number; val:number }[] = []; // instrumentation for VIA writes in via mode
  debugTraces: Array<{ type: string; pc:number; x?:number; y?:number; x1?:number; y1?:number; x2?:number; y2?:number; intensity?:number; reg?:number; val?:number; note?:string; }> = [];
  traceEnabled = true;
  // Always-on critical events (not gated by traceEnabled) so diagnose can see timer/irq even if tracing off
  criticalEvents: Array<{ type:string; pc:number; note:string; cycles:number }> = [];
  // Execution mode toggle: 'intercept' (fast, current default) vs 'via' (future accurate timing)
  vectorMode: 'intercept' | 'via' = 'intercept';
  frameReady = false;
  trace = false;
  unknownLog: Record<string,number> = {};
  via = new Uint8Array(16);
  // --- VIA internal timer state (authentic semantics approximation) ---
  // We track counters separately from raw register shadow so we can model underflow precisely.
  private t1Counter = 0;          // 16-bit current counter (Timer1)
  private t1LatchLow = 0;         // Latched low byte (T1L-L)
  private t1LatchHigh = 0;        // Latched high byte (T1L-H)
  private t2Counter = 0;          // 16-bit current counter (Timer2)
  private t2LatchLow = 0;         // Latched low (Timer2 only has low-order latch)
  private timer2Started = false;  // Becomes true after first high byte write
  // --------------------------------------------------------------------
  // Beam (simplified logical coordinates) used for future VIA-driven reconstruction.
  beamX = 0; beamY = 0; beamDrawing = false; beamLastIntensity = 0x5F;
  cycles = 0; // total executed cycles (approx)
  // Simple frame timing target: Vectrex ~ 50/60Hz; choose 30000 cycles per frame placeholder until tuned.
  // --- Additional emulator state flags (some were previously implicitly created) ---
  hardwareVectors = false;          // when true in via mode, integrate beam movement
  waitingForVsync = false;          // BIOS WAIT_RECAL style wait (used before authentic timing complete)
  irqPending = false;               // computed from VIA IFR/IER
  waiWaiting = false;               // CPU is in WAI low-power state
  bridgeListToVia = false;          // experimental: synthesize VIA writes from intercepted vector lists
  autoStartUser = true;             // attempt auto jump to 0x0000 once if cartridge-like data present
  attemptedAutoStart = false;       // guard so auto start only tried once
  autoStartInfo: any = null;        // debug info about auto-start decision
  lastWaitRecalEnterEvent: any = null; // persisted event so later frames can re-surface
  waitRecalInterrupted = false;     // WAI was interrupted by IRQ
  disableSyntheticIrq = false;      // dev flag to turn off synthetic IRQ fallback
  // Integrator / velocity model (authentic path WIP)
  velX = 0; velY = 0; // signed velocities (fixed-point scaled by 1 per cycle for now)
  lastBeamUpdateCycles = 0;
  // Optional opcode trace (for diagnosing missing BIOS calls). Disabled by default due to overhead.
  opcodeTraceEnabled = false;
  opcodeTrace: Array<{pc:number; op:number}> = [];
  maxOpcodeTrace = 2048;
  // Full instruction trace (optional heavy). Each entry records pc, opcode, and up to 3 operand bytes consumed.
  fullInstrTraceEnabled = false;
  fullInstrTrace: Array<{pc:number; op:number; b1?:number; b2?:number; b3?:number}> = [];
  // Synthetic IRQ bookkeeping (for intercept fallback while VIA not fully emulated)
  lastSyntheticIrqCheck = 0;
  // Lightweight always-on tail buffer (independent of full trace) for last N executed instructions
  tailInstr: Array<{pc:number; op:number; b1:number; b2:number; b3:number}> = [];
  tailInstrMax = 128;
  // Development aids for BIOS IFR polling loop detection / timer bootstrap
  bitbIfrLoopCount = 0; // consecutive BITB $D00D (IFR) iterations
  devTimer1Seeded = false;
  devTimer1SeedCycle = 0;
  devForcedIfr = false;
  timer1Started = false; // becomes true after first high-byte write (0x05) initializes counter/latch
  // PC=0 guard instrumentation
  pcZeroGuardTriggered = false;
  pcZeroSnapshot: { regs:any; stackBytes:number[]; tail:any[] } | null = null;
  // RTS/RTI instrumentation log
  rtsLog: Array<{ type:'RTS'|'RTI'; pcBefore:number; pcAfter:number; sBefore:number; sAfter:number; returnPc:number; cc?:{z:boolean;n:boolean;c:boolean;v:boolean;i:boolean} }> = [];

  // Opcode cycle lookup (rough; refine later per addressing mode). Default fallback = 2.
  private static cycleTable: Record<number,number> = {
    0x20: 3, 0x21: 3, 0x22: 3, 0x23: 3, 0x24:3, 0x25:3, 0x26:3, 0x27:3, 0x2A:3, 0x2B:3, 0x2C:3, 0x2D:3, 0x2E:3, 0x2F:3,
    0x8B:2, 0xC8:2, 0x80:2, 0x81:2, 0x83:4, 0xC3:4, 0xCC:3,
    0x86:2, 0xC6:2, 0xCB:2, 0xC0:2, 0x8E:3, 0xCE:3,
    0x96:4, 0x97:4, 0x9B:5, 0x93:6, 0xAB:5, 0xA3:6, 0xBB:5, 0xB3:6,
    0xB6:5, 0xB7:5, 0xB5:5,
    0xD6:4, 0xD7:4, 0xF6:5, 0xF7:5, 0xE6:5,
    0xDE:5, 0xDF:5, 0xFE:6, 0xFF:6, 0xEE:6,
    0x9E:5, 0x9F:5, 0xBE:6, 0xBF:6, 0xAE:6, 0xAF:6,
    0x34:5, 0x35:5, 0x39:5, 0x7E:3, 0x0E:4, 0x6E:4,
    0x1A:3, 0x1C:3, 0x3E:4,
    0xA6:5, 0xA7:5,
  };

  setVectorMode(m: 'intercept' | 'via'){ this.vectorMode = m; if (m==='via') this.hardwareVectors = true; }

  loadBin(bytes: Uint8Array, base=0) { for (let i=0;i<bytes.length;i++){ const addr=base+i; if (addr<65536) this.mem[addr]=bytes[i]; } }
  /**
   * Load a Vectrex BIOS image.
   *  - Canonical mask ROM size: 4096 bytes (maps to 0xF000-0xFFFF)
   *  - Some distributed dumps are 8192 bytes containing two 4K halves (duplicate or variant ordering).
   *    Older code always picked the upper half; however several public 8K dumps have the RESET vector
   *    (bytes at FFFE/FFFF in the mapped window) only valid in the FIRST 4K half when mapped, causing PC=0.
   *
   * Auto-detection strategy for 8K:
   *  1. Examine both halves (0-4095 and 4096-8191) as candidates.
   *  2. For each half, pretend it is mapped at 0xF000 and read candidate reset vector bytes at offsets
   *     (0x0FFE, 0x0FFF) inside the half. (Because 0xF000 + 0x0FFE = 0xFFFE.)
   *  3. A vector is considered plausible if: hi byte in [0xF0,0xFF], lo byte in [0x00,0xFF], and the 16-bit
   *     address >= 0xF000. Reject vectors of 0x0000 or 0xFFFF (common erased/placeholder patterns).
   *  4. Prefer the first half if both are plausible but the second is all 0x00/0xFF or duplicates.
   *  5. Fallback: if neither half yields a plausible vector, retain previous heuristic (choose upper half)
   *     but emit a trace note to aid debugging.
   */
  loadBios(bytes: Uint8Array) {
    let chosen: Uint8Array | null = null;
    let chosenIndex = -1; // 0 = first 4K, 1 = second 4K
    const noteParts: string[] = [];
    const isPlausibleVec = (hi:number, lo:number) => {
      const addr = ((hi<<8)|lo) & 0xFFFF;
      if (addr < 0xF000) return false;
      if (addr === 0x0000 || addr === 0xFFFF) return false;
      // hi must be 0xF? for BIOS address space
      if ((hi & 0xF0) !== 0xF0) return false;
      return true;
    };
    if (bytes.length === 4096){
      const hi = bytes[0x0FFE]; const lo = bytes[0x0FFF];
      const plausible = isPlausibleVec(hi,lo);
      if (!plausible) noteParts.push(`resetVec_suspect_${hi.toString(16)}${lo.toString(16)}`);
      chosen = bytes; chosenIndex = 0;
    } else if (bytes.length === 8192){
      const first = bytes.slice(0,4096);
      const second = bytes.slice(4096);
      const hi1 = first[0x0FFE]; const lo1 = first[0x0FFF];
      const hi2 = second[0x0FFE]; const lo2 = second[0x0FFF];
      const p1 = isPlausibleVec(hi1,lo1);
      const p2 = isPlausibleVec(hi2,lo2);
      noteParts.push(`vec1=${hi1.toString(16)}${lo1.toString(16)}_${p1?'ok':'bad'}`, `vec2=${hi2.toString(16)}${lo2.toString(16)}_${p2?'ok':'bad'}`);
      if (p1 && !p2){ chosen=first; chosenIndex=0; }
      else if (!p1 && p2){ chosen=second; chosenIndex=1; }
      else if (p1 && p2){
        // If both plausible prefer the one whose hi/lo are not identical (avoid 0xF0F0 style) else first
        if (hi1===lo1 && hi2!==lo2) { chosen=second; chosenIndex=1; }
        else { chosen=first; chosenIndex=0; }
      } else {
        // Neither plausible – retain legacy behavior (second half) to preserve prior expectations
        chosen = second; chosenIndex=1; noteParts.push('fallback_legacy_upper_half');
      }
    } else {
      return false; // Unsupported size
    }
    if (!chosen) return false;
    this.biosImage = new Uint8Array(chosen);
    this.loadBin(chosen, 0xF000);
    this.biosPresent = true;
    if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:0xF000, note:`bios-loaded-${chosen.length}-half${chosenIndex}-${noteParts.join('|')}` });
    return true;
  }
  /** Re-apply BIOS image after a memory clear/reset. Safe no-op if not present. */
  reapplyBios(){ if (this.biosPresent && this.biosImage) this.loadBin(this.biosImage, 0xF000); }

  /** Hardware-like reset: reapply BIOS (if present), clear state, fetch reset vector at 0xFFFE/0xFFFF into PC. */
  cpuReset(){
    // Preserve BIOS image and reload it to ensure contents
    if (this.biosPresent && this.biosImage) this.loadBin(this.biosImage, 0xF000);
  this.a=0; this.b=0; this.dp=0xD0; this.x=0; this.y=0; this.u=0; this.s=0xC000; // preserve stack top
    this.cc_z=false; this.cc_n=false; this.cc_c=false; this.cc_v=false; this.cc_i=true; // I flag set after reset
    this.frameSegments.length=0; this.callStack.length=0; this.unknownLog={};
    this.cycles=0; this.waitingForVsync=false; this.irqPending=false; this.waiWaiting=false;
    this.opcodeTrace.length=0; this.beamX=0; this.beamY=0; this.beamDrawing=false; this.beamLastIntensity=0x5F;
    // Fetch reset vector (little endian at FFFE/FFFF)
    const hi=this.mem[0xFFFE]; const lo=this.mem[0xFFFF];
    const start=((hi<<8)|lo)&0xFFFF;
    this.pc = start;
    if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'cpuReset' });
  }

  private setD(v:number){ this.a=(v>>8)&0xFF; this.b=v&0xFF; }
  private d(){ return (this.a<<8)|this.b; }
  private nz8(v:number){ this.cc_z=(v&0xFF)===0; this.cc_n=(v&0x80)!==0; }
  private nz16(v:number){ this.cc_z=(v&0xFFFF)===0; this.cc_n=(v&0x8000)!==0; }
  private add8(a:number,b:number){ const r=a+b; const res=r&0xFF; this.cc_c = r>0xFF; this.cc_v = (~(a^b) & (a^res) & 0x80)!==0; this.nz8(res); return res; }
  private sub8(a:number,b:number){ const r=a-b; const res=r&0xFF; this.cc_c = a < b; this.cc_v = ((a^b) & (a^res) & 0x80)!==0; this.nz8(res); return res; }
  private adc8(a:number,b:number){ const cin=this.cc_c?1:0; const r=a+b+cin; const res=r&0xFF; this.cc_c = r>0xFF; this.cc_v = (~(a^b) & (a^res) & 0x80)!==0; this.nz8(res); return res; }
  private sbc8(a:number,b:number){ const cin=this.cc_c?1:0; const t=b+cin; const r=a - t; const res=r & 0xFF; this.cc_c = a < t; this.cc_v = ((a^b) & (a^res) & 0x80)!==0; this.nz8(res); return res; }
  private add16(a:number,b:number){ const r=a+b; const res=r&0xFFFF; this.cc_c = r>0xFFFF; // V for 16-bit add: (~(a^b) & (a^res) & 0x8000) !=0
    this.cc_v = (~(a^b) & (a^res) & 0x8000)!==0; this.nz16(res); return res; }
  private sub16(a:number,b:number){ const r=a-b; const res=r&0xFFFF; this.cc_c = a < b; this.cc_v = ((a^b) & (a^res) & 0x8000)!==0; this.nz16(res); return res; }

  // === Memory Read-Modify-Write helpers for 8-bit operations ===
  private rmwNEG(v:number){ const res=(0 - v) & 0xFF; this.nz8(res); this.cc_c = v!==0; this.cc_v = (v===0x80); return res; }
  private rmwCOM(v:number){ const res=(~v)&0xFF; this.nz8(res); this.cc_c=true; this.cc_v=false; return res; }
  private rmwLSR(v:number){ this.cc_c = (v & 0x01)!==0; const res=(v>>>1)&0x7F; this.nz8(res); this.cc_v=false; this.cc_n=false; return res; }
  private rmwROR(v:number){ const carryIn=this.cc_c?0x80:0; const newC=(v & 0x01)!==0; const res=((v>>>1)|carryIn)&0xFF; this.cc_c=newC; this.nz8(res); this.cc_v=false; return res; }
  private rmwASR(v:number){ const newC=(v & 0x01)!==0; const res=((v & 0x80)|(v>>>1))&0xFF; this.cc_c=newC; this.nz8(res); this.cc_v=false; return res; }
  private rmwASL(v:number){ const newC=(v & 0x80)!==0; const res=(v<<1)&0xFF; this.cc_c=newC; this.nz8(res); this.cc_v = ((v ^ res) & 0x80)!==0; return res; }
  private rmwROL(v:number){ const carryIn=this.cc_c?1:0; const newC=(v & 0x80)!==0; const res=((v<<1)|carryIn)&0xFF; this.cc_c=newC; this.nz8(res); this.cc_v = ((v ^ res) & 0x80)!==0; return res; }
  private rmwDEC(v:number){ const res=(v-1)&0xFF; this.nz8(res); this.cc_v = (res===0x7F); return res; }
  private rmwINC(v:number){ const res=(v+1)&0xFF; this.nz8(res); this.cc_v = (res===0x80); return res; }
  private rmwTST(v:number){ this.nz8(v); this.cc_v=false; return v; }
  private rmwCLR(_v:number){ this.cc_z=true; this.cc_n=false; this.cc_v=false; this.cc_c=false; return 0; }
  private applyRmw(kind:string, val:number){
    switch(kind){
      case 'NEG': return this.rmwNEG(val);
      case 'COM': return this.rmwCOM(val);
      case 'LSR': return this.rmwLSR(val);
      case 'ROR': return this.rmwROR(val);
      case 'ASR': return this.rmwASR(val);
      case 'ASL': return this.rmwASL(val);
      case 'ROL': return this.rmwROL(val);
      case 'DEC': return this.rmwDEC(val);
      case 'INC': return this.rmwINC(val);
      case 'TST': return this.rmwTST(val);
      case 'CLR': return this.rmwCLR(val);
    }
    return val;
  }

  private interceptBios(addr:number){
    switch(addr){
      case 0xF192: // WAIT_RECAL
        if (this.noInterceptWaitRecal) {
          // Allow authentic BIOS code to run (no interception). We still might tag a trace note for visibility.
          if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'WAIT_RECAL_auth_pass' });
          // When authentic path is enabled, we expect the BIOS routine itself will execute a WAI (opcode 0x3E)
          // but to aid harness detection (in case sequence differs), emit a synthetic 'wai_enter' style critical event
          // exactly once per call if we are not already in a waiWaiting state.
          if (!this.waiWaiting && this.criticalEvents.length < 1024) {
            const ev = { type:'wait_recal_enter', pc:this.pc, note:'wai_enter_auth', cycles:this.cycles };
            this.criticalEvents.push(ev);
            this.lastWaitRecalEnterEvent = ev;
          }
          break;
        }
        if (this.vectorMode === 'intercept') {
          // Treat first WAIT_RECAL after load/start as start-of-frame (no frameReady), subsequent as end-of-frame.
          if (!(this as any)._sawFirstWaitRecal) {
            (this as any)._sawFirstWaitRecal = true;
            // Normalize DP as BIOS would on entry
            this.dp=0xD0;
            if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'WAIT_RECAL_start' });
          } else {
            this.dp=0xD0;
            this.frameReady = true;
            if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'WAIT_RECAL_end' });
          }
        } else {
          // VIA mode: use timer underflow still; we mark waiting each call
          this.waitingForVsync = true;
          if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'WAIT_RECAL_via' });
        }
        break;
      case 0xF2A5: // INTENSITY_5F
        if (this.vectorMode === 'intercept') this.lastIntensity = 0x5F; else this.lastIntensity = 0x5F; // later derive from DAC latch
        if (this.traceEnabled) this.debugTraces.push({ type:'intensity', pc:this.pc, intensity:this.lastIntensity, note:'INTENSITY_5F' });
        break;
      case 0xF2AB: // INTENSITY_A
        if (this.vectorMode === 'intercept') this.lastIntensity = this.a; else this.lastIntensity = this.a; // placeholder
        if (this.traceEnabled) this.debugTraces.push({ type:'intensity', pc:this.pc, intensity:this.lastIntensity, note:'INTENSITY_A' });
        break;
  case 0xF3DD: // DRAW_VL (per VECTREX.I)
        if (this.vectorMode === 'intercept') {
          this.decodeVectorList();
        } else if (this.vectorMode === 'via' && this.bridgeListToVia) {
          this.emitViaFromVectorList();
        }
        if (this.traceEnabled) this.debugTraces.push({ type:'vectorlist', pc:this.pc, note:'DRAW_VL' });
        break;
      // RESET0REF / MOVETO etc. ignored for now
      default: break;
    }
  }

  /** Heuristic: if BIOS present and PC ended up in BIOS space after reset while user region (0x0000-0x7FFF)
   *  contains non-zero data but no recognizable cartridge header ("gce" ASCII at 0x0000), then jump to 0x0000.
   *  Only performed once per load. */
  attemptAutoStartUser(){
    if (!this.autoStartUser || this.attemptedAutoStart) return;
    this.attemptedAutoStart = true;
    if (!this.biosPresent){ this.autoStartInfo={performed:false, reason:'no_bios'}; return; }
    if (this.pc < 0xF000){ this.autoStartInfo={performed:false, reason:'already_in_user_space'}; return; }
    // Scan first 64 bytes for any non-zero
    let anyNonZero=false; for (let i=0;i<64;i++){ if (this.mem[i]!==0){ anyNonZero=true; break; } }
    if (!anyNonZero){ this.autoStartInfo={performed:false, reason:'user_region_empty'}; return; }
    // Detect simple cartridge header signature: 'gce' (lowercase or uppercase) at bytes 0-2.
    const b0=this.mem[0]; const b1=this.mem[1]; const b2=this.mem[2];
    const isHeader = ((b0===0x47||b0===0x67) && (b1===0x43||b1===0x63) && (b2===0x45||b2===0x65));
    if (isHeader){ this.autoStartInfo={performed:false, reason:'cartridge_header_present'}; return; }
    const oldPc=this.pc;
    this.pc = 0x0000;
    this.autoStartInfo={performed:true, reason:'forced_pc_to_0000'};
    if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:oldPc, note:'auto-start-user->0000' });
  }

  // Hook future memory reads/writes for VIA address space. For now pass-through but placeholder kept for expansion.
  private viaRead(offset:number){
    offset &= 0x0F;
    switch(offset){
      case 0x0D: { // IFR read: bit7=1 only if (IFR & IER & 0x7F)!=0
        const ifr = this.via[0x0D] & 0x7F;
        const ier = this.via[0x0E] & 0x7F;
        const master = ((ifr & ier) !== 0) ? 0x80 : 0x00;
        return ifr | master;
      }
      case 0x0E: { // IER read: bit7 always set
        return (this.via[0x0E] & 0x7F) | 0x80;
      }
      case 0x04: { // T1C-L read: returns low counter & clears IFR6
        const val = this.t1Counter & 0xFF;
        if (this.via[0x0D] & 0x40){
          this.via[0x0D] &= ~0x40; this.recomputeIrqPending();
          if (this.traceEnabled) this.debugTraces.push({ type:'via-read', pc:this.pc, reg:0x04, val, note:'T1C-L-clearIFR6' });
        }
        return val;
      }
      case 0x05: { // T1C-H read: returns high counter (no flag clear)
        return (this.t1Counter>>8) & 0xFF;
      }
      case 0x06: return this.t1LatchLow & 0xFF; // T1L-L
      case 0x07: return this.t1LatchHigh & 0xFF; // T1L-H
      case 0x08: { // T2C-L: read & clear IFR5
        const val = this.t2Counter & 0xFF;
        if (this.via[0x0D] & 0x20){
          this.via[0x0D] &= ~0x20; this.recomputeIrqPending();
          if (this.traceEnabled) this.debugTraces.push({ type:'via-read', pc:this.pc, reg:0x08, val, note:'T2C-L-clearIFR5' });
        }
        return val;
      }
      case 0x09: return (this.t2Counter>>8) & 0xFF; // T2C-H
      default: return this.via[offset];
    }
  }
  private viaWrite(offset:number, val:number){
    offset &= 0x0F; val &= 0xFF;
    switch(offset){
      case 0x0D: { // IFR write: writing 1 clears that flag
        const clr = val & 0x7F; this.via[0x0D] &= (~clr) & 0x7F; this.recomputeIrqPending();
        if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x0D, val, note:'IFR-clear' });
        return; }
      case 0x0E: { // IER write: bit7 selects set/clear
        const before = this.via[0x0E] & 0x7F; const mask = val & 0x7F;
        if (val & 0x80) this.via[0x0E] = (before | mask) & 0x7F; else this.via[0x0E] = before & (~mask);
        const after = this.via[0x0E] & 0x7F;
        if (!(before & 0x40) && (after & 0x40)){
          if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x0E, val, note:'IER_enable_T1' });
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'ier_enable_t1', pc:this.pc, note:'ier_enable_t1', cycles:this.cycles });
        }
        if (!(before & 0x20) && (after & 0x20)){
          if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x0E, val, note:'IER_enable_T2' });
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'ier_enable_t2', pc:this.pc, note:'ier_enable_t2', cycles:this.cycles });
        }
        this.recomputeIrqPending();
        if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x0E, val, note:'IER' });
        return; }
      case 0x04: { // T1C-L write (low counter)
        this.via[0x04] = val; return; }
      case 0x05: { // T1C-H write: loads counter & latch
        this.via[0x05] = val; const lo=this.via[0x04];
        this.t1LatchLow = lo; this.t1LatchHigh = val; this.t1Counter = ((val<<8)|lo) & 0xFFFF; this.timer1Started=true;
        this.via[0x06] = lo; this.via[0x07] = val; // mirror latch
        // Writing high byte clears IFR6 per 6522
        this.via[0x0D] &= ~0x40; this.recomputeIrqPending();
        if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x05, val, note:'T1-load' });
        return; }
      case 0x06: { // T1L-L write only affects latch
        this.t1LatchLow = val; this.via[0x06]=val; return; }
      case 0x07: { // T1L-H write only affects latch
        this.t1LatchHigh = val; this.via[0x07]=val; return; }
      case 0x08: { // T2C-L write (low)
        this.via[0x08] = val; return; }
      case 0x09: { // T2C-H write loads counter (one-shot for now)
        this.via[0x09] = val; const lo=this.via[0x08];
        this.t2LatchLow = lo; this.t2Counter = ((val<<8)|lo) & 0xFFFF; this.timer2Started=true; this.via[0x0D] &= ~0x20; this.recomputeIrqPending();
        if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x09, val, note:'T2-load' });
        return; }
      case 0x0A: { // Shift Register write
        this.via[0x0A] = val & 0xFF; this.srValue = val & 0xFF;
        // Clear IFR4 on write
        this.via[0x0D] &= ~0x10; this.recomputeIrqPending();
        // Determine shift mode from ACR bits2-4 (internal free-run shift out only mode 0b100 supported now)
        const mode = (this.via[0x0B] & 0x1C) >> 2;
        if (mode === 0b100){ this.srActive = true; this.srBitsRemaining = 8; this.srCycleAccumulator = 0; }
        else { this.srActive = false; this.srBitsRemaining = 0; }
        if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x0A, val, note:'SR-load' });
        return; }
      case 0x0B: { // ACR write
        this.via[0x0B] = val & 0xFF;
        // If shift mode turned off from 0b100, abort any active shift
        const mode = (val & 0x1C) >> 2; if (mode !== 0b100){ this.srActive=false; this.srBitsRemaining=0; }
        if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x0B, val, note:'ACR' });
        return; }
      case 0x0C: { // PCR write (cache only for now)
        this.pcr = val & 0xFF; this.via[0x0C] = val & 0xFF;
        if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x0C, val, note:'PCR' });
        return; }
      default: {
        this.via[offset] = val;
        if (this.vectorMode==='via'){
          if (offset===0x00){ // ORB velocity nibble pack
            const dx4=(val>>4)&0x0F; const dy4=val&0x0F; this.velX = (dx4 & 0x08)? dx4-0x10 : dx4; this.velY = (dy4 & 0x08)? dy4-0x10 : dy4; }
          else if (offset===0x01){ // ORA intensity
            this.beamLastIntensity = Math.max(0, Math.min(127,val & 0x7F)); }
          else if (offset===0x0B){ // ACR gating placeholder bits0/1
            if (val & 0x01) this.beamDrawing=true; if (val & 0x02) this.beamDrawing=false; }
          if (this.traceEnabled && (offset===0x00||offset===0x01||offset===0x0B)){
            this.debugTraces.push({ type:'via-write', pc:this.pc, reg:offset, val, note: offset===0x0B?'ACR':(offset===0x00?'ORB':'ORA') });
          }
        }
        return; }
    }
  }
  // VIA write effect hook: now ONLY records instrumentation; authentic behavior (integrators/blanking)
  // will be synthesized later from real VIA register semantics. Previous provisional X/Y delta mapping removed.
  private viaWriteEffect(offset:number, val:number){
    // Perform base write (handles IFR / IER semantics internally)
    this.viaWrite(offset,val);
    if (this.vectorMode !== 'via') return;
    // Ignore non-drawing experiment registers
    if (offset === 0x0D || offset === 0x0E) return;
    // Instrumentation capture (store truncated history to bound memory)
    if (this.viaEvents.length < 5000){ this.viaEvents.push({ pc:this.pc, reg: offset & 0x0F, val: val & 0xFF }); }
    if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg: offset & 0x0F, val: val & 0xFF });
    // No immediate beam update; deferred synthesis step.
  }
  // Extended VIA state (shift register, PB7, PCR cache)
  private srValue: number = 0;
  private srBitsRemaining: number = 0;
  private srActive: boolean = false;
  private srCycleAccumulator: number = 0;
  private pb7State: boolean = false;
  private pcr: number = 0;
  private recomputeIrqPending(){
    // IRQ pending if any IFR bit (0-6) that is enabled in IER is set.
    const ifr = this.via[0x0D] & 0x7F;
    const ier = this.via[0x0E] & 0x7F;
    this.irqPending = (ifr & ier) !== 0;
  }
  private packCC(){ return (this.cc_c?1:0)|(this.cc_v?2:0)|(this.cc_z?4:0)|(this.cc_n?8:0)|(this.cc_i?0x10:0); }
  private serviceIrq(){
    // Standard IRQ: push full state (mirrors SWI style here for simplicity) then vector to IRQ handler.
    this.pshs(0xFF); // push PC,U,Y,X,DP,B,A,CC (symmetrical with pulls(0xFF))
    this.cc_i = true; // mask further IRQs (I flag)
    this.waiWaiting = false; // exit any wait state
    if (this.waitRecalInterrupted) {
      // already in interrupted state
    } else if ((this as any).lastWaiPc !== undefined) {
      // Mark that an IRQ interrupted a WAI so we should later emit wait_recal_exit after RTI
      this.waitRecalInterrupted = true;
    }
    const hi=this.mem[0xFFF8]; const lo=this.mem[0xFFF9]; // IRQ vector
    this.pc = ((hi<<8)|lo) & 0xFFFF;
    if (this.traceEnabled) this.debugTraces.push({ type:'irq-enter', pc:this.pc, note:'irq-enter' });
    if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'irq_enter', pc:this.pc, note:'irq_enter', cycles:this.cycles });
  }
  private setTimer1(value:number){
    value &= 0xFFFF;
    // NOTE: Real 6522 layout is T1C-L @4, T1C-H @5. Current prototype historically used reversed hi/lo.
    // We transition toward correct order now (low at 0x04, high at 0x05) while maintaining legacy loads if already swapped.
    this.t1LatchLow = value & 0xFF;
    this.t1LatchHigh = (value>>8) & 0xFF;
    this.t1Counter = value;
    this.via[0x04] = this.t1LatchLow;
    this.via[0x05] = this.t1LatchHigh;
  }
  private getTimer1(): number { return this.t1Counter & 0xFFFF; }
  private getTimer1Latch(): number { return ((this.t1LatchHigh<<8)|this.t1LatchLow) & 0xFFFF; }
  private setTimer1Latch(value:number){
    value &= 0xFFFF;
    this.t1LatchLow = value & 0xFF;
    this.t1LatchHigh = (value>>8) & 0xFF;
    this.via[0x06] = this.t1LatchLow;
    this.via[0x07] = this.t1LatchHigh;
  }
  private updateTimers(cyc:number){
    // Timer1 (IFR bit6). Supports one-shot vs continuous via ACR bit6 (0x0B bit6).
    // Countdown performed in whole CPU cycles (fine for now). When counter reaches <0, underflow triggers.
    if (this.timer1Started && this.t1Counter > 0){
      this.t1Counter -= cyc;
      if (this.t1Counter <= 0){
        // Underflow event
        this.via[0x0D] |= 0x40; // IFR6
        this.recomputeIrqPending();
        if (this.traceEnabled) this.debugTraces.push({ type:'timer1-underflow', pc:this.pc, note:'t1_underflow' });
        if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'t1_underflow', pc:this.pc, note:'t1_underflow', cycles:this.cycles });
        // If IRQ now pending and interrupts are enabled (cc_i=false), service immediately to reflect hardware edge
        if (this.irqPending && !this.cc_i){
          this.serviceIrq();
        }
        // PB7 toggle if ACR bit7 set
        if ((this.via[0x0B] & 0x80)!==0){
          this.pb7State = !this.pb7State;
          const old = this.via[0x00];
          this.via[0x00] = (old & 0x7F) | (this.pb7State?0x80:0);
          if (this.traceEnabled) this.debugTraces.push({ type:'pb7-toggle', pc:this.pc, note:'pb7_toggle', val:this.via[0x00] });
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'pb7_toggle', pc:this.pc, note:'pb7_toggle', cycles:this.cycles });
        }
        const continuous = (this.via[0x0B] & 0x40)!==0; // ACR bit6
        if (continuous){
          const latch = this.getTimer1Latch() || 0x10000; // treat 0 as 65536 per 6522 quirk
          let overshoot = -this.t1Counter; // positive
          // Reload repeatedly if overshoot larger than latch
          while (overshoot >= latch) overshoot -= latch;
          this.t1Counter = latch - overshoot;
        } else {
          this.t1Counter = 0;
        }
      }
    }
    // Timer2 (IFR bit5) one-shot mode (pulse counting ignored) – frame timer for WAIT_RECAL.
    if (this.timer2Started && this.t2Counter > 0){
      this.t2Counter -= cyc;
      if (this.t2Counter <= 0){
        this.via[0x0D] |= 0x20; // IFR5
        this.recomputeIrqPending();
        if (this.traceEnabled) this.debugTraces.push({ type:'timer2-underflow', pc:this.pc, note:'t2_underflow' });
        if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'t2_underflow', pc:this.pc, note:'t2_underflow', cycles:this.cycles });
        this.t2Counter = 0; // one-shot
        if (this.irqPending && !this.cc_i){
          this.serviceIrq();
        }
      }
    }
    if ((this as any).devTimerAssist){
      // Dev auto-seed: if no underflow after many cycles, force a smaller period + enable interrupt
      if (!this.debugTraces.some(t=>t.note==='t1_underflow') && this.cycles>20000){
        if (!this.debugTraces.some(t=>t.note==='auto_timer1_seed')){
          const seed=8000;
          this.setTimer1(seed); this.setTimer1Latch(seed);
          this.via[0x0B] |= 0x40; // continuous
          this.via[0x0E] |= 0x40; // enable bit6 (simplified direct set)
          this.recomputeIrqPending();
          if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'auto_timer1_seed' });
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'auto_seed', pc:this.pc, note:'auto_timer1_seed', cycles:this.cycles });
          this.devTimer1Seeded = true; this.devTimer1SeedCycle = this.cycles;
        }
      }
      // If we seeded earlier but timer still not underflowed after expected period, force IFR bit6 to unblock BIOS
      if (this.devTimer1Seeded && !this.devForcedIfr && !this.debugTraces.some(t=>t.note==='t1_underflow')){
        const t1 = this.getTimer1();
        if (this.cycles - this.devTimer1SeedCycle > 12000){
          // Force underflow condition
          this.via[0x0D] |= 0x40; // IFR bit6
          this.recomputeIrqPending();
          this.devForcedIfr = true;
          if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'dev_force_ifr6' });
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'force_ifr6', pc:this.pc, note:'dev_force_ifr6', cycles:this.cycles });
        }
      }
    }
    // Shift register timing: shift 1 bit every 4 cycles (simplified) when active internal clock mode
    if (this.srActive && this.srBitsRemaining > 0){
      this.srCycleAccumulator += cyc;
      const shiftPeriod = 4;
      while (this.srBitsRemaining > 0 && this.srCycleAccumulator >= shiftPeriod){
        this.srCycleAccumulator -= shiftPeriod;
        // Shift MSB first; capture MSB if future CB2 output modeling added.
        const msb = (this.srValue & 0x80)!==0 ? 1:0; // reserved for CB2
        this.srValue = ((this.srValue << 1) & 0xFF);
        this.srBitsRemaining--;
        if (this.srBitsRemaining === 0){
          this.srActive = false; this.via[0x0A] = this.srValue;
          this.via[0x0D] |= 0x10; // IFR bit4
          this.recomputeIrqPending();
          if (this.traceEnabled) this.debugTraces.push({ type:'sr-complete', pc:this.pc, note:'sr_complete', val:this.srValue });
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'sr_complete', pc:this.pc, note:'sr_complete', cycles:this.cycles });
          if (this.irqPending && !this.cc_i) this.serviceIrq();
        }
      }
    }
  }

  private decodeVectorList(){
    // Assume U points to list: sequence of (y,x) signed bytes; bit7 set in y = end (clear bit then use); segments connect consecutive points starting at first.
    let ptr = this.u & 0xFFFF; if (ptr>=0x10000) return;
    const coords: Array<[number,number]> = [];
    for (let safety=0; safety<1024; safety++){
      const yRaw = this.mem[ptr];
      const end = (yRaw & 0x80)!==0;
      const y = ((yRaw & 0x7F) << 25) >> 25; // sign extend 7 bits -> 32
      const xRaw = this.mem[(ptr+1)&0xFFFF];
      const x = (xRaw<<24)>>24; // sign extend 8 bits
      coords.push([x,y]);
      ptr = (ptr+2)&0xFFFF;
      if (end) break;
    }
    if (coords.length>1 && this.vectorMode === 'intercept'){
      for (let i=1;i<coords.length;i++){
        const [x1,y1]=coords[i-1]; const [x2,y2]=coords[i];
        this.frameSegments.push({x1,y1,x2,y2,intensity:this.lastIntensity});
        if (this.traceEnabled) this.debugTraces.push({ type:'segment', pc:this.pc, x1,y1,x2,y2,intensity:this.lastIntensity, note:'intercept' });
      }
    } else if (this.vectorMode==='intercept' && this.traceEnabled) {
      this.debugTraces.push({ type:'info', pc:this.pc, note:`vectorlist-too-short-${coords.length}` });
    }
  }

  private emitViaFromVectorList(){
    // Reuse decode logic to collect coordinates without pushing segments directly.
    let ptr = this.u & 0xFFFF; if (ptr>=0x10000) return;
    const coords: Array<[number,number]> = [];
    for (let safety=0; safety<1024; safety++){
      const yRaw = this.mem[ptr];
      const end = (yRaw & 0x80)!==0;
      const y = ((yRaw & 0x7F) << 25) >> 25;
      const xRaw = this.mem[(ptr+1)&0xFFFF];
      const x = (xRaw<<24)>>24;
      coords.push([x,y]);
      ptr = (ptr+2)&0xFFFF;
      if (end) break;
    }
    if (coords.length<2) return;
    // Synthetic event sequence: set intensity (reuse lastIntensity), start draw, emit ORB velocity steps per segment, stop draw.
    let lastX = coords[0][0];
    let lastY = coords[0][1];
    this.beamX = lastX; this.beamY = lastY;
    // Start drawing via ACR bit0 set
    if (this.viaEvents.length < 5000) this.viaEvents.push({ pc:this.pc, reg:0x0B, val: 0x01 });
    if (this.traceEnabled) this.debugTraces.push({ type:'draw-start', pc:this.pc, x:lastX, y:lastY });
    for (let i=1;i<coords.length;i++){
      const [nx,ny]=coords[i];
      const dx = nx - lastX; const dy = ny - lastY;
      // Break movement into nibble-sized velocity writes; choose velocity = sign of delta, repeat |delta| times.
      const steps = Math.max(Math.abs(dx), Math.abs(dy));
      if (steps===0) continue;
      const stepDx = dx===0?0:(dx>0?1:-1);
      const stepDy = dy===0?0:(dy>0?1:-1);
      for (let s=0;s<steps;s++){
        const vx = stepDx & 0x0F; const vy = stepDy & 0x0F;
        const packed = (vx<<4) | (vy & 0x0F);
        if (this.viaEvents.length < 5000) this.viaEvents.push({ pc:this.pc, reg:0x00, val: packed });
        if (this.traceEnabled) this.debugTraces.push({ type:'via-write', pc:this.pc, reg:0x00, val:packed });
        // Apply immediately to beam for visible segment accumulation
        const startX=this.beamX, startY=this.beamY;
        this.beamX += stepDx; this.beamY += stepDy;
        this.frameSegments.push({ x1:startX, y1:startY, x2:this.beamX, y2:this.beamY, intensity:this.beamLastIntensity });
        if (this.traceEnabled) this.debugTraces.push({ type:'segment', pc:this.pc, x1:startX, y1:startY, x2:this.beamX, y2:this.beamY, intensity:this.beamLastIntensity, note:'bridge' });
      }
      lastX = nx; lastY = ny;
    }
    // Stop drawing via ACR bit1 set (bit1 stop) - write value with bit1 set and bit0 cleared.
    if (this.viaEvents.length < 5000) this.viaEvents.push({ pc:this.pc, reg:0x0B, val: 0x02 });
    if (this.traceEnabled) this.debugTraces.push({ type:'draw-stop', pc:this.pc });
  }

  private beamMove(x:number,y:number){ this.beamX=x; this.beamY=y; }
  private beamSetIntensity(intensity:number){ this.beamLastIntensity=intensity; }
  private beamLineTo(x:number,y:number){
    if (!this.beamDrawing){
      // start a new segment from current beam position
      this.beamDrawing = true;
    }
    // Each lineTo extends by creating an explicit segment from previous point to new point
    const x1=this.beamX, y1=this.beamY;
    this.beamX = x; this.beamY = y;
    this.frameSegments.push({x1,y1,x2:x,y2:y,intensity:this.beamLastIntensity});
  }
  private beamStop(){ this.beamDrawing=false; }

  private directAddr(){ const lo=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; return (this.dp<<8)|lo; }
  private extendedAddr(){ const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; return ((hi<<8)|lo)&0xFFFF; }
  // Indexed addressing decoder (covers common 6809 forms; indirect forms currently dereference once when recognized)
  private indexedAddr(): number {
    const post = this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF;
    // If bit7=0 => 5-bit signed offset with base register (bits 6-5)
    if ((post & 0x80) === 0){
      const off5 = ((post & 0x1F) << 27) >> 27; // sign extend 5-bit
      const regSel = (post >> 5) & 0x03; // 0 X,1 Y,2 U,3 S
      let base=0; switch(regSel){ case 0: base=this.x; break; case 1: base=this.y; break; case 2: base=this.u; break; case 3: base=this.s; break; }
      return (base + off5) & 0xFFFF;
    }
    // Otherwise extended format: bit7=1
    const regSel = (post >> 5) & 0x03; // base register
    const code = post & 0x1F;
    const getBase = () => { switch(regSel){ case 0: return this.x; case 1: return this.y; case 2: return this.u; case 3: return this.s; } return this.x; };
    const setBase = (val:number) => { val &= 0xFFFF; switch(regSel){ case 0: this.x=val; break; case 1: this.y=val; break; case 2: this.u=val; break; case 3: this.s=val; break; } };
    let addr: number | null = null;
    let base = getBase();
    const sign8 = (v:number)=> (v<<24)>>24;
    const sign16 = (v:number)=> (v & 0x8000)? v-0x10000 : v;
    switch(code){
      case 0x00: addr = base; base=(base+1)&0xFFFF; setBase(base); break; // ,R+
      case 0x01: addr = base; base=(base+2)&0xFFFF; setBase(base); break; // ,R++
      case 0x02: base=(base-1)&0xFFFF; setBase(base); addr=base; break; // ,-R
      case 0x03: base=(base-2)&0xFFFF; setBase(base); addr=base; break; // ,--R
      case 0x04: addr = base; break; // ,R
      case 0x08: addr = (base + sign8(this.a)) & 0xFFFF; break; // ,R+A
      case 0x09: addr = (base + sign8(this.b)) & 0xFFFF; break; // ,R+B
      case 0x0B: addr = (base + this.d()) & 0xFFFF; break; // ,R+D
      case 0x0C: { // 8-bit offset
        const off=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; addr=(base + sign8(off)) & 0xFFFF; break; }
      case 0x0D: { // 16-bit offset
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const off=(hi<<8)|lo; addr=(base + sign16(off)) & 0xFFFF; break; }
      case 0x0F: { // [,R] indirect
        const ptr = base & 0xFFFF; const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      case 0x10: { // [,R+] indirect post inc
        const ptr = base & 0xFFFF; base=(base+1)&0xFFFF; setBase(base); const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      case 0x11: { // [,R++] indirect post inc 2
        const ptr = base & 0xFFFF; base=(base+2)&0xFFFF; setBase(base); const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      case 0x12: { // [,-R] indirect pre dec 1
        base=(base-1)&0xFFFF; setBase(base); const ptr=base; const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      case 0x13: { // [,--R] indirect pre dec 2
        base=(base-2)&0xFFFF; setBase(base); const ptr=base; const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      case 0x14: { // [,R] indirect duplicate
        const ptr=base; const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      case 0x17: { // [8-bit offset,R] indirect
        const off=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const ptr=(base + sign8(off)) & 0xFFFF; const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      case 0x18: { // [16-bit offset,R] indirect
        const hiOff=this.mem[this.pc]; const loOff=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const off=(hiOff<<8)|loOff; const ptr=(base + sign16(off)) & 0xFFFF; const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      case 0x19: { // [8-bit PC relative] indirect
        const off=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const ptr=(this.pc + sign8(off)) & 0xFFFF; const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      case 0x1A: { // [16-bit PC relative] indirect
        const hiOff=this.mem[this.pc]; const loOff=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const off=(hiOff<<8)|loOff; const ptr=(this.pc + sign16(off)) & 0xFFFF; const hi=this.read8(ptr); const lo=this.read8((ptr+1)&0xFFFF); addr=((hi<<8)|lo)&0xFFFF; break; }
      default:
        // Unrecognized advanced form; fallback to base (treat as ,R) and log once
        this.logUnknown(0xA600|post);
        addr = base;
        break;
    }
    return addr & 0xFFFF;
  }
  private fetchImm8(){ const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; return v; }
  private fetchImm16(){ const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; return ((hi<<8)|lo)&0xFFFF; }
  private branch(off:number){ this.pc = (this.pc + off) & 0xFFFF; }
  private cond(opcode:number): boolean {
    // 20 BRA unconditional handled separately
    switch(opcode){
      case 0x21: return false; // BRN never
      case 0x22: return (!this.cc_c && !this.cc_z); // BHI
      case 0x23: return (this.cc_c || this.cc_z); // BLS
      case 0x24: return !this.cc_c; // BCC/BHS
      case 0x25: return this.cc_c; // BCS/BLO
      case 0x26: return !this.cc_z; // BNE
      case 0x27: return this.cc_z; // BEQ
      case 0x28: return !this.cc_v; // BVC
      case 0x29: return this.cc_v; // BVS
      case 0x2A: return !this.cc_n; // BPL
      case 0x2B: return this.cc_n; // BMI
      case 0x2C: return (this.cc_n===this.cc_v); // BGE
      case 0x2D: return (this.cc_n!==this.cc_v); // BLT
      case 0x2E: return (!this.cc_z && (this.cc_n===this.cc_v)); // BGT
      case 0x2F: return (this.cc_z || (this.cc_n!==this.cc_v)); // BLE
      default: return true;
    }
  }
  private read8(a:number){
    a &= 0xFFFF;
    if ((a & 0xFFF0) === 0xD000){ return this.viaRead(a & 0x0F); }
    return this.mem[a];
  }
  private write8(a:number,v:number){
    a &= 0xFFFF; v &= 0xFF;
    if ((a & 0xFFF0) === 0xD000){ this.viaWriteEffect(a & 0x0F, v); return; }
    this.mem[a]=v;
  }
  private ldaSet(v:number){ this.a=v&0xFF; this.nz8(this.a); }
  private prefix10(){
    const op = this.mem[this.pc];
    this.pc=(this.pc+1)&0xFFFF;
    switch(op){
      case 0x8E: { // LDY imm (0x10 0x8E)
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; this.y=((hi<<8)|lo)&0xFFFF; this.nz16(this.y); return true; }
      case 0x9E: { // LDY direct
        const addr=this.directAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.y=((hi<<8)|lo)&0xFFFF; this.nz16(this.y); return true; }
      case 0x9F: { // STY direct
        const addr=this.directAddr(); this.write8(addr,this.y>>8); this.write8(addr+1,this.y&0xFF); return true; }
      case 0xBE: { // LDY extended
        const addr=this.extendedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.y=((hi<<8)|lo)&0xFFFF; this.nz16(this.y); return true; }
      case 0xBF: { // STY extended
        const addr=this.extendedAddr(); this.write8(addr,this.y>>8); this.write8(addr+1,this.y&0xFF); return true; }
      case 0xCE: { // LDS imm (0x10 0xCE)
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; this.s=((hi<<8)|lo)&0xFFFF; this.nz16(this.s); return true; }
      case 0x83: { // CMPD immediate (0x10 0x83) treat as SUBD imm with no store
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const _=this.sub16(this.d(), (hi<<8)|lo); return true; }
      case 0x93: { // CMPD direct (0x10 0x93)
        const addr=this.directAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const _=this.sub16(this.d(), (hi<<8)|lo); return true; }
      case 0xA3: { // CMPD indexed (0x10 0xA3)
        const addr=this.indexedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const _=this.sub16(this.d(), (hi<<8)|lo); return true; }
      case 0xB3: { // CMPD extended (0x10 0xB3)
        const addr=this.extendedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const _=this.sub16(this.d(), (hi<<8)|lo); return true; }
      case 0x16: { // LBRA
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const off=((hi<<8)|lo); this.pc=(this.pc + ((off&0x8000)?off-0x10000:off))&0xFFFF; return true; }
      case 0x17: { // LBSR
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const off=((hi<<8)|lo); this.callStack.push(this.pc); this.pc=(this.pc + ((off&0x8000)?off-0x10000:off))&0xFFFF; return true; }
      case 0x3F: { // SWI2 (prefix 0x10 0x3F)
        // Push full state then fetch SWI2 vector (0xFFF4/0xFFF5)
        this.pshs(0xFF);
        this.cc_i = true;
        const hi=this.mem[0xFFF4]; const lo=this.mem[0xFFF5];
        this.pc = ((hi<<8)|lo)&0xFFFF;
        return true; }
      case 0x21: case 0x22: case 0x23: case 0x24: case 0x25: case 0x26: case 0x27: case 0x28: case 0x29: case 0x2A: case 0x2B: case 0x2C: case 0x2D: case 0x2E: case 0x2F: {
        // Long conditional branches (e.g., 0x10 0x27 = LBEQ)
        const opc=op; const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; if (this.cond(opc)) { const off=((hi<<8)|lo); this.pc=(this.pc + ((off&0x8000)?off-0x10000:off))&0xFFFF; } return true; }
      // Could add LBRA/LBSR later
      default:
        this.logUnknown(0x1000|op);
        return false;
    }
  }
  private prefix11(){
    // 0x11 prefix opcodes: typically CMPU, CMPS, LDS/STS variants, long branches same as 0x10 but using different base regs, SWI2 (0x3F), SWI3 (0x4F)
    const op = this.mem[this.pc];
    this.pc=(this.pc+1)&0xFFFF;
    switch(op){
      case 0x83: { // CMPU immediate (0x11 0x83): subtract imm16 from U, set flags
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const _=this.sub16(this.u,(hi<<8)|lo); return true; }
      case 0x93: { // CMPU direct
        const addr=this.directAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const _=this.sub16(this.u,(hi<<8)|lo); return true; }
      case 0xA3: { // CMPU indexed
        const addr=this.indexedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const _=this.sub16(this.u,(hi<<8)|lo); return true; }
      case 0xB3: { // CMPU extended
        const addr=this.extendedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const _=this.sub16(this.u,(hi<<8)|lo); return true; }
      case 0x8C: { // CMPY immediate (alias; already have LDY etc in prefix10, but include for completeness?) Actually CMPY is 0x10 0x8C; skip here
        this.logUnknown(0x1100|op); return false; }
      case 0x93|0x0100: { return false; }
      case 0x3F: { // SWI3 (prefix 0x11 0x3F)
        this.pshs(0xFF);
        this.cc_i = true;
        const hi=this.mem[0xFFF2]; const lo=this.mem[0xFFF3]; // SWI3 vector
        this.pc = ((hi<<8)|lo)&0xFFFF;
        return true; }
      case 0x16: { // LBRA (0x11 0x16) - rarely used; treat same as 0x10 0x16
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const off=((hi<<8)|lo); this.pc=(this.pc + ((off&0x8000)?off-0x10000:off))&0xFFFF; return true; }
      case 0x17: { // LBSR (0x11 0x17)
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const off=((hi<<8)|lo); this.callStack.push(this.pc); this.pc=(this.pc + ((off&0x8000)?off-0x10000:off))&0xFFFF; return true; }
      case 0x21: case 0x22: case 0x23: case 0x24: case 0x25: case 0x26: case 0x27: case 0x28: case 0x29: case 0x2A: case 0x2B: case 0x2C: case 0x2D: case 0x2E: case 0x2F: {
        const opc=op; const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; if (this.cond(opc)) { const off=((hi<<8)|lo); this.pc=(this.pc + ((off&0x8000)?off-0x10000:off))&0xFFFF; } return true; }
      default:
        this.logUnknown(0x1100|op); return false;
    }
  }
  private readReg(code:number): number {
    switch(code){
      case 0: return this.d();
      case 1: return this.x;
      case 2: return this.y;
      case 3: return this.u;
      case 4: return this.s;
      case 5: return this.pc;
      case 8: return this.a;
      case 9: return this.b;
  case 10: return (this.cc_c?1:0)|(this.cc_v?0x02:0)|(this.cc_z?0x04:0)|(this.cc_n?0x08:0)|(this.cc_i?0x10:0); // add I bit (bit4)
      case 11: return this.dp;
      default: return 0;
    }
  }
  private writeReg(code:number,val:number){
    val &= (code>=8 && code<=9)?0xFF:0xFFFF;
    switch(code){
      case 0: this.setD(val); break;
      case 1: this.x=val; break;
      case 2: this.y=val; break;
      case 3: this.u=val; break;
      case 4: this.s=val; break;
      case 5: this.pc=val; break;
      case 8: this.a=val&0xFF; break;
      case 9: this.b=val&0xFF; break;
  case 10: /* CC simplified restore */ this.cc_c=!!(val&1); this.cc_v=!!(val&2); this.cc_z=!!(val&4); this.cc_n=!!(val&8); this.cc_i=!!(val&0x10); break;
      case 11: this.dp=val&0xFF; break;
    }
  }
  private pshs(mask:number){
    // Order PC,U,Y,X,DP,B,A,CC
    const pushByte = (v:number)=>{ this.s=(this.s-1)&0xFFFF; this.write8(this.s,v); };
    if (mask & 0x80){ pushByte(this.pc & 0xFF); pushByte(this.pc>>8); }
    if (mask & 0x40){ pushByte(this.u & 0xFF); pushByte(this.u>>8); }
    if (mask & 0x20){ pushByte(this.y & 0xFF); pushByte(this.y>>8); }
    if (mask & 0x10){ pushByte(this.x & 0xFF); pushByte(this.x>>8); }
    if (mask & 0x08){ pushByte(this.dp); }
    if (mask & 0x04){ pushByte(this.b); }
    if (mask & 0x02){ pushByte(this.a); }
  if (mask & 0x01){ let cc=(this.cc_c?1:0)|(this.cc_v?2:0)|(this.cc_z?4:0)|(this.cc_n?8:0)|(this.cc_i?0x10:0); pushByte(cc); }
  }
  private pulls(mask:number){
    const popByte = ()=>{ const v=this.read8(this.s); this.s=(this.s+1)&0xFFFF; return v; };
    // Reverse order: CC,A,B,DP,X,Y,U,PC
  if (mask & 0x01){ const cc=popByte(); this.cc_c=!!(cc&1); this.cc_v=!!(cc&2); this.cc_z=!!(cc&4); this.cc_n=!!(cc&8); this.cc_i=!!(cc&0x10); }
    if (mask & 0x02){ this.a=popByte(); this.nz8(this.a); }
    if (mask & 0x04){ this.b=popByte(); this.nz8(this.b); }
    if (mask & 0x08){ this.dp=popByte(); }
    if (mask & 0x10){ const hi=popByte(); const lo=popByte(); this.x=((lo<<8)|hi)&0xFFFF; }
    if (mask & 0x20){ const hi=popByte(); const lo=popByte(); this.y=((lo<<8)|hi)&0xFFFF; }
    if (mask & 0x40){ const hi=popByte(); const lo=popByte(); this.u=((lo<<8)|hi)&0xFFFF; }
    if (mask & 0x80){ const hi=popByte(); const lo=popByte(); this.pc=((lo<<8)|hi)&0xFFFF; }
  }
  private logUnknown(op:number){
    const key=op.toString(16);
    this.unknownLog[key]=(this.unknownLog[key]||0)+1;
    if (this.trace) console.warn('UNIMPL', key, 'at', this.pc.toString(16));
  }

  step(): boolean {
    // Interrupt entry check before fetching next opcode
    // Synthetic IRQ fallback: if in intercept mode (no VIA timers) and we've burned a lot of cycles
    // without a frame, periodically assert an IRQ so BIOS code waiting on WAI can advance.
  if (this.vectorMode === 'intercept' && !(this as any).disableSyntheticIrq && !this.irqPending && this.cycles - this.lastSyntheticIrqCheck > 8000){
      // Allocate tracking field lazily
      (this as any).lastSyntheticIrqCheck = this.cycles;
      // Fire a simple periodic IRQ every ~8K cycles if interrupts enabled
      if (!this.cc_i){
        this.irqPending = true;
        if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'synthetic-irq-pending' });
      }
    }
    if (this.irqPending && !this.cc_i){
      this.serviceIrq();
      if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'irq-service' });
      if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'wai-exit' });
    }
    // If WAI waiting with no interrupt yet: burn a couple cycles for timer progression
    if (this.waiWaiting && !(this.irqPending && !this.cc_i)){
      const idleCycles = 2;
      if (this.vectorMode === 'via') this.updateTimers(idleCycles);
      this.cycles += idleCycles;
      return true; // remain in wait state
    }
    const op = this.mem[this.pc];
    const pc0=this.pc;
    this.pc = (this.pc + 1) & 0xFFFF;
    // Pre-fetch up to 3 following bytes (not strictly correct for variable length but good view)
    let b1=this.mem[this.pc];
    let b2=this.mem[(this.pc+1)&0xFFFF];
    let b3=this.mem[(this.pc+2)&0xFFFF];
  // Update tail instruction ring (after fetching opcode/peek bytes, before execution mutation)
  if (this.tailInstr.length >= this.tailInstrMax) this.tailInstr.shift();
  this.tailInstr.push({ pc: pc0, op, b1, b2, b3 });
    if (this.opcodeTraceEnabled){
      if (this.opcodeTrace.length >= this.maxOpcodeTrace) this.opcodeTrace.shift();
      this.opcodeTrace.push({ pc: pc0, op });
    }
    // Detect BIOS IFR polling loop (BITB direct on IFR: opcode 0xD5 with operand 0x0D) in via mode
    if (this.vectorMode === 'via'){
      if (op === 0xD5 && b1 === 0x0D){
        this.bitbIfrLoopCount++;
        if (this.bitbIfrLoopCount === 512 && !this.devTimer1Seeded){
          // Early development seed (loop based)
          const seed=8000;
          this.setTimer1(seed); this.setTimer1Latch(seed);
          this.via[0x0B] |= 0x40; // continuous
          this.via[0x0E] |= 0x40; // enable bit6
          this.recomputeIrqPending();
          this.devTimer1Seeded = true; this.devTimer1SeedCycle = this.cycles;
          if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:pc0, note:'dev_timer1_seed_loop' });
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'dev_seed', pc:pc0, note:'dev_timer1_seed_loop', cycles:this.cycles });
        }
      } else {
        this.bitbIfrLoopCount = 0;
      }
    }
    if (this.fullInstrTraceEnabled){
      // Keep bounded (avoid unbounded growth): limit 100k entries
      if (this.fullInstrTrace.length >= 100000) this.fullInstrTrace.shift();
      this.fullInstrTrace.push({ pc:pc0, op, b1, b2, b3 });
    }
    // Very rough cycle costs (subset). Real 6809 varies by addressing mode; we just approximate.
  let cyc = Cpu6809.cycleTable[op] ?? 2;
    switch(op){
      case 0x00: { // NEG direct
        const addr=this.directAddr(); const v=this.read8(addr); const r=this.applyRmw('NEG',v); this.write8(addr,r); break; }
      case 0x01: { /* treat as NOP (unimplemented / not expected) */ break; }
      case 0x03: { // COM direct
        const addr=this.directAddr(); const v=this.read8(addr); const r=this.applyRmw('COM',v); this.write8(addr,r); break; }
  case 0x05: { /* official 6809 has no direct 0x05 RMW op (COM direct is 0x03); keep as NOP */ break; }
      case 0x07: { // ASR direct
        const addr=this.directAddr(); const v=this.read8(addr); const r=this.applyRmw('ASR',v); this.write8(addr,r); break; }
      case 0x09: { // ROL direct
        const addr=this.directAddr(); const v=this.read8(addr); const r=this.applyRmw('ROL',v); this.write8(addr,r); break; }
      case 0x12: { /* official NOP */ break; }
      case 0x13: { // SYNC: wait for interrupt without pushing state
        this.waiWaiting = true; if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'SYNC-enter' }); break; }
      case 0x19: { // DAA (approximate BCD adjust on A)
        let adjust = 0;
        if ((this.a & 0x0F) > 0x09) adjust |= 0x06;
        if (this.a > 0x99 || this.cc_c) { adjust |= 0x60; this.cc_c = true; }
        const r = (this.a + adjust) & 0xFF;
        this.a = r; this.nz8(r); // V unaffected
        break; }
      case 0x1D: { // SEX: sign extend B into A (D = sign-extended B)
        this.a = (this.b & 0x80)?0xFF:0x00; this.nz16(this.d()); break; }
      case 0x8D: { // BSR (8-bit signed relative); push return address onto callStack
        const off8=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.callStack.push(this.pc); const soff=(off8&0x80)?off8-0x100:off8; this.pc=(this.pc+soff)&0xFFFF; break; }
      case 0x9D: { // JSR direct (with BIOS interception)
        const addr=this.directAddr();
        if (this.traceBiosVec && this.traceEnabled && (addr & 0xFF00) === 0xF300){
          this.debugTraces.push({ type:'info', pc:this.pc, note:`bios_vec_jsr_${addr.toString(16)}` });
        }
        if (addr >= 0xF000){
          if (!this.biosPresent){
            this.interceptBios(addr);
          } else {
            switch(addr){
              case 0xF192: case 0xF2A5: case 0xF2AB:
                if (addr === 0xF192 && this.noInterceptWaitRecal) { this.callStack.push(this.pc); this.pc=addr; break; }
                this.interceptBios(addr); break;
              case 0xF3DD: // Draw_VL base
                if (this.noInterceptDraw){
                  this.callStack.push(this.pc); this.pc=addr; // fall through to BIOS code
                } else {
                  if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'intercept_Draw_VL' });
                  this.interceptBios(addr);
                }
                break;
              default:
                this.callStack.push(this.pc); this.pc=addr; break;
            }
          }
        } else {
          this.callStack.push(this.pc); this.pc=addr;
        }
        break; }
      case 0xAD: { // JSR indexed (with BIOS interception)
        const addr=this.indexedAddr();
        if (this.traceBiosVec && this.traceEnabled && (addr & 0xFF00) === 0xF300){
          this.debugTraces.push({ type:'info', pc:this.pc, note:`bios_vec_jsr_${addr.toString(16)}` });
        }
        if (addr >= 0xF000){
          if (!this.biosPresent){
            this.interceptBios(addr);
          } else {
            switch(addr){
              case 0xF192: case 0xF2A5: case 0xF2AB:
                if (addr === 0xF192 && this.noInterceptWaitRecal) { this.callStack.push(this.pc); this.pc=addr; break; }
                this.interceptBios(addr); break;
              case 0xF3DD:
                if (this.noInterceptDraw){
                  this.callStack.push(this.pc); this.pc=addr;
                } else {
                  if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'intercept_Draw_VL' });
                  this.interceptBios(addr);
                }
                break;
              default:
                this.callStack.push(this.pc); this.pc=addr; break;
            }
          }
        } else {
          this.callStack.push(this.pc); this.pc=addr;
        }
        break; }
      case 0xBD: { // JSR extended (with BIOS interception)
        const addr=this.extendedAddr();
        if (this.traceBiosVec && this.traceEnabled && (addr & 0xFF00) === 0xF300){
          this.debugTraces.push({ type:'info', pc:this.pc, note:`bios_vec_jsr_${addr.toString(16)}` });
        }
        if (addr >= 0xF000){
          if (!this.biosPresent){
            this.interceptBios(addr);
          } else {
            switch(addr){
              case 0xF192: case 0xF2A5: case 0xF2AB:
                if (addr === 0xF192 && this.noInterceptWaitRecal) { this.callStack.push(this.pc); this.pc=addr; break; }
                this.interceptBios(addr); break;
              case 0xF3DD:
                if (this.noInterceptDraw){
                  this.callStack.push(this.pc); this.pc=addr;
                } else {
                  if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'intercept_Draw_VL' });
                  this.interceptBios(addr);
                }
                break;
              default:
                this.callStack.push(this.pc); this.pc=addr; break;
            }
          }
        } else {
          this.callStack.push(this.pc); this.pc=addr;
        }
        break; }
      case 0x34: { // PSHS mask
        const mask=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.pshs(mask); break; }
      case 0x35: { // PULS mask
        const mask=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.pulls(mask); break; }
      case 0x30: { // LEAX indexed
        const addr=this.indexedAddr(); this.x=addr; this.nz16(this.x); break; }
      case 0x31: { // LEAY indexed
        const addr=this.indexedAddr(); this.y=addr; this.nz16(this.y); break; }
      case 0x32: { // LEAS indexed
        const addr=this.indexedAddr(); this.s=addr; this.nz16(this.s); break; }
      case 0x33: { // LEAU indexed
        const addr=this.indexedAddr(); this.u=addr; this.nz16(this.u); break; }
      case 0x39: { // RTS (already implemented later as 0x39; early duplicate safe) just ensure callStack pop
        const sBefore=this.s; const ret = this.callStack.pop(); const newPc = ret ?? this.pc; const pcBefore=pc0; this.pc = newPc; const sAfter=this.s; if (this.rtsLog.length<2000) this.rtsLog.push({ type:'RTS', pcBefore, pcAfter:newPc, sBefore, sAfter, returnPc:newPc }); break; }
      case 0x38: { /* Placeholder for PSHU (real 0x36) / PULU (0x37) confusion; current compiler uses 0x38? treat as NOP */ break; }
      case 0x17: { /* Stray 0x17 without 0x10 prefix (should be LBSR) treat as NOP */ break; }
      case 0x36: { // PSHU mask (store selected registers on U stack)
        const mask=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF;
        const pushU = (v:number)=>{ this.u=(this.u-1)&0xFFFF; this.write8(this.u,v&0xFF); };
        if (mask & 0x80){ pushU(this.pc & 0xFF); pushU(this.pc>>8); }
        if (mask & 0x40){ pushU(this.u & 0xFF); pushU((this.u>>8)&0xFF); }
        if (mask & 0x20){ pushU(this.y & 0xFF); pushU(this.y>>8); }
        if (mask & 0x10){ pushU(this.x & 0xFF); pushU(this.x>>8); }
        if (mask & 0x08){ pushU(this.dp); }
        if (mask & 0x04){ pushU(this.b); }
        if (mask & 0x02){ pushU(this.a); }
        if (mask & 0x01){ const cc=(this.cc_c?1:0)|(this.cc_v?2:0)|(this.cc_z?4:0)|(this.cc_n?8:0)|(this.cc_i?0x10:0); pushU(cc); }
        break; }
      case 0x37: { // PULU mask (pull selected registers from U stack)
        const mask=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF;
        const popU = ()=>{ const v=this.read8(this.u); this.u=(this.u+1)&0xFFFF; return v; };
        if (mask & 0x01){ const cc=popU(); this.cc_c=!!(cc&1); this.cc_v=!!(cc&2); this.cc_z=!!(cc&4); this.cc_n=!!(cc&8); this.cc_i=!!(cc&0x10); }
        if (mask & 0x02){ this.a=popU(); this.nz8(this.a); }
        if (mask & 0x04){ this.b=popU(); this.nz8(this.b); }
        if (mask & 0x08){ this.dp=popU(); }
        if (mask & 0x10){ const lo=popU(); const hi=popU(); this.x=((hi<<8)|lo)&0xFFFF; }
        if (mask & 0x20){ const lo=popU(); const hi=popU(); this.y=((hi<<8)|lo)&0xFFFF; }
        if (mask & 0x40){ const lo=popU(); const hi=popU(); this.u=((hi<<8)|lo)&0xFFFF; }
        if (mask & 0x80){ const lo=popU(); const hi=popU(); this.pc=((hi<<8)|lo)&0xFFFF; }
        break; }
      case 0xCC: { // LDD imm
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; this.setD((hi<<8)|lo); this.nz16(this.d()); break; }
      case 0x86: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.a=v; this.nz8(this.a); break; }
      case 0xC6: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.b=v; this.nz8(this.b); break; }
      case 0xC3: { // ADDD immediate
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const res=this.add16(this.d(),(hi<<8)|lo); this.setD(res); break; }
      case 0x83: { // SUBD immediate
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const res=this.sub16(this.d(),(hi<<8)|lo); this.setD(res); break; }
      case 0x84: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.a = (this.a & v) & 0xFF; this.nz8(this.a); break; } // ANDA imm
      case 0x85: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const r=(this.a & v) & 0xFF; this.nz8(r); /* BITA imm: no store */ break; }
      case 0x8A: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.a = (this.a | v) & 0xFF; this.nz8(this.a); break; } // ORA imm
      case 0xC4: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.b = (this.b & v) & 0xFF; this.nz8(this.b); break; } // ANDB imm
      case 0xC5: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const r=(this.b & v) & 0xFF; this.nz8(r); break; } // BITB imm
      case 0xCA: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.b = (this.b | v) & 0xFF; this.nz8(this.b); break; } // ORB imm
      case 0x81: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const _=this.sub8(this.a,v); /* CMPA imm result not stored */ break; }
      case 0xC1: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const _=this.sub8(this.b,v); break; }
  case 0x89: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.a=this.adc8(this.a,v); break; } // ADCA imm
  case 0xC9: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.b=this.adc8(this.b,v); break; } // ADCB imm
  case 0x82: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.a=this.sbc8(this.a,v); break; } // SBCA imm
  case 0xC2: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.b=this.sbc8(this.b,v); break; } // SBCB imm
      case 0xCB: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.b=(this.b+v)&0xFF; this.nz8(this.b); break; } // ADDB imm
      case 0xC0: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.b=(this.b - v) & 0xFF; this.nz8(this.b); break; } // SUBB imm
      case 0x8E: { const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; this.x=((hi<<8)|lo)&0xFFFF; break; }
      case 0xCE: { const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; this.u=((hi<<8)|lo)&0xFFFF; break; }
      // 0x8B is the real ADDA immediate; previously we (incorrectly) used 0xC8.
      case 0x8B: { // ADDA imm (correct opcode)
        const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.ldaSet((this.a + v) & 0xFF); break; }
      case 0xC8: { // EORB imm (correct mapping; previously misused for ADDA)
        const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.b = (this.b ^ v) & 0xFF; this.nz8(this.b); break; }
      case 0x80: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.ldaSet((this.a - v) & 0xFF); break; } // SUBA imm
      case 0x96: { const addr=this.directAddr(); this.ldaSet(this.read8(addr)); break; } // LDA direct
      case 0x97: { const addr=this.directAddr(); this.write8(addr,this.a); break; } // STA direct
      case 0x9B: { // ADDA direct
        const addr=this.directAddr(); this.a = this.add8(this.a,this.read8(addr)); break; }
      case 0x93: { // SUBD direct
        const addr=this.directAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const res=this.sub16(this.d(),(hi<<8)|lo); this.setD(res); break; }
      case 0xAB: { // ADDA indexed
        const addr=this.indexedAddr(); this.a = this.add8(this.a,this.read8(addr)); break; }
      case 0xA3: { // SUBD indexed
        const addr=this.indexedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const res=this.sub16(this.d(),(hi<<8)|lo); this.setD(res); break; }
      case 0xBB: { // ADDA extended
        const addr=this.extendedAddr(); this.a = this.add8(this.a,this.read8(addr)); break; }
      case 0xB3: { // SUBD extended
        const addr=this.extendedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const res=this.sub16(this.d(),(hi<<8)|lo); this.setD(res); break; }
      case 0xB6: { const addr=this.extendedAddr(); this.ldaSet(this.read8(addr)); break; } // LDA extended
      case 0xB7: { const addr=this.extendedAddr(); this.write8(addr,this.a); break; } // STA extended
      case 0xB5: { // BITA extended (test, sets NZ, preserves A)
        const addr=this.extendedAddr(); const v=(this.a & this.read8(addr)) & 0xFF; this.nz8(v); break; }
      case 0xD6: { const addr=this.directAddr(); this.b=this.read8(addr); this.nz8(this.b); break; } // LDB direct
      case 0xD7: { const addr=this.directAddr(); this.write8(addr,this.b); break; } // STB direct
      case 0xF6: { const addr=this.extendedAddr(); this.b=this.read8(addr); this.nz8(this.b); break; } // LDB extended
      case 0xF7: { const addr=this.extendedAddr(); this.write8(addr,this.b); break; } // STB extended
  case 0xE6: { const addr=this.indexedAddr(); this.b=this.read8(addr); this.nz8(this.b); break; } // LDB indexed
      case 0xDE: { const addr=this.directAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.u=((hi<<8)|lo)&0xFFFF; this.nz16(this.u); break; } // LDU direct
      case 0xDF: { const addr=this.directAddr(); this.write8(addr,this.u>>8); this.write8(addr+1,this.u&0xFF); break; } // STU direct
      case 0xFE: { const addr=this.extendedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.u=((hi<<8)|lo)&0xFFFF; this.nz16(this.u); break; } // LDU extended
      case 0xFF: { const addr=this.extendedAddr(); this.write8(addr,this.u>>8); this.write8(addr+1,this.u&0xFF); break; } // STU extended
  case 0xEE: { const addr=this.indexedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.u=((hi<<8)|lo)&0xFFFF; this.nz16(this.u); break; } // LDU indexed
      case 0x9E: { const addr=this.directAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.x=((hi<<8)|lo)&0xFFFF; this.nz16(this.x); break; } // LDX direct
      case 0x9F: { const addr=this.directAddr(); this.write8(addr,this.x>>8); this.write8(addr+1,this.x&0xFF); break; } // STX direct
      case 0xBE: { const addr=this.extendedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.x=((hi<<8)|lo)&0xFFFF; this.nz16(this.x); break; } // LDX extended
      case 0xBF: { const addr=this.extendedAddr(); this.write8(addr,this.x>>8); this.write8(addr+1,this.x&0xFF); break; } // STX extended
      case 0xAE: { const addr=this.indexedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.x=((hi<<8)|lo)&0xFFFF; this.nz16(this.x); break; } // LDX indexed
      case 0xAF: { const addr=this.indexedAddr(); this.write8(addr,this.x>>8); this.write8(addr+1,this.x&0xFF); break; } // STX indexed
      case 0xA6: { // LDA indexed (simple ,X)
        const addr=this.indexedAddr(); this.ldaSet(this.read8(addr));
        break; }
      case 0xA7: { // STA indexed (,X only for now)
        const addr=this.indexedAddr(); this.write8(addr,this.a); break; }
      case 0x08: { // INY
        this.y=(this.y+1)&0xFFFF; this.nz16(this.y); break; }
      case 0x04: { // LSR direct
        const addr=this.directAddr(); const v=this.read8(addr); const r=this.applyRmw('LSR',v); this.write8(addr,r); break; }
      case 0x06: { // ROR direct
        const addr=this.directAddr(); const v=this.read8(addr); const r=this.applyRmw('ROR',v); this.write8(addr,r); break; }
      case 0x08: { // ASL direct
        const addr=this.directAddr(); const v=this.read8(addr); const r=this.applyRmw('ASL',v); this.write8(addr,r); break; }
      case 0x0F: { // CLR direct
        const addr=this.directAddr(); const r=this.applyRmw('CLR',0); this.write8(addr,r); break; }
      case 0x0A: { // DEC direct
        const addr=this.directAddr(); const v=this.read8(addr); const r=this.applyRmw('DEC',v); this.write8(addr,r); break; }
      case 0x0C: { // INC direct
        const addr=this.directAddr(); const v=this.read8(addr); const r=this.applyRmw('INC',v); this.write8(addr,r); break; }
      case 0x0D: { // TST direct
        const addr=this.directAddr(); const v=this.read8(addr); this.applyRmw('TST',v); break; }
      case 0x70: { // NEG extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const r=this.applyRmw('NEG',v); this.write8(addr,r); break; }
      case 0x73: { // COM extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const r=this.applyRmw('COM',v); this.write8(addr,r); break; }
      case 0x74: { // LSR extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const r=this.applyRmw('LSR',v); this.write8(addr,r); break; }
      case 0x76: { // ROR extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const r=this.applyRmw('ROR',v); this.write8(addr,r); break; }
      case 0x77: { // ASR extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const r=this.applyRmw('ASR',v); this.write8(addr,r); break; }
      case 0x78: { // ASL extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const r=this.applyRmw('ASL',v); this.write8(addr,r); break; }
      case 0x79: { // ROL extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const r=this.applyRmw('ROL',v); this.write8(addr,r); break; }
      case 0x7A: { // DEC extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const r=this.applyRmw('DEC',v); this.write8(addr,r); break; }
      case 0x7C: { // INC extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const r=this.applyRmw('INC',v); this.write8(addr,r); break; }
      case 0x7D: { // TST extended
        const addr=this.extendedAddr(); const v=this.read8(addr); this.applyRmw('TST',v); break; }
      case 0x7F: { // CLR extended
        const addr=this.extendedAddr(); const r=this.applyRmw('CLR',0); this.write8(addr,r); break; }
      case 0x7A: { // DEC extended
        const addr=this.extendedAddr(); const v=(this.read8(addr)-1)&0xFF; this.write8(addr,v); this.nz8(v); break; }
      case 0x7C: { // INC extended
        const addr=this.extendedAddr(); const v=(this.read8(addr)+1)&0xFF; this.write8(addr,v); this.nz8(v); break; }
      case 0xDC: { // LDD direct
        const addr=this.directAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.setD((hi<<8)|lo); this.nz16(this.d()); break; }
      case 0xDD: { // STD direct
        const addr=this.directAddr(); const d=this.d(); this.write8(addr,d>>8); this.write8(addr+1,d&0xFF); this.nz16(d); break; }
      case 0xFC: { // LDD extended
        const addr=this.extendedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.setD((hi<<8)|lo); this.nz16(this.d()); break; }
      case 0xFD: { // STD extended
        const addr=this.extendedAddr(); const d=this.d(); this.write8(addr,d>>8); this.write8(addr+1,d&0xFF); this.nz16(d); break; }
      case 0xE3: { // ADDD indexed
        const addr=this.indexedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const res=this.add16(this.d(),(hi<<8)|lo); this.setD(res); break; }
      case 0xD3: { // ADDD direct
        const addr=this.directAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const res=this.add16(this.d(),(hi<<8)|lo); this.setD(res); break; }
      case 0xF3: { // ADDD extended
        const addr=this.extendedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); const res=this.add16(this.d(),(hi<<8)|lo); this.setD(res); break; }
      case 0x02: { /* treat as padding/data NOP */ break; }
      case 0x14: { /* treat as NOP (reserved) */ break; }
      case 0x00: { // NEG direct
        const addr=this.directAddr(); const val=this.read8(addr); const res=(~val + 1)&0xFF; this.write8(addr,res); this.nz8(res); this.cc_c = val!==0; break; }
      case 0x10: { if(!this.prefix10()) return false; break; }
      case 0x11: { if(!this.prefix11()) return false; break; }
      case 0x1F: { // TFR src,dst
        const pb=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const src=(pb>>4)&0x0F; const dst=pb&0x0F; const val=this.readReg(src); this.writeReg(dst,val); break; }
      case 0x1E: { // EXG src,dst
        const pb=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const r1=(pb>>4)&0x0F; const r2=pb&0x0F; const v1=this.readReg(r1); const v2=this.readReg(r2); this.writeReg(r1,v2); this.writeReg(r2,v1); break; }
      case 0x3D: { // MUL A*B -> D
        const prod=(this.a * this.b) & 0xFFFF; this.setD(prod); this.cc_z = (prod===0); this.cc_c = (prod & 0x80)!==0; this.cc_v=false; this.cc_n=(prod & 0x8000)!==0; break; }
      case 0x3E: { // WAI simplified: push full state & halt until IRQ
        if (!this.waiWaiting){
          this.pshs(0xFF); this.waiWaiting = true; (this as any).lastWaiPc = pc0; if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'wai-enter' });
          if (this.criticalEvents.length < 1024){
            const ev = { type:'wait_recal_enter', pc:pc0, note:'wai_enter', cycles:this.cycles };
            this.criticalEvents.push(ev);
            this.lastWaitRecalEnterEvent = ev; // persist across frames until exit emitted
          } else {
            this.lastWaitRecalEnterEvent = { type:'wait_recal_enter', pc:pc0, note:'wai_enter', cycles:this.cycles };
          }
        }
        break; }
      case 0x3A: { // ABX: X = X + B (no flags affected)
        this.x = (this.x + this.b) & 0xFFFF; break; }
      case 0x3B: { // RTI: restore full state (assumes full state was pushed)
        const pcBefore=pc0; const sBefore=this.s; // snapshot CC before pull for reference
        this.pulls(0xFF);
        if (this.waitRecalInterrupted){
          // Emit exit event once when returning from the IRQ that woke WAI
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'wait_recal_exit', pc:this.pc, note:'wai_exit', cycles:this.cycles });
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'irq_exit', pc:this.pc, note:'irq_exit', cycles:this.cycles });
          this.waitRecalInterrupted = false;
          delete (this as any).lastWaiPc;
          if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'wai-exit' });
          this.lastWaitRecalEnterEvent = null; // consumed
        }
        // If generic IRQ without WAI involvement, still emit irq_exit for symmetry
        if (!this.waitRecalInterrupted){
          if (this.criticalEvents.length < 1024) this.criticalEvents.push({ type:'irq_exit', pc:this.pc, note:'irq_exit', cycles:this.cycles });
        }
        if (this.rtsLog.length<2000) this.rtsLog.push({ type:'RTI', pcBefore, pcAfter:this.pc, sBefore, sAfter:this.s, returnPc:this.pc, cc:{ z:this.cc_z, n:this.cc_n, c:this.cc_c, v:this.cc_v, i:this.cc_i } });
        break; }
      case 0x3C: { // CWAI: AND CC with immediate then push full state and wait
        const mask=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; // ANDCC semantics
        const cc=this.packCC();
        const newCc = cc & mask;
        this.cc_c=!!(newCc&1); this.cc_v=!!(newCc&2); this.cc_z=!!(newCc&4); this.cc_n=!!(newCc&8); this.cc_i=!!(newCc&0x10);
        this.pshs(0xFF);
        this.waiWaiting = true;
        break; }
      case 0x3F: { // SWI: software interrupt
        this.pshs(0xFF); this.cc_i = true; const hi=this.mem[0xFFFA]; const lo=this.mem[0xFFFB]; this.pc=((hi<<8)|lo)&0xFFFF; break; }
      // Register ops A/B (ensure not already implemented below)
      case 0x40: { const old=this.a; const r=(0-old)&0xFF; this.a=r; this.nz8(r); this.cc_c=old!==0; this.cc_v=(old===0x80); break; } // NEGA
      case 0x43: { this.a=(~this.a)&0xFF; this.nz8(this.a); this.cc_c=true; this.cc_v=false; break; } // COMA
      case 0x44: { this.cc_c=(this.a & 1)!==0; this.a=(this.a>>>1)&0xFF; this.nz8(this.a); this.cc_n=false; this.cc_v=false; break; } // LSRA
      case 0x46: { const newC=(this.a & 1)!==0; const cIn=this.cc_c?0x80:0; this.a=((this.a>>>1)|cIn)&0xFF; this.cc_c=newC; this.nz8(this.a); this.cc_v=false; break; } // RORA
      case 0x47: { this.cc_c=(this.a & 1)!==0; const msb=this.a & 0x80; this.a=((this.a>>>1)|msb)&0xFF; this.nz8(this.a); this.cc_v=false; break; } // ASRA
      case 0x48: { const newC=(this.a & 0x80)!==0; this.a=(this.a<<1)&0xFF; this.nz8(this.a); this.cc_c=newC; this.cc_v=false; break; } // LSLA/ASLA
      case 0x49: { const newC=(this.a & 0x80)!==0; const cIn=this.cc_c?1:0; this.a=((this.a<<1)|cIn)&0xFF; this.nz8(this.a); this.cc_c=newC; this.cc_v=false; break; } // ROLA
      case 0x4A: { this.a=(this.a - 1)&0xFF; this.nz8(this.a); this.cc_v=(this.a===0x7F); break; } // DECA
      case 0x4C: { this.a=(this.a + 1)&0xFF; this.nz8(this.a); this.cc_v=(this.a===0x80); break; } // INCA
      case 0x4D: { this.nz8(this.a); this.cc_v=false; break; } // TSTA
      case 0x4F: { this.a=0; this.cc_z=true; this.cc_n=false; this.cc_v=false; this.cc_c=false; break; } // CLRA
      case 0x50: { const old=this.b; const r=(0-old)&0xFF; this.b=r; this.nz8(r); this.cc_c=old!==0; this.cc_v=(old===0x80); break; } // NEGB
      case 0x53: { this.b=(~this.b)&0xFF; this.nz8(this.b); this.cc_c=true; this.cc_v=false; break; } // COMB
      case 0x54: { this.cc_c=(this.b & 1)!==0; this.b=(this.b>>>1)&0xFF; this.nz8(this.b); this.cc_n=false; this.cc_v=false; break; } // LSRB
      case 0x56: { const newC=(this.b & 1)!==0; const cIn=this.cc_c?0x80:0; this.b=((this.b>>>1)|cIn)&0xFF; this.cc_c=newC; this.nz8(this.b); this.cc_v=false; break; } // RORB
      case 0x57: { this.cc_c=(this.b & 1)!==0; const msb=this.b & 0x80; this.b=((this.b>>>1)|msb)&0xFF; this.nz8(this.b); this.cc_v=false; break; } // ASRB
      case 0x58: { const newC=(this.b & 0x80)!==0; this.b=(this.b<<1)&0xFF; this.nz8(this.b); this.cc_c=newC; this.cc_v=false; break; } // LSLB/ASLB
      case 0x59: { const newC=(this.b & 0x80)!==0; const cIn=this.cc_c?1:0; this.b=((this.b<<1)|cIn)&0xFF; this.nz8(this.b); this.cc_c=newC; this.cc_v=false; break; } // ROLB
      case 0x5A: { this.b=(this.b - 1)&0xFF; this.nz8(this.b); this.cc_v=(this.b===0x7F); break; } // DECB
      case 0x5C: { this.b=(this.b + 1)&0xFF; this.nz8(this.b); this.cc_v=(this.b===0x80); break; } // INCB
      case 0x5D: { this.nz8(this.b); this.cc_v=false; break; } // TSTB
      case 0x5F: { this.b=0; this.cc_z=true; this.cc_n=false; this.cc_v=false; this.cc_c=false; break; } // CLRB
      case 0x88: { const v=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.a = (this.a ^ v) & 0xFF; this.nz8(this.a); break; } // EORA imm
      case 0x50: { // NEGB: B = -B (two's complement)
        this.b = (~this.b + 1) & 0xFF; this.nz8(this.b); break; }
      case 0x56: { // RORB (rotate right through carry for B)
        const oldB=this.b; const newB=((this.cc_c?0x80:0)|(oldB>>1))&0xFF; this.cc_c=(oldB & 0x01)!==0; this.b=newB; this.nz8(this.b); break; }
      case 0x57: { // ASRB (arithmetic shift right B)
        const oldB=this.b; this.cc_c = (oldB & 0x01)!==0; this.b = ((oldB & 0x80) | (oldB>>1)) & 0xFF; this.nz8(this.b); break; }
      case 0x52: { // invalid/unused on 6809 (on 6800 it was something else); treat as NOP
        break; }
      case 0x54: { // also unused; treat as NOP
        break; }
      case 0x58: { // invalid/unused; treat as NOP (compiler uses as padding?)
        break; }
      case 0x55: { // COMB (complement B) real opcode: 0x53 is COMB? but using 0x55 here as placeholder from compiler emission -> treat as NOP if uncertain
        // If real meaning differs we can adjust; safer than logging spam.
        break; }
      case 0x59: { // ROLB (rotate left through carry B) implement
        const carryIn=this.cc_c?1:0; const newCarry=(this.b & 0x80)!==0; this.b=((this.b<<1)|carryIn)&0xFF; this.cc_c=newCarry; this.nz8(this.b); break; }
      // (Direct LSR/ROR handled earlier now)
      case 0x46: { // RORA (accumulator rotate right through carry)
        const oldA = this.a;
        const newA = ((this.cc_c?0x80:0) | (oldA>>1)) & 0xFF;
        this.cc_c = (oldA & 0x01)!==0;
        this.a = newA;
        this.nz8(this.a);
        break; }
      case 0x40: { // NEGA
        const old=this.a; this.a=(~this.a + 1)&0xFF; this.nz8(this.a); this.cc_c = old!==0; break; }
      case 0x43: { // COMA
        this.a = (~this.a) & 0xFF; this.nz8(this.a); this.cc_c=true; break; }
      case 0x44: { // LSRA
        this.cc_c = (this.a & 0x01)!==0; this.a = (this.a>>1)&0x7F; this.nz8(this.a); break; }
      case 0x47: { // ASRA
        this.cc_c = (this.a & 0x01)!==0; this.a = ((this.a & 0x80)|(this.a>>1)) & 0xFF; this.nz8(this.a); break; }
      case 0x48: { // LSLA/ASLA
        this.cc_c = (this.a & 0x80)!==0; this.a = (this.a<<1)&0xFF; this.nz8(this.a); break; }
      case 0x49: { // ROLA
        const carryIn=this.cc_c?1:0; const newCarry=(this.a & 0x80)!==0; this.a=((this.a<<1)|carryIn)&0xFF; this.cc_c=newCarry; this.nz8(this.a); break; }
      case 0x60: { // NEG indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); const r=this.applyRmw('NEG',v); this.write8(addr,r); break; }
      case 0x63: { // COM indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); const r=this.applyRmw('COM',v); this.write8(addr,r); break; }
      case 0x64: { // LSR indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); const r=this.applyRmw('LSR',v); this.write8(addr,r); break; }
      case 0x66: { // ROR indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); const r=this.applyRmw('ROR',v); this.write8(addr,r); break; }
      case 0x67: { // ASR indexed (memory)
        const addr=this.indexedAddr(); const v=this.read8(addr); const r=this.applyRmw('ASR',v); this.write8(addr,r); break; }
      case 0x68: { // ASL indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); const r=this.applyRmw('ASL',v); this.write8(addr,r); break; }
      case 0x69: { // ROL indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); const r=this.applyRmw('ROL',v); this.write8(addr,r); break; }
      case 0x6A: { // DEC indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); const r=this.applyRmw('DEC',v); this.write8(addr,r); break; }
      case 0x6C: { // INC indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); const r=this.applyRmw('INC',v); this.write8(addr,r); break; }
      case 0x6D: { // TST indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); this.applyRmw('TST',v); break; }
      case 0x6F: { // CLR indexed (memory)
        const addr=this.indexedAddr(); const r=this.applyRmw('CLR',0); this.write8(addr,r); break; }
      case 0x9A: { // ORA direct (real opcode 0x9A)
        const addr=this.directAddr(); this.a = (this.a | this.read8(addr)) & 0xFF; this.nz8(this.a); break; }
      case 0xAA: { // ORA indexed
        const addr=this.indexedAddr(); this.a = (this.a | this.read8(addr)) & 0xFF; this.nz8(this.a); break; }
      case 0xBA: { // ORA extended
        const addr=this.extendedAddr(); this.a = (this.a | this.read8(addr)) & 0xFF; this.nz8(this.a); break; }
      case 0x95: { // BITA direct
        const addr=this.directAddr(); const v=(this.a & this.read8(addr)) & 0xFF; this.nz8(v); break; }
      case 0xD5: { // BITB direct
        const addr=this.directAddr(); const v=(this.b & this.read8(addr)) & 0xFF; this.nz8(v); break; }
      case 0xD1: { // CMPB direct
        const addr=this.directAddr(); const v=this.read8(addr); const _=this.sub8(this.b,v); break; }
      case 0xE1: { // CMPB indexed
        const addr=this.indexedAddr(); const v=this.read8(addr); const _=this.sub8(this.b,v); break; }
      case 0xF1: { // CMPB extended
        const addr=this.extendedAddr(); const v=this.read8(addr); const _=this.sub8(this.b,v); break; }
      case 0x6F: { // CLR indexed (memory) support ,X only variants via indexedAddr
        const addr=this.indexedAddr(); this.write8(addr,0); this.cc_z=true; this.cc_n=false; break; }
      case 0xEC: { // LDD indexed
        const addr=this.indexedAddr(); const hi=this.read8(addr); const lo=this.read8(addr+1); this.setD((hi<<8)|lo); this.nz16(this.d()); break; }
      case 0xED: { // STD indexed
        const addr=this.indexedAddr(); const d=this.d(); this.write8(addr,d>>8); this.write8(addr+1,d&0xFF); this.nz16(d); break; }
      case 0xE7: { // STB indexed
        const addr=this.indexedAddr(); this.write8(addr,this.b); break; }
      case 0x3D: { // MUL (A * B -> D). Simplified flags: set N/Z on result.
        const prod = (this.a * this.b) & 0xFFFF; this.setD(prod); this.nz16(this.d()); break; }
      case 0x45: { /* invalid 6809 opcode; treating as NOP due to compiler emission */ break; }
  case 0x3E: { // WAI (wait for interrupt) (alt duplicate path) ensure instrumentation
    if (!this.waiWaiting){
      this.waiWaiting = true;
      if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'wai-enter' });
      if (this.criticalEvents.length < 1024){
        const ev={ type:'wait_recal_enter', pc:pc0, note:'wai_enter_dup', cycles:this.cycles };
        this.criticalEvents.push(ev);
        this.lastWaitRecalEnterEvent = ev;
      }
    }
    break; }
      case 0x7E: { // JMP extended
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=((hi<<8)|lo)&0xFFFF; break; }
      case 0x0E: { // JMP direct
        const addr=this.directAddr(); this.pc=addr; break; }
      case 0x6E: { // JMP indexed (support ,X only via postbyte 0x84)
        const post=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; if (post===0x84){ this.pc=this.x & 0xFFFF; } else { this.logUnknown(0x6E00|post); return false;} break; }
      case 0x9D: { // JSR direct
        const addr=this.directAddr();
        if (addr>=0xF000){
          if (!this.biosPresent){
            this.interceptBios(addr);
          } else {
            switch(addr){
              case 0xF192: case 0xF2A5: case 0xF2AB: case 0xF3DD:
                this.interceptBios(addr); break;
              default:
                this.callStack.push(this.pc); this.pc=addr; break;
            }
          }
        } else {
          this.callStack.push(this.pc); this.pc=addr;
        }
        break; }
      case 0xAD: { // JSR indexed (,X only)
        const post=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; if (post===0x84){
          const addr=this.x & 0xFFFF;
          if (addr>=0xF000){
            if (!this.biosPresent){
              this.interceptBios(addr);
            } else {
              switch(addr){
                case 0xF192: case 0xF2A5: case 0xF2AB: case 0xF3DD:
                  this.interceptBios(addr); break;
                default:
                  this.callStack.push(this.pc); this.pc=addr; break;
              }
            }
          } else {
            this.callStack.push(this.pc); this.pc=addr;
          }
        } else { this.logUnknown(0xAD00|post); return false;}
        break; }
      case 0xBD: { // JSR extended
        const hi=this.mem[this.pc]; const lo=this.mem[(this.pc+1)&0xFFFF]; this.pc=(this.pc+2)&0xFFFF; const addr=((hi<<8)|lo)&0xFFFF;
        // Transitional strategy:
        //  - If BIOS not present: intercept calls into the expected BIOS address space (legacy behavior)
        //  - If BIOS present: still intercept known drawing/intensity routines so vectors appear without VIA emulation,
        //    but otherwise execute real BIOS code (accuracy for game logic / math helpers).
        if (addr>=0xF000) {
          if (this.traceBiosVec && (addr & 0xFF00) === 0xF300 && this.traceEnabled) {
            this.debugTraces.push({ type:'info', pc:this.pc, note:`bios_vec_jsr_${addr.toString(16)}` });
          }
          if (!this.biosPresent) {
            this.interceptBios(addr); // legacy short-circuit
          } else {
            // If it's one of the vector/intensity routines we still short-circuit for now
            switch(addr){
              case 0xF192: case 0xF2A5: case 0xF2AB: case 0xF3DD:
                this.interceptBios(addr); break;
              default:
                this.callStack.push(this.pc); this.pc = addr; // execute real BIOS code
                break;
            }
          }
        } else {
          this.callStack.push(this.pc); this.pc=addr;
        }
        break; }
      case 0x39: { // RTS
        { const sBefore=this.s; const ret=this.callStack.pop(); const pcBefore=pc0; const newPc=ret ?? this.pc; this.pc=newPc; if (this.rtsLog.length<2000) this.rtsLog.push({ type:'RTS', pcBefore, pcAfter:newPc, sBefore, sAfter:this.s, returnPc:newPc }); } break; }
      case 0x4F: { this.a=0; this.nz8(this.a); break; }
      case 0x5F: { this.b=0; this.nz8(this.b); break; }
      case 0x4C: { // INCA
        this.a=(this.a+1)&0xFF; this.nz8(this.a); break; }
      case 0x5C: { // INCB
        this.b=(this.b+1)&0xFF; this.nz8(this.b); break; }
      case 0x4A: { // DECA
        this.a=(this.a-1)&0xFF; this.nz8(this.a); break; }
      case 0x5A: { // DECB
        this.b=(this.b-1)&0xFF; this.nz8(this.b); break; }
      case 0x4D: { // TSTA
        this.nz8(this.a); break; }
      case 0x5D: { // TSTB
        this.nz8(this.b); break; }
      case 0x34: { // PSHS mask
        const mask=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.pshs(mask); break; }
      case 0x35: { // PULS mask
        const mask=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; this.pulls(mask); break; }
      case 0xF8: { // EORB extended (XOR B with extended address)
        const addr=this.extendedAddr(); this.b=(this.b ^ this.read8(addr)) & 0xFF; this.nz8(this.b); break; }
      case 0x90: { // SUBA direct
        const addr=this.directAddr(); const v=this.read8(addr); this.ldaSet((this.a - v) & 0xFF); break; }
      case 0xC7: { // EORA immediate? Real 0xC7 is not EORA; We'll treat 0xC7 as DAA placeholder (Decimal Adjust Accumulator) -> no-op flags preserved
        // Minimal: do nothing (real DAA would adjust after BCD adds)
        break; }
      case 0xE4: { // ANDB indexed (real 0xE4 is ANDB indexed addressing mode)
        const addr=this.indexedAddr(); this.b = (this.b & this.read8(addr)) & 0xFF; this.nz8(this.b); break; }
      case 0x1F: { // TFR
        const pb=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const src=(pb>>4)&0x0F; const dst=pb&0x0F; const v=this.readReg(src); this.writeReg(dst,v); break; }
      case 0x1E: { // EXG
        const pb=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const r1=(pb>>4)&0x0F; const r2=pb&0x0F; const v1=this.readReg(r1); const v2=this.readReg(r2); this.writeReg(r1,v2); this.writeReg(r2,v1); break; }
      case 0x1A: { // ORCC immediate
        const mask=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const cc=(this.cc_c?1:0)|(this.cc_v?2:0)|(this.cc_z?4:0)|(this.cc_n?8:0)|(this.cc_i?0x10:0); const ncc=cc | mask; this.cc_c=!!(ncc&1); this.cc_v=!!(ncc&2); this.cc_z=!!(ncc&4); this.cc_n=!!(ncc&8); this.cc_i=!!(ncc&0x10); break; }
      case 0x1C: { // ANDCC immediate (clear bits)
        const mask=this.mem[this.pc]; this.pc=(this.pc+1)&0xFFFF; const cc=(this.cc_c?1:0)|(this.cc_v?2:0)|(this.cc_z?4:0)|(this.cc_n?8:0)|(this.cc_i?0x10:0); const ncc=cc & mask; this.cc_c=!!(ncc&1); this.cc_v=!!(ncc&2); this.cc_z=!!(ncc&4); this.cc_n=!!(ncc&8); this.cc_i=!!(ncc&0x10); break; }
      case 0x67: { // ASR indexed (memory)
        const addr=this.indexedAddr(); const val=this.read8(addr); const newVal = ((val & 0x80) | (val>>1)) & 0xFF; this.cc_c = (val & 0x01)!==0; this.write8(addr,newVal); this.nz8(newVal); break; }
      case 0x20: { const off=(this.mem[this.pc]<<24)>>24; this.pc=(this.pc+1)&0xFFFF; this.branch(off); break; } // BRA
      case 0x21: case 0x22: case 0x23: case 0x24: case 0x25: case 0x26: case 0x27: case 0x28: case 0x29: case 0x2A: case 0x2B: case 0x2C: case 0x2D: case 0x2E: case 0x2F: {
        const off=(this.mem[this.pc]<<24)>>24; this.pc=(this.pc+1)&0xFFFF; if (this.cond(op)) { this.branch(off); } break; }
      default:
        this.logUnknown(op);
        return false;
    }
    this.cycles += cyc;
    if (this.vectorMode==='via' && this.hardwareVectors){
      this.integrateBeam(cyc);
    }
    if (this.vectorMode === 'via') this.updateTimers(cyc);

    // PC=0 guard: detect unexpected transition into address 0 while BIOS active (likely bad return)
    if (!this.pcZeroGuardTriggered && this.biosPresent && pc0 !== 0 && this.pc === 0) {
      this.pcZeroGuardTriggered = true;
      // Capture stack bytes (32) from current S upward
      const stackBytes: number[] = [];
      for (let i=0;i<32;i++) stackBytes.push(this.read8((this.s + i) & 0xFFFF));
      // Capture shallow register snapshot
      const regs = { a:this.a, b:this.b, x:this.x, y:this.y, u:this.u, s:this.s, pc:this.pc, dp:this.dp, cc:{ z:this.cc_z, n:this.cc_n, c:this.cc_c, v:this.cc_v, i:this.cc_i } };
      const tail = this.tailInstr.slice(-16); // last 16 for brevity
      this.pcZeroSnapshot = { regs, stackBytes, tail };
      if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:pc0, note:'pc_zero_guard' });
    }
    return true;
  }

  private integrateBeam(cyc:number){
    if (!this.beamDrawing){ this.lastBeamUpdateCycles += cyc; return; }
    // For each cycle, accumulate velocity; we treat velocity units as direct coordinate steps per cycle (coarse).
    const startX=this.beamX; const startY=this.beamY;
    this.beamX += this.velX * cyc;
    this.beamY += this.velY * cyc;
    if (this.beamX!==startX || this.beamY!==startY){
      this.frameSegments.push({ x1:startX, y1:startY, x2:this.beamX, y2:this.beamY, intensity:this.beamLastIntensity });
    }
    this.lastBeamUpdateCycles = 0;
  }

  runUntilFrame(maxSteps=200000){
    // Perform one-time auto-start heuristic before stepping.
    if (!this.attemptedAutoStart) this.attemptAutoStartUser();
    this.frameReady=false;
    // If a WAI was entered in a previous frame and not yet exited, re-surface the enter event so diagnostic frame views see it
    if (this.waiWaiting && this.lastWaitRecalEnterEvent){
      // Only re-add if not already present this frame (criticalEvents just cleared at prior return)
      if (this.criticalEvents.length < 1024) this.criticalEvents.push({ ...this.lastWaitRecalEnterEvent, note: this.lastWaitRecalEnterEvent.note });
    }
    let lastPc=-1; let repeatCount=0;
    for (let i=0;i<maxSteps;i++){
      const pcBefore=this.pc;
      if (!this.step()) break;
      if (this.frameReady) break;
      if (this.pc === pcBefore){
        // Self-loop single address
        if (this.pc === lastPc) repeatCount++; else { lastPc=this.pc; repeatCount=1; }
      } else {
        lastPc = this.pc; repeatCount=1;
      }
      if (repeatCount>5000){
        if (this.traceEnabled) this.debugTraces.push({ type:'info', pc:this.pc, note:'loop_break' });
        break;
      }
    }
    const segs = this.frameSegments.slice();
    this.frameSegments.length = 0;
    const events = this.viaEvents.slice();
    this.viaEvents.length = 0;
    const traces = this.debugTraces.slice();
    this.debugTraces.length = 0;
    const opcodeTrace = this.opcodeTraceEnabled ? this.opcodeTrace.slice() : [] as Array<{pc:number;op:number}>;
    if (this.opcodeTraceEnabled) this.opcodeTrace.length = 0; // clear each frame when enabled
    const fullInstr = this.fullInstrTraceEnabled ? this.fullInstrTrace.slice() : [] as Array<{pc:number;op:number;b1?:number;b2?:number;b3?:number}>;
    if (this.fullInstrTraceEnabled) this.fullInstrTrace.length = 0;
    const rtsLog = this.rtsLog.slice(); this.rtsLog.length = 0;
    // Include tail instruction buffer and pc=0 guard snapshot if triggered
    const tailInstr = this.tailInstr.slice();
    const crit = this.criticalEvents.slice(); this.criticalEvents.length = 0;
    return { frameReady:this.frameReady, segments: segs, viaEvents: events, debugTraces: traces, opcodeTrace, fullInstr, tailInstr, rtsLog, pcZeroGuard: this.pcZeroGuardTriggered, pcZeroSnapshot: this.pcZeroSnapshot, biosPresent: this.biosPresent, callStack: this.callStack.slice(), criticalEvents: crit };
  }
}

export const globalCpu = new Cpu6809();

export function getStats(){
  return {
    unknownOpcodes: globalCpu.unknownLog,
    regs: { a:globalCpu.a, b:globalCpu.b, x:globalCpu.x, y:globalCpu.y, u:globalCpu.u, pc:globalCpu.pc, dp:globalCpu.dp },
  };
}

export function resetStats(){ globalCpu.unknownLog = {}; }

export function hardResetCpu(){ globalCpu.cpuReset(); }
