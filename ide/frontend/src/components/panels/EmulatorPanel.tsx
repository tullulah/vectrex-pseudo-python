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
  const animRef = useRef<number|null>(null);

  // Simple demo drawing placeholder (rotating triangle) until real emulator loop feeds segments.
  useEffect(() => {
    let angle = 0;
    const tick = () => {
      angle += 0.02;
      // generate triangle in logical coords
      const r = 100;
      const cx = 0; const cy = 0;
      const pts = [0, -r, r*Math.sin(Math.PI/3), r*Math.cos(Math.PI/3), -r*Math.sin(Math.PI/3), r*Math.cos(Math.PI/3)];
      const rot = (x:number,y:number):[number,number] => [ x*Math.cos(angle)-y*Math.sin(angle), x*Math.sin(angle)+y*Math.cos(angle) ];
      const p0 = rot(pts[0], pts[1]);
      const p1 = rot(pts[2], pts[3]);
      const p2 = rot(pts[4], pts[5]);
      setSegments([
        { x1:p0[0], y1:p0[1], x2:p1[0], y2:p1[1], intensity:0x5F },
        { x1:p1[0], y1:p1[1], x2:p2[0], y2:p2[1], intensity:0x5F },
        { x1:p2[0], y1:p2[1], x2:p0[0], y2:p0[1], intensity:0x5F },
      ]);
      animRef.current = requestAnimationFrame(tick);
    };
    tick();
    return () => { if (animRef.current) cancelAnimationFrame(animRef.current); };
  }, []);

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
    // For now triggers a file open dialog (Electron preload expected to expose files API)
    const w: any = window as any;
    if (w.files?.openFile) {
      w.files.openFile({ binary:true }).then((res: any) => {
        if (!res || res.error || !res.content) return;
        setLoadedBinName(res.path?.split(/[/\\]/).pop());
        // TODO: Send content to backend emulator (Rust) via IPC or WASM once implemented.
        console.debug('[EMU] Loaded BIN bytes', res.content.length);
      });
    } else {
      console.warn('No file open API available');
    }
  };

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', width:'100%', padding:8, boxSizing:'border-box', gap:8}}>
      <div style={{display:'flex', alignItems:'center', gap:12, flexWrap:'wrap'}}>
        <strong>{t('panel.emulator')}</strong>
        <button onClick={onLoadBin} style={{padding:'4px 8px'}}>{loadedBinName ? t('emu.reload','Reload BIN') : t('emu.load','Load BIN')}</button>
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
