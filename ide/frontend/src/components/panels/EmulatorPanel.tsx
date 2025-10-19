import React, { useEffect, useRef, useState, useCallback } from 'react';
import { useEmulatorStore } from '../../state/emulatorStore';
import { useEditorStore } from '../../state/editorStore';
import { useEmulatorSettings } from '../../state/emulatorSettings';
import { useDebugStore } from '../../state/debugStore';
import { psgAudio } from '../../psgAudio';
import { inputManager } from '../../inputManager';
import { asmAddressToVpyLine, formatAddress } from '../../utils/debugHelpers';

// Tipos para JSVecX
interface VecxMetrics {
  totalCycles: number;
  instructionCount: number;
  frameCount: number;
  running: boolean;
}

interface VecxRegs {
  PC: number;
  A: number; B: number;
  X: number; Y: number; U: number; S: number;
  DP: number; CC: number;
}

// Componente simple para gráficas de barras
const MiniChart: React.FC<{ 
  label: string; 
  value: number; 
  max: number; 
  color: string; 
  dangerZone?: number;
  unit?: string;
}> = ({ label, value, max, color, dangerZone, unit = '' }) => {
  const percentage = Math.min((value / max) * 100, 100);
  const isDanger = dangerZone && value >= dangerZone;
  const dangerPercentage = dangerZone ? (dangerZone / max) * 100 : 0;
  
  return (
    <div style={{ marginBottom: '8px', position: 'relative' }}>
      <div style={{
        display: 'flex',
        justifyContent: 'space-between',
        fontSize: '9px',
        marginBottom: '2px',
        color: isDanger ? '#ff6666' : '#aaa'
      }}>
        <span style={{ fontWeight: isDanger ? 'bold' : 'normal' }}>
          {label} {isDanger ? '⚠️' : ''}
        </span>
        <span>{value.toLocaleString()}{unit}</span>
      </div>
      <div style={{
        width: '100%',
        height: '14px',
        background: '#2a2a2a',
        borderRadius: '7px',
        overflow: 'hidden',
        border: '1px solid #444',
        position: 'relative'
      }}>
        {/* Zona de peligro de fondo */}
        {dangerZone && (
          <div style={{
            position: 'absolute',
            left: `${dangerPercentage}%`,
            width: `${100 - dangerPercentage}%`,
            height: '100%',
            background: 'rgba(255, 68, 68, 0.15)',
            zIndex: 1
          }} />
        )}
        
        {/* Barra de progreso principal */}
        <div style={{
          width: `${percentage}%`,
          height: '100%',
          background: isDanger ? 
            'linear-gradient(90deg, #ff4444, #ff6666)' :
            `linear-gradient(90deg, ${color}, ${color}99)`,
          transition: 'width 0.5s ease-out',
          borderRadius: '7px',
          zIndex: 2,
          position: 'relative',
          boxShadow: isDanger ? '0 0 8px rgba(255, 68, 68, 0.5)' : `0 0 4px ${color}33`
        }} />
        
        {/* Línea marcadora de zona peligro */}
        {dangerZone && (
          <div style={{
            position: 'absolute',
            left: `${dangerPercentage}%`,
            width: '2px',
            height: '100%',
            background: '#ff4444',
            zIndex: 3,
            boxShadow: '0 0 3px #ff4444'
          }} />
        )}
      </div>
    </div>
  );
};

