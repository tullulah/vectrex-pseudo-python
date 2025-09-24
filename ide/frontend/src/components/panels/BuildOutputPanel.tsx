import React, { useEffect, useRef, useState } from 'react';

interface Line { id: number; text: string; kind: 'info' | 'warn' | 'error' | 'stdout' | 'stderr' | 'diagnostic'; }

export const BuildOutputPanel: React.FC = () => {
  const [lines, setLines] = useState<Line[]>([]);
  const nextId = useRef(1);
  const scrollerRef = useRef<HTMLDivElement|null>(null);
  const autoScrollRef = useRef(true);

  const append = (text: string, kind: Line['kind']) => {
    setLines(l => [...l, { id: nextId.current++, text, kind }]);
  };

  useEffect(() => {
    const w:any = window as any;
    w?.electronAPI?.onRunStdout?.((chunk: string) => {
      chunk.split(/\r?\n/).filter(Boolean).forEach(l => append(l, 'stdout'));
    });
    w?.electronAPI?.onRunStderr?.((chunk: string) => {
      chunk.split(/\r?\n/).filter(Boolean).forEach(l => append(l, 'stderr'));
    });
    w?.electronAPI?.onRunDiagnostics?.((diags: Array<{file:string; line:number; col:number; message:string}>) => {
      diags.forEach(d => append(`${d.file}:${d.line+1}:${d.col+1}: ${d.message}`, 'diagnostic'));
    });
    w?.electronAPI?.onRunStatus?.((msg: string) => append(msg, 'info'));
  }, []);

  useEffect(() => {
    if (!autoScrollRef.current) return;
    const el = scrollerRef.current; if (!el) return;
    el.scrollTop = el.scrollHeight;
  }, [lines]);

  const clear = () => setLines([]);

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', fontSize:12, fontFamily:'monospace'}}>
      <div style={{padding:'4px 8px', borderBottom:'1px solid #333', display:'flex', gap:12, alignItems:'center'}}>
        <strong>Build Output</strong>
        <button onClick={clear} style={btn}>Clear</button>
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input type='checkbox' defaultChecked onChange={e=>{autoScrollRef.current=e.target.checked;}} /> AutoScroll
        </label>
        <span style={{marginLeft:'auto', opacity:0.6}}>{lines.length} lines</span>
      </div>
      <div ref={scrollerRef} style={{flex:1, overflow:'auto', padding:8, background:'#111'}}>
        {lines.map(l => (
          <div key={l.id} style={{whiteSpace:'pre', color: colorFor(l.kind)}}>{l.text}</div>
        ))}
        {lines.length===0 && <div style={{opacity:0.5}}>No build output yet. Press Run.</div>}
      </div>
    </div>
  );
};

const btn: React.CSSProperties = { background:'#1e1e1e', color:'#ddd', border:'1px solid #333', padding:'2px 6px', cursor:'pointer', fontSize:11 };
function colorFor(kind: Line['kind']): string {
  switch(kind){
    case 'error': return '#f66';
    case 'warn': return '#fc3';
    case 'stderr': return '#f88';
    case 'stdout': return '#8cf';
    case 'diagnostic': return '#ffaaff';
    default: return '#ccc';
  }
}
