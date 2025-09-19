import React, { useEffect, useState, useCallback } from 'react';
import { globalEmu } from '../../emulatorWasm';

// Simple panel to display BIOS call stack (recent calls) fetched from wasm.
// Non-intrusive, polls only on manual refresh to avoid overhead.

export const BiosCallsPanel: React.FC = () => {
  const [calls, setCalls] = useState<string[]>([]);
  const [ts, setTs] = useState<number>(0);
  const [auto, setAuto] = useState<boolean>(false);
  const [intervalMs, setIntervalMs] = useState<number>(1000);

  const refresh = useCallback(()=>{
    try { setCalls(globalEmu.biosCalls()); setTs(Date.now()); } catch { /* ignore */ }
  },[]);

  useEffect(()=>{ refresh(); },[refresh]);

  useEffect(()=>{
    if(!auto) return; const id = setInterval(()=>refresh(), intervalMs); return ()=>clearInterval(id);
  },[auto, intervalMs, refresh]);

  const clear = () => { globalEmu.clearBiosCalls(); refresh(); };

  return (
    <div style={{display:'flex',flexDirection:'column',height:'100%',fontFamily:'monospace'}}>
      <div style={{display:'flex',gap:8,alignItems:'center',padding:4,borderBottom:'1px solid #444'}}>
        <button onClick={refresh}>Refresh</button>
        <button onClick={clear}>Clear</button>
        <label style={{display:'flex',alignItems:'center',gap:4}}>Auto
          <input type='checkbox' checked={auto} onChange={e=>setAuto(e.target.checked)}/>
        </label>
        {auto && <label>Interval(ms) <input style={{width:70}} type='number' value={intervalMs} onChange={e=>setIntervalMs(parseInt(e.target.value)||1000)}/></label>}
        <span style={{marginLeft:'auto',opacity:0.7}}>Calls: {calls.length} @ {new Date(ts).toLocaleTimeString()}</span>
      </div>
      <div style={{flex:1,overflow:'auto',padding:4,fontSize:12,lineHeight:1.25}}>
        {calls.length===0 && <div style={{opacity:0.6}}>No BIOS calls yet.</div>}
        {calls.map((c,i)=>(<div key={i}>{c}</div>))}
      </div>
    </div>
  );
};

export default BiosCallsPanel;
