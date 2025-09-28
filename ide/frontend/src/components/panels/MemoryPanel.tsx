import React, { useEffect, useState, useCallback } from 'react';

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
    const vecx = (window as any).vecx;
    if (!vecx) {
      setText('[memory] JSVecX not available');
      return;
    }
    
    console.log('[MemoryPanel] Starting memory snapshot...');
    
    // Pausar emulador temporalmente para evitar conflictos
    const wasRunning = vecx.isRunning && vecx.isRunning();
    if (wasRunning) {
      vecx.stop();
    }
    
    try {
      // Crear snapshot de memoria de regiones importantes solamente
      const snap = new Uint8Array(65536);
      
      // Leer solo las regiones que realmente importan (no todo el espacio de direcciones)
      const importantRegions = [
        { start: 0xC800, end: 0xCFFF }, // RAM
        { start: 0xD000, end: 0xD07F }, // VIA registers
        { start: 0xE000, end: 0xFFFF }  // BIOS
      ];
      
      for (const region of importantRegions) {
        for (let addr = region.start; addr <= region.end; addr++) {
          snap[addr] = vecx.read8(addr);
        }
      }
      
      // Para cartridge, leer solo las primeras páginas si hay ROM cargada
      const Globals = (window as any).Globals;
      if (Globals && Globals.cartdata && Globals.cartdata.length > 0) {
        const maxCartRead = Math.min(0x2000, Globals.cartdata.length); // Primeras 8K máximo
        for (let addr = 0; addr < maxCartRead; addr++) {
          snap[addr] = vecx.read8(addr);
        }
      }
      
      const parts: string[] = [];
      REGIONS.forEach(r => parts.push(dumpRegion(snap, r)));
      setText(parts.join('\n\n'));
      setTs(Date.now());
      
      console.log('[MemoryPanel] ✓ Memory snapshot completed');
    } catch (e) {
      setText('[memory] Failed to read memory from JSVecX: ' + e);
      console.error('[MemoryPanel] Memory read error:', e);
    } finally {
      // Reanudar emulador si estaba corriendo
      if (wasRunning) {
        setTimeout(() => vecx.start(), 100);
      }
    }
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
  const saveBin = async () => {
    const vecx = (window as any).vecx;
    if (!vecx) {
      alert('JSVecX not available');
      return;
    }
    
    console.log('[MemoryPanel] Starting binary dump...');
    
    // Pausar emulador temporalmente
    const wasRunning = vecx.isRunning && vecx.isRunning();
    if (wasRunning) {
      vecx.stop();
    }
    
    try {
      // Crear snapshot de memoria completa para dump binario
      const data = new Uint8Array(65536);
      
      // Leer en chunks más grandes para mejor performance
      const chunkSize = 1024;
      for (let start = 0; start < 65536; start += chunkSize) {
        const end = Math.min(start + chunkSize, 65536);
        for (let addr = start; addr < end; addr++) {
          data[addr] = vecx.read8(addr);
        }
        // Pequeña pausa cada chunk para no bloquear UI
        if (start % (chunkSize * 10) === 0) {
          await new Promise(resolve => setTimeout(resolve, 1));
        }
      }
      
      const blob = new Blob([data.buffer], { type: 'application/octet-stream' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `memory_dump_${ts}.bin`;
      a.click();
      URL.revokeObjectURL(url);
      
      console.log('[MemoryPanel] ✓ Binary dump completed');
    } catch (e) {
      alert('Failed to read memory from JSVecX: ' + e);
      console.error('[MemoryPanel] Binary dump error:', e);
    } finally {
      // Reanudar emulador
      if (wasRunning) {
        setTimeout(() => vecx.start(), 100);
      }
    }
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
