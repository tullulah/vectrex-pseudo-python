import React, { useState, useEffect, useCallback } from 'react';
import { globalEmu } from '../../emulatorWasm';

interface TraceRow { pc:number; op:number; sub:number; hex:string; m:string; a:number; b:number; x:number; y:number; u:number; s:number; dp:number; operand?:string|null; repeat?:number; flags?:number; cycles?:number; illegal?:boolean; }

export const TracePanel: React.FC = () => {
  const [rows, setRows] = useState<TraceRow[]>([]);
  const [capturing, setCapturing] = useState(false);
  const [limit, setLimit] = useState(10000);

  const refresh = useCallback(()=>{
    const list = globalEmu.traceLog();
    setRows(list);
  },[]);

  const startCapture = () => {
    globalEmu.clearTrace();
    globalEmu.enableTraceCapture(true, limit);
    setCapturing(true);
  };
  const stopCapture = () => { globalEmu.enableTraceCapture(false, limit); setCapturing(false); refresh(); };
  const clear = () => { globalEmu.clearTrace(); refresh(); };

  useEffect(()=>{ refresh(); },[refresh]);

  const exportText = () => {
    const lines = rows.map(r => `${r.pc.toString(16).padStart(4,'0')}: ${r.m} (0x${r.hex.replace(' ',' 0x')})${r.operand? ' '+r.operand:''}${r.repeat&&r.repeat>0? ' [x'+r.repeat+']':''} A=${r.a.toString(16).padStart(2,'0')} B=${r.b.toString(16).padStart(2,'0')} X=${r.x.toString(16).padStart(4,'0')} Y=${r.y.toString(16).padStart(4,'0')} U=${r.u.toString(16).padStart(4,'0')} S=${r.s.toString(16).padStart(4,'0')} DP=${r.dp.toString(16).padStart(2,'0')}`);
    const blob = new Blob([lines.join('\n')], { type:'text/plain' });
    const url = URL.createObjectURL(blob); const a=document.createElement('a'); a.href=url; a.download='trace.txt'; a.click(); URL.revokeObjectURL(url);
  };

  return <div style={{display:'flex',flexDirection:'column',height:'100%',fontFamily:'monospace'}}>
    <div style={{padding:4,borderBottom:'1px solid #444',display:'flex',gap:8,alignItems:'center'}}>
      <label>Limit <input style={{width:80}} type='number' value={limit} onChange={e=>setLimit(parseInt(e.target.value)||0)} disabled={capturing}/></label>
      {!capturing && <button onClick={startCapture}>Capture Init</button>}
      {capturing && <button onClick={stopCapture}>Stop</button>}
      <button onClick={refresh}>Refresh</button>
      <button onClick={clear}>Clear</button>
      <button onClick={exportText} disabled={!rows.length}>Export</button>
      <span style={{marginLeft:'auto',opacity:0.7}}>Entries: {rows.length}</span>
    </div>
    <div style={{flex:1,overflow:'auto',fontSize:11,lineHeight:1.2,padding:4}}>
      {rows.map((r,i)=>{
        const opDisp = `${r.m}${r.hex?` (0x${r.hex.replace(' ',' 0x')})`:''}`;
        const extra = r.operand ? ` ${r.operand}` : '';
        const loop = (r.repeat && r.repeat>0) ? ` [x${r.repeat}]` : '';
        let flagsStr='';
        if (typeof r.flags==='number') {
          const f=r.flags; // bit layout we encoded: C,V,Z,N,I,H,F,E (0..7)
          const c = (f & 0x01)?'C':''; const v=(f&0x02)?'V':''; const z=(f&0x04)?'Z':''; const n=(f&0x08)?'N':''; const iF=(f&0x10)?'I':''; const h=(f&0x20)?'H':''; const ff=(f&0x40)?'F':''; const e=(f&0x80)?'E':'';
          flagsStr = `[${n}${z}${v}${c}${iF}${h}${ff}${e}]`;
        }
        const cycStr = r.cycles!==undefined? ` cyc=${r.cycles}`:'';
        const ill = r.illegal? ' ILLEGAL':'';
        return (<div key={i}>{r.pc.toString(16).padStart(4,'0')}: {opDisp.padEnd(18,' ')}{extra} A={r.a.toString(16).padStart(2,'0')} B={r.b.toString(16).padStart(2,'0')} X={r.x.toString(16).padStart(4,'0')} Y={r.y.toString(16).padStart(4,'0')} U={r.u.toString(16).padStart(4,'0')} S={r.s.toString(16).padStart(4,'0')} DP={r.dp.toString(16).padStart(2,'0')} {flagsStr}{cycStr}{loop}{ill}</div>);
      })}
    </div>
  </div>;
};

export default TracePanel;
