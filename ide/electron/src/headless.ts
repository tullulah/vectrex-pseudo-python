#!/usr/bin/env node
// Headless harness for Cpu6809 to enable automated integration tests without Electron UI.
// Usage examples:
//   node dist/headless.js --bios ./bios/vectrex.bin --bin ./examples/triangle_simple.bin --frames 5 --opcodeTrace --trace
//   node dist/headless.js --diagnose --frames 12

import { readFileSync } from 'fs';
import { resolve, join, isAbsolute } from 'path';
import { globalCpu, hardResetCpu } from './emu6809';

interface Args { [k:string]: string|boolean|number; }
function parseArgs(argv:string[]): Args {
  const out:Args = {};
  for (let i=2;i<argv.length;i++){
    const a = argv[i];
    if (a === '--') continue;
    if (a.startsWith('--')){
      const eq = a.indexOf('=');
      if (eq !== -1){
        out[a.slice(2,eq)] = a.slice(eq+1);
      } else {
        const key = a.slice(2);
        const next = argv[i+1];
        if (next && !next.startsWith('--')){ out[key]=next; i++; } else { out[key]=true; }
      }
    }
  }
  return out;
}

const args=parseArgs(process.argv);
// Fallback positional parsing (because nested npm scripts may strip leading -- flags):
//   headless <biosPath?> <binPath?> <frames?> <diagnose?>
let rawArgs = process.argv.slice(2);
// If first arg is a lone '--' (injected by script to preserve forwarding), drop it.
if (rawArgs[0] === '--') rawArgs = rawArgs.slice(1);
const positionals = rawArgs.filter(a => !a.startsWith('--'));
// Repo root (this file compiles to .../ide/electron/dist/headless.js) so go up THREE levels to reach repository root
const repoRoot = resolve(__dirname, '..', '..', '..');
function fixPath(p:string | null): string | null {
  if (!p) return null;
  // When invoked through root script, npm --prefix may have inserted paths relative to electron
  // We treat incoming relative paths as relative to repo root for user convenience.
  if (!isAbsolute(p)) return resolve(repoRoot, p);
  return p;
}
// Support env-based overrides (useful when npm script argument forwarding strips flags)
const envArg = (k:string) => process.env[k] || process.env['npm_config_'+k];
const eBios = envArg('bios'); if (eBios && !args.bios) args.bios = String(eBios);
const eBin = envArg('bin'); if (eBin && !args.bin) args.bin = String(eBin);
const eFrames = envArg('frames'); if (eFrames && !args.frames) args.frames = String(eFrames);
const eDiag = envArg('diagnose'); if (eDiag && !args.diagnose) args.diagnose = String(eDiag);
const eTrace = envArg('trace'); if (eTrace && !args.trace) args.trace = String(eTrace);
const eOp = envArg('opcodeTrace'); if (eOp && !args.opcodeTrace) args.opcodeTrace = String(eOp);
// Allow vectorMode to be supplied via environment (VECTOR_MODE, vectorMode or npm_config_vectorMode)
const eVectorMode = envArg('vectorMode') || process.env.VECTOR_MODE; if (eVectorMode && !args.vectorMode) args.vectorMode = String(eVectorMode);

