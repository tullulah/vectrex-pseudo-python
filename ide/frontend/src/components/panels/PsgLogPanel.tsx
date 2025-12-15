import React, { useState, useEffect, useCallback } from 'react';
import { emuCore } from '../../emulatorCoreSingleton';

interface PsgWrite {
  reg: number;
  value: number;
  frame: number;
  pc: number;
}

interface MusicCall {
  type: string;
  from: number;
  to: number;
  funcName: string;
  frame: number;
  timestamp: number;
}

const REG_NAMES: Record<number, string> = {
  0: 'ToneA_Lo',
  1: 'ToneA_Hi',
  2: 'ToneB_Lo',
  3: 'ToneB_Hi',
  4: 'ToneC_Lo',
  5: 'ToneC_Hi',
  6: 'Noise',
  7: 'Mixer',
  8: 'VolA',
  9: 'VolB',
  10: 'VolC',
  11: 'EnvPer_Lo',
  12: 'EnvPer_Hi',
  13: 'EnvShape',
  14: 'IO_A',
  15: 'IO_B'
};

export const PsgLogPanel: React.FC = () => {
  const [writes, setWrites] = useState<PsgWrite[]>([]);
  const [musicWrites, setMusicWrites] = useState<PsgWrite[]>([]);
  const [vectorWrites, setVectorWrites] = useState<PsgWrite[]>([]);
  const [calls, setCalls] = useState<MusicCall[]>([]);
  const [capturing, setCapturing] = useState(true); // Start capturing by default
  const [limit, setLimit] = useState(10000);
  const [showCalls, setShowCalls] = useState(true);
  const [showMode, setShowMode] = useState<'all' | 'music' | 'vector'>('music'); // Default to music only

  const refresh = useCallback(() => {
    const win = window as any;
    const log = win.PSG_WRITE_LOG || [];
    const musicLog = win.PSG_MUSIC_LOG || [];
    const vectorLog = win.PSG_VECTOR_LOG || [];
    const callLog = win.MUSIC_CALL_LOG || [];
    console.log('[PsgLogPanel] Refresh - Total:', log.length, 'Music:', musicLog.length, 'Vector:', vectorLog.length, 'Calls:', callLog.length);
    // Create a new array copy so React detects changes when array is cleared
    setWrites([...log]);
    setMusicWrites([...musicLog]);
    setVectorWrites([...vectorLog]);
    setCalls([...callLog]);
  }, []);

  // Auto-refresh when capturing
  useEffect(() => {
    if (!capturing) return;
    const interval = setInterval(refresh, 500);
    return () => clearInterval(interval);
  }, [capturing, refresh]);

  const startCapture = () => {
    const win = window as any;
    if (win.PSG_WRITE_LOG) win.PSG_WRITE_LOG.length = 0;
    if (win.PSG_MUSIC_LOG) win.PSG_MUSIC_LOG.length = 0;
    if (win.PSG_VECTOR_LOG) win.PSG_VECTOR_LOG.length = 0;
    if (win.MUSIC_CALL_LOG) win.MUSIC_CALL_LOG.length = 0;
    win.PSG_LOG_ENABLED = true;
    win.MUSIC_CALL_LOG_ENABLED = true;
    win.PSG_LOG_LIMIT = limit;
    win.MUSIC_CALL_LOG_LIMIT = limit;
    setCapturing(true);
    refresh();
  };

  const stopCapture = () => {
    const win = window as any;
    win.PSG_LOG_ENABLED = false;
    win.MUSIC_CALL_LOG_ENABLED = false;
    setCapturing(false);
    refresh();
  };

  const clear = () => {
    const win = window as any;
    if (win.PSG_WRITE_LOG) win.PSG_WRITE_LOG.length = 0;
    if (win.PSG_MUSIC_LOG) win.PSG_MUSIC_LOG.length = 0;
    if (win.PSG_VECTOR_LOG) win.PSG_VECTOR_LOG.length = 0;
    if (win.MUSIC_CALL_LOG) win.MUSIC_CALL_LOG.length = 0;
    refresh();
  };

  useEffect(() => {
    refresh();
  }, [refresh]);

  const exportText = () => {
    const activeWrites = showMode === 'music' ? musicWrites : showMode === 'vector' ? vectorWrites : writes;
    const lines = activeWrites.map(w => {
      const regName = REG_NAMES[w.reg] || `Reg${w.reg}`;
      const mixer = w.reg === 7 ? ` (ToneA:${(w.value & 1) ? 'OFF' : 'ON'} ToneB:${(w.value & 2) ? 'OFF' : 'ON'} ToneC:${(w.value & 4) ? 'OFF' : 'ON'} NoiseA:${(w.value & 8) ? 'ON' : 'OFF'} NoiseB:${(w.value & 16) ? 'ON' : 'OFF'} NoiseC:${(w.value & 32) ? 'ON' : 'OFF'})` : '';
      return `Frame ${w.frame.toString().padStart(5, ' ')} PC:${w.pc.toString(16).padStart(4, '0')} ${regName.padEnd(12, ' ')} = 0x${w.value.toString(16).padStart(2, '0')} (${w.value.toString().padStart(3, ' ')})${mixer}`;
    });
    const blob = new Blob([lines.join('\n')], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `psg_log_${showMode}.txt`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const formatMixer = (value: number) => {
    const toneA = (value & 1) ? 'OFF' : 'ON';
    const toneB = (value & 2) ? 'OFF' : 'ON';
    const toneC = (value & 4) ? 'OFF' : 'ON';
    const noiseA = (value & 8) ? 'ON' : 'OFF';
    const noiseB = (value & 16) ? 'ON' : 'OFF';
    const noiseC = (value & 32) ? 'ON' : 'OFF';
    return `Tone:${toneA}/${toneB}/${toneC} Noise:${noiseA}/${noiseB}/${noiseC}`;
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%', fontFamily: 'monospace' }}>
      <div style={{ padding: 4, borderBottom: '1px solid #444', display: 'flex', gap: 8, alignItems: 'center' }}>
        <label>
          Limit
          <input
            style={{ width: 80, marginLeft: 4 }}
            type="number"
            value={limit}
            onChange={e => setLimit(parseInt(e.target.value) || 0)}
            disabled={capturing}
          />
        </label>
        {!capturing && <button onClick={startCapture}>Start Capture</button>}
        {capturing && <button onClick={stopCapture}>Stop</button>}
        <button onClick={refresh}>Refresh</button>
        <button onClick={clear}>Clear</button>
        <button onClick={exportText} disabled={!writes.length}>
          Export
        </button>
        <label style={{ marginLeft: 8 }}>
          <input
            type="checkbox"
            checked={showCalls}
            onChange={e => setShowCalls(e.target.checked)}
          />
          Show Function Calls
        </label>
        <label style={{ marginLeft: 8 }}>
          View:
          <select value={showMode} onChange={e => setShowMode(e.target.value as any)} style={{ marginLeft: 4 }}>
            <option value="music">Music Only</option>
            <option value="vector">Vector Only</option>
            <option value="all">All (Legacy)</option>
          </select>
        </label>
        <span style={{ marginLeft: 'auto', opacity: 0.7 }}>
          Total: {writes.length} | Music: {musicWrites.length} | Vector: {vectorWrites.length} | Calls: {calls.length}
        </span>
      </div>
      <div style={{ flex: 1, overflow: 'auto', fontSize: 11, lineHeight: 1.3, padding: 4 }}>
        {showCalls && calls.length > 0 && (
          <div style={{ marginBottom: 12, borderBottom: '1px solid #333', paddingBottom: 8 }}>
            <div style={{ fontWeight: 'bold', color: '#4af', marginBottom: 4 }}>
              === MUSIC FUNCTION CALLS ({calls.length}) ===
            </div>
            {calls.map((call, i) => (
              <div key={`call-${i}`} style={{ color: '#4af', fontWeight: 'bold' }}>
                Frame {call.frame.toString().padStart(5, ' ')} {call.type} {call.funcName} 
                {' '}from PC:${call.from.toString(16).padStart(4, '0')} 
                {' '}to PC:${call.to.toString(16).padStart(4, '0')}
              </div>
            ))}
          </div>
        )}
        {(showMode === 'music' ? musicWrites : showMode === 'vector' ? vectorWrites : writes).map((w, i) => {
          const regName = REG_NAMES[w.reg] || `Reg${w.reg}`;
          const extra = w.reg === 7 ? ` (${formatMixer(w.value)})` : '';
          const pcStr = w.pc ? `PC:${w.pc.toString(16).padStart(4, '0')}` : 'PC:????';
          
          return (
            <div key={i} style={{ opacity: w.reg === 6 || w.reg === 7 ? 1 : 0.85 }}>
              Frame {w.frame.toString().padStart(5, ' ')} {pcStr} {regName.padEnd(12, ' ')} = 0x
              {w.value.toString(16).padStart(2, '0')} ({w.value.toString().padStart(3, ' ')})
              {extra}
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default PsgLogPanel;
