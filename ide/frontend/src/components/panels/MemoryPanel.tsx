import React, { useEffect, useState, useCallback } from 'react';
import { globalEmu } from '../../emulatorWasm';

interface Region { name:string; start:number; end:number; }

// Memory map aligned with README (inclusive ranges)
const REGIONS: Region[] = [
  { name: 'Cartridge 0000-BFFF', start: 0x0000, end: 0xBFFF },
  { name: 'Gap C000-C7FF', start: 0xC000, end: 0xC7FF },
  { name: 'RAM Shadow C800-CFFF', start: 0xC800, end: 0xCFFF },
  { name: 'VIA Shadow D000-D7FF', start: 0xD000, end: 0xD7FF },
  { name: 'Illegal D800-DFFF', start: 0xD800, end: 0xDFFF },
  { name: 'BIOS E000-FFFF', start: 0xE000, end: 0xFFFF },
];

function formatChunk(bytes: Uint8Array, addr: number, width = 16): string {
  let hex = '';
  let ascii = '';
  for (let i = 0; i < width; i++) {
    const b = bytes[addr + i];
    if (i === 8) hex += ' '; // mid separator
    hex += b.toString(16).padStart(2, '0') + ' ';
    ascii += (b >= 0x20 && b < 0x7f) ? String.fromCharCode(b) : '.';
  }
  return `${addr.toString(16).padStart(4,'0')}: ${hex} ${ascii}`;
}

function dumpRegion(bytes: Uint8Array, r: Region): string {
  const lines: string[] = [`>>> ${r.name}`];
  for (let a = r.start; a <= r.end; a += 16) {
    lines.push(formatChunk(bytes, a));
  }
  return lines.join('\n');
}

export const MemoryPanel: React.FC = () => {
  const [text, setText] = useState<string>('');
  const [ts, setTs] = useState<number>(0);

  const refresh = useCallback(() => {
    const snap = globalEmu.snapshotMemory();
    if (snap.length !== 65536) {
      setText('[memory] Snapshot failed or emulator not initialized.');
      return;
    }
    const parts: string[] = [];
    REGIONS.forEach(r => parts.push(dumpRegion(snap, r)));
    setText(parts.join('\n\n'));
    setTs(Date.now());
  }, []);

  useEffect(() => { refresh(); }, [refresh]);

  const saveText = () => {
    const blob = new Blob([text], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `memory_dump_${ts}.txt`;
    a.click();
    URL.revokeObjectURL(url);
  };
  const saveBin = () => {
    const data = globalEmu.snapshotMemory();
    // Use a copy of the underlying ArrayBuffer to satisfy BlobPart typing across environments
  const copy = new Uint8Array(data.length);
  copy.set(data);
  const blob = new Blob([copy.buffer], { type: 'application/octet-stream' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `memory_dump_${ts}.bin`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', fontFamily:'monospace'}}>
      <div style={{padding:'4px', borderBottom:'1px solid #444', display:'flex', gap:8}}>
        <button onClick={refresh}>Refresh</button>
        <button onClick={saveText}>Save TXT</button>
        <button onClick={saveBin}>Save BIN</button>
        <span style={{marginLeft:'auto', opacity:0.7}}>Snapshot: {new Date(ts).toLocaleTimeString()}</span>
      </div>
      <div style={{flex:1, overflow:'auto', padding:'4px', whiteSpace:'pre', fontSize:'11px', lineHeight:1.2}}>
        {text || 'No data'}
      </div>
    </div>
  );
};

export default MemoryPanel;
