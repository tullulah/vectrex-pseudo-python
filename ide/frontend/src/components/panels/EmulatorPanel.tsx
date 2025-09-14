import React, { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';

// Vectrex visible area ~ 3:4 portrait style. We'll use logical 300x400 and scale to fit.
const LOGICAL_W = 300;
const LOGICAL_H = 400;

interface VectorSegment { x1:number; y1:number; x2:number; y2:number; intensity:number; }

export const EmulatorPanel: React.FC = () => {
  const { t } = useTranslation(['common']);
  const canvasRef = useRef<HTMLCanvasElement|null>(null);
  const [segments, setSegments] = useState<VectorSegment[]>([]);
  const [loadedBinName, setLoadedBinName] = useState<string|undefined>();
  const playRef = useRef(false);
  const frameTimerRef = useRef<number|null>(null);

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

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', width:'100%', padding:8, boxSizing:'border-box', gap:8}}>
      <div style={{display:'flex', alignItems:'center', gap:12, flexWrap:'wrap'}}>
        <strong>{t('panel.emulator')}</strong>
  <button onClick={onLoadBin} style={{padding:'4px 8px'}}>{loadedBinName ? t('emu.reload','Reload BIN') : t('emu.load','Load BIN')}</button>
  <button disabled={!loadedBinName} onClick={togglePlay} style={{padding:'4px 8px'}}>{playRef.current ? t('emu.pause','Pause') : t('emu.play','Play')}</button>
        {loadedBinName && <span style={{fontSize:12, color:'#888'}}>{loadedBinName}</span>}
        <span style={{marginLeft:'auto', fontSize:11, color:'#666'}}>{t('emu.aspect','Aspect 3:4')}</span>
      </div>
      <div style={{flex:1, position:'relative', minHeight:120}}>
        <canvas ref={canvasRef} style={{display:'block', margin:'0 auto', background:'#000', border:'1px solid #222', borderRadius:4}} />
      </div>
      <div style={{fontSize:11, color:'#777', lineHeight:1.4}}>
        {t('emu.hint','Placeholder vector preview (rotating triangle). Loading a BIN will route bytes to future emulator core.')}
      </div>
    </div>
  );
};
