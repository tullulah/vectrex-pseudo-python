import React, { useMemo, useState, useRef, useEffect } from 'react';

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
  const [watchList, setWatchList] = useState<string[]>([]);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [searchResults, setSearchResults] = useState<string[]>([]);
  const [currentSearchIndex, setCurrentSearchIndex] = useState<number>(0);
  const gridRef = useRef<HTMLDivElement>(null);

  // Add variable to watch list
  const addToWatch = (varName: string) => {
    if (!watchList.includes(varName)) {
      setWatchList([...watchList, varName]);
    }
  };

  // Remove variable from watch list
  const removeFromWatch = (varName: string) => {
    setWatchList(watchList.filter(v => v !== varName));
  };

  // Search functionality
  useEffect(() => {
    if (!searchQuery.trim()) {
      setSearchResults([]);
      setCurrentSearchIndex(0);
      return;
    }

    const query = searchQuery.toLowerCase();
    const results = Object.keys(variables).filter(name => 
      name.toLowerCase().includes(query)
    );
    setSearchResults(results);
    setCurrentSearchIndex(0);
  }, [searchQuery, variables]);

  // Scroll to search result
  const scrollToVariable = (varName: string) => {
    if (!gridRef.current) return;
    
    const varInfo = variables[varName];
    if (!varInfo) return;

    const addr = parseAddr(varInfo.address);
    const element = gridRef.current.querySelector(`[data-addr="${addr}"]`);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'center' });
      setSelectedVar(varName);
    }
  };

  // Navigate search results
  const goToNextResult = () => {
    if (searchResults.length === 0) return;
    const nextIndex = (currentSearchIndex + 1) % searchResults.length;
    setCurrentSearchIndex(nextIndex);
    scrollToVariable(searchResults[nextIndex]);
  };

  const goToPrevResult = () => {
    if (searchResults.length === 0) return;
    const prevIndex = currentSearchIndex === 0 ? searchResults.length - 1 : currentSearchIndex - 1;
    setCurrentSearchIndex(prevIndex);
    scrollToVariable(searchResults[prevIndex]);
  };

  // Auto scroll to current result when it changes
  useEffect(() => {
    if (searchResults.length > 0 && currentSearchIndex < searchResults.length) {
      scrollToVariable(searchResults[currentSearchIndex]);
    }
  }, [currentSearchIndex, searchResults]);

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
    { name: 'System Vars (RESULT area)', start: 0xC880, end: 0xC8FF, cols: 16 }, // Menos columnas para cuadrados más grandes
    { name: 'User Variables', start: 0xC900, end: 0xCFFF, cols: 16 }, // FIXED: Start at 0xC900 to include all user vars (enemy arrays, etc.)
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
                    data-addr={addr}
                    onMouseEnter={() => setHoveredByte(addr)}
                    onMouseLeave={() => setHoveredByte(null)}
                    onClick={() => {
                      if (varInfo) {
                        setSelectedVar(varInfo.var.name);
                        addToWatch(varInfo.var.name);
                      }
                    }}
                    style={{
                      width: 40,
                      height: 40,
                      backgroundColor: bgColor,
                      border: `1px solid ${borderColor}`,
                      cursor: varInfo ? 'pointer' : 'default',
                      transition: 'all 0.1s',
                      transform: isHovered ? 'scale(1.15)' : 'scale(1)',
                      zIndex: isHovered ? 10 : 1,
                      position: 'relative',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      flexDirection: 'column',
                      fontSize: 9,
                      color: '#fff',
                      overflow: 'hidden',
                      textOverflow: 'ellipsis',
                      whiteSpace: 'nowrap',
                    }}
                    title={varInfo 
                      ? `${varInfo.var.name} [${varInfo.var.address}+${varInfo.offset}] = 0x${byte.toString(16).padStart(2, '0')} (${byte})`
                      : `0x${addr.toString(16).padStart(4, '0')} = 0x${byte.toString(16).padStart(2, '0')} (${byte})`
                    }
                  >
                    {varInfo && varInfo.offset === 0 && (
                      <>
                        <span style={{ fontSize: 8, fontWeight: 'bold', marginBottom: 2 }}>
                          {varInfo.var.name.slice(0, 4)}
                        </span>
                        <span style={{ fontSize: 9, opacity: 0.9 }}>
                          {byte}
                        </span>
                      </>
                    )}
                    {!varInfo && (
                      <span style={{ fontSize: 8, opacity: 0.5 }}>
                        {byte.toString(16).toUpperCase()}
                      </span>
                    )}
                  </div>
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
      <div ref={gridRef} style={{ flex: 1, overflow: 'auto', padding: 16, backgroundColor: '#1e272e' }}>
        {regions.map(region => renderRegion(region))}
      </div>
      
      {/* Sidebar - Watch list and search */}
      <div style={{ 
        width: 320, 
        borderLeft: '1px solid #444', 
        display: 'flex', 
        flexDirection: 'column',
        backgroundColor: '#2c3e50'
      }}>
        {/* Search box */}
        <div style={{ padding: 12, borderBottom: '1px solid #444' }}>
          <div style={{ fontSize: 12, fontWeight: 'bold', color: '#ecf0f1', marginBottom: 8 }}>
            Search Variables
          </div>
          <div style={{ display: 'flex', gap: 4 }}>
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Type variable name..."
              style={{
                flex: 1,
                padding: '6px 8px',
                fontSize: 11,
                backgroundColor: '#34495e',
                border: '1px solid #555',
                color: '#ecf0f1',
                borderRadius: 2,
              }}
            />
          </div>
          {searchResults.length > 0 && (
            <div style={{ 
              marginTop: 8, 
              display: 'flex', 
              alignItems: 'center', 
              gap: 8,
              fontSize: 11,
              color: '#bdc3c7'
            }}>
              <span>
                {currentSearchIndex + 1} / {searchResults.length}
              </span>
              <button
                onClick={goToPrevResult}
                disabled={searchResults.length === 0}
                style={{
                  padding: '4px 8px',
                  fontSize: 11,
                  backgroundColor: '#34495e',
                  border: '1px solid #555',
                  color: '#ecf0f1',
                  cursor: searchResults.length > 0 ? 'pointer' : 'not-allowed',
                  borderRadius: 2,
                }}
              >
                ← Prev
              </button>
              <button
                onClick={goToNextResult}
                disabled={searchResults.length === 0}
                style={{
                  padding: '4px 8px',
                  fontSize: 11,
                  backgroundColor: '#34495e',
                  border: '1px solid #555',
                  color: '#ecf0f1',
                  cursor: searchResults.length > 0 ? 'pointer' : 'not-allowed',
                  borderRadius: 2,
                }}
              >
                Next →
              </button>
            </div>
          )}
        </div>

        {/* Watch list */}
        <div style={{ 
          borderBottom: '1px solid #444',
          maxHeight: '40%',
          display: 'flex',
          flexDirection: 'column'
        }}>
          <div style={{ 
            padding: 12,
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center'
          }}>
            <div style={{ fontSize: 12, fontWeight: 'bold', color: '#ecf0f1' }}>
              Watch List ({watchList.length})
            </div>
            {watchList.length > 0 && (
              <button
                onClick={() => setWatchList([])}
                style={{
                  padding: '2px 6px',
                  fontSize: 10,
                  backgroundColor: '#e74c3c',
                  border: 'none',
                  color: '#fff',
                  cursor: 'pointer',
                  borderRadius: 2,
                }}
              >
                Clear All
              </button>
            )}
          </div>
          <div style={{ flex: 1, overflow: 'auto', padding: '0 12px 12px' }}>
            {watchList.length === 0 ? (
              <div style={{ 
                fontSize: 11, 
                color: '#7f8c8d', 
                fontStyle: 'italic',
                textAlign: 'center',
                paddingTop: 20
              }}>
                Click on variables in the grid to add them to watch
              </div>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                {watchList.map(varName => {
                  const varInfo = variables[varName];
                  if (!varInfo) return null;
                  
                  const value = readVariableValue(memory, varInfo);
                  
                  return (
                    <div
                      key={varName}
                      style={{
                        padding: 8,
                        backgroundColor: '#34495e',
                        borderLeft: `3px solid ${TYPE_COLORS[varInfo.type] || TYPE_COLORS.unknown}`,
                        borderRadius: 2,
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'flex-start'
                      }}
                    >
                      <div style={{ flex: 1 }}>
                        <div style={{ 
                          fontSize: 11, 
                          fontWeight: 'bold', 
                          color: '#ecf0f1',
                          marginBottom: 4
                        }}>
                          {varName}
                        </div>
                        <div style={{ fontSize: 10, color: '#95a5a6' }}>
                          {value}
                        </div>
                        <div style={{ fontSize: 9, color: '#7f8c8d', marginTop: 2 }}>
                          {varInfo.address} • {varInfo.size}B
                        </div>
                      </div>
                      <button
                        onClick={() => removeFromWatch(varName)}
                        style={{
                          padding: '2px 6px',
                          fontSize: 10,
                          backgroundColor: 'transparent',
                          border: '1px solid #e74c3c',
                          color: '#e74c3c',
                          cursor: 'pointer',
                          borderRadius: 2,
                        }}
                      >
                        ✕
                      </button>
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        </div>
        
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
                  onClick={() => {
                    setSelectedVar(name);
                    scrollToVariable(name);
                  }}
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
    // Show ALL elements (no truncation)
    const elements = [];
    const numElements = varInfo.size / 2;
    for (let i = 0; i < numElements; i++) {
      const offset = i * 2;
      const high = memory[addr + offset];
      const low = memory[addr + offset + 1];
      const value = (high << 8) | low;
      elements.push(value.toString());
    }
    return `[${elements.join(', ')}]`;
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
