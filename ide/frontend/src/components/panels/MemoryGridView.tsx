import React, { useMemo, useState } from 'react';

interface VariableInfo {
  name: string;
  address: string;
  size: number;
  type: string;
  declLine?: number;
}

interface MemoryGridViewProps {
  memory: Uint8Array;
  variables: Record<string, VariableInfo>;
  timestamp: number;
}

// Color palette for different variable types
const TYPE_COLORS: Record<string, string> = {
  int: '#3498db',      // Blue
  array: '#9b59b6',    // Purple
  arg: '#e67e22',      // Orange
  system: '#95a5a6',   // Gray
  audio: '#e74c3c',    // Red
  unknown: '#34495e'   // Dark gray
};

// Helper to parse hex address
const parseAddr = (addr: string): number => {
  return parseInt(addr.replace('0x', ''), 16);
};

export const MemoryGridView: React.FC<MemoryGridViewProps> = ({ memory, variables, timestamp }) => {
  const [hoveredByte, setHoveredByte] = useState<number | null>(null);
  const [selectedVar, setSelectedVar] = useState<string | null>(null);

  // Log re-renders for debugging
  React.useEffect(() => {
    console.log('[MemoryGridView] Rendered with timestamp:', new Date(timestamp).toLocaleTimeString(), 
                'vars:', Object.keys(variables).length);
  }, [timestamp, variables]);

  // Build memory map: address -> variable info
  const memoryMap = useMemo(() => {
    const map = new Map<number, { var: VariableInfo; offset: number }>();
    
    Object.entries(variables).forEach(([name, varInfo]) => {
      const startAddr = parseAddr(varInfo.address);
      for (let i = 0; i < varInfo.size; i++) {
        map.set(startAddr + i, { var: varInfo, offset: i });
      }
    });
    
    return map;
  }, [variables]);

  // Focus on interesting memory regions (RAM areas)
  const regions = [
    { name: 'System Vars (RESULT area)', start: 0xC880, end: 0xC8FF, cols: 32 },
    { name: 'User Variables', start: 0xCF10, end: 0xCFFF, cols: 32 },
  ];

  const renderRegion = (region: typeof regions[0]) => {
    const { start, end, cols } = region;
    const rows: number[][] = [];
    
    for (let addr = start; addr <= end; addr += cols) {
      const row: number[] = [];
      for (let i = 0; i < cols && addr + i <= end; i++) {
        row.push(addr + i);
      }
      rows.push(row);
    }

    return (
      <div key={region.name} style={{ marginBottom: 32 }}>
        <div style={{ 
          fontWeight: 'bold', 
          marginBottom: 8, 
          fontSize: 12,
          color: '#ecf0f1',
          borderBottom: '1px solid #444',
          paddingBottom: 4
        }}>
          {region.name} (${start.toString(16).toUpperCase()}-${end.toString(16).toUpperCase()})
        </div>
        
        <div style={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
          {rows.map((row, rowIdx) => (
            <div key={rowIdx} style={{ display: 'flex', gap: 1 }}>
              {row.map((addr) => {
                const varInfo = memoryMap.get(addr);
                const byte = memory[addr];
                const isHovered = hoveredByte === addr;
                const isSelected = varInfo && selectedVar === varInfo.var.name;
                
                let bgColor = '#2c3e50'; // Default empty
                let borderColor = '#34495e';
                
                if (varInfo) {
                  bgColor = TYPE_COLORS[varInfo.var.type] || TYPE_COLORS.unknown;
                  borderColor = bgColor;
                  
                  // Darken if not first byte of variable
                  if (varInfo.offset > 0) {
                    bgColor = adjustBrightness(bgColor, -30);
                  }
                }
                
                if (isHovered || isSelected) {
                  borderColor = '#f39c12';
                }

                return (
                  <div
                    key={addr}
                    onMouseEnter={() => setHoveredByte(addr)}
                    onMouseLeave={() => setHoveredByte(null)}
                    onClick={() => varInfo && setSelectedVar(varInfo.var.name)}
                    style={{
                      width: 12,
                      height: 12,
                      backgroundColor: bgColor,
                      border: `1px solid ${borderColor}`,
                      cursor: varInfo ? 'pointer' : 'default',
                      transition: 'all 0.1s',
                      transform: isHovered ? 'scale(1.3)' : 'scale(1)',
                      zIndex: isHovered ? 10 : 1,
                      position: 'relative',
                    }}
                    title={varInfo 
                      ? `${varInfo.var.name} [${varInfo.var.address}+${varInfo.offset}] = 0x${byte.toString(16).padStart(2, '0')}`
                      : `0x${addr.toString(16).padStart(4, '0')} = 0x${byte.toString(16).padStart(2, '0')}`
                    }
                  />
                );
              })}
            </div>
          ))}
        </div>
      </div>
    );
  };

  // Show selected variable details
  const selectedVarInfo = selectedVar ? variables[selectedVar] : null;
  const selectedVarValue = selectedVarInfo ? readVariableValue(memory, selectedVarInfo) : null;

  return (
    <div style={{ display: 'flex', height: '100%', fontFamily: 'monospace' }}>
      {/* Grid view */}
      <div style={{ flex: 1, overflow: 'auto', padding: 16, backgroundColor: '#1e272e' }}>
        {regions.map(region => renderRegion(region))}
      </div>
      
      {/* Sidebar - Variable list and details */}
      <div style={{ 
        width: 300, 
        borderLeft: '1px solid #444', 
        display: 'flex', 
        flexDirection: 'column',
        backgroundColor: '#2c3e50'
      }}>
        {/* Selected variable details */}
        {selectedVarInfo && (
          <div style={{ 
            padding: 12, 
            borderBottom: '1px solid #444',
            backgroundColor: '#34495e'
          }}>
            <div style={{ fontSize: 14, fontWeight: 'bold', color: '#ecf0f1', marginBottom: 8 }}>
              {selectedVarInfo.name}
            </div>
            <div style={{ fontSize: 11, color: '#bdc3c7', lineHeight: 1.6 }}>
              <div>Address: {selectedVarInfo.address}</div>
              <div>Type: {selectedVarInfo.type}</div>
              <div>Size: {selectedVarInfo.size} byte{selectedVarInfo.size > 1 ? 's' : ''}</div>
              <div>Value: {selectedVarValue}</div>
            </div>
          </div>
        )}
        
        {/* Legend */}
        <div style={{ padding: 12, borderBottom: '1px solid #444' }}>
          <div style={{ fontSize: 12, fontWeight: 'bold', marginBottom: 8, color: '#ecf0f1' }}>
            Variable Types
          </div>
          {Object.entries(TYPE_COLORS).map(([type, color]) => (
            <div key={type} style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 4 }}>
              <div style={{ width: 16, height: 16, backgroundColor: color, border: '1px solid #555' }} />
              <span style={{ fontSize: 11, color: '#bdc3c7' }}>{type}</span>
            </div>
          ))}
        </div>
        
        {/* Variable list */}
        <div style={{ flex: 1, overflow: 'auto', padding: 12 }}>
          <div style={{ fontSize: 12, fontWeight: 'bold', marginBottom: 8, color: '#ecf0f1' }}>
            All Variables ({Object.keys(variables).length})
          </div>
          <div style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            {Object.entries(variables)
              .sort((a, b) => parseAddr(a[1].address) - parseAddr(b[1].address))
              .map(([name, varInfo]) => (
                <div
                  key={name}
                  onClick={() => setSelectedVar(name)}
                  style={{
                    padding: 6,
                    fontSize: 11,
                    backgroundColor: selectedVar === name ? '#34495e' : 'transparent',
                    cursor: 'pointer',
                    borderLeft: `3px solid ${TYPE_COLORS[varInfo.type] || TYPE_COLORS.unknown}`,
                    paddingLeft: 8,
                    color: '#bdc3c7',
                    transition: 'background-color 0.1s'
                  }}
                  onMouseEnter={(e) => e.currentTarget.style.backgroundColor = '#2c3e50'}
                  onMouseLeave={(e) => {
                    if (selectedVar !== name) {
                      e.currentTarget.style.backgroundColor = 'transparent';
                    }
                  }}
                >
                  <div style={{ fontWeight: selectedVar === name ? 'bold' : 'normal' }}>
                    {name}
                  </div>
                  <div style={{ fontSize: 10, opacity: 0.7 }}>
                    {varInfo.address} • {varInfo.size}B
                  </div>
                </div>
              ))}
          </div>
        </div>
      </div>
    </div>
  );
};

