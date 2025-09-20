import React, { useEffect, useRef, useState, useCallback } from 'react';
import { globalEmu, VectorEvent, Segment } from '../../emulatorWasm';
// @ts-ignore import binary asset (Vite will turn into URL)
import biosUrl from '../../assets/bios.bin';
import { inputManager } from '../../inputManager';
import { useEmulatorStore } from '../../state/emulatorStore';
import { useEditorStore } from '../../state/editorStore';

export const EmulatorPanel: React.FC = () => {
  const status = useEmulatorStore(s => s.status);
  const setStatus = useEmulatorStore(s => s.setStatus);
  const [frameCount, setFrameCount] = useState(0);
  const [biosFrame, setBiosFrame] = useState(0);
  const [cycleFrame, setCycleFrame] = useState(0);
  const [vecEvents, setVecEvents] = useState<VectorEvent[]>([]);
  const [viaMetrics, setViaMetrics] = useState<{t1:number, ifr:number, ier:number, irq_line:boolean, irq_count:number}|null>(null);
  const [showLoopWatch, setShowLoopWatch] = useState(false);
  const [loopSamples, setLoopSamples] = useState<any[]>([]);
  const [segmentsCount, setSegmentsCount] = useState(0);
  const [lastSegments, setLastSegments] = useState<Segment[]>([]);
  const [instrBudget, setInstrBudget] = useState(25000); // ~1.5M / 60 fps
  const [demoMode, setDemoMode] = useState(false);
  const [demoStatus, setDemoStatus] = useState<'idle'|'waiting'|'ok'|'fallback'>('idle');
  const [traceVectors, setTraceVectors] = useState(() => { try { return localStorage.getItem('emu_trace_vectors')==='1'; } catch { return false; } });
  const [holdSegments, setHoldSegments] = useState(() => { try { return localStorage.getItem('emu_hold_segments')==='1'; } catch { return false; } });
  const [baseAddrHex, setBaseAddrHex] = useState('0000');
  const [lastBinInfo, setLastBinInfo] = useState<{ path?:string; size?:number; base:number; bytes?:Uint8Array }|null>(null);
  const [toasts, setToasts] = useState<Array<{ id:number; msg:string; kind:'info'|'error'; ts:number }>>([]);
  const toastIdRef = useRef(1);
  // Removed manual path & detect workflow (now rely solely on dropdown or active editor)
  const [sourceList, setSourceList] = useState<Array<{path:string; kind:'vpy'|'asm'}>>([]);
  const [selectedSource, setSelectedSource] = useState('');
  // Hook editor store for active document (reactive)
  const editorActive = useEditorStore(s => s.active);
  const editorDocuments = useEditorStore(s => s.documents);
  const activeDoc = editorDocuments.find(d => d.uri === editorActive);
  const rafRef = useRef<number | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const lastVecEventsRef = useRef<VectorEvent[]>([]);

  const fixedCanvasInit = useCallback(() => {
    const canvas = canvasRef.current; if (!canvas) return;
    const dpr = window.devicePixelRatio || 1;
    const WIDTH = 300; const HEIGHT = 400; // 3:4 aspect
    canvas.width = WIDTH * dpr; canvas.height = HEIGHT * dpr;
    canvas.style.width = WIDTH + 'px'; canvas.style.height = HEIGHT + 'px';
  }, []);

  const drawSegments = (segments: any[]) => {
    const canvas = canvasRef.current; if (!canvas) return;
    const ctx = canvas.getContext('2d'); if (!ctx) return;
    const dpr = window.devicePixelRatio || 1; ctx.save(); ctx.scale(dpr,dpr);
    const WIDTH = canvas.width / dpr; const HEIGHT = canvas.height / dpr;
    ctx.fillStyle = 'black'; ctx.fillRect(0,0,WIDTH,HEIGHT);
    if (!segments.length) {
      ctx.fillStyle = '#0f0'; ctx.font = '12px monospace'; ctx.textAlign = 'center';
      const cx = WIDTH/2, cy = HEIGHT/2;
      if (!globalEmu.isBiosLoaded()) { ctx.fillText('Loading BIOS...', cx, cy); }
      else { ctx.fillText('No segments yet', cx, cy); }
      ctx.restore(); return;
    }
    ctx.lineWidth = 1;
    // 1) Detectar si los valores parecen ya normalizados (-1..1) o son crudos (rango grande)
    let maxAbs = 0;
    for (const s of segments) {
      const {x0,y0,x1,y1} = normalizeSeg(s);
      maxAbs = Math.max(maxAbs, Math.abs(x0), Math.abs(y0), Math.abs(x1), Math.abs(y1));
    }
    // Si maxAbs <= 1.2 asumimos normalizados. Si no, trabajamos con valores crudos (no dividimos todavía)
    const normalizedInput = maxAbs <= 1.2;
    // 2) Calcular bounding box en el espacio “bruto” (si normalizados, el bruto ya es [-1,1])
    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
    for (const sRaw of segments) {
      const s = normalizeSeg(sRaw);
      if (!Number.isFinite(s.x0) || !Number.isFinite(s.y0) || !Number.isFinite(s.x1) || !Number.isFinite(s.y1)) continue;
      minX = Math.min(minX, s.x0, s.x1);
      maxX = Math.max(maxX, s.x0, s.x1);
      minY = Math.min(minY, s.y0, s.y1);
      maxY = Math.max(maxY, s.y0, s.y1);
    }
    if (minX === Infinity) { ctx.restore(); return; }
    const spanX = Math.max(1e-6, maxX - minX);
    const spanY = Math.max(1e-6, maxY - minY);
    // 3) Padding (5%)
    const padX = spanX * 0.05;
    const padY = spanY * 0.05;
    minX -= padX; maxX += padX; minY -= padY; maxY += padY;
    // 4) Escala uniforme para encajar dentro del canvas manteniendo aspecto
    const spanX2 = maxX - minX; const spanY2 = maxY - minY;
    const scale = Math.min(WIDTH / spanX2, HEIGHT / spanY2);
    // 5) Origen (minX,minY) -> (0,0) y centrado
    const offsetX = minX; const offsetY = minY;
    // 6) Dibujar
    for (const sRaw of segments) {
      const s = normalizeSeg(sRaw);
      if (!Number.isFinite(s.x0) || !Number.isFinite(s.y0) || !Number.isFinite(s.x1) || !Number.isFinite(s.y1)) {
        if (holdSegments) console.debug('[draw] skip invalid seg', sRaw);
        continue;
      }
      const x0 = ( (s.x0 - offsetX) * scale );
      const y0 = ( (s.y0 - offsetY) * scale );
      const x1 = ( (s.x1 - offsetX) * scale );
      const y1 = ( (s.y1 - offsetY) * scale );
      // Centrar en canvas
      const dx = (WIDTH - spanX2 * scale) * 0.5;
      const dy = (HEIGHT - spanY2 * scale) * 0.5;
      const i = Math.max(0, Math.min(255, s.intensity|0));
      ctx.strokeStyle = `rgba(0,255,120,${(i/255).toFixed(3)})`;
      ctx.beginPath(); ctx.moveTo(dx + x0, HEIGHT - (dy + y0)); ctx.lineTo(dx + x1, HEIGHT - (dy + y1)); ctx.stroke();
      if (holdSegments && (window as any).console) {
        // Log formato compacto: frame idx; coords originales y transformadas
        // Nota: se deja nivel debug para poder filtrarlo fácilmente
        try { console.debug('[draw] seg', {frame:s.frame, raw:{x0:s.x0,y0:s.y0,x1:s.x1,y1:s.y1,i:s.intensity}, bb:{minX,maxX,minY,maxY}, canvas:{x0:dx+x0,y0:HEIGHT-(dy+y0),x1:dx+x1,y1:HEIGHT-(dy+y1)}, scale}); } catch {}
      }
    }
    ctx.restore();
  };

  // Normaliza un segmento que puede venir como objeto {x0,y0,x1,y1,intensity,frame}
  // o como array [x0,y0,x1,y1,intensity,frame].
  const normalizeSeg = (s:any):Segment => {
    if (Array.isArray(s)) {
      return { x0:s[0], y0:s[1], x1:s[2], y1:s[3], intensity:s[4] ?? 0, frame:s[5] ?? 0 };
    }
    return s as Segment;
  };

  const animationLoop = useCallback(() => {
    if (status !== 'running' || demoMode) return; // paused/stopped or demo mode stops loop
    if (traceVectors) console.debug('[panel] animationLoop runFrame');
    // Update input state before running frame
    try {
      const snap = inputManager.update();
      const emuSvc: any = (globalEmu as any);
      if (emuSvc && typeof emuSvc.setInput === 'function') {
        emuSvc.setInput(snap.x, snap.y, snap.buttons);
      } else {
        // Fallback: legacy direct wasm export access
        const direct = (emuSvc?.emu) || (window as any).emu;
        if (direct && typeof direct.set_input_state === 'function') {
          direct.set_input_state(snap.x, snap.y, snap.buttons);
        }
      }
    } catch(e){ /* non-fatal */ }
  // Throttle: limit instructions per visual frame to approximate target CPU rate
  globalEmu.runFrame(instrBudget | 0);
    const regs = globalEmu.registers(); const m = globalEmu.metrics();
  if (m) { const cf=(m as any).cycle_frame as number|undefined; const bf=(m as any).bios_frame as number|undefined; if (typeof cf==='number'){ setCycleFrame(cf); if (cf>frameCount) setFrameCount(cf);} if (typeof bf==='number') setBiosFrame(bf);} 
    if (regs) { if (!m) setFrameCount(regs.frame_count); if ((regs as any).bios_frame && biosFrame===0) setBiosFrame((regs as any).bios_frame); if ((regs as any).cycle_frame && cycleFrame===0) setCycleFrame((regs as any).cycle_frame); }
    const metrics = globalEmu.metrics(); if (metrics) { const t1=(metrics as any).via_t1; const ifr=(metrics as any).via_ifr; const ier=(metrics as any).via_ier; const irq_line=(metrics as any).via_irq_line; const irq_count=(metrics as any).via_irq_count; if ([t1,ifr,ier,irq_line,irq_count].every(v=>v!==undefined)) { setViaMetrics({t1,ifr,ier,irq_line,irq_count}); } }
    let segs = globalEmu.getSegmentsShared();
    if (!segs.length) {
      if (traceVectors) console.debug('[panel] shared empty -> drain json/peek');
      segs = holdSegments ? globalEmu.peekSegmentsJson() : globalEmu.drainSegmentsJson();
      // Fallback 1: si métricas dicen que existen segmentos acumulados (total >0) pero ambos métodos vacíos, intentar peek.
      if (!segs.length) {
        const mm:any = metrics || globalEmu.metrics();
        if (mm && typeof mm.integrator_total_segments === 'number' && mm.integrator_total_segments > 0) {
          if (traceVectors) console.debug('[panel] metrics report segments total >0; trying peekSegmentsJson');
          segs = globalEmu.peekSegmentsJson();
        }
      }
      // Fallback 2: si seguimos sin nada y auto-demo debería haberse generado, invocar demoTriangle manual.
      if (!segs.length) {
        const anyEmu:any = (globalEmu as any).emu || (window as any).emu;
        try {
          const autoDemo = anyEmu?.auto_demo_enabled ? anyEmu.auto_demo_enabled() : true;
          if (autoDemo) {
            if (traceVectors) console.debug('[panel] invoking demo_triangle manual fallback');
            globalEmu.demoTriangle();
            segs = globalEmu.peekSegmentsJson();
          }
        } catch {/* ignore */}
      }
    }
    if (traceVectors) console.debug('[panel] segments fetched count=', segs.length);
    if (segs.length){ setSegmentsCount(segs.length); setLastSegments(segs);} if (showLoopWatch){ const lw=globalEmu.loopWatch(); if (lw.length) setLoopSamples(lw);} drawSegments(segs.length?segs:lastSegments); rafRef.current=requestAnimationFrame(animationLoop);
  }, [status, showLoopWatch, lastSegments, demoMode, frameCount, biosFrame, cycleFrame, traceVectors, instrBudget]);

  // Inline small control rendering helpers (inserción tardía para no reordenar demasiado código existente)
  const renderInstrBudgetControl = () => (
    <label style={{display:'flex',alignItems:'center',gap:4,fontSize:11}} title='Instrucciones máximas por frame visual (~1.5M/s objetivo)'>
      Instr/frame
      <input type='number' min={1000} max={200000} step={1000} value={instrBudget} onChange={e=>setInstrBudget(Math.max(1000,Math.min(200000, parseInt(e.target.value)||25000)))} style={{width:80,background:'#111',color:'#ddd',border:'1px solid #333'}} />
    </label>
  );

  // Init
  useEffect(()=>{ let cancelled=false; (async()=>{ try { await globalEmu.init(); await (globalEmu as any).ensureBios?.({ urlCandidates:[biosUrl,'bios.bin','/bios.bin','/core/src/bios/bios.bin'] }); } catch(e){ console.error('Emulator init/BIOS load failed', e); } if (!cancelled){ setStatus('stopped'); } })(); return ()=>{ cancelled=true; if (rafRef.current) cancelAnimationFrame(rafRef.current); }; }, []); // eslint-disable-line

  // (listener moved below utility declarations)

  useEffect(()=>{ fixedCanvasInit(); }, [fixedCanvasInit]);
  useEffect(()=>{ if (status==='running' && !demoMode){ if (!rafRef.current) rafRef.current=requestAnimationFrame(animationLoop); } else if (rafRef.current){ cancelAnimationFrame(rafRef.current); rafRef.current=null; } }, [status, animationLoop, demoMode]);

  // Demo mode effect with retry logic before falling back
  useEffect(() => {
    let cancelled = false;
    const attemptDemo = async () => {
      setDemoStatus('waiting');
      // Up to 6 attempts (~6 * 120ms) to allow wasm init & segment generation
      for (let i=0;i<6 && !cancelled;i++) {
        // Wait for emulator initialization
        if (!globalEmu.registers()) {
          await new Promise(r=>setTimeout(r, 120));
          continue;
        }
        try {
          const segs = globalEmu.demoTriangle();
          if (segs.length) {
            setLastSegments(segs);
            drawSegments(segs);
            setDemoStatus('ok');
            return;
          }
        } catch (e) {
          console.warn('demoTriangle attempt failed', e);
        }
        await new Promise(r=>setTimeout(r, 120));
      }
      if (cancelled) return;
      // Fallback
      const fallback: Segment[] = [
        { x0: -0.5, y0: -0.5, x1: 0.5, y1: -0.5, intensity: 255, frame: 0 },
        { x0: 0.5, y0: -0.5, x1: 0, y1: 0.6, intensity: 255, frame: 0 },
        { x0: 0, y0: 0.6, x1: -0.5, y1: -0.5, intensity: 255, frame: 0 },
      ];
      setLastSegments(fallback);
      drawSegments(fallback);
      setDemoStatus('fallback');
      pushToast('Demo fallback triangle','info');
    };
    if (demoMode) {
      attemptDemo();
    } else {
      setDemoStatus('idle');
    }
    return () => { cancelled = true; };
  }, [demoMode]);

  const performFullReset = () => {
    globalEmu.reset();
    setFrameCount(0); setBiosFrame(0); setCycleFrame(0); setVecEvents([]); lastVecEventsRef.current=[]; setViaMetrics(null); setLoopSamples([]); setSegmentsCount(0); setLastSegments([]);
  };
  const onPlay = () => setStatus('running');
  const onPause = () => setStatus('paused');
  const onStop = () => { // Stop: full reset & remain stopped
    performFullReset();
    setStatus('stopped');
  };
  const onReset = () => { performFullReset(); if (status==='running') setStatus('running'); };

  // Toast helper
  const pushToast = (msg:string, kind:'info'|'error'='info') => {
    const id = toastIdRef.current++;
    setToasts(t => [...t, { id, msg, kind, ts: Date.now() }]);
    setTimeout(()=>{ setToasts(t => t.filter(x=>x.id!==id)); }, 4000);
  };

  // Parse the currently entered base (always forced to 0x0000 for Vectrex cartridge images now).
  const parseBase = () => {
    const v = baseAddrHex.trim();
    let n = parseInt(v.replace(/^0x/i,''),16);
    if (!Number.isFinite(n) || isNaN(n)) n = 0x0000;
    n &= 0xFFFF;
    return n;
  };

  // Detect a Vectrex cartridge header at offset 0 (very lightweight heuristic: bytes 0 & 1 are 0, reset vector near end non-zero).
  // Classic Vectrex header layout: 0x00.. vectors; we just gate on first two bytes zero & length > 0x800.
  const detectVectrexHeader = (bytes: Uint8Array) => {
    if (!bytes || bytes.length < 0x20) return false;
    if (bytes[0] !== 0x00 || bytes[1] !== 0x00) return false; // reset vector high/low may be later; simple sentinel
    return true;
  };

  const enforceBaseForHeader = (bytes: Uint8Array, requestedBase: number) => {
    if (detectVectrexHeader(bytes) && requestedBase !== 0x0000) {
      pushToast('Cartridge header detected – forcing base 0000 (was '+requestedBase.toString(16)+')','info');
      setBaseAddrHex('0000');
      return 0x0000;
    }
    return requestedBase;
  };
  const saveLastBin = (info:{path?:string; size?:number; base:number; bytes?:Uint8Array}) => {
    setLastBinInfo(info);
    try { const json = JSON.stringify({ path:info.path, size:info.size, base:info.base }); localStorage.setItem('emu_last_bin_meta', json); } catch {}
  };
  useEffect(()=>{ // restore meta & migrate legacy 0xC000 default
    try {
      const meta = localStorage.getItem('emu_last_bin_meta');
      if (meta){
        const m = JSON.parse(meta);
        if (m.base !== undefined) {
          if (m.base === 0xC000) {
            // If the loaded binary appears to have a Vectrex header at offset 0, force migrate to 0000
            setBaseAddrHex('0000');
            pushToast('Migrated base from C000 to 0000 (fixed cart origin)', 'info');
          } else {
            setBaseAddrHex(m.base.toString(16).toUpperCase().padStart(4,'0'));
          }
        }
      }
    } catch{}
  }, []);

  // Listen for compiledBin events from main (run:compile completion) and load into WASM
  useEffect(()=>{
    const w:any = window as any;
    if (!w.electronAPI?.onCompiledBin) return;
    const handler = (payload:{ base64:string; size:number; binPath:string }) => {
      try {
        const bytes = Uint8Array.from(atob(payload.base64), c=>c.charCodeAt(0));
        let base = parseBase();
        base = enforceBaseForHeader(bytes, base);
        globalEmu.loadProgram(bytes, base);
        saveLastBin({ path: payload.binPath, size: bytes.length, base, bytes });
        performFullReset();
        setStatus('running');
        pushToast('Compiled & loaded '+(payload.binPath||'binary'));
      } catch(e){ console.warn('onCompiledBin load failed', e); pushToast('Compiled bin load failed','error'); }
    };
    w.electronAPI.onCompiledBin(handler);
    return () => { try { w.electronAPI?.onCompiledBin?.(()=>{}); } catch {} };
  }, []);

  // Persist selected source
  useEffect(()=>{ if (selectedSource) { try { localStorage.setItem('emu_selected_source', selectedSource); } catch {} } }, [selectedSource]);
  useEffect(()=>{ if (!selectedSource) { try { const s = localStorage.getItem('emu_selected_source'); if (s) setSelectedSource(s); } catch {} } }, [selectedSource]);

  // Load available sources via IPC once
  useEffect(()=>{
    const w:any = window as any;
    (async()=>{
      if (!w.electronAPI?.listSources) return;
      try { const res = await w.electronAPI.listSources({ limit: 400 }); if (res?.ok && res.sources){ const slim = res.sources.map((s:any)=>({path:s.path, kind:s.kind})); setSourceList(slim); if (!selectedSource && slim.length) setSelectedSource(slim[0].path); } } catch(e){ console.warn('listSources failed', e); }
    })();
  }, [selectedSource]);

  // Attempt to resolve the active document (.vpy or .asm) from the editor store with multiple fallbacks
  // Replace resolveActiveDoc with store-based resolution
  const resolveActiveDoc = (): any | null => {
    if (activeDoc) return activeDoc;
    if (editorDocuments.length === 1) return editorDocuments[0];
    let doc = editorDocuments.find(d => /\.vpy$/i.test(d.uri)) || editorDocuments.find(d => /\.asm$/i.test(d.uri));
    return doc || null;
  };

  // attemptAutoDetectExample removed

  // Build pipeline: request electron to build current active .vpy, receive binary, load, reset & run
  const onBuild = async () => {
    const w:any = window as any;
    try {
      // Determine effective source path (priority: dropdown then active doc)
      const activeDocResolved = resolveActiveDoc();
      const activeUri: string|undefined = activeDocResolved?.uri;
      const effectiveUri = (selectedSource || activeUri || '').trim();
      if (!effectiveUri) {
        pushToast('No source selected – choose from Source dropdown or open a file','error');
        return;
      }
      const isAsm = /\.asm$/i.test(effectiveUri);
      // Decide diskPath (activeDoc may not correspond to effectiveUri if user picked dropdown/manual)
      let diskPath = effectiveUri;
  if (activeDocResolved && activeUri === effectiveUri && activeDocResolved.diskPath) diskPath = activeDocResolved.diskPath;
      // If it's a .vpy source, invoke runCompile (compiles and auto-loads into legacy emulator backend)
      if (!isAsm && w.electronAPI?.runCompile) {
        const normalized = diskPath.replace(/^file:\/+/, '');
  const saveIfDirty = (activeDocResolved && activeUri === effectiveUri && activeDocResolved.dirty) ? { content: activeDocResolved.content, expectedMTime: activeDocResolved.mtime } : undefined;
        const res = await w.electronAPI.runCompile({ path: normalized, saveIfDirty, autoStart: true });
        if (!res || !res.ok) { console.warn('runCompile failed', res); pushToast('Compile failed','error'); setStatus('stopped'); return; }
        // The run:compile path already loaded binary into emulator (TypeScript backend). If WASM backend expects manual load, we could fetch the produced .bin.
        // Attempt to locate produced .bin (replace .vpy with .bin) and load into WASM service.
        const binGuess = normalized.replace(/\.[^.]+$/, '.bin');
        // Attempt secure IPC-based binary read first (avoids CSP file:// restrictions)
        try {
          if (w.files?.readFileBin) {
            const binRes = await w.files.readFileBin(binGuess);
            if (binRes && !binRes.error && binRes.base64) {
              const bytes = Uint8Array.from(atob(binRes.base64), c=>c.charCodeAt(0));
              let base = parseBase();
              base = enforceBaseForHeader(bytes, base);
              globalEmu.loadProgram(bytes, base);
              saveLastBin({ path: binGuess, size: bytes.length, base, bytes });
              pushToast('Loaded '+binGuess);
            } else {
              console.warn('IPC readFileBin failed or returned error', binRes);
            }
          } else {
            // Fallback to fetch (may be blocked by CSP if using file://)
            const resp = await fetch(binGuess);
            if (resp.ok) {
              const buf=new Uint8Array(await resp.arrayBuffer());
              let base = parseBase();
              base = enforceBaseForHeader(buf, base);
              globalEmu.loadProgram(buf, base);
              saveLastBin({ path:binGuess, size:buf.length, base, bytes:buf });
              pushToast('Loaded '+binGuess);
            }
          }
        } catch(e) {
          console.warn('Binary load (post-compile) failed', e);
          pushToast('Post-compile load failed','error');
        }
      } else if (isAsm && w.electronAPI?.emuAssemble) {
        // Assemble raw .asm directly via lwasm -> load into WASM
        const normalized = diskPath.replace(/^file:\/+/, '');
        const assembleRes = await w.electronAPI.emuAssemble({ asmPath: normalized });
        if (!assembleRes || !assembleRes.ok || !assembleRes.base64) { console.warn('Assembly failed', assembleRes); pushToast('Assembly failed','error'); setStatus('stopped'); return; }
        const bytes = Uint8Array.from(atob(assembleRes.base64), c=>c.charCodeAt(0));
  let base = parseBase();
  base = enforceBaseForHeader(bytes, base);
  globalEmu.loadProgram(bytes, base);
  saveLastBin({ path: normalized, size: bytes.length, base, bytes });
        pushToast('Assembled & loaded');
      } else {
        console.warn('No build mechanism detected (runCompile / emuAssemble missing)'); pushToast('No build mechanism','error'); setStatus('stopped'); return;
      }
      // After loading binary, reset & run
      performFullReset();
      setStatus('running');
    } catch(e){ console.error('Build pipeline failed', e); setStatus('stopped'); }
  };
  // (Replaced by performFullReset / onReset earlier)
  const onClearStats = () => { (globalEmu as any).resetStats?.(); };

  // Manual load of arbitrary .bin cartridge (always mapped at 0x0000 now)
  const onLoadBin = async () => {
    const w:any = window as any;
    try {
      if (w.files?.openBin) {
        const res = await w.files.openBin();
        if (!res || res.error || !res.base64) { console.warn('openBin canceled or failed', res); return; }
        const bytes = Uint8Array.from(atob(res.base64), c=>c.charCodeAt(0));
  let base = parseBase();
  base = enforceBaseForHeader(bytes, base);
  globalEmu.loadProgram(bytes, base);
  performFullReset();
  saveLastBin({ path: res.path, size: bytes.length, base, bytes });
        pushToast('Loaded '+(res.path||'binary'));
        setStatus('running');
        return;
      }
      // Fallback: generic file input (web build scenario)
      const input = document.createElement('input');
      input.type='file'; input.accept='.bin';
      input.onchange = () => {
        const f = input.files?.[0]; if (!f) return;
        const reader = new FileReader();
        reader.onload = () => {
          const arr = new Uint8Array(reader.result as ArrayBuffer);
          let base = parseBase();
          base = enforceBaseForHeader(arr, base);
          globalEmu.loadProgram(arr, base);
          performFullReset();
          saveLastBin({ size: arr.length, base, bytes:arr });
          pushToast('Loaded file');
          setStatus('running');
        };
        reader.readAsArrayBuffer(f);
      };
      input.click();
    } catch (e){ console.error('Manual load failed', e); }
  };

  const onQuickReload = () => {
    if (!lastBinInfo?.bytes) { pushToast('No previous binary','error'); return; }
    globalEmu.loadProgram(lastBinInfo.bytes, lastBinInfo.base);
    performFullReset();
    setStatus('running');
    pushToast('Reloaded last binary');
  };

  // Apply trace flag to emulator service
  useEffect(() => { (globalEmu as any).enableTrace?.(traceVectors); try { localStorage.setItem('emu_trace_vectors', traceVectors?'1':'0'); } catch {} }, [traceVectors]);

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', padding:8, boxSizing:'border-box', fontFamily:'monospace', fontSize:12}}>
      <div style={{display:'flex', alignItems:'center', gap:12, flexWrap:'wrap'}}>
        {(() => { const statusColor = status==='running' ? '#0f0' : status==='paused' ? '#fa0' : '#f55'; return (<span>Status: <span style={{color:statusColor}}>{status}</span></span>); })()}
        <span style={{fontSize:11, padding:'2px 6px', borderRadius:4, background: globalEmu.isBiosLoaded()? '#1d4d1d':'#4d1d1d', color:'#eee'}}>BIOS {globalEmu.isBiosLoaded()? 'OK':'…'}</span>
        <label style={{display:'flex', alignItems:'center', gap:4}}>Base
          <input value={baseAddrHex} onChange={e=> setBaseAddrHex(e.target.value.toUpperCase())} style={{width:60, background:'#111', color:'#ccc', border:'1px solid #333', fontSize:11, padding:'1px 4px'}} />
        </label>
  {lastBinInfo && <span style={{opacity:0.7}}>Last: {(lastBinInfo.path? lastBinInfo.path.split(/[\\\/]/).pop(): 'binary')} @ {'0x'+lastBinInfo.base.toString(16).toUpperCase()} ({lastBinInfo.size}B)</span>}
  {activeDoc && <span style={{opacity:0.6}}>Active: {activeDoc.diskPath ? activeDoc.diskPath : activeDoc.uri}</span>}
        <span>Frames: {frameCount}</span>
        {sourceList.length>0 && (
          <label style={{display:'flex', alignItems:'center', gap:4}}>Source
            <select value={selectedSource} onChange={e=> setSelectedSource(e.target.value)} style={{background:'#111', color:'#ccc', border:'1px solid #333', fontSize:11}}>
              {sourceList.map(s => (<option key={s.path} value={s.path}>{s.kind}:{s.path.replace(/^.*[\\\/]/,'')}</option>))}
            </select>
          </label>
        )}
        <span>BIOS frames: {biosFrame}</span>
        <span>(legacy count: {frameCount})</span>
        {cycleFrame===0 && globalEmu.isBiosLoaded() && <span style={{color:'#fa0'}}>No frame boundary yet (WAIT_RECAL?)</span>}
        {!globalEmu.isBiosLoaded() && <span style={{color:'#f55'}}>BIOS not loaded</span>}
        <span>Segments(last frame): {segmentsCount}</span>
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input type='checkbox' checked={showLoopWatch} onChange={e=> setShowLoopWatch(e.target.checked)} /> loop watch
        </label>
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input type='checkbox' checked={traceVectors} onChange={e=> setTraceVectors(e.target.checked)} /> trace vectors
        </label>
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input type='checkbox' checked={holdSegments} onChange={e=> { const v=e.target.checked; setHoldSegments(v); try { localStorage.setItem('emu_hold_segments', v?'1':'0'); } catch{} }} /> hold segs
        </label>
        <div style={{marginLeft:'auto', display:'flex', gap:6}}>
          <button style={btn} onClick={onBuild} title='Compile active .vpy or assemble .asm, load & run'>Build & Run</button>
          <button style={btn} onClick={onLoadBin} title='Load arbitrary .bin at $0000 and run'>Load .bin</button>
          {status !== 'running' && <button style={btn} onClick={onPlay}>{status==='paused' ? 'Resume':'Play'}</button>}
          {status === 'running' && <button style={btn} onClick={onPause}>Pause</button>}
          <button style={btn} onClick={onStop} title='Stop (same as pause for now)'>Stop</button>
          <button style={btn} onClick={onReset}>Reset</button>
          <button style={btn} onClick={onClearStats} title='Clear counters without full reset'>Clear Stats</button>
          <button style={btn} disabled={!lastBinInfo?.bytes} onClick={onQuickReload} title='Reload last binary at same base'>Reload</button>
          <button style={{...btn, background: demoMode? '#264':'#1d1d1d', color: demoMode? '#8ef':'#ddd'}} onClick={()=> setDemoMode(d=>!d)}>{demoMode? 'Demo: ON':'Demo: OFF'}</button>
        </div>
      </div>
      <div style={{flex:1, display:'flex', flexDirection:'column', marginTop:8}}>
        <div style={{display:'flex', justifyContent:'center'}}>
          <div style={{position:'relative'}}>
            <canvas ref={canvasRef} style={{border:'1px solid #333', background:'#000', borderRadius:4, width:300, height:400}} />
            {demoMode && demoStatus!=='ok' && lastSegments.length===0 && (
              <div style={{position:'absolute', inset:0, display:'flex', alignItems:'center', justifyContent:'center', color:'#68f', fontSize:12, textAlign:'center', padding:8, background:'rgba(0,0,0,0.4)'}}>
                {demoStatus==='waiting' && 'Preparing demo… (building wasm or awaiting segments)'}
                {demoStatus==='fallback' && 'Fallback triangle in use (wasm demo returned no segments). Rebuild wasm to restore native demo.'}
              </div>
            )}
          </div>
        </div>
        <div style={{marginTop:12, display:'grid', gridTemplateColumns:'repeat(auto-fit,minmax(220px,1fr))', gap:12, alignItems:'start'}}>
          <div style={statBox}>
            <div style={statTitle}>Recent Events ({vecEvents.length})</div>
            <ul style={{margin:0, paddingLeft:16, listStyle:'none', maxHeight:140, overflow:'auto'}}>
              {vecEvents.slice(-20).map((e,i)=>(<li key={i}>{e.kind}@{e.pc.toString(16)}</li>))}
              {vecEvents.length===0 && <li style={{opacity:0.5}}>(none)</li>}
            </ul>
          </div>
          <div style={statBox}>
            <div style={statTitle}>VIA</div>
            {viaMetrics ? (<>
              <div>T1: {viaMetrics.t1}</div>
              <div>IFR: 0x{Number(viaMetrics.ifr).toString(16).padStart(2,'0')} IER: 0x{Number(viaMetrics.ier).toString(16).padStart(2,'0')}</div>
              <div>IRQ: {viaMetrics.irq_line ? 'HIGH':'low'} (#{viaMetrics.irq_count})</div>
            </>) : <div style={{opacity:0.5}}>(no metrics)</div>}
          </div>
          {showLoopWatch && (
            <div style={statBox}>
              <div style={statTitle}>Loop Watch ({loopSamples.length})</div>
              <div style={{maxHeight:140, overflow:'auto'}}>
                <table style={{width:'100%', borderCollapse:'collapse'}}>
                  <thead><tr><th>PC</th><th>A</th><th>B</th><th>X</th><th>Y</th><th>U</th><th>IFR</th><th>IER</th><th>ACR</th><th>PCR</th><th>cyc</th></tr></thead>
                  <tbody>{loopSamples.slice(-16).map((s,i)=>(<tr key={i}><td>{s.pc?.toString(16)}</td><td>{s.a?.toString(16)}</td><td>{s.b?.toString(16)}</td><td>{s.x?.toString(16)}</td><td>{s.y?.toString(16)}</td><td>{s.u?.toString(16)}</td><td>{(s.via_ifr??0).toString(16).padStart(2,'0')}</td><td>{(s.via_ier??0).toString(16).padStart(2,'0')}</td><td>{(s.via_acr??0).toString(16).padStart(2,'0')}</td><td>{(s.via_pcr??0).toString(16).padStart(2,'0')}</td><td>{s.cycles}</td></tr>))}</tbody>
                </table>
              </div>
            </div>
          )}
          <div style={statBox}>
            <div style={statTitle}>Hot 0x00 / 0xFF PCs</div>
            {(() => { const m:any = globalEmu.metrics(); if (!m) return <div style={{opacity:0.5}}>(no metrics)</div>; const h00=(m.hot00||[]) as [number,number][]; const hff=(m.hotff||[]) as [number,number][]; if (!h00.length && !hff.length) return <div style={{opacity:0.5}}>(none)</div>; return (
              <div style={{display:'flex', gap:12}}>
                <div style={{flex:1}}>
                  <div style={{fontWeight:'bold', color:'#ccc'}}>0x00</div>
                  <table style={{width:'100%', borderCollapse:'collapse'}}><thead><tr><th style={{textAlign:'left'}}>PC</th><th style={{textAlign:'right'}}>Count</th></tr></thead><tbody>{h00.map(([pc,c]) => (<tr key={'00_'+pc}><td>{'0x'+pc.toString(16).padStart(4,'0')}</td><td style={{textAlign:'right'}}>{c}</td></tr>))}</tbody></table>
                </div>
                <div style={{flex:1}}>
                  <div style={{fontWeight:'bold', color:'#ccc'}}>0xFF</div>
                  <table style={{width:'100%', borderCollapse:'collapse'}}><thead><tr><th style={{textAlign:'left'}}>PC</th><th style={{textAlign:'right'}}>Count</th></tr></thead><tbody>{hff.map(([pc,c]) => (<tr key={'ff_'+pc}><td>{'0x'+pc.toString(16).padStart(4,'0')}</td><td style={{textAlign:'right'}}>{c}</td></tr>))}</tbody></table>
                </div>
              </div>
            ); })()}
          </div>
        </div>
        <div style={{marginTop:8, fontSize:11, color:'#777'}}>
          {demoMode ? 'Demo triangle (static) — disable to resume live BIOS execution.' : 'Running integrator backend; heuristic MOVETO_D vector emission (early prototype).'}
        </div>
        {/* Toasts */}
        <div style={{position:'absolute', top:8, right:8, display:'flex', flexDirection:'column', gap:6, pointerEvents:'none'}}>
          {toasts.map(t => (
            <div key={t.id} style={{background: t.kind==='error'? '#552222':'#223344', color:'#ddd', padding:'4px 8px', border:'1px solid #444', borderRadius:4, fontSize:11, boxShadow:'0 2px 4px rgba(0,0,0,0.4)'}}>{t.msg}</div>
          ))}
        </div>
      </div>
    </div>
  );
};

const statBox: React.CSSProperties = { fontFamily:'monospace', fontSize:11, lineHeight:1.4, padding:'6px 8px', background:'#111', border:'1px solid #333', borderRadius:4, minHeight:80, display:'flex', flexDirection:'column', gap:4 };
const statTitle: React.CSSProperties = { fontWeight:'bold', color:'#8cf', marginBottom:2 };
const btn: React.CSSProperties = { background:'#1d1d1d', color:'#ddd', border:'1px solid #444', padding:'2px 8px', fontSize:11, cursor:'pointer', borderRadius:3 };
