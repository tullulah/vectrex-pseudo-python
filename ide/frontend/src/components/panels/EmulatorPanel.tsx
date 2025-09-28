import React, { useEffect, useRef, useState } from 'react';
import { useEmulatorStore } from '../../state/emulatorStore';
import { useEditorStore } from '../../state/editorStore';
import { psgAudio } from '../../psgAudio';
import { inputManager } from '../../inputManager';

export const EmulatorPanel: React.FC = () => {
  const status = useEmulatorStore(s => s.status);
  const setStatus = useEmulatorStore(s => s.setStatus);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  
  // Estados básicos necesarios
  const [audioEnabled, setAudioEnabled] = useState(() => {
    try { return localStorage.getItem('emu_audio_enabled') !== '0'; } 
    catch { return true; }
  });
  const [audioStats, setAudioStats] = useState<{ 
    sampleRate:number; pushed:number; consumed:number; 
    bufferedSamples:number; bufferedMs:number; overflowCount:number 
  }|null>(null);
  
  // Hook editor store para documentos activos
  const editorActive = useEditorStore(s => s.active);
  const editorDocuments = useEditorStore(s => s.documents);

  // Inicialización JSVecX exactamente como el test que funciona
  useEffect(() => {
    let cancelled = false;
    
    const initJSVecX = () => {
      const canvas = canvasRef.current;
      if (!canvas) return;
      
      // Configurar canvas igual que el test HTML
      canvas.id = 'screen';
      canvas.width = 330;
      canvas.height = 410;
      canvas.style.width = '300px';
      canvas.style.height = '400px';
      canvas.style.border = '1px solid #333';
      canvas.style.background = '#000';
      
      const vecx = (window as any).vecx;
      if (!vecx) {
        console.error('[EmulatorPanel] Global vecx instance not found');
        return;
      }
      
      console.log('[EmulatorPanel] Initializing JSVecX exactly like working test...');
      
      try {
        vecx.reset();
        console.log('[EmulatorPanel] ✓ vecx.reset() successful');
        
        vecx.main();
        console.log('[EmulatorPanel] ✓ vecx.main() called successfully');
        
        if (!cancelled) {
          setStatus('running');
        }
      } catch (e) {
        console.error('[EmulatorPanel] JSVecX initialization failed:', e);
        if (!cancelled) {
          setStatus('stopped');
        }
      }
    };
    
    // Esperar un poco para que el canvas esté listo
    setTimeout(initJSVecX, 100);
    
    return () => {
      cancelled = true;
    };
  }, [setStatus]);

  // Audio lifecycle: init worklet on enable; start/stop with status
  useEffect(() => {
    (async () => {
      if (audioEnabled) {
        try {
          await psgAudio.init();
          if (status === 'running') psgAudio.start();
        } catch(e) {
          console.warn('[EmulatorPanel] audio init failed', e);
        }
      } else {
        psgAudio.stop();
      }
    })();
  }, [audioEnabled]);

  useEffect(() => {
    if (!audioEnabled) return;
    if (status === 'running') {
      psgAudio.start();
    } else {
      psgAudio.stop();
    }
  }, [status, audioEnabled]);

  // Poll de estadísticas de audio (cada ~500ms mientras audioEnabled)
  useEffect(() => {
    if (!audioEnabled) { 
      setAudioStats(null); 
      return; 
    }
    let cancelled = false;
    const tick = () => {
      try {
        const s = psgAudio.getStats?.();
        if (s && !cancelled) setAudioStats(s);
      } catch {/* ignore */}
    };
    tick();
    const id = setInterval(tick, 500);
    return () => { 
      cancelled = true; 
      clearInterval(id); 
    };
  }, [audioEnabled]);

  const onPlay = () => {
    const vecx = (window as any).vecx;
    if (vecx) {
      vecx.start();
      setStatus('running');
      console.log('[EmulatorPanel] JSVecX started');
    }
  };
  
  const onPause = () => {
    const vecx = (window as any).vecx;
    if (vecx) {
      vecx.stop();
      setStatus('paused');
      console.log('[EmulatorPanel] JSVecX paused');
    }
  };
  
  const onStop = () => {
    const vecx = (window as any).vecx;
    if (vecx) {
      vecx.stop();
      setStatus('stopped');
      console.log('[EmulatorPanel] JSVecX stopped');
    }
  };
  
  const onReset = () => {
    const vecx = (window as any).vecx;
    if (vecx) {
      vecx.reset();
      console.log('[EmulatorPanel] JSVecX reset');
      if (status === 'running') {
        vecx.start();
        console.log('[EmulatorPanel] JSVecX restarted after reset');
      }
    }
  };

  const btn: React.CSSProperties = { 
    background: '#1d1d1d', 
    color: '#ddd', 
    border: '1px solid #444', 
    padding: '4px 12px', 
    fontSize: 12, 
    cursor: 'pointer', 
    borderRadius: 3 
  };

  return (
    <div style={{
      display: 'flex', 
      flexDirection: 'column', 
      height: '100%', 
      padding: 8, 
      boxSizing: 'border-box', 
      fontFamily: 'monospace', 
      fontSize: 12
    }}>
      {/* Controles expandidos */}
      <div style={{
        display: 'flex', 
        alignItems: 'center', 
        gap: 12, 
        marginBottom: 12,
        flexWrap: 'wrap'
      }}>
        <span>Status: <span style={{
          color: status === 'running' ? '#0f0' : status === 'paused' ? '#fa0' : '#f55'
        }}>{status}</span></span>
        
        {/* Control de audio */}
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input 
            type='checkbox' 
            checked={audioEnabled} 
            onChange={e => {
              const v = e.target.checked; 
              setAudioEnabled(v); 
              try { 
                localStorage.setItem('emu_audio_enabled', v ? '1' : '0'); 
              } catch{} 
              if(!v) psgAudio.stop(); 
            }} 
          /> 
          audio
        </label>

        {/* Información del documento activo */}
        {editorActive && (
          <span style={{opacity: 0.6}}>
            Active: {editorDocuments.find(d => d.uri === editorActive)?.diskPath || editorActive}
          </span>
        )}
        
        <div style={{ marginLeft: 'auto', display: 'flex', gap: 6 }}>
          {status !== 'running' && <button style={btn} onClick={onPlay}>
            {status === 'paused' ? 'Resume' : 'Play'}
          </button>}
          {status === 'running' && <button style={btn} onClick={onPause}>Pause</button>}
          <button style={btn} onClick={onStop}>Stop</button>
          <button style={btn} onClick={onReset}>Reset</button>
        </div>
      </div>

      {/* Canvas para JSVecX */}
      <div style={{
        flex: 1, 
        display: 'flex', 
        justifyContent: 'center', 
        alignItems: 'center'
      }}>
        <canvas 
          ref={canvasRef} 
          id="screen" 
          width="330" 
          height="410" 
          style={{
            border: '1px solid #333', 
            background: '#000', 
            width: 300, 
            height: 400
          }} 
        />
      </div>

      {/* Estadísticas de audio si está habilitado */}
      {audioEnabled && audioStats && (
        <div style={{
          marginTop: 12,
          padding: 8,
          background: '#111',
          border: '1px solid #333',
          borderRadius: 4,
          fontSize: 11,
          fontFamily: 'monospace'
        }}>
          <div style={{ fontWeight: 'bold', color: '#8cf', marginBottom: 4 }}>Audio PSG</div>
          <div>Rate: {audioStats.sampleRate} Hz</div>
          <div>Buffered: {audioStats.bufferedMs.toFixed(1)} ms ({audioStats.bufferedSamples} smp)</div>
          <div>Pushed/Consumed: {audioStats.pushed}/{audioStats.consumed}</div>
          <div style={{color: audioStats.overflowCount > 0 ? '#f66' : '#6c6'}}>
            Overflows: {audioStats.overflowCount}{audioStats.overflowCount > 0 && ' (!!)'}
          </div>
        </div>
      )}

      <div style={{
        marginTop: 12, 
        fontSize: 11, 
        color: '#777', 
        textAlign: 'center'
      }}>
        JSVecX Emulator - Canvas renders automatically
      </div>
    </div>
  );
};