// Helper to read variable value from memory
function readVariableValue(memory: Uint8Array, varInfo: VariableInfo): string {
  const addr = parseAddr(varInfo.address);
  
  if (varInfo.type === 'array') {
    // Show first few elements
    const elements = [];
    const maxShow = Math.min(4, varInfo.size / 2);
    for (let i = 0; i < maxShow; i++) {
      const offset = i * 2;
      const high = memory[addr + offset];
      const low = memory[addr + offset + 1];
      const value = (high << 8) | low;
      elements.push(value.toString());
    }
    return `[${elements.join(', ')}${varInfo.size / 2 > maxShow ? '...' : ''}]`;
  } else if (varInfo.size === 1) {
    // 8-bit value
    const value = memory[addr];
    return `0x${value.toString(16).padStart(2, '0')} (${value})`;
  } else {
    // 16-bit value (big-endian)
    const high = memory[addr];
    const low = memory[addr + 1];
    const value = (high << 8) | low;
    return `0x${value.toString(16).padStart(4, '0')} (${value})`;
  }
}

// Helper to adjust color brightness
function adjustBrightness(hex: string, percent: number): string {
  const num = parseInt(hex.replace('#', ''), 16);
  const r = Math.max(0, Math.min(255, ((num >> 16) & 0xff) + percent));
  const g = Math.max(0, Math.min(255, ((num >> 8) & 0xff) + percent));
  const b = Math.max(0, Math.min(255, (num & 0xff) + percent));
  return `#${((r << 16) | (g << 8) | b).toString(16).padStart(6, '0')}`;
}
