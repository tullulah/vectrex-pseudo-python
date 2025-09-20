import React, { useEffect, useRef, useState, useCallback } from 'react';
import { emuCore } from '../emulatorCoreSingleton';
import type { Segment } from '../emulatorCore';
// VectorEvent mínimo para compatibilidad rápida
type VectorEvent = { kind:string; pc:number };
import { useEmulatorStore } from '../state/emulatorStore';

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
  const rafRef = useRef<number | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  // Fixed size canvas (legacy style) -> no ResizeObserver needed
  const resizeObsRef = useRef<ResizeObserver | null>(null);
  // Keep last vector events in a ref to avoid adding vecEvents as a dependency of animationLoop (preventing infinite re-creations)
  const lastVecEventsRef = useRef<VectorEvent[]>([]);

  const fixedCanvasInit = useCallback(() => {
    const canvas = canvasRef.current; if (!canvas) return;
    const dpr = window.devicePixelRatio || 1;
    // 3:4 aspect ratio (width:height). Choose height 400 -> width 300.
    const WIDTH = 300; const HEIGHT = 400;
    canvas.width = WIDTH * dpr;
    canvas.height = HEIGHT * dpr;
    canvas.style.width = WIDTH + 'px';
    canvas.style.height = HEIGHT + 'px';
  }, []);

  const drawVectors = (events: VectorEvent[]) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    ctx.save();
    ctx.fillStyle = 'black';
    ctx.fillRect(0,0,canvas.width, canvas.height);
    ctx.strokeStyle = 'lime';
    ctx.lineWidth = 1;
    const centerX = canvas.width/2;
    const centerY = canvas.height/2;
    const draws = events.filter(e => e.kind === 'draw_vl');
    const total = draws.length;
    ctx.fillStyle = '#0f0';
    ctx.font = `${12 * (window.devicePixelRatio||1)}px monospace`;
    ctx.textAlign = 'center';
  if (!emuCore.isBiosLoaded()) {
      ctx.fillText('BIOS missing', centerX, centerY - 10);
      ctx.fillText('Place bios.bin in /ide/frontend/public', centerX, centerY + 10);
    } else if (!total) {
      ctx.fillText('Running... no vector events yet', centerX, centerY);
    }
    for (let i=0;i<total;i++) {
      const angle = (i/Math.max(1,total)) * Math.PI*2;
      const len = (canvas.width/2) * 0.6 * (0.3 + ((i % 17)/17));
      const x = centerX + Math.cos(angle)*len;
      const y = centerY + Math.sin(angle)*len;
      ctx.beginPath();
      ctx.moveTo(centerX, centerY);
      ctx.lineTo(x,y);
      ctx.stroke();
    }
    ctx.restore();
  };

  const drawSegments = (segments: Segment[]) => {
    const canvas = canvasRef.current; if (!canvas) return;
    const ctx = canvas.getContext('2d'); if (!ctx) return;
    const dpr = window.devicePixelRatio || 1;
    ctx.save();
    ctx.scale(dpr,dpr);
    const WIDTH = canvas.width / dpr;
    const HEIGHT = canvas.height / dpr;
    ctx.fillStyle = 'black';
    ctx.fillRect(0,0,WIDTH,HEIGHT);
    if (!segments.length) {
      ctx.fillStyle = '#0f0';
      ctx.font = '12px monospace';
      ctx.textAlign = 'center';
      const centerX = WIDTH/2, centerY = HEIGHT/2;
  if (!emuCore.isBiosLoaded()) {
        ctx.fillText('BIOS missing', centerX, centerY - 10);
        ctx.fillText('Place bios.bin in /ide/frontend/public', centerX, centerY + 10);
      } else {
        ctx.fillText('No segments yet', centerX, centerY);
      }
      ctx.restore();
      return;
    }
    // Transform from approx -1..1 space to canvas
    const scale = Math.min(WIDTH, HEIGHT)/2 * 0.95;
    const cx = WIDTH/2;
    const cy = HEIGHT/2;
    ctx.lineWidth = 1;
    for (const s of segments) {
      const x0 = cx + s.x0 * scale;
      const y0 = cy - s.y0 * scale;
      const x1 = cx + s.x1 * scale;
      const y1 = cy - s.y1 * scale;
      const i = Math.max(0, Math.min(255, s.intensity|0));
      ctx.strokeStyle = `rgba(0,255,120,${(i/255).toFixed(3)})`;
      ctx.beginPath();
      ctx.moveTo(x0,y0);
      ctx.lineTo(x1,y1);
      ctx.stroke();
    }
    ctx.restore();
  };

  const animationLoop = useCallback(() => {
    if (status !== 'running') return; // paused/stopped halts loop
  emuCore.runFrame();
  const regs = emuCore.registers();
  const m = emuCore.metrics();
    if (m) {
      // Prefer authoritative cycle_frame (timing counter) if present; fallback to registers.frame_count
      const cf = (m as any).cycle_frame as number | undefined;
      const bf = (m as any).bios_frame as number | undefined;
      if (typeof cf === 'number') { setCycleFrame(cf); if (cf > frameCount) setFrameCount(cf); }
      if (typeof bf === 'number') setBiosFrame(bf);
    }
    if (regs) {
      if (!m) setFrameCount(regs.frame_count); // legacy fallback
      if ((regs as any).bios_frame && biosFrame === 0) setBiosFrame((regs as any).bios_frame);
      if ((regs as any).cycle_frame && cycleFrame === 0) setCycleFrame((regs as any).cycle_frame);
    }
  const metrics = emuCore.metrics();
    if (metrics) {
      const t1 = (metrics as any).via_t1;
      const ifr = (metrics as any).via_ifr;
      const ier = (metrics as any).via_ier;
      const irq_line = (metrics as any).via_irq_line;
      const irq_count = (metrics as any).via_irq_count;
      if (t1 !== undefined && ifr !== undefined && ier !== undefined && irq_line !== undefined && irq_count !== undefined) {
        setViaMetrics({ t1, ifr, ier, irq_line, irq_count });
      }
    }
    // Prefer shared memory segment export; fallback to JSON drain if unavailable
  let segs = emuCore.getSegmentsShared();
    if (!segs.length) {
      // try drain JSON once per frame
  segs = emuCore.drainSegmentsJson ? emuCore.drainSegmentsJson() : segs;
    }
    if (segs.length) {
      setSegmentsCount(segs.length);
      setLastSegments(segs);
    }
    if (showLoopWatch) {
  const lw = emuCore.loopWatch ? emuCore.loopWatch() : [];
      if (lw.length) setLoopSamples(lw);
    }
    drawSegments(segs.length ? segs : lastSegments);
    rafRef.current = requestAnimationFrame(animationLoop);
  }, [status, showLoopWatch, drawSegments, lastSegments]);

  // One-time init effect (do not depend on animationLoop to avoid re-running)
  useEffect(() => {
    let cancelled = false;
    (async () => {
  if (emuCore.registers()) return; // already initialized
      try {
  await emuCore.init();
        const biosPaths = ['bios.bin','/bios.bin','/core/src/bios/bios.bin'];
        for (const p of biosPaths) {
          if (emuCore.isBiosLoaded()) break;
          try {
            const resp = await fetch(p);
            if (resp.ok) {
              const buf = new Uint8Array(await resp.arrayBuffer());
              emuCore.loadBios(buf);
              break;
            }
          } catch {/*ignore*/}
        }
        if (!cancelled) {
          if (status !== 'running') setStatus('running');
        }
      } catch (e) {
        console.error('Emulator init failed', e);
        setStatus('stopped');
      }
    })();
    return () => { cancelled = true; if (rafRef.current) cancelAnimationFrame(rafRef.current); };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Fixed init sizing only
  useEffect(() => { fixedCanvasInit(); }, [fixedCanvasInit]);

  useEffect(() => {
    if (status === 'running') {
      if (!rafRef.current) rafRef.current = requestAnimationFrame(animationLoop);
    } else if (rafRef.current) {
      cancelAnimationFrame(rafRef.current); rafRef.current = null;
    }
  }, [status, animationLoop]);

  const onPlay = () => setStatus('running');
  const onPause = () => setStatus('stopped');
  const onReset = () => { emuCore.reset(); setFrameCount(0); setVecEvents([]); lastVecEventsRef.current = []; drawVectors([]); };

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', padding:8, boxSizing:'border-box', fontFamily:'monospace', fontSize:12}}>
      <div style={{display:'flex', alignItems:'center', gap:12}}>
        <span>Status: {status}</span>
        <span>Frames: {frameCount}</span>
  <span>BIOS frames: {biosFrame}</span>
  <span>(legacy count: {frameCount})</span>
  {cycleFrame===0 && emuCore.isBiosLoaded() &&
          <span style={{color:'#fa0'}}>No frame boundary reached yet (check WAIT_RECAL instrumentation)</span>}
  {!emuCore.isBiosLoaded() &&
          <span style={{color:'#f55'}}>BIOS not loaded (place bios.bin in /ide/frontend/public)</span>}
        <span>Last events: {vecEvents.length}</span>
  <span>PC: {(() => { const r=emuCore.registers(); return r? r.pc.toString(16):'--'; })()}</span>
  <span>Cycles: {(() => { const r=emuCore.registers(); return r? r.cycles:'--'; })()}</span>
  <span>Bios: {emuCore.isBiosLoaded() ? 'loaded' : 'missing'}</span>
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input type='checkbox' checked={showLoopWatch} onChange={e=> setShowLoopWatch(e.target.checked)} /> loop watch
        </label>
        <div style={{marginLeft:'auto', display:'flex', gap:6}}>
          <button style={btn} onClick={()=>{
            // Attempt to call global commandExec (menu system) or electron API if present
            try {
              const w:any = window;
              if (w.commandExec) { w.commandExec('build.build'); }
              else if (w.electronAPI?.sendCommand) { w.electronAPI.sendCommand('build.build'); }
              else { console.log('[build] commandExec not available'); }
            } catch(e){ console.warn('Build command failed', e); }
          }}>Build</button>
          {status !== 'running' && <button style={btn} onClick={onPlay}>Play</button>}
          {status === 'running' && <button style={btn} onClick={onPause}>Pause</button>}
          <button style={btn} onClick={onReset}>Reset</button>
        </div>
      </div>
      <div style={{flex:1, display:'flex', gap:16, marginTop:8, minHeight:240}}>
        <div style={{flex:'0 0 auto', display:'flex', alignItems:'flex-start', justifyContent:'center'}}>
          <canvas ref={canvasRef} style={{border:'1px solid #333', background:'#000', borderRadius:4, width:300, height:400}} />
        </div>
        <div style={{display:'flex', flexDirection:'column', flex:1, overflow:'auto', gap:8}}>
          <ul style={{margin:0, paddingLeft:16, listStyle:'none'}}>
            {vecEvents.slice(-12).map((e,i)=>(<li key={i}>{e.kind}@{e.pc.toString(16)}</li>))}
          </ul>
          <div style={{fontFamily:'monospace', fontSize:11, lineHeight:1.4, padding:'4px 8px', background:'#111', border:'1px solid #333', borderRadius:4}}>
            <div style={{fontWeight:'bold', color:'#8cf'}}>VIA</div>
            {viaMetrics ? (
              <>
                <div>T1: {viaMetrics.t1}</div>
                <div>IFR: 0x{Number(viaMetrics.ifr).toString(16).padStart(2,'0')} IER: 0x{Number(viaMetrics.ier).toString(16).padStart(2,'0')}</div>
                <div>IRQ line: {viaMetrics.irq_line ? 'HIGH' : 'low'} (count {viaMetrics.irq_count})</div>
              </>
            ) : <div>(no metrics)</div>}
          </div>
          {showLoopWatch && (
            <div style={{fontFamily:'monospace', fontSize:11, lineHeight:1.3, padding:'4px 8px', background:'#111', border:'1px solid #333', borderRadius:4}}>
              <div style={{fontWeight:'bold', color:'#fc8'}}>Loop Watch ({loopSamples.length})</div>
              <div style={{maxHeight:140, overflow:'auto'}}>
                <table style={{width:'100%', borderCollapse:'collapse'}}>
                  <thead>
                    <tr style={{textAlign:'left'}}>
                      <th>PC</th><th>A</th><th>B</th><th>X</th><th>Y</th><th>U</th><th>IFR</th><th>IER</th><th>ACR</th><th>PCR</th><th>cyc</th>
                    </tr>
                  </thead>
                  <tbody>
                    {loopSamples.slice(-16).map((s,i)=>(
                      <tr key={i}>
                        <td>{s.pc?.toString(16)}</td>
                        <td>{s.a?.toString(16)}</td>
                        <td>{s.b?.toString(16)}</td>
                        <td>{s.x?.toString(16)}</td>
                        <td>{s.y?.toString(16)}</td>
                        <td>{s.u?.toString(16)}</td>
                        <td>{(s.via_ifr??0).toString(16).padStart(2,'0')}</td>
                        <td>{(s.via_ier??0).toString(16).padStart(2,'0')}</td>
                        <td>{(s.via_acr??0).toString(16).padStart(2,'0')}</td>
                        <td>{(s.via_pcr??0).toString(16).padStart(2,'0')}</td>
                        <td>{s.cycles}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

const btn: React.CSSProperties = { background:'#1d1d1d', color:'#ddd', border:'1px solid #444', padding:'2px 8px', fontSize:11, cursor:'pointer', borderRadius:3 };
