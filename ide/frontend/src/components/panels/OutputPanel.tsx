import React, { useEffect, useState, useRef } from 'react';

interface UnknownMap { [opcode: string]: number }

export const OutputPanel: React.FC = () => {
  const [unknown, setUnknown] = useState<UnknownMap>({});
  const [regs, setRegs] = useState<any>({});
  const [auto, setAuto] = useState(true);
  const timerRef = useRef<number|null>(null);

  const fetchStats = () => {
    const w: any = window as any; if (!w.electronAPI?.emuStats) return;
    w.electronAPI.emuStats().then((s: any) => { if (!s) return; setUnknown(s.unknownOpcodes||{}); setRegs(s.regs||{}); });
  };
  const resetStats = () => {
    const w:any = window as any; if (!w.electronAPI?.emuStatsReset) return;
    w.electronAPI.emuStatsReset().then(()=>{ fetchStats(); });
  };

  useEffect(() => { fetchStats(); }, []);
  useEffect(() => {
    if (auto) {
      timerRef.current = window.setInterval(fetchStats, 1500);
    } else if (timerRef.current) {
      clearInterval(timerRef.current); timerRef.current=null;
    }
    return () => { if (timerRef.current) clearInterval(timerRef.current); };
  }, [auto]);

  const entries = Object.entries(unknown).sort((a,b)=>b[1]-a[1]);

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', fontSize:12}}>
      <div style={{padding:'4px 8px', borderBottom:'1px solid #333', display:'flex', alignItems:'center', gap:12}}>
        <strong>Output / Emulator Log</strong>
        <button onClick={fetchStats} style={btnStyle}>Refresh</button>
        <button onClick={resetStats} style={btnStyle}>Reset</button>
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input type='checkbox' checked={auto} onChange={e=>setAuto(e.target.checked)} /> Auto
        </label>
        <span style={{marginLeft:'auto', opacity:0.7}}>Unknown opcodes: {entries.length}</span>
      </div>
      <div style={{padding:'4px 8px', borderBottom:'1px solid #222', display:'flex', gap:20}}>
        <div>PC: {hex16(regs.pc)}</div>
        <div>A: {hex8(regs.a)} B: {hex8(regs.b)} X: {hex16(regs.x)} Y: {hex16(regs.y)} U: {hex16(regs.u)} DP: {hex8(regs.dp)}</div>
      </div>
      <div style={{overflow:'auto', flex:1}}>
        {entries.length===0 && <div style={{padding:8, color:'#666'}}>No unknown opcodes yet.</div>}
        {entries.length>0 && (
          <table style={{width:'100%', borderCollapse:'collapse'}}>
            <thead>
              <tr style={{textAlign:'left', background:'#222'}}>
                <th style={th}>Opcode</th>
                <th style={th}>Count</th>
              </tr>
            </thead>
            <tbody>
              {entries.map(([op, count]) => (
                <tr key={op} style={{background:'#1e1e1e'}}>
                  <td style={td}>{op.toUpperCase()}</td>
                  <td style={td}>{count}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
};

const btnStyle: React.CSSProperties = { background:'#1e1e1e', color:'#ddd', border:'1px solid #333', padding:'2px 6px', cursor:'pointer', fontSize:11 };
const th: React.CSSProperties = { padding:'4px 6px' };
const td: React.CSSProperties = { padding:'2px 6px', fontFamily:'monospace' };
function hex8(v:any){ if (typeof v!=='number') return '--'; return '0x'+(v&0xFF).toString(16).padStart(2,'0'); }
function hex16(v:any){ if (typeof v!=='number') return '--'; return '0x'+(v&0xFFFF).toString(16).padStart(4,'0'); }