// Componente para mostrar información técnica del emulador (métricas reales)
const EmulatorOutputInfo: React.FC = () => {
  const [metrics, setMetrics] = useState<VecxMetrics | null>(null);
  const [regs, setRegs] = useState<VecxRegs | null>(null);
  const [vecxRunning, setVecxRunning] = useState<boolean>(false);
  
  // Get debug state from debugStore
  const debugState = useDebugStore(s => s.state);

  const fetchStats = () => {
    try {
      const vecx = (window as any).vecx;
      if (!vecx) {
        setMetrics(null);
        setRegs(null);
        setVecxRunning(false);
        return;
      }
      
      const fetchedMetrics = vecx.getMetrics && vecx.getMetrics();
      const fetchedRegs = vecx.getRegisters && vecx.getRegisters();
      
      setMetrics(fetchedMetrics || null);
      setRegs(fetchedRegs || null);
      setVecxRunning(vecx.running || false);
    } catch (e) {
      setMetrics(null);
      setRegs(null);
      setVecxRunning(false);
    }
  };

  useEffect(() => {
    fetchStats();
    const interval = setInterval(fetchStats, 1000);
    return () => clearInterval(interval);
  }, []);

  const hex8 = (v: any) => typeof v === 'number' ? `0x${(v & 0xFF).toString(16).padStart(2, '0')}` : '--';
  const hex16 = (v: any) => typeof v === 'number' ? `0x${(v & 0xFFFF).toString(16).padStart(4, '0')}` : '--';
  
  const avgCyclesPerFrame = (metrics && metrics.frameCount > 0) ? 
    Math.round(metrics.totalCycles / metrics.frameCount) : 0;

  return (
    <div style={{
      background: '#1a1a1a',
      border: '1px solid #333',
      borderRadius: 4,
      padding: '6px 10px',
      marginBottom: 12,
      fontSize: '11px',
      color: '#ccc',
      fontFamily: 'monospace'
    }}>
      <div style={{ 
        fontWeight: 'bold', 
        color: '#0f0',
        marginBottom: '4px',
        fontSize: '10px',
        textTransform: 'uppercase',
        letterSpacing: '0.5px',
        fontFamily: 'system-ui'
      }}>
        Emulator Output
      </div>
      
      <div style={{ marginBottom: '2px' }}>
        PC: {hex16(regs?.PC)}
      </div>
      
      <div style={{ marginBottom: '2px' }}>
        Debug State: <span style={{ 
          color: debugState === 'running' ? '#0f0' : debugState === 'paused' ? '#ff0' : '#f00',
          fontWeight: 'bold'
        }}>{debugState.toUpperCase()}</span>
        {' | '}
        JSVecx: <span style={{ 
          color: vecxRunning ? '#0f0' : '#f00',
          fontWeight: 'bold'
        }}>{vecxRunning ? 'RUNNING' : 'STOPPED'}</span>
        {debugState !== (vecxRunning ? 'running' : 'stopped') && (
          <span style={{ color: '#f00', marginLeft: '8px' }}>⚠️ MISMATCH</span>
        )}
      </div>
      
      <div style={{ marginBottom: '2px' }}>
        A: {hex8(regs?.A)} B: {hex8(regs?.B)} X: {hex16(regs?.X)} Y: {hex16(regs?.Y)} U: {hex16(regs?.U)} S: {hex16(regs?.S)} DP: {hex8(regs?.DP)} CC: {hex8(regs?.CC)}
      </div>
      
      <div style={{ marginBottom: '2px' }}>
        Total Cycles: {metrics?.totalCycles ?? 0}
      </div>
      
      <div style={{ marginBottom: '2px' }}>
        Instructions: {metrics?.instructionCount ?? 0}
      </div>
      
      <div style={{ marginBottom: '2px' }}>
        Frames: {metrics?.frameCount ?? 0}
      </div>
      
      <div>
        Avg Cycles/frame: {avgCyclesPerFrame > 0 ? avgCyclesPerFrame : '--'}
      </div>
    </div>
  );
};

