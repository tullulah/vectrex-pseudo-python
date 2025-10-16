// DebugToolbar.tsx - Debug control buttons (Run, Pause, Stop, Step Over, Step Into, Step Out)
import React from 'react';
import { useDebugStore } from '../state/debugStore';
import './DebugToolbar.css';

export function DebugToolbar() {
  const state = useDebugStore(s => s.state);
  const run = useDebugStore(s => s.run);
  const pause = useDebugStore(s => s.pause);
  const stop = useDebugStore(s => s.stop);
  const stepOver = useDebugStore(s => s.stepOver);
  const stepInto = useDebugStore(s => s.stepInto);
  const stepOut = useDebugStore(s => s.stepOut);
  
  const currentVpyLine = useDebugStore(s => s.currentVpyLine);
  const currentAsmAddress = useDebugStore(s => s.currentAsmAddress);
  const cycles = useDebugStore(s => s.cycles);
  const fps = useDebugStore(s => s.currentFps);
  
  // Keyboard shortcuts
  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // F5 - Continue/Run
      if (e.key === 'F5') {
        e.preventDefault();
        if (state === 'paused') {
          run();
        }
      }
      // F10 - Step Over
      else if (e.key === 'F10') {
        e.preventDefault();
        if (state === 'paused') {
          stepOver();
        }
      }
      // F11 - Step Into
      else if (e.key === 'F11') {
        e.preventDefault();
        if (state === 'paused') {
          stepInto();
        }
      }
      // Shift+F11 - Step Out
      else if (e.key === 'F11' && e.shiftKey) {
        e.preventDefault();
        if (state === 'paused') {
          stepOut();
        }
      }
    };
    
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [state, run, stepOver, stepInto, stepOut]);
  
  return (
    <div className="debug-toolbar">
      <div className="debug-controls">
        {state === 'stopped' && (
          <button 
            className="debug-btn debug-btn-run" 
            onClick={run}
            title="Run (F5)"
          >
            <span className="icon">▶️</span>
            <span className="label">Run</span>
          </button>
        )}
        
        {state === 'running' && (
          <button 
            className="debug-btn debug-btn-pause" 
            onClick={pause}
            title="Pause"
          >
            <span className="icon">⏸️</span>
            <span className="label">Pause</span>
          </button>
        )}
        
        {state === 'paused' && (
          <button 
            className="debug-btn debug-btn-continue" 
            onClick={run}
            title="Continue (F5)"
          >
            <span className="icon">▶️</span>
            <span className="label">Continue</span>
          </button>
        )}
        
        {(state === 'running' || state === 'paused') && (
          <button 
            className="debug-btn debug-btn-stop" 
            onClick={stop}
            title="Stop"
          >
            <span className="icon">⏹️</span>
            <span className="label">Stop</span>
          </button>
        )}
        
        <div className="debug-separator" />
        
        {state === 'paused' && (
          <>
            <button 
              className="debug-btn debug-btn-step-over" 
              onClick={stepOver}
              title="Step Over (F10)"
            >
              <span className="icon">↗️</span>
              <span className="label">Step Over</span>
            </button>
            
            <button 
              className="debug-btn debug-btn-step-into" 
              onClick={stepInto}
              title="Step Into (F11)"
            >
              <span className="icon">↘️</span>
              <span className="label">Step Into</span>
            </button>
            
            <button 
              className="debug-btn debug-btn-step-out" 
              onClick={stepOut}
              title="Step Out (Shift+F11)"
            >
              <span className="icon">↖️</span>
              <span className="label">Step Out</span>
            </button>
          </>
        )}
      </div>
      
      <div className="debug-info">
        {currentVpyLine !== null && (
          <span className="debug-info-item">
            <span className="debug-info-label">Line:</span>
            <span className="debug-info-value">{currentVpyLine}</span>
          </span>
        )}
        
        {currentAsmAddress !== null && (
          <span className="debug-info-item">
            <span className="debug-info-label">PC:</span>
            <span className="debug-info-value">{currentAsmAddress}</span>
          </span>
        )}
        
        {cycles > 0 && (
          <span className="debug-info-item">
            <span className="debug-info-label">Cycles:</span>
            <span className="debug-info-value">{cycles.toLocaleString()}</span>
          </span>
        )}
        
        {fps > 0 && (
          <span className="debug-info-item">
            <span className="debug-info-label">FPS:</span>
            <span className="debug-info-value">{fps.toFixed(1)}</span>
          </span>
        )}
        
        <span className="debug-info-item debug-state-badge" data-state={state}>
          {state.toUpperCase()}
        </span>
      </div>
    </div>
  );
}
