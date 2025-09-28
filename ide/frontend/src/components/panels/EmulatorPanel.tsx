import React, { useEffect, useRef, useState, useCallback } from 'react';
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
  const [loadedROM, setLoadedROM] = useState<string | null>(null);
  const [availableROMs, setAvailableROMs] = useState<string[]>([]);
  const [selectedROM, setSelectedROM] = useState<string>('');
  const [overlayEnabled, setOverlayEnabled] = useState<boolean>(true);
  const [currentOverlay, setCurrentOverlay] = useState<string | null>(null);
  const overlayCanvasRef = useRef<HTMLCanvasElement | null>(null);
  
  // Hook editor store para documentos activos
  const editorActive = useEditorStore(s => s.active);
  const editorDocuments = useEditorStore(s => s.documents);

  // Cargar lista de ROMs disponibles (lista hardcodeada ya que Vite no soporta directory listing)
  useEffect(() => {
    // Lista basada en las ROMs que vimos en la carpeta public/roms/
    const knownROMs = [
      'ARMOR.BIN', 'BEDLAM.BIN', 'BERZERK.BIN', 'BerzerkDebugged.vec', 'BirdsofPrey(1999).vec', 
      'BLITZ.BIN', 'CASTLE.BIN', 'CHASM.BIN', 'DKTOWER.BIN', 'FROGGER.BIN',
      'HEADSUP.BIN', 'HYPER.BIN', 'MailPlane.BIN', 'MINE3.BIN', 'MineStorm.bin', 'MSTORM2.BIN', 
      'NARZOD.BIN', 'NEBULA.BIN', 'PATRIOT.BIN', 'PatriotsIII.vec', 'POLAR.BIN', 'POLE.BIN', 
      'RIPOFF.BIN', 'ROCKS.BIN', 'SCRAMBLE.BIN', 'SFPD.BIN', 'SOLAR.BIN', 'SPACE.BIN', 
      'SPIKE.BIN', 'SPINBALL.BIN', 'STARHAWK.BIN', 'starship.vec', 'STARTREK.BIN', 
      'SWEEP.BIN', 'THRUST.BIN', 'Vectrexians-1999-PD.vec', 'WEBWARS.BIN', 'WOTR.BIN'
    ];
    setAvailableROMs(knownROMs);
    console.log('[EmulatorPanel] Loaded ROM list:', knownROMs.length, 'ROMs');
  }, []); // Sin auto-carga de ROM

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

  const onLoadROM = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.bin,.vec';
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;
      
      try {
        console.log(`[EmulatorPanel] Loading ROM: ${file.name} (${file.size} bytes)`);
        
        const arrayBuffer = await file.arrayBuffer();
        const romData = new Uint8Array(arrayBuffer);
        
        const vecx = (window as any).vecx;
        if (!vecx) {
          console.error('[EmulatorPanel] vecx instance not available');
          return;
        }
        
        // Convertir Uint8Array a string para JSVecX
        let cartDataString = '';
        for (let i = 0; i < romData.length; i++) {
          cartDataString += String.fromCharCode(romData[i]);
        }
        
        // Cargar ROM en Globals.cartdata (método correcto para JSVecX)
        // Globals es una variable global, no está en window
        const Globals = (window as any).Globals || (globalThis as any).Globals;
        if (!Globals) {
          console.error('[EmulatorPanel] Globals not available');
          return;
        }
        
        Globals.cartdata = cartDataString;
        console.log(`[EmulatorPanel] ✓ ROM loaded into Globals.cartdata (${romData.length} bytes)`);
        
        // Actualizar estado del ROM cargado
        setLoadedROM(`${file.name} (${romData.length} bytes)`);
        
        // Reset después de cargar - esto copiará cartdata al array cart[]
        vecx.reset();
        console.log('[EmulatorPanel] ✓ Reset after ROM load');
        
        // Si estaba corriendo, reiniciar
        if (status === 'running') {
          vecx.start();
          console.log('[EmulatorPanel] ✓ Restarted after ROM load');
        }
        
      } catch (error) {
        console.error('[EmulatorPanel] Failed to load ROM:', error);
      }
    };
    
    input.click();
  };

  // Función para cargar overlay basado en nombre de ROM
  const loadOverlay = useCallback(async (romName: string) => {
    const baseName = romName.replace(/\.(bin|BIN|vec)$/, '').toLowerCase();
    const overlayPath = `/overlays/${baseName}.png`;
    
    try {
      // Verificar si existe el overlay
      const response = await fetch(overlayPath, { method: 'HEAD' });
      if (response.ok) {
        setCurrentOverlay(overlayPath);
        console.log(`[EmulatorPanel] ✓ Overlay found: ${overlayPath}`);
      } else {
        setCurrentOverlay(null);
        console.log(`[EmulatorPanel] No overlay found for: ${baseName}`);
      }
    } catch (e) {
      setCurrentOverlay(null);
      console.log(`[EmulatorPanel] No overlay found for: ${baseName}`);
    }
  }, []); // sin dependencias

  // Función para cargar ROM desde dropdown
  const loadROMFromDropdown = useCallback(async (romName: string) => {
    if (!romName) return;
    
    try {
      console.log(`[EmulatorPanel] Loading ROM from dropdown: ${romName}`);
      
      const response = await fetch(`/roms/${romName}`);
      if (!response.ok) {
        console.error(`[EmulatorPanel] Failed to fetch ROM: ${response.status}`);
        return;
      }
      
      const arrayBuffer = await response.arrayBuffer();
      const romData = new Uint8Array(arrayBuffer);
      
      const vecx = (window as any).vecx;
      if (!vecx) {
        console.error('[EmulatorPanel] vecx instance not available');
        return;
      }
      
      // Convertir Uint8Array a string para JSVecX
      let cartDataString = '';
      for (let i = 0; i < romData.length; i++) {
        cartDataString += String.fromCharCode(romData[i]);
      }
      
      // Cargar ROM en Globals.cartdata (método correcto para JSVecX)
      const Globals = (window as any).Globals || (globalThis as any).Globals;
      if (!Globals) {
        console.error('[EmulatorPanel] Globals not available');
        return;
      }
      
      Globals.cartdata = cartDataString;
      console.log(`[EmulatorPanel] ✓ ROM loaded into Globals.cartdata (${romData.length} bytes)`);
      
      // Actualizar estado del ROM cargado
      setLoadedROM(`${romName} (${romData.length} bytes)`);
      
      // Cargar overlay automáticamente
      await loadOverlay(romName);
      
      // Reset después de cargar - esto copiará cartdata al array cart[]
      vecx.reset();
      console.log('[EmulatorPanel] ✓ Reset after ROM load');
      
      // Si estaba corriendo, reiniciar
      if (status === 'running') {
        vecx.start();
        console.log('[EmulatorPanel] ✓ Restarted after ROM load');
      }
      
    } catch (error) {
      console.error('[EmulatorPanel] Failed to load ROM from dropdown:', error);
    }
  }, [status, loadOverlay]); // dependencias: status, loadOverlay

  // Manejar cambio de ROM en dropdown
  const handleROMChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const romName = e.target.value;
    setSelectedROM(romName);
    if (romName) {
      loadROMFromDropdown(romName);
    }
  };

  // Toggle overlay visibility
  const toggleOverlay = () => {
    setOverlayEnabled(!overlayEnabled);
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
        
        {loadedROM && <span style={{ fontSize: '11px', opacity: 0.8 }}>ROM: {loadedROM}</span>}
        {currentOverlay && <span style={{ fontSize: '11px', opacity: 0.6 }}>Overlay: {overlayEnabled ? 'On' : 'Off'}</span>}
        
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
          
          {/* Dropdown selector de ROMs */}
          <select 
            value={selectedROM} 
            onChange={handleROMChange}
            style={{
              ...btn,
              background: '#2a4a2a',
              minWidth: '120px'
            }}
          >
            <option value="">Select ROM...</option>
            {availableROMs.map(rom => (
              <option key={rom} value={rom}>{rom}</option>
            ))}
          </select>
          
          {/* Toggle Overlay */}
          {currentOverlay && (
            <button 
              style={{
                ...btn, 
                background: overlayEnabled ? '#2a4a2a' : '#4a2a2a',
                color: overlayEnabled ? '#afa' : '#faa'
              }} 
              onClick={toggleOverlay}
            >
              {overlayEnabled ? 'Hide Overlay' : 'Show Overlay'}
            </button>
          )}
          
          {/* Botón Load ROM manual (como fallback) */}
          <button style={{...btn, background: '#3a3a3a', fontSize: '10px'}} onClick={onLoadROM}>Load File...</button>
        </div>
      </div>

      {/* Canvas para JSVecX con overlay */}
      <div style={{
        flex: 1, 
        display: 'flex', 
        justifyContent: 'center', 
        alignItems: 'center'
      }}>
        <div style={{ position: 'relative', display: 'inline-block' }}>
          <canvas 
            ref={canvasRef} 
            id="screen" 
            width="330" 
            height="410" 
            style={{
              border: '1px solid #333', 
              background: '#000', 
              width: 300, 
              height: 400,
              display: 'block'
            }} 
          />
          
          {/* Overlay image - approach similar to JSVecX original */}
          {currentOverlay && overlayEnabled && (
            <div
              style={{
                position: 'absolute',
                top: 0,
                left: 0,
                width: 300,
                height: 400,
                pointerEvents: 'none',
                zIndex: 10,
                backgroundImage: `url(${currentOverlay})`,
                backgroundSize: '300px 400px',
                backgroundPosition: 'center',
                backgroundRepeat: 'no-repeat',
                mixBlendMode: 'screen',
                opacity: 0.8
              }}
              onError={(e) => {
                console.warn(`[EmulatorPanel] Failed to load overlay: ${currentOverlay}`);
                setCurrentOverlay(null);
              }}
            />
          )}
        </div>
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