export const EmulatorPanel: React.FC = () => {
  const status = useEmulatorStore(s => s.status);
  const setStatus = useEmulatorStore(s => s.setStatus);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  
  // Persistent emulator settings
  const { 
    audioEnabled, 
    overlayEnabled, 
    lastRomPath, 
    lastRomName,
    setAudioEnabled, 
    setOverlayEnabled, 
    setLastRom 
  } = useEmulatorSettings();
  
  // Estados básicos necesarios
  const [audioStats, setAudioStats] = useState<{ 
    sampleRate:number; pushed:number; consumed:number; 
    bufferedSamples:number; bufferedMs:number; overflowCount:number 
  }|null>(null);
  const [loadedROM, setLoadedROM] = useState<string | null>(null);
  const [availableROMs, setAvailableROMs] = useState<string[]>([]);
  const [selectedROM, setSelectedROM] = useState<string>(lastRomName || '');
  const [currentOverlay, setCurrentOverlay] = useState<string | null>(null);
  const overlayCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const containerRef = useRef<HTMLDivElement | null>(null);
  const [canvasSize, setCanvasSize] = useState({ width: 300, height: 400 });
  const defaultOverlayLoaded = useRef<boolean>(false); // Flag para evitar recargar overlay por defecto
  
  // Phase 3: Breakpoint system
  const [breakpoints, setBreakpoints] = useState<Set<number>>(new Set());
  const debugState = useDebugStore(s => s.state);
  const pdbData = useDebugStore(s => s.pdbData);
  const breakpointCheckIntervalRef = useRef<number | null>(null);
  
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
      
      // Optimización para múltiples lecturas de canvas (elimina warning Canvas2D)
      // JSVecX hace muchas operaciones getImageData, necesitamos willReadFrequently
      try {
        const ctx = canvas.getContext('2d', { willReadFrequently: true });
        if (ctx) {
          console.log('[EmulatorPanel] Canvas context configured with willReadFrequently optimization');
          // Asegurar que JSVecX use este contexto optimizado
          (canvas as any)._optimizedContext = ctx;
        }
      } catch (e) {
        console.warn('[EmulatorPanel] Could not configure willReadFrequently, using default context');
      }
      
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

  // Función para cargar overlay basado en nombre de ROM (definida antes de ser usada)
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

  // Phase 3: Breakpoint management functions
  const addBreakpoint = useCallback((address: number) => {
    console.log(`[EmulatorPanel] addBreakpoint called with:`, address, `type: ${typeof address}`);
    setBreakpoints(prev => {
      const next = new Set(prev);
      next.add(address);
      console.log(`[EmulatorPanel] ✓ Breakpoint added at ${formatAddress(address)}`);
      console.log(`[EmulatorPanel] 📍 Total breakpoints: ${next.size}, addresses: [${Array.from(next).map(formatAddress).join(', ')}]`);
      console.log(`[EmulatorPanel] 📍 Raw Set contents:`, Array.from(next));
      return next;
    });
  }, []);

  const removeBreakpoint = useCallback((address: number) => {
    setBreakpoints(prev => {
      const next = new Set(prev);
      next.delete(address);
      console.log(`[EmulatorPanel] ✓ Breakpoint removed from ${formatAddress(address)}`);
      console.log(`[EmulatorPanel] 📍 Total breakpoints: ${next.size}, addresses: [${Array.from(next).map(formatAddress).join(', ')}]`);
      return next;
    });
  }, []);

  const toggleBreakpoint = useCallback((address: number) => {
    setBreakpoints(prev => {
      const next = new Set(prev);
      if (next.has(address)) {
        next.delete(address);
        console.log(`[EmulatorPanel] ✓ Breakpoint removed from ${formatAddress(address)}`);
      } else {
        next.add(address);
        console.log(`[EmulatorPanel] ✓ Breakpoint added at ${formatAddress(address)}`);
      }
      return next;
    });
  }, []);

  const clearAllBreakpoints = useCallback(() => {
    setBreakpoints(new Set());
    console.log('[EmulatorPanel] ✓ All breakpoints cleared');
  }, []);

  // Expose breakpoint functions globally for debugStore integration
  useEffect(() => {
    (window as any).emulatorDebug = {
      addBreakpoint,
      removeBreakpoint,
      toggleBreakpoint,
      clearAllBreakpoints,
      getBreakpoints: () => Array.from(breakpoints)
    };
    
    return () => {
      delete (window as any).emulatorDebug;
    };
  }, [addBreakpoint, removeBreakpoint, toggleBreakpoint, clearAllBreakpoints, breakpoints]);

  // Helper function to apply audio state to vecx
  const applyAudioState = useCallback((enabled?: boolean) => {
    const vecx = (window as any).vecx;
    if (!vecx || !vecx.e8910) {
      console.warn('[EmulatorPanel] Cannot apply audio state - vecx or e8910 not available');
      return;
    }
    
    const audioState = enabled !== undefined ? enabled : audioEnabled;
    
    try {
      // Verificar estado actual del audio en e8910
      const currentAudioEnabled = vecx.e8910.enabled;
      console.log('[EmulatorPanel] Applying audio state:', audioState ? 'enabled' : 'muted', {
        currentAudioEnabled,
        targetAudioState: audioState,
        needsToggle: currentAudioEnabled !== audioState
      });
      
      // Solo hacer toggle si el estado actual es diferente al deseado
      if (currentAudioEnabled !== audioState) {
        if (vecx.toggleSoundEnabled) {
          const newState = vecx.toggleSoundEnabled();
          console.log(`[EmulatorPanel] ✓ Toggled audio: ${currentAudioEnabled} → ${newState}`);
        } else {
          console.warn('[EmulatorPanel] toggleSoundEnabled not available');
        }
      } else {
        console.log(`[EmulatorPanel] ✓ Audio already in desired state: ${audioState}`);
      }
      
      // Verificar estado final
      const finalState = vecx.e8910.enabled;
      console.log('[EmulatorPanel] ✓ Audio state application complete. Final state:', finalState);
      
    } catch (error) {
      console.error('[EmulatorPanel] Error applying audio state:', error);
    }
  }, [audioEnabled]);

  // Helper function to get current audio state from vecx
  const getCurrentAudioState = useCallback(() => {
    const vecx = (window as any).vecx;
    if (vecx && vecx.e8910) {
      return vecx.e8910.enabled;
    }
    return audioEnabled; // fallback to stored state
  }, [audioEnabled]);

  // Sync audio state periodically to ensure UI matches reality
  useEffect(() => {
    const interval = setInterval(() => {
      const vecx = (window as any).vecx;
      if (vecx && vecx.e8910 && status === 'running') {
        const actualState = vecx.e8910.enabled;
        if (actualState !== audioEnabled) {
          console.log('[EmulatorPanel] Audio state desync detected, syncing:', actualState);
          setAudioEnabled(actualState);
        }
      }
    }, 1000); // Check every second

    return () => clearInterval(interval);
  }, [audioEnabled, status, setAudioEnabled]);

  // Phase 3: Breakpoint detection system (REACTIVE - no polling)
  // The WASM emulator checks breakpoints after EVERY instruction
  // We just need to detect when it has paused
  const checkBreakpointHit = useCallback(() => {
    // Solo verificar si estamos en modo debug
    if (debugState !== 'running') return;
    
    try {
      const vecx = (window as any).vecx;
      if (!vecx || !vecx.e6809) return;
      
      // Check if WASM paused by breakpoint (reactive check)
      if (vecx.isPausedByBreakpoint && vecx.isPausedByBreakpoint()) {
        const currentPC = vecx.e6809?.reg_pc;
        console.log(`[EmulatorPanel] 🔴 Breakpoint hit detected at PC: ${formatAddress(currentPC)}`);
        
        // Pausar emulador
        if (vecx.running) {
          vecx.stop();
          console.log('[EmulatorPanel] ✓ Emulator paused by breakpoint');
        }
        
        // Actualizar debug state
        const debugStore = useDebugStore.getState();
        debugStore.setState('paused');
        debugStore.setCurrentAsmAddress(formatAddress(currentPC));
        
        // Map address → VPy line using helper
        if (pdbData) {
          const vpyLine = asmAddressToVpyLine(currentPC, pdbData);
          if (vpyLine !== null) {
            debugStore.setCurrentVpyLine(vpyLine);
            console.log(`[EmulatorPanel] ✓ Mapped to VPy line: ${vpyLine}`);
          } else {
            console.log(`[EmulatorPanel] ⚠️  No VPy line mapping for address ${formatAddress(currentPC)}`);
          }
        }
        
        console.log('[EmulatorPanel] 🛑 Execution paused at breakpoint');
      }
    } catch (e) {
      console.error('[EmulatorPanel] Error checking breakpoint:', e);
    }
  }, [debugState, pdbData]);

  // Phase 3: Setup breakpoint checking interval
  useEffect(() => {
    // Limpiar interval previo
    if (breakpointCheckIntervalRef.current !== null) {
      clearInterval(breakpointCheckIntervalRef.current);
      breakpointCheckIntervalRef.current = null;
    }
    
    // Activar verificación cuando estamos en debug session (running O paused)
    // checkBreakpointHit() internamente verifica que solo actúe cuando running
    if (debugState === 'running' || debugState === 'paused') {
      console.log(`[EmulatorPanel] ✓ Starting reactive breakpoint checking (state=${debugState})`);
      breakpointCheckIntervalRef.current = window.setInterval(checkBreakpointHit, 50);
    } else {
      console.log('[EmulatorPanel] Breakpoint checking disabled (stopped)');
    }
    
    return () => {
      if (breakpointCheckIntervalRef.current !== null) {
        clearInterval(breakpointCheckIntervalRef.current);
        breakpointCheckIntervalRef.current = null;
      }
    };
  }, [debugState, checkBreakpointHit]);

  // Phase 3: Listen for debug commands from debugStore
  useEffect(() => {
    const handleDebugMessage = (event: MessageEvent) => {
      if (event.source !== window) return;
      
      const vecx = (window as any).vecx;
      if (!vecx) return;
      
      const { type, address, line } = event.data;
      
      switch (type) {
        case 'debug-add-breakpoint':
          console.log(`[EmulatorPanel] ➕ Adding breakpoint: line ${line} → address ${address}`);
          if (address !== undefined) {
            // Call WASM API directly
            if (vecx.addBreakpoint) {
              vecx.addBreakpoint(address);
              console.log(`[EmulatorPanel] ✓ Breakpoint added at 0x${address.toString(16)}`);
            } else {
              console.error('[EmulatorPanel] ❌ vecx.addBreakpoint not available');
            }
          }
          break;
          
        case 'debug-remove-breakpoint':
          console.log(`[EmulatorPanel] ➖ Removing breakpoint: line ${line} → address ${address}`);
          if (address !== undefined) {
            // Call WASM API directly
            if (vecx.removeBreakpoint) {
              vecx.removeBreakpoint(address);
              console.log(`[EmulatorPanel] ✓ Breakpoint removed at 0x${address.toString(16)}`);
            } else {
              console.error('[EmulatorPanel] ❌ vecx.removeBreakpoint not available');
            }
          }
          break;
          
        case 'debug-clear-breakpoints':
          console.log('[EmulatorPanel] 🗑️  Clearing all breakpoints');
          if (vecx.clearBreakpoints) {
            vecx.clearBreakpoints();
            console.log('[EmulatorPanel] ✓ All breakpoints cleared');
          }
          break;
          
        case 'debug-continue':
          console.log('[EmulatorPanel] 🟢 Debug: Continue execution');
          // Resume from breakpoint if paused by one
          if (vecx.isPausedByBreakpoint && vecx.isPausedByBreakpoint()) {
            console.log('[EmulatorPanel] 🔓 Resuming from breakpoint');
            if (vecx.resumeFromBreakpoint) {
              vecx.resumeFromBreakpoint();
            }
          }
          if (!vecx.running) {
            vecx.start();
            console.log('[EmulatorPanel] ✓ Emulator started');
          }
          break;
          
        case 'debug-pause':
          console.log('[EmulatorPanel] ⏸️  Debug: Pause execution');
          if (vecx.running) {
            vecx.stop();
          }
          break;
          
        case 'debug-stop':
          console.log('[EmulatorPanel] 🛑 Debug: Stop execution');
          if (vecx.running) {
            vecx.stop();
          }
          vecx.reset();
          break;
          
        case 'debug-step-over':
          console.log('[EmulatorPanel] ⏭️  Debug: Step over');
          // TODO: Implement proper step-over with temporary breakpoint
          break;
          
        case 'debug-step-into':
          console.log('[EmulatorPanel] 🔽 Debug: Step into');
          // TODO: Implement single-step execution
          break;
          
        case 'debug-step-out':
          console.log('[EmulatorPanel] 🔼 Debug: Step out');
          // TODO: Implement step out (run until RTS)
          break;
      }
    };
    
    window.addEventListener('message', handleDebugMessage);
    return () => window.removeEventListener('message', handleDebugMessage);
  }, [addBreakpoint, removeBreakpoint]);

  // Función para cargar ROM desde dropdown (definida antes de useEffects que la usan)
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
      
      // Reset DOBLE después de cargar - esto copiará cartdata al array cart[]
      // Primer reset para cargar cartdata
      vecx.reset();
      console.log('[EmulatorPanel] ✓ First reset after ROM load');
      
      // Esperar un poco y hacer segundo reset para asegurarse
      setTimeout(() => {
        vecx.reset();
        console.log('[EmulatorPanel] ✓ Second reset after ROM load');
        
        // Si estaba corriendo, reiniciar
        if (status === 'running') {
          vecx.start();
          console.log('[EmulatorPanel] ✓ Restarted after ROM load');
        }
        
        // CRÍTICO: Aplicar estado de audio después de reset/start
        setTimeout(() => {
          applyAudioState();
        }, 100);
      }, 50);
      
    } catch (error) {
      console.error('[EmulatorPanel] Failed to load ROM from dropdown:', error);
    }
  }, [status, loadOverlay, applyAudioState]); // dependencias: status, loadOverlay, applyAudioState

  // Auto-load last ROM on emulator start - only trigger when availableROMs is populated
  useEffect(() => {
    console.log('[EmulatorPanel] Auto-load ROM check:', {
      lastRomName,
      selectedROM,
      availableROMs: availableROMs.length,
      loadedROM,
      condition1: !!lastRomName,
      condition2: availableROMs.length > 0,
      condition3: !loadedROM?.includes(lastRomName || '')
    });
    
    // Only auto-load if we have a stored ROM, available ROMs are loaded, and we haven't loaded this ROM yet
    if (lastRomName && availableROMs.length > 0 && !loadedROM?.includes(lastRomName)) {
      console.log('[EmulatorPanel] Auto-restoring last ROM:', lastRomName, 'from', availableROMs.length, 'available ROMs');
      setSelectedROM(lastRomName);
      // If it's in the dropdown, load it automatically
      if (availableROMs.includes(lastRomName)) {
        console.log('[EmulatorPanel] ✓ Found ROM in list, loading automatically:', lastRomName);
        loadROMFromDropdown(lastRomName);
      } else {
        console.log('[EmulatorPanel] ⚠️ ROM not found in available list:', lastRomName, 'Available:', availableROMs);
      }
    }
  }, [lastRomName, availableROMs, loadedROM]); // Removed selectedROM to avoid dependency cycle

  // Apply initial audio state when emulator starts
  useEffect(() => {
    if (status === 'running') {
      applyAudioState();
    }
  }, [status, applyAudioState]); // Apply when status changes to running

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
        
        // Save the loaded ROM info for persistence
        setLastRom(null, file.name); // File object doesn't have path, just name
        
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

  // Listener para cargar binarios compilados automáticamente
  useEffect(() => {
    const electronAPI: any = (window as any).electronAPI;
    if (!electronAPI?.onCompiledBin) return;

    const handleCompiledBin = (payload: { base64: string; size: number; binPath: string; pdbData?: any }) => {
      console.log(`[EmulatorPanel] Loading compiled binary: ${payload.binPath} (${payload.size} bytes)`);
      
      // Si hay datos de debug (.pdb), cargarlos en el debugStore
      if (payload.pdbData) {
        console.log('[EmulatorPanel] ✓ Debug symbols (.pdb) received');
        useDebugStore.getState().loadPdbData(payload.pdbData);
      }
      
      // Verificar si estamos cargando para debug session (no auto-start)
      const loadingForDebug = useDebugStore.getState().loadingForDebug;
      
      try {
        // Convertir base64 a bytes y cargar en JSVecX
        const binaryData = atob(payload.base64);
        const vecx = (window as any).vecx;
        
        if (!vecx) {
          console.error('[EmulatorPanel] JSVecX instance not available for loading binary');
          return;
        }

        // Detener emulador antes de cargar
        console.log('[EmulatorPanel] Stopping emulator before load...');
        vecx.stop();
        console.log('[EmulatorPanel] Emulator stopped');
        
        // Cargar el binario en la instancia global Globals.cartdata
        const Globals = (window as any).Globals;
        if (Globals) {
          Globals.cartdata = binaryData;
          console.log('[EmulatorPanel] ✓ Binary loaded into Globals.cartdata');
        }
        
        // Reset
        console.log('[EmulatorPanel] Resetting emulator...');
        vecx.reset();
        console.log('[EmulatorPanel] Emulator reset complete');
        
        // Comportamiento de auto-start dependiendo del modo:
        // - Normal compilation (F7): auto-start
        // - Debug session (Ctrl+F5): ALWAYS auto-start in 'running' mode
        console.log(`[EmulatorPanel] Binary load - loadingForDebug=${loadingForDebug}`);
        
        if (!loadingForDebug) {
          // Compilación normal → siempre auto-start
          vecx.start();
          console.log('[EmulatorPanel] ✓ Emulator started (normal mode)');
        } else {
          // Modo debug → SETEAR estado a 'running' e iniciar
          const debugStore = useDebugStore.getState();
          debugStore.setState('running');
          console.log('[EmulatorPanel] ✓ Debug mode: state set to running');
          
          vecx.start();
          console.log('[EmulatorPanel] ✓ Emulator started in debug mode (running until breakpoint)');
          
          // Limpiar flag
          debugStore.setLoadingForDebug(false);
        }
        
        // Actualizar ROM cargada y buscar overlay
        const romName = payload.binPath.split(/[/\\]/).pop()?.replace(/\.(bin|BIN)$/, '') || 'compiled';
        setLoadedROM(`Compiled - ${romName}`);
        
        // Save the compiled ROM info for persistence
        setLastRom(payload.binPath, `Compiled - ${romName}`);
        
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
    // Save the last ROM selection
    setLastRom(null, romName); // We don't have the path here, just the name
    if (romName) {
      loadROMFromDropdown(romName);
    }
  };

  // Toggle overlay visibility
  const toggleOverlay = () => {
    const newState = !overlayEnabled;
    setOverlayEnabled(newState);
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
      {/* Controles de ROM - Simplificados */}
      <div style={{
        display: 'flex',
        alignItems: 'center',
        gap: 8,
        marginBottom: 8,
        justifyContent: 'center'
      }}>
        {/* Dropdown selector de ROMs */}
        <select 
          value={selectedROM} 
          onChange={handleROMChange}
          style={{
            ...btn,
            background: '#2a4a2a',
            minWidth: '120px',
            maxWidth: '180px'
          }}
        >
          <option value="">Select ROM...</option>
          {availableROMs.map(rom => (
            <option key={rom} value={rom}>{rom}</option>
          ))}
        </select>
        
        {/* Botón Load ROM manual (como fallback) */}
        <button 
          style={{
            ...btn, 
            background: '#3a3a3a', 
            fontSize: '10px',
            display: 'flex',
            alignItems: 'center',
            gap: '4px',
            padding: '6px 8px'
          }} 
          onClick={onLoadROM}
          title="Load ROM file from disk"
        >
          📁 <span>Load File...</span>
        </button>
      </div>

      {/* Canvas para JSVecX con overlay responsive */}
      <div 
        ref={containerRef}
        style={{
          flex: 1,
          display: 'flex', 
          justifyContent: 'center', 
          alignItems: 'center',
          minHeight: '400px',
          marginBottom: '8px'
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
                  opacity: 1
                }}
              />
            </>
          )}
        </div>
      </div>

      {/* Emulator Output - Información técnica del emulador */}
      <EmulatorOutputInfo />

      {/* Controles principales debajo del canvas - Estilo homogéneo */}
      <div style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        gap: 8,
        marginTop: 12,
        paddingTop: 8,
        borderTop: '1px solid #333'
      }}>
        {/* Botón Start/Stop unificado */}
        <button 
          style={{
            ...btn,
            backgroundColor: status === 'running' ? '#4a2a2a' : '#2a4a2a',
            color: status === 'running' ? '#faa' : '#afa',
            fontSize: '20px',
            padding: '10px',
            minWidth: '50px',
            minHeight: '50px',
            borderRadius: '6px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center'
          }} 
          onClick={status === 'running' ? onPause : onPlay}
          title={status === 'running' ? 'Pause emulation' : (status === 'paused' ? 'Resume emulation' : 'Start emulation')}
        >
          {status === 'running' ? '⏸️' : '▶️'}
        </button>
        
        {/* Botón Reset */}
        <button 
          style={{
            ...btn,
            backgroundColor: '#3a3a4a',
            color: '#aaf',
            fontSize: '20px',
            padding: '10px',
            minWidth: '50px',
            minHeight: '50px',
            borderRadius: '6px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center'
          }} 
          onClick={onReset}
          title="Reset emulation"
        >
          🔄
        </button>
        
        {/* Botón Audio Mute/Unmute */}
        <button 
          style={{
            ...btn,
            backgroundColor: getCurrentAudioState() ? '#2a4a2a' : '#4a2a2a',
            color: getCurrentAudioState() ? '#afa' : '#faa',
            fontSize: '20px',
            padding: '10px',
            minWidth: '50px',
            minHeight: '50px',
            borderRadius: '6px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center'
          }} 
          onClick={() => {
            const currentRealState = getCurrentAudioState();
            const newState = !currentRealState;
            
            console.log('[EmulatorPanel] Audio button clicked:', {
              storedState: audioEnabled,
              realCurrentState: currentRealState,
              newState,
              status,
              vecxAvailable: !!(window as any).vecx
            });
            
            setAudioEnabled(newState); 
            
            const vecx = (window as any).vecx;
            if (vecx && vecx.toggleSoundEnabled) {
              const resultState = vecx.toggleSoundEnabled();
              console.log(`[EmulatorPanel] ✓ Audio toggled: ${currentRealState} → ${resultState}`);
              
              if (resultState !== newState) {
                console.log('[EmulatorPanel] Correcting stored state to match result:', resultState);
                setAudioEnabled(resultState);
              }
            }
            
            try {
              const finalState = getCurrentAudioState();
              if (finalState) {
                psgAudio.start();
                console.log('[EmulatorPanel] ✓ PSG Audio started');
              } else {
                psgAudio.stop();
                console.log('[EmulatorPanel] ✓ PSG Audio stopped');
              }
            } catch (e) {
              console.warn('[EmulatorPanel] Could not control PSG audio:', e);
            }
          }}
          title={getCurrentAudioState() ? 'Mute audio' : 'Unmute audio'}
        >
          {getCurrentAudioState() ? '🔊' : '🔇'}
        </button>
        
        {/* Botón Toggle Overlay - Solo visible si hay overlay disponible */}
        {currentOverlay && (
          <button 
            style={{
              ...btn,
              backgroundColor: overlayEnabled ? '#2a4a2a' : '#4a2a2a',
              color: overlayEnabled ? '#afa' : '#888', // Gris cuando está desactivado
              fontSize: '20px',
              padding: '10px',
              minWidth: '50px',
              minHeight: '50px',
              borderRadius: '6px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center'
            }} 
            onClick={toggleOverlay}
            title={overlayEnabled ? 'Hide overlay' : 'Show overlay'} >
            🖼️
          </button>
        )}
      </div>

    </div>
  );
};