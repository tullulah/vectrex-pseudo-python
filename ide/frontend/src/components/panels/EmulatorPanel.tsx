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
  const containerRef = useRef<HTMLDivElement | null>(null);
  const [canvasSize, setCanvasSize] = useState({ width: 300, height: 400 });
  const defaultOverlayLoaded = useRef<boolean>(false); // Flag para evitar recargar overlay por defecto
  
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

  // Inicialización JSVecX con dimensiones responsive
  useEffect(() => {
    let cancelled = false;
    
    const initJSVecX = () => {
      const canvas = canvasRef.current;
      if (!canvas) return;
      
      // Configurar canvas con dimensiones responsive
      canvas.id = 'screen';
      canvas.width = 330;  // Resolución interna fija para JSVecX
      canvas.height = 410;
      canvas.style.width = `${canvasSize.width}px`;
      canvas.style.height = `${canvasSize.height}px`;
      canvas.style.border = '1px solid #333';
      canvas.style.background = '#000';
      
      const vecx = (window as any).vecx;
      if (!vecx) {
        console.error('[EmulatorPanel] Global vecx instance not found');
        return;
      }
      
      console.log(`[EmulatorPanel] Initializing JSVecX with canvas size: ${canvasSize.width}x${canvasSize.height}`);
      
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
  }, [setStatus]); // Solo re-inicializar cuando cambie setStatus, no canvasSize

  // Actualizar dimensiones del canvas sin re-inicializar JSVecX
  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas) {
      canvas.style.width = `${canvasSize.width}px`;
      canvas.style.height = `${canvasSize.height}px`;
      console.log(`[EmulatorPanel] Canvas resized to: ${canvasSize.width}x${canvasSize.height}`);
    }
  }, [canvasSize]);

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
        
        // Resetear combo selector (carga manual no debe seleccionar combo)
        setSelectedROM('');
        
        // Recalcular overlay basado en nombre del archivo
        await loadOverlay(file.name);
        
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
        // No se encontró overlay - quitarlo
        setCurrentOverlay(null);
        console.log(`[EmulatorPanel] No overlay found for: ${baseName} - removing overlay`);
      }
    } catch (e) {
      // Error al buscar overlay - quitarlo
      setCurrentOverlay(null);
      console.log(`[EmulatorPanel] Error loading overlay for: ${baseName} - removing overlay`);
    }
  }, []); // sin dependencias

  // Cargar overlay de Minestorm al arrancar (default BIOS game) - SOLO UNA VEZ
  useEffect(() => {
    const loadDefaultOverlay = async () => {
      if (defaultOverlayLoaded.current) return; // Ya se cargó, no volver a cargar
      
      // Esperar un poco para que JSVecX esté completamente inicializado
      setTimeout(async () => {
        await loadOverlay('minestorm.bin');
        setLoadedROM('BIOS - Minestorm');
        defaultOverlayLoaded.current = true; // Marcar como cargado
      }, 1500);
    };
    loadDefaultOverlay();
  }, []); // Sin dependencias - solo se ejecuta al montar el componente

  // Responsive canvas sizing
  useEffect(() => {
    const updateCanvasSize = () => {
      if (!containerRef.current) return;
      
      const container = containerRef.current;
      const rect = container.getBoundingClientRect();
      
      // Aspect ratio Vectrex: 330x410 (aprox 4:5)
      const aspectRatio = 330 / 410;
      
      // Calcular tamaño máximo que cabe en el contenedor
      const maxWidth = rect.width - 40; // padding
      const maxHeight = rect.height - 40;
      
      let width = maxWidth;
      let height = width / aspectRatio;
      
      // Si la altura calculada es muy grande, ajustar por altura
      if (height > maxHeight) {
        height = maxHeight;
        width = height * aspectRatio;
      }
      
      // Mínimo tamaño
      width = Math.max(200, width);
      height = Math.max(250, height);
      
      // Máximo tamaño (mantener buena calidad)
      width = Math.min(500, width);
      height = Math.min(625, height);
      
      setCanvasSize({ width: Math.round(width), height: Math.round(height) });
    };
    
    // Ejecutar al inicio
    updateCanvasSize();
    
    // Observer para cambios de tamaño
    const resizeObserver = new ResizeObserver(updateCanvasSize);
    if (containerRef.current) {
      resizeObserver.observe(containerRef.current);
    }
    
    return () => {
      resizeObserver.disconnect();
    };
  }, []);

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

  // Listener para cargar binarios compilados automáticamente
  useEffect(() => {
    const electronAPI: any = (window as any).electronAPI;
    if (!electronAPI?.onCompiledBin) return;

    const handleCompiledBin = (payload: { base64: string; size: number; binPath: string }) => {
      console.log(`[EmulatorPanel] Loading compiled binary: ${payload.binPath} (${payload.size} bytes)`);
      
      try {
        // Convertir base64 a bytes y cargar en JSVecX
        const binaryData = atob(payload.base64);
        const vecx = (window as any).vecx;
        
        if (!vecx) {
          console.error('[EmulatorPanel] JSVecX instance not available for loading binary');
          return;
        }

        // Detener emulador antes de cargar
        vecx.stop();
        
        // Cargar el binario en la instancia global Globals.cartdata
        const Globals = (window as any).Globals;
        if (Globals) {
          Globals.cartdata = binaryData;
          console.log('[EmulatorPanel] ✓ Binary loaded into Globals.cartdata');
        }
        
        // Reset y reiniciar
        vecx.reset();
        vecx.start();
        
        // Actualizar ROM cargada y buscar overlay
        const romName = payload.binPath.split(/[/\\]/).pop()?.replace(/\.(bin|BIN)$/, '') || 'compiled';
        setLoadedROM(`Compiled - ${romName}`);
        
        // Intentar cargar overlay si existe
        loadOverlay(romName + '.bin');
        
        console.log('[EmulatorPanel] ✓ Compiled binary loaded and emulator restarted');
        
      } catch (error) {
        console.error('[EmulatorPanel] Failed to load compiled binary:', error);
      }
    };

    electronAPI.onCompiledBin(handleCompiledBin);
    console.log('[EmulatorPanel] ✓ Registered onCompiledBin listener');
    
    // No cleanup function needed - onCompiledBin typically doesn't return one
  }, [loadOverlay, setLoadedROM]);

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
      {/* Controles responsive */}
      <div style={{
        display: 'flex', 
        flexDirection: 'column',
        gap: 8, 
        marginBottom: 12
      }}>
        {/* Fila 1: Info y status */}
        <div style={{
          display: 'flex',
          alignItems: 'center',
          gap: 8,
          flexWrap: 'wrap',
          fontSize: '11px'
        }}>
          <span>Status: <span style={{
            color: status === 'running' ? '#0f0' : status === 'paused' ? '#fa0' : '#f55'
          }}>{status}</span></span>
          
          {loadedROM && <span style={{ opacity: 0.8 }}>ROM: {loadedROM}</span>}
          {currentOverlay && <span style={{ opacity: 0.6 }}>Overlay: {overlayEnabled ? 'On' : 'Off'}</span>}
          
          {/* Botón de control de audio */}
          <button 
            style={{
              ...btn, 
              background: audioEnabled ? '#2a4a2a' : '#4a2a2a',
              color: audioEnabled ? '#afa' : '#faa',
              fontSize: '10px',
              padding: '4px 8px'
            }} 
            onClick={() => {
              const newState = !audioEnabled;
              setAudioEnabled(newState); 
              try { 
                localStorage.setItem('emu_audio_enabled', newState ? '1' : '0'); 
              } catch{} 
              
              // Control JSVecX audio directamente
              const vecx = (window as any).vecx;
              if (vecx) {
                console.log('[EmulatorPanel] JSVecX audio methods:', Object.keys(vecx).filter(k => k.toLowerCase().includes('snd') || k.toLowerCase().includes('audio') || k.toLowerCase().includes('sound')));
                
                try {
                  if (newState) {
                    // HABILITAR AUDIO
                    if (vecx.toggleSoundEnabled) vecx.toggleSoundEnabled(true);
                    if (vecx.enableSound) vecx.enableSound();
                    if (vecx.volume !== undefined) vecx.volume = 1.0;
                    if (vecx.soundEnabled !== undefined) vecx.soundEnabled = true;
                    psgAudio.start(); // También iniciar PSG Audio
                    console.log('[EmulatorPanel] ✓ Audio HABILITADO');
                  } else {
                    // MUTEAR AUDIO
                    if (vecx.toggleSoundEnabled) vecx.toggleSoundEnabled(false);
                    if (vecx.disableSound) vecx.disableSound();
                    if (vecx.volume !== undefined) vecx.volume = 0.0;
                    if (vecx.soundEnabled !== undefined) vecx.soundEnabled = false;
                    psgAudio.stop(); // También parar PSG Audio
                    console.log('[EmulatorPanel] ✓ Audio MUTEADO');
                  }
                } catch (e) {
                  console.warn('[EmulatorPanel] Could not control JSVecX audio:', e);
                }
              }
            }}
          >
            {audioEnabled ? '🔊' : '🔇'} {audioEnabled ? 'Mute' : 'Audio'}
          </button>
        </div>
        
        {/* Fila 2: Controles principales */}
        <div style={{
          display: 'flex',
          alignItems: 'center',
          gap: 6,
          flexWrap: 'wrap'
        }}>
          {/* Controles de reproducción */}
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
              minWidth: '100px',
              maxWidth: '150px'
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
                color: overlayEnabled ? '#afa' : '#faa',
                fontSize: '10px'
              }} 
              onClick={toggleOverlay}
            >
              {overlayEnabled ? 'Hide' : 'Show'} Overlay
            </button>
          )}
          
          {/* Botón Load ROM manual (como fallback) */}
          <button style={{...btn, background: '#3a3a3a', fontSize: '10px'}} onClick={onLoadROM}>
            Load File...
          </button>
        </div>
      </div>

      {/* Canvas para JSVecX con overlay responsive */}
      <div 
        ref={containerRef}
        style={{
          flex: 1, 
          display: 'flex', 
          justifyContent: 'center', 
          alignItems: 'center',
          minHeight: '300px'
        }}
      >
        <div style={{ position: 'relative', display: 'inline-block' }}>
          <canvas 
            ref={canvasRef} 
            id="screen" 
            width="330" 
            height="410" 
            style={{
              border: '1px solid #333', 
              background: '#000', 
              width: canvasSize.width, 
              height: canvasSize.height,
              display: 'block',
              position: 'relative',
              zIndex: 1
            }} 
          />
          
          {/* Sistema dual-overlay: mezcla de colores + visibilidad */}
          {currentOverlay && overlayEnabled && (
            <>
              {/* Overlay 1: Multiply - mezcla colores con los vectores */}
              <div
                style={{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  width: canvasSize.width,
                  height: canvasSize.height,
                  pointerEvents: 'none',
                  zIndex: 2,
                  backgroundImage: `url(${currentOverlay})`,
                  backgroundSize: `${canvasSize.width}px ${canvasSize.height}px`,
                  backgroundPosition: 'center',
                  backgroundRepeat: 'no-repeat',
                  mixBlendMode: 'multiply',
                  opacity: 0.7
                }}
                onError={(e) => {
                  console.warn(`[EmulatorPanel] Failed to load overlay (multiply): ${currentOverlay}`);
                  setCurrentOverlay(null);
                }}
              />
              {/* Overlay 2: Screen - hace visible el overlay sin afectar vectores */}
              <div
                style={{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  width: canvasSize.width,
                  height: canvasSize.height,
                  pointerEvents: 'none',
                  zIndex: 3,
                  backgroundImage: `url(${currentOverlay})`,
                  backgroundSize: `${canvasSize.width}px ${canvasSize.height}px`,
                  backgroundPosition: 'center',
                  backgroundRepeat: 'no-repeat',
                  mixBlendMode: 'screen',
                  opacity: 0.5
                }}
              />
            </>
          )}
        </div>
      </div>



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