let biosPath = typeof args.bios === 'string' ? String(args.bios) : null;
let binPath = typeof args.bin === 'string' ? String(args.bin) : null;
// Some npm argument forwarding quirks can leave a bare --bios (no value) which becomes 'true'
if (biosPath && (/^true$/i.test(biosPath))) biosPath = null;
if (binPath && (/^true$/i.test(binPath))) binPath = null;
biosPath = biosPath ? fixPath(biosPath) : null;
binPath = binPath ? fixPath(binPath) : null;
if (!biosPath && positionals[0] && /\.bin$/i.test(positionals[0])) biosPath = fixPath(positionals[0]);
if (!binPath && positionals[1] && /\.bin$/i.test(positionals[1])) binPath = fixPath(positionals[1]);
// Derive frames & optional maxSteps from numeric positionals if explicit flags absent.
// Heuristic: first numeric => frames, second numeric => maxSteps (if --maxSteps not provided)
let frames = args.frames ? Number(args.frames) : NaN;
let positionalMaxSteps: number | null = null;
const numericPositionals = positionals.map(p=>Number(p)).filter(n=>Number.isFinite(n) && n>=0);
if (!Number.isFinite(frames)) {
  if (numericPositionals.length>0) frames = numericPositionals[0];
}
if (args.maxSteps === undefined && numericPositionals.length>1){
  positionalMaxSteps = numericPositionals[1];
}
if (!Number.isFinite(frames) || frames < 1) frames = 1;
const diagnose = !!args.diagnose || rawArgs.some(a => a === '--diagnose') || /diagnose|diag/i.test(positionals.join(' '));
// vectorMode argument (intercept|via) default intercept
const vectorModeArg = ((): string => {
  // Accept explicit --vectorMode=via|intercept, environment overrides, or shorthand --via / --intercept flags
  if (typeof args.vectorMode === 'string') return String(args.vectorMode);
  const viaFlag = rawArgs.find(a=>a.startsWith('--vectorMode='));
  if (viaFlag) return viaFlag.split('=')[1];
  if (rawArgs.includes('--via')) return 'via';
  if (rawArgs.includes('--intercept')) return 'intercept';
  return 'intercept';
})();
const forceUserStart = !!args.forceUserStart || rawArgs.includes('--forceUserStart') || rawArgs.some(a => /forceuserstart/i.test(a)) || positionals.some(p=>/forceuserstart/i.test(p));
const maxSteps = args.maxSteps ? Number(args.maxSteps) : (positionalMaxSteps || 50000);
let doOpcode = !!args.opcodeTrace;
const doTrace = !!args.trace;
const traceCalls = !!args.traceCalls || rawArgs.includes('--traceCalls');
const entryMode = typeof args.entryMode === 'string' ? String(args.entryMode) : (rawArgs.includes('--entryMode=reset') ? 'reset' : (rawArgs.includes('--entryMode') ? 'reset' : 'heuristic'));
const noInterceptDraw = !!args.noInterceptDraw || rawArgs.includes('--noInterceptDraw');
const noSyntheticIrq = !!args.noSyntheticIrq || rawArgs.includes('--noSyntheticIrq');
const traceAllInstr = !!args.traceAllInstr || rawArgs.includes('--traceAllInstr');
const devTimerAssist = !!args.devTimerAssist || rawArgs.includes('--devTimerAssist') || !!envArg('devTimerAssist');
const biosStart = !!args.biosStart || rawArgs.includes('--biosStart');
const authMode = !!args.auth || rawArgs.includes('--auth');
if (diagnose) doOpcode = true; // force opcode trace in diagnose mode
const traceBiosVec = !!args.traceBiosVec || rawArgs.includes('--traceBiosVec');
console.log(JSON.stringify({ event:'args_interpreted', cwd:process.cwd(), repoRoot, biosPath, binPath, frames, diagnose, vectorMode: vectorModeArg, trace:doTrace, opcodeTrace:doOpcode, traceCalls, traceBiosVec, entryMode, noInterceptDraw, noSyntheticIrq, traceAllInstr, forceUserStart, maxSteps, devTimerAssist, biosStart, auth:authMode, raw:rawArgs }));

