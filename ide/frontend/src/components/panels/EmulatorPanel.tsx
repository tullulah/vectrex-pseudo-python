import React, { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useEditorStore } from '../../state/editorStore';

// Vectrex visible area ~ 3:4 portrait style. We'll use logical 300x400 and scale to fit.
const LOGICAL_W = 300;
const LOGICAL_H = 400;

interface VectorSegment { x1:number; y1:number; x2:number; y2:number; intensity:number; }

export const EmulatorPanel: React.FC = () => {
  const { t } = useTranslation(['common']);
  const canvasRef = useRef<HTMLCanvasElement|null>(null);
  const [segments, setSegments] = useState<VectorSegment[]>([]);
  const [loadedBinName, setLoadedBinName] = useState<string|undefined>();
  const [running, setRunning] = useState(false);
  const [runStatus, setRunStatus] = useState<string|undefined>();
  const playRef = useRef(false);
  const frameTimerRef = useRef<number|null>(null);
  const [vectorMode, setVectorMode] = useState<'intercept'|'via'>('intercept');

  // Frame polling loop
  const requestNextFrame = () => {
    const w:any = window as any;
    if (!playRef.current || !w.electronAPI?.emuRunFrame) return;
    w.electronAPI.emuRunFrame().then((res:any) => {
      if (res?.segments) setSegments(res.segments as VectorSegment[]);
      if (playRef.current) frameTimerRef.current = window.setTimeout(requestNextFrame, 16);
    });
  };

  const togglePlay = () => {
    playRef.current = !playRef.current;
    if (playRef.current) requestNextFrame();
    else if (frameTimerRef.current) { clearTimeout(frameTimerRef.current); frameTimerRef.current=null; }
  };

  // Render segments to canvas
  useEffect(() => {
    const canvas = canvasRef.current; if (!canvas) return;
    const ctx = canvas.getContext('2d'); if (!ctx) return;
    // Fit canvas element to parent size while preserving aspect
    const parent = canvas.parentElement; if (parent) {
      const pw = parent.clientWidth - 16; const ph = parent.clientHeight - 80; // padding + controls
      const targetRatio = LOGICAL_W / LOGICAL_H; // 0.75
      let w = pw; let h = w / targetRatio;
      if (h > ph) { h = ph; w = h * targetRatio; }
      canvas.width = w * window.devicePixelRatio;
      canvas.height = h * window.devicePixelRatio;
      canvas.style.width = w + 'px';
      canvas.style.height = h + 'px';
  ctx.save();
  ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
      // Clear
      ctx.fillStyle = '#000';
      ctx.fillRect(0,0,w,h);
      // Transform logical -150..150 (width 300) / -200..200 (height 400) into canvas
      ctx.save();
      ctx.translate(w/2, h/2);
      const sx = w / LOGICAL_W; const sy = h / LOGICAL_H;
      ctx.scale(sx, sy);
      ctx.strokeStyle = '#0F0';
      ctx.lineWidth = 1 / Math.max(sx, sy);
      for (const seg of segments) {
        ctx.globalAlpha = Math.min(1, seg.intensity / 127);
        ctx.beginPath();
        ctx.moveTo(seg.x1, seg.y1);
        ctx.lineTo(seg.x2, seg.y2);
        ctx.stroke();
      }
  ctx.restore(); // undo translate/scale for drawing
  ctx.restore(); // undo dpi scale
    }
  }, [segments]);

  const onLoadBin = () => {
    const w:any = window as any;
    if (!w.files?.openBin || !w.electronAPI?.emuLoad) { console.warn('BIN APIs missing'); return; }
    w.files.openBin().then((res:any) => {
      if (!res || res.error || !res.base64) return;
      const name = res.path.split(/[/\\]/).pop();
      w.electronAPI.emuLoad(res.base64).then((r:any) => {
        if (!r || r.error) { console.warn('emuLoad failed', r); return; }
        setLoadedBinName(name);
        setSegments([]);
      });
    });
  };

  // Run current active editor file (renderer should expose selected path via custom event or global)
  const onRunActive = () => {
    const w:any = window as any;
    if (!w?.electronAPI?.runCompile) { console.warn('Run API missing'); return; }
    const state = (useEditorStore as any).getState();
    const activeUri: string | undefined = state.active;
    if (!activeUri) { setRunStatus(t('run.noActive','No .vpy active')); return; }
    const doc = state.documents.find((d:any)=>d.uri===activeUri);
    if (!doc) { setRunStatus(t('run.noActive','No .vpy active')); return; }
    if (!/\.vpy$|\.pseudo$/i.test(doc.uri)) { setRunStatus(t('run.noActive','No .vpy active')); return; }
    setRunning(true); setRunStatus(t('run.compiling','Compiling...'));
    const saveIfDirty = doc.dirty ? { content: doc.content, expectedMTime: doc.mtime } : undefined;
    w.electronAPI.runCompile({ path: doc.uri, saveIfDirty, autoStart:true }).then((res:any) => {
      if (res?.ok) {
        const name = (res.binPath || doc.uri).split(/[/\\]/).pop();
        setLoadedBinName(name?.replace(/\.bin$/,'') + '.bin');
        setRunStatus(t('run.success','Compiled & loaded'));
        if (!playRef.current) { togglePlay(); }
      } else if (res?.conflict) {
        setRunStatus(t('run.conflict','File modified on disk. Reload first.'));
      } else {
        setRunStatus((res?.error||'fail') + (res?.detail ? (': '+res.detail) : ''));
      }
    }).catch((e: any) => setRunStatus('error: '+e?.message)).finally(() => setRunning(false));
  };

  // Subscribe to run diagnostics once
  useEffect(() => {
    const w:any = window as any;
    const listener = (diags:any) => { console.log('[Run diagnostics]', diags); };
    w?.electronAPI?.onRunDiagnostics?.(listener);
    // Auto-play on emulator load if user previously initiated a run
    const onLoaded = (info:{size:number}) => {
      // Clear previous frame data so first frame shows fresh output
      setSegments([]);
      if (!playRef.current) {
        togglePlay();
      }
    };
    w?.electronAPI?.onEmuLoaded?.(onLoaded);
    return () => {
      // No explicit off() provided; rely on page lifecycle. (Could add ipcRenderer.removeListener if exposed.)
    };
  }, []);

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', width:'100%', padding:8, boxSizing:'border-box', gap:8}}>
      <div style={{display:'flex', alignItems:'center', gap:12, flexWrap:'wrap'}}>
        <strong>{t('panel.emulator')}</strong>
  <button onClick={onLoadBin} style={{padding:'4px 8px'}}>{loadedBinName ? t('emu.reload','Reload BIN') : t('emu.load','Load BIN')}</button>
  <button disabled={!loadedBinName} onClick={togglePlay} style={{padding:'4px 8px'}}>{playRef.current ? t('emu.pause','Pause') : t('emu.play','Play')}</button>
  <button onClick={onRunActive} disabled={running} style={{padding:'4px 8px'}}>{running ? t('run.running','Running...') : t('run.run','Run')}</button>
        {loadedBinName && <span style={{fontSize:12, color:'#888'}}>{loadedBinName}</span>}
        <label style={{marginLeft:'auto', fontSize:11, color:'#ccc', display:'flex', alignItems:'center', gap:4}}>
          <span>{t('emu.vectorMode','Mode')}:</span>
          <select value={vectorMode} onChange={e=>{
            const m = e.target.value as 'intercept'|'via';
            setVectorMode(m);
            const w:any = window as any;
            w?.electronAPI?.setVectorMode?.(m);
          }} style={{fontSize:11, background:'#111', color:'#ccc', border:'1px solid #333'}}>
            <option value="intercept">intercept</option>
            <option value="via">via</option>
          </select>
        </label>
      </div>
      <div style={{flex:1, position:'relative', minHeight:120}}>
        <canvas ref={canvasRef} style={{display:'block', margin:'0 auto', background:'#000', border:'1px solid #222', borderRadius:4}} />
      </div>
      {runStatus && <div style={{fontSize:11, color:'#9cf'}}>{runStatus}</div>}
      <div style={{fontSize:11, color:'#777', lineHeight:1.4}}>
        {vectorMode==='intercept' ? t('emu.hint','Fast intercept mode: BIOS draw/intensity calls are short-circuited.') : 'VIA mode (experimental placeholder): future hardware-timed vector reconstruction.'}
      </div>
    </div>
  );
};
