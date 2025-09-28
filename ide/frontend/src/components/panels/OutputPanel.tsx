import React, { useEffect, useState, useRef } from 'react';

// Tipos simples para JSVecX
interface VecxMetrics {
  totalCycles: number;
  instructionCount: number;
  frameCount: number;
  running: boolean;
}

interface VecxRegs {
  PC: number;
  A: number; B: number;
  X: number; Y: number; U: number; S: number;
  DP: number; CC: number;
}

export const OutputPanel: React.FC = () => {
  const [metrics, setMetrics] = useState<VecxMetrics | null>(null);
  const [regs, setRegs] = useState<VecxRegs | null>(null);
  const [auto, setAuto] = useState(true);
  const timerRef = useRef<number|null>(null);

  const fetchStats = () => {
    try {
      const vecx = (window as any).vecx;
      if (!vecx) {
        console.warn('[OutputPanel] JSVecX instance not found');
        setMetrics(null);
        setRegs(null);
        return;
      }
      
      // Usar las nuevas funciones añadidas a JSVecX
      const fetchedMetrics = vecx.getMetrics && vecx.getMetrics();
      const fetchedRegs = vecx.getRegisters && vecx.getRegisters();
      
      setMetrics(fetchedMetrics || null);
      setRegs(fetchedRegs || null);
    } catch (e) {
      console.warn('[OutputPanel] Error fetching JSVecX stats:', e);
      setMetrics(null);
      setRegs(null);
    }
  };

  useEffect(() => { fetchStats(); }, []);
  useEffect(() => {
    if (auto) {
      timerRef.current = window.setInterval(fetchStats, 1000);
    } else if (timerRef.current) {
      clearInterval(timerRef.current); timerRef.current=null;
    }
    return () => { if (timerRef.current) clearInterval(timerRef.current); };
  }, [auto]);

  const avgCyclesPerFrame = (metrics && metrics.frameCount > 0) ? 
    Math.round(metrics.totalCycles / metrics.frameCount) : 0;

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', fontSize:12}}>
      <div style={{padding:'4px 8px', borderBottom:'1px solid #333', display:'flex', alignItems:'center', gap:12}}>
        <strong>JSVecX Emulator Metrics</strong>
        <button onClick={fetchStats} style={btnStyle}>Refresh</button>
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input type='checkbox' checked={auto} onChange={e=>setAuto(e.target.checked)} /> Auto
        </label>
        <span style={{marginLeft:'auto', opacity:0.7}}>
          Status: {metrics?.running ? 'Running' : 'Stopped'}
        </span>
      </div>
      <div style={{padding:'6px 10px', borderBottom:'1px solid #222', display:'flex', flexWrap:'wrap', gap:20}}>
        <div>PC: {hex16(regs?.PC)}</div>
        <div>
          A: {hex8(regs?.A)} B: {hex8(regs?.B)} X: {hex16(regs?.X)} Y: {hex16(regs?.Y)} 
          U: {hex16(regs?.U)} S: {hex16(regs?.S)} DP: {hex8(regs?.DP)} CC: {hex8(regs?.CC)}
        </div>
        <div>Total Cycles: {metrics?.totalCycles ?? 0}</div>
        <div>Instructions: {metrics?.instructionCount ?? 0}</div>
        <div>Frames: {metrics?.frameCount ?? 0}</div>
        <div>Avg Cycles/frame: {avgCyclesPerFrame > 0 ? avgCyclesPerFrame : '--'}</div>
      </div>
      <div style={{padding:20, color:'#888', textAlign:'center'}}>
        <div>JSVecX Simple Output Panel</div>
        <div style={{fontSize:11, marginTop:8}}>
          Basic CPU register and emulator metrics display.
        </div>
      </div>
    </div>
  );
};

const btnStyle: React.CSSProperties = { background:'#1e1e1e', color:'#ddd', border:'1px solid #333', padding:'2px 6px', cursor:'pointer', fontSize:11 };
const th: React.CSSProperties = { padding:'4px 6px' };
const td: React.CSSProperties = { padding:'2px 6px', fontFamily:'monospace' };
function hex8(v:any){ if (typeof v!=='number') return '--'; return '0x'+(v&0xFF).toString(16).padStart(2,'0'); }
function hex16(v:any){ if (typeof v!=='number') return '--'; return '0x'+(v&0xFFFF).toString(16).padStart(4,'0'); }
