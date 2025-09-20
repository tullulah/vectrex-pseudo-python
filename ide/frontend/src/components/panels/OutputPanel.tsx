import React, { useEffect, useState, useRef } from 'react';
import { globalEmu, MetricsSnapshot } from '../../emulatorWasm';

export const OutputPanel: React.FC = () => {
  const [metrics, setMetrics] = useState<MetricsSnapshot | null>(null);
  const [regs, setRegs] = useState<any>(null);
  const [auto, setAuto] = useState(true);
  const timerRef = useRef<number|null>(null);

  const fetchStats = () => {
    try {
      const m = globalEmu.metrics();
      const r = globalEmu.registers();
      if (m) setMetrics(m);
      if (r) setRegs(r);
    } catch {/* ignore until wasm loaded */}
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

  const unknownList = metrics?.unique_unimplemented || [];
  const topOpcodes = metrics?.top_opcodes || [];

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', fontSize:12}}>
      <div style={{padding:'4px 8px', borderBottom:'1px solid #333', display:'flex', alignItems:'center', gap:12}}>
        <strong>Emulator Metrics</strong>
        <button onClick={fetchStats} style={btnStyle}>Refresh</button>
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input type='checkbox' checked={auto} onChange={e=>setAuto(e.target.checked)} /> Auto
        </label>
        <span style={{marginLeft:'auto', opacity:0.7}}>Frames: {metrics?.frames ?? 0}</span>
      </div>
      <div style={{padding:'6px 10px', borderBottom:'1px solid #222', display:'flex', flexWrap:'wrap', gap:20}}>
        <div>PC: {hex16(regs?.pc)}</div>
        <div>A: {hex8(regs?.a)} B: {hex8(regs?.b)} X: {hex16(regs?.x)} Y: {hex16(regs?.y)} U: {hex16(regs?.u)} S: {hex16(regs?.s)} DP: {hex8(regs?.dp)}</div>
        <div>Cycles: {metrics?.cycles ?? 0}</div>
        <div>Avg Cycles/frame: {metrics?.avg_cycles_per_frame ? metrics!.avg_cycles_per_frame.toFixed(0) : '--'}</div>
  <div>Draw VL: {metrics?.draw_vl ?? (regs?.draw_vl_count ?? 0)}</div>
  <div>BIOS Frames: {metrics?.bios_frame ?? (regs?.bios_frame ?? 0)}</div>
        <div>Last Intensity: {hex8(metrics?.last_intensity)}</div>
        <div>Unimpl Count: {metrics?.unimplemented ?? 0}</div>
        <div>Unique Unimpl: {unknownList.length}</div>
      </div>
      <div style={{display:'flex', flex:1, overflow:'hidden'}}>
        <div style={{flex:1, overflow:'auto', borderRight:'1px solid #222'}}>
          <div style={{padding:'4px 8px', fontWeight:600, background:'#181818'}}>Top Opcodes</div>
          {topOpcodes.length===0 && <div style={{padding:8, color:'#555'}}>No executions yet.</div>}
          {topOpcodes.length>0 && (
            <table style={{width:'100%', borderCollapse:'collapse'}}>
              <thead>
                <tr style={{textAlign:'left', background:'#222'}}>
                  <th style={th}>Opcode</th>
                  <th style={th}>Count</th>
                </tr>
              </thead>
              <tbody>
                {topOpcodes.map(([op, cnt]) => (
                  <tr key={op} style={{background:'#1e1e1e'}}>
                    <td style={td}>{hex8(op)}</td>
                    <td style={td}>{cnt}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
        <div style={{flex:1, overflow:'auto'}}>
          <div style={{padding:'4px 8px', fontWeight:600, background:'#181818'}}>Unique Unimplemented</div>
          {unknownList.length===0 && <div style={{padding:8, color:'#555'}}>None</div>}
          {unknownList.length>0 && (
            <div style={{display:'flex', flexWrap:'wrap', gap:6, padding:8}}>
              {unknownList.map(op => (
                <span key={op} style={{background:'#3a1e1e', color:'#ff9f9f', padding:'2px 6px', borderRadius:4, fontFamily:'monospace'}}>{hex8(op)}</span>
              ))}
            </div>
          )}
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