function retryPath(original:string): string | null {
  // If path erroneously includes an extra 'ide' segment (e.g. /repo/ide/core/...), try removing first '/ide'
  const parts = original.split(/\\|\//);
  const idx = parts.indexOf('ide');
  if (idx !== -1) {
    const altParts = parts.slice(0, idx).concat(parts.slice(idx+1));
    const alt = altParts.join('/');
    return alt;
  }
  return null;
}

function loadBios(p:string){
  try {
    const buf = readFileSync(p);
    const ok = (globalCpu as any).loadBios?.(new Uint8Array(buf));
    if (!ok) console.error(JSON.stringify({ event:'bios_load_failed', path:p, size:buf.length }));
    else console.log(JSON.stringify({ event:'bios_loaded', path:p, size:buf.length }));
  } catch(e:any){
    if (e?.code === 'ENOENT') {
      const alt = retryPath(p);
      if (alt && alt !== p) {
        try {
          console.log(JSON.stringify({ event:'bios_retry', from:p, to:alt }));
          const buf2 = readFileSync(alt);
          const ok2 = (globalCpu as any).loadBios?.(new Uint8Array(buf2));
          if (!ok2) console.error(JSON.stringify({ event:'bios_load_failed', path:alt, size:buf2.length }));
          else { console.log(JSON.stringify({ event:'bios_loaded', path:alt, size:buf2.length, retry:true })); return; }
        } catch(e2:any){
          console.error(JSON.stringify({ event:'bios_retry_error', from:p, to:alt, error:e2?.message }));
        }
      }
    }
    console.error(JSON.stringify({ event:'bios_error', path:p, error:e?.message }));
  }
}
function loadBin(p:string){
  try {
    const buf = readFileSync(p);
    globalCpu.mem.fill(0);
    if (globalCpu.biosPresent && (globalCpu as any).reapplyBios) (globalCpu as any).reapplyBios();
  globalCpu.a=0; globalCpu.b=0; globalCpu.dp=0xD0; globalCpu.x=0; globalCpu.y=0; globalCpu.u=0; globalCpu.s=0xC000; globalCpu.pc=0; // fixed ORG $0000
    // Load cartridge image at 0x0000 user space
    globalCpu.mem.set(new Uint8Array(buf), 0x0000);
    // Decide initial PC based on entryMode
    if (globalCpu.biosPresent){
      if (entryMode === 'reset') {
        hardResetCpu();
      } else {
        const rvHi = globalCpu.mem[0xFFFE]; const rvLo = globalCpu.mem[0xFFFF];
        const resetVec = ((rvHi<<8)|rvLo)&0xFFFF;
        if (resetVec >= 0xF000) globalCpu.pc = resetVec;
      }
    }
    console.log(JSON.stringify({ event:'bin_loaded', path:p, size:buf.length }));
  } catch(e:any){
    if (e?.code === 'ENOENT') {
      const alt = retryPath(p);
      if (alt && alt !== p) {
        try {
          console.log(JSON.stringify({ event:'bin_retry', from:p, to:alt }));
          const buf2 = readFileSync(alt);
            globalCpu.mem.fill(0);
            if (globalCpu.biosPresent && (globalCpu as any).reapplyBios) (globalCpu as any).reapplyBios();
            globalCpu.a=0; globalCpu.b=0; globalCpu.dp=0xD0; globalCpu.x=0; globalCpu.y=0; globalCpu.u=0; globalCpu.s=0xC000; globalCpu.pc=0; // fixed ORG $0000
            if (globalCpu.biosPresent){
              const rvHi = globalCpu.mem[0xFFFE]; const rvLo = globalCpu.mem[0xFFFF];
              globalCpu.pc = ((rvHi<<8)|rvLo)&0xFFFF;
            }
            globalCpu.mem.set(new Uint8Array(buf2), 0x0000);
            console.log(JSON.stringify({ event:'bin_loaded', path:alt, size:buf2.length, retry:true }));
            return;
        } catch(e2:any){
          console.error(JSON.stringify({ event:'bin_retry_error', from:p, to:alt, error:e2?.message }));
        }
      }
    }
    console.error(JSON.stringify({ event:'bin_error', path:p, error:e?.message }));
  }
}

if (!biosPath) {
  // Attempt auto BIOS path discovery: 1) ENV VPY_BIOS, 2) repoRoot/runtime/vectrex_bios.bin, 3) repoRoot/include/vectrex.bin
  const candidateEnv = process.env.VPY_BIOS && isAbsolute(process.env.VPY_BIOS) ? process.env.VPY_BIOS : null;
  const candidate1 = resolve(repoRoot, 'runtime', 'vectrex_bios.bin');
  const candidate2 = resolve(repoRoot, 'include', 'vectrex.bin');
  const fs = require('fs');
  const pick = candidateEnv && fs.existsSync(candidateEnv) ? candidateEnv : (fs.existsSync(candidate1) ? candidate1 : (fs.existsSync(candidate2) ? candidate2 : null));
  if (pick) {
    biosPath = pick;
    console.log(JSON.stringify({ event:'bios_auto_pick', path:pick }));
  }
}
if (biosPath) loadBios(biosPath); else console.log(JSON.stringify({ event:'bios_skip', reason:'no_path' }));
if (binPath) loadBin(binPath); else {
  console.log(JSON.stringify({ event:'bin_skip', reason:'no_path' }));
  if (entryMode === 'reset' && globalCpu.biosPresent) {
    hardResetCpu();
  }
}

// Propagate devTimerAssist flag into CPU (default false when absent)
(globalCpu as any).devTimerAssist = typeof devTimerAssist === 'boolean' ? devTimerAssist : false;

if (forceUserStart && entryMode !== 'reset'){
  const cpu:any = globalCpu as any;
  // Emit header bytes (first 32) to aid debugging of entry selection.
  const headerBytes:number[] = []; for (let i=0;i<32;i++) headerBytes.push(cpu.mem[i]||0);
  let entry = 0x0000;
  let entryHeuristicDetail: any = {};
  // Loosened header detection: search first 16 bytes for sequence G C E (case-insensitive)
  let hasCartHeader = false;
  outer: for (let start=0; start<16; start++){
    if ((cpu.mem[start]===0x47||cpu.mem[start]===0x67) && (cpu.mem[(start+1)&0xFFFF]===0x43||cpu.mem[(start+1)&0xFFFF]===0x63) && (cpu.mem[(start+2)&0xFFFF]===0x45||cpu.mem[(start+2)&0xFFFF]===0x65)) { hasCartHeader=true; break outer; }
  }
  let waitRecalFound = false;
  let waitRecalAt = -1;
  // Collect all JSR WAIT_RECAL candidates in first 2KB
  const waitRecalSites:number[] = [];
  for (let addr=0; addr<0x0800; addr++){
    if (cpu.mem[addr] === 0xBD && cpu.mem[(addr+1)&0xFFFF] === 0xF1 && cpu.mem[(addr+2)&0xFFFF] === 0x92){
      waitRecalSites.push(addr);
    }
  }
  if (waitRecalSites.length){
    waitRecalFound = true; waitRecalAt = waitRecalSites[0];
  }
  // Gather Draw_VL variant call sites (reuse table from static scan)
  const drawVariants = [0xF3AD,0xF3B1,0xF3B5,0xF3B7,0xF3B9,0xF3BC,0xF3BE,0xF3CE,0xF3D2,0xF3D6,0xF3D8,0xF3DA,0xF3DD];
  const drawCalls:number[] = [];
  for (let addr=0; addr<0x8000; addr++){
    if (cpu.mem[addr] === 0xBD){
      const hi=cpu.mem[(addr+1)&0xFFFF]; const lo=cpu.mem[(addr+2)&0xFFFF]; const tgt=(hi<<8)|lo;
      if (drawVariants.includes(tgt)) drawCalls.push(addr);
    }
  }
  // Heuristic strategy:
  // 1. If we have both waitRecalSites and drawCalls, choose the latest WAIT_RECAL that is still < first drawCall (prologue to frame logic).
  // 2. Else choose first WAIT_RECAL.
  // 3. If no WAIT_RECAL, fall back to first non-ASCII opcode marker.
  if (waitRecalSites.length && drawCalls.length){
    const firstDraw = drawCalls[0];
    let candidate = waitRecalSites[0];
    for (const w of waitRecalSites){ if (w < firstDraw) candidate = w; else break; }
    entry = candidate;
    entryHeuristicDetail = { mode:'waitRecal_before_draw', candidate, firstDraw };
  } else if (waitRecalSites.length){
    entry = waitRecalSites[0];
    entryHeuristicDetail = { mode:'first_waitRecal', candidate: entry };
  }
  // Secondary heuristic: if we detected a header and did NOT find JSR yet, skip ASCII/padding header zone to first non-ASCII opcode-ish byte.
  if (hasCartHeader && !waitRecalFound){
    for (let addr=0; addr<0x0100; addr++){
      const b = cpu.mem[addr];
      if (b===0xBD || b===0x8D || b===0x34 || b===0x35) { entry = addr; break; }
    }
    entryHeuristicDetail = { mode:'header_fallback_scan', entry };
  }
  cpu.pc = entry & 0xFFFF;
  cpu.attemptedAutoStart = true;
  if (cpu.traceEnabled) cpu.debugTraces.push({ type:'info', pc:cpu.pc, note:'force_user_start' });
  console.log(JSON.stringify({ event:'force_user_start', pc: cpu.pc, cartHeader:hasCartHeader, entryHeuristic: entry, entryHeuristicDetail, waitRecalFound, waitRecalAt, headerBytes }));
}

(globalCpu as any).opcodeTraceEnabled = doOpcode;
(globalCpu as any).traceEnabled = doTrace;
(globalCpu as any).traceBiosVec = traceBiosVec;
(globalCpu as any).noInterceptDraw = noInterceptDraw;
(globalCpu as any).disableSyntheticIrq = noSyntheticIrq;
(globalCpu as any).fullInstrTraceEnabled = traceAllInstr;
// If biosStart requested, disable autoStartUser heuristic so execution flows naturally through BIOS including WAIT_RECAL
if (biosStart) { (globalCpu as any).autoStartUser = false; }
if (authMode){
  (globalCpu as any).autoStartUser = false;
  (globalCpu as any).disableSyntheticIrq = true;
  (globalCpu as any).devTimerAssist = false;
  (globalCpu as any).noInterceptWaitRecal = true; // allow authentic BIOS WAIT_RECAL execution
  // Re-apply BIOS reset start if BIOS already loaded
  if ((globalCpu as any).biosPresent){ try { (globalCpu as any).cpuReset(); } catch {} }
}
if (typeof (globalCpu as any).setVectorMode === 'function') {
  try { (globalCpu as any).setVectorMode(vectorModeArg === 'via' ? 'via' : 'intercept'); } catch {}
}

if (diagnose){
  const cpu:any = globalCpu as any;
  // Static pattern scans BEFORE running frames (helps choose better entry heuristics later)
  const drawVlJsrs:number[] = []; // addresses of JSR to any Draw_VL variant (Mov_Draw_VL*, Draw_VL*)
  const drawVlTargets = new Set([
    0xF3AD,0xF3B1,0xF3B5,0xF3B7,0xF3B9,0xF3BC,0xF3BE, // Mov_Draw_VL* family
    0xF3CE,0xF3D2,0xF3D6,0xF3D8,0xF3DA,0xF3DD // Draw_VL* family (base Draw_VL at F3DD)
  ]);
  const waitRecalJsrs:number[] = []; // addresses whose first byte starts a JSR $F192 sequence (BD F1 92)
  try {
    for (let addr=0; addr<0x8000; addr++){ // scan user region only for speed
      const op = cpu.mem[addr];
      if (op === 0xBD){
        const hi = cpu.mem[(addr+1)&0xFFFF];
        const lo = cpu.mem[(addr+2)&0xFFFF];
        const tgt = (hi<<8)|lo;
        if (drawVlTargets.has(tgt)) drawVlJsrs.push(addr);
        if (hi===0xF1 && lo===0x92) waitRecalJsrs.push(addr);
      }
    }
  } catch(e:any){
    console.error(JSON.stringify({ event:'scan_error', error:e?.message }));
  }
  const out:any = { type:'diagnose', frames:[], regsStart:{ a:cpu.a,b:cpu.b,x:cpu.x,y:cpu.y,u:cpu.u,pc:cpu.pc,dp:cpu.dp }, staticScan:{ drawVlJsrs, waitRecalJsrs, counts:{ drawVl:drawVlJsrs.length, waitRecal:waitRecalJsrs.length } } };
  for (let i=0;i<frames;i++){
    const beforeU = cpu.u & 0xFFFF;
    const uBytes:number[] = []; for (let j=0;j<16;j++) uBytes.push(cpu.mem[(beforeU+j)&0xFFFF]);
    const startPc = cpu.pc & 0xFFFF;
    const firstOpcode = cpu.mem[startPc];
    const pcBytes:number[] = []; for (let j=0;j<8;j++) pcBytes.push(cpu.mem[(startPc+j)&0xFFFF]);
    // Allow larger budget for first frame in auth mode (to reach BIOS WAIT_RECAL naturally)
    const frameSteps = (authMode && i===0) ? Math.max(maxSteps, 300000) : maxSteps;
    const { frameReady, segments, debugTraces, opcodeTrace, fullInstr, tailInstr, rtsLog, pcZeroGuard, pcZeroSnapshot, callStack, criticalEvents } = cpu.runUntilFrame(frameSteps);
    // Fallback jump to user code ONLY if not authentic mode
    if (!authMode && !forceUserStart && i===0 && startPc >= 0xF000 && !(debugTraces||[]).some((t:any)=> (t.note||'').includes('WAIT_RECAL')) ){
      cpu.pc = 0x0000; cpu.attemptedAutoStart = true;
      if (cpu.traceEnabled) debugTraces.push({ type:'info', pc:cpu.pc, note:'auto_force_user_after_bios_loop' });
    } else if (authMode && i===0 && startPc >= 0xF000 && cpu.traceEnabled){
      debugTraces.push({ type:'info', pc:startPc, note:'auth_skip_force_user' });
    }
    const callSlice = traceCalls ? (callStack||[]).slice(-8) : undefined;
    const traceNotes = (debugTraces||[]).map((t:any)=>t.note||t.type);
    const biosVecTraces = traceNotes.filter((n:string)=> n.startsWith('bios_vec_jsr_'));
    let instrSample:any = undefined;
    if (traceAllInstr) {
      // include first 64 instructions to keep JSON manageable in diagnose mode
  instrSample = (fullInstr||[]).slice(0,64).map((r: any)=>({pc:r.pc,op:r.op,b1:r.b1,b2:r.b2,b3:r.b3}));
    }
    const viaSnapshot = Array.from({length:16}, (_:any,idx:number)=> cpu.via[idx] & 0xFF);
  out.frames.push({ i, frameReady, segs:segments.length, traces:traceNotes, debugTracesRaw: (debugTraces||[]), criticalEvents: (criticalEvents||[]), biosVecJsrs: biosVecTraces, biosVecJsrsCount: biosVecTraces.length, opcodes: (opcodeTrace||[]).slice(0,8), instr: instrSample, instrCount: (fullInstr||[]).length, tailInstr: (tailInstr||[]).slice(-16), rtsLog: (rtsLog||[]), pcZeroGuard, pcZeroSnapshot, startPc, firstOpcode, pcBytes, u:beforeU, uBytes, callStack: callSlice, via: viaSnapshot });
  }
  const cpu2:any = globalCpu as any;
  out.regsEnd = { a:cpu2.a,b:cpu2.b,x:cpu2.x,y:cpu2.y,u:cpu2.u,pc:cpu2.pc,dp:cpu2.dp };
  out.unknownOpcodes = { ...(cpu2.unknownLog||{}) };
  console.log(JSON.stringify(out));
  process.exit(0);
}

for (let i=0;i<frames;i++){
  const cpu:any = globalCpu as any;
  const res = cpu.runUntilFrame(maxSteps);
  console.log(JSON.stringify({ event:'frame', i, ready:res.frameReady, segs:res.segments.length, traces: (res.debugTraces||[]).map((t:any)=>t.note||t.type), opcodeCount:(res.opcodeTrace||[]).length, pc:cpu.pc, u:cpu.u }));
}

const final:any = globalCpu as any;
console.log(JSON.stringify({ event:'done', pc:final.pc, u:final.u, bios:final.biosPresent }));
