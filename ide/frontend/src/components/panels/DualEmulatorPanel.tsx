// Dual Emulator Comparison Panel
// Permite ejecutar el mismo test en ambos emuladores (Rust y jsvecx) y comparar resultados
import React, { useState, useRef, useCallback, useEffect } from 'react';
import { createEmulatorCore, readPreference, persistPreference } from '../../emulatorFactory';
import type { IEmulatorCore, Segment, RegistersSnapshot, MetricsSnapshot, EmulatorBackend } from '../../emulatorCore';

interface ComparisonResult {
  testName: string;
  rustSegments: Segment[];
  jsvecxSegments: Segment[];
  rustRegisters: RegistersSnapshot | null;
  jsvecxRegisters: RegistersSnapshot | null;
  rustMetrics: MetricsSnapshot | null;
  jsvecxMetrics: MetricsSnapshot | null;
  rustTrace?: any[];
  jsvecxTrace?: any[];
  divergences: string[];
  timestamp: number;
}

export function DualEmulatorPanel() {
  const [isRunning, setIsRunning] = useState(false);
  const [results, setResults] = useState<ComparisonResult[]>([]);
  const [selectedTest, setSelectedTest] = useState<string>('bios-presentation');
  const [maxFrames, setMaxFrames] = useState(10);
  const [biosLoaded, setBiosLoaded] = useState(false);
  const [enableTracing, setEnableTracing] = useState(false);
  const [currentBackend, setCurrentBackend] = useState<EmulatorBackend>(readPreference);
  
  const rustCore = useRef<IEmulatorCore | null>(null);
  const jsvecxCore = useRef<IEmulatorCore | null>(null);

  // Initialize both emulators
  const initEmulators = useCallback(async () => {
    try {
      console.log('[DualEmulator] Starting initialization...');
      
      // Create instances
      console.log('[DualEmulator] Creating rust core...');
      rustCore.current = createEmulatorCore('rust');
      console.log('[DualEmulator] Creating jsvecx core...');
      jsvecxCore.current = createEmulatorCore('jsvecx');
      
      // Initialize
      console.log('[DualEmulator] Initializing rust core...');
      await rustCore.current?.init?.();
      console.log('[DualEmulator] Initializing jsvecx core...');
      await jsvecxCore.current?.init?.();
      
      console.log('[DualEmulator] Cores created and initialized');
      console.log('[DualEmulator] Rust core:', rustCore.current);
      console.log('[DualEmulator] jsvecx core:', jsvecxCore.current);
      
      // Load BIOS - try multiple methods
      let biosLoaded = false;
      
      if (rustCore.current?.ensureBios) {
        console.log('[DualEmulator] Attempting to load BIOS via ensureBios...');
        // First try to ensure BIOS is loaded in rust emulator using default candidates
        const candidates = [
          '/bios.bin',
          '/assets/bios.bin', 
          'assets/bios.bin',
          'bios.bin'
        ];
        biosLoaded = await rustCore.current.ensureBios({ urlCandidates: candidates });
        console.log('[DualEmulator] ensureBios result:', biosLoaded);
        
        if (biosLoaded && rustCore.current.isBiosLoaded()) {
          console.log('[DualEmulator] BIOS loaded in rust, extracting memory snapshot...');
          // Get BIOS bytes from rust emulator memory snapshot
          const memSnapshot = rustCore.current.snapshotMemory?.();
          console.log('[DualEmulator] Memory snapshot size:', memSnapshot?.length);
          
          if (memSnapshot && memSnapshot.length >= 0x10000) {
            // Extract BIOS region (0xE000-0xFFFF = 8KB)
            const biosBytes = memSnapshot.slice(0xE000, 0x10000);
            console.log('[DualEmulator] BIOS bytes extracted, size:', biosBytes.length);
            
            // Verify it's not all zeros/FF
            const nonZero = biosBytes.some(b => b !== 0 && b !== 0xFF);
            const firstBytes = Array.from(biosBytes.slice(0, 16)).map(b => `0x${b.toString(16).padStart(2, '0')}`).join(' ');
            console.log('[DualEmulator] First 16 BIOS bytes:', firstBytes);
            console.log('[DualEmulator] Non-zero bytes found:', nonZero);
            
            if (nonZero && jsvecxCore.current) {
              console.log('[DualEmulator] Loading BIOS into jsvecx...');
              jsvecxCore.current.loadBios(biosBytes);
              console.log('[DualEmulator] BIOS copied from rust to jsvecx, size:', biosBytes.length);
              setBiosLoaded(true);
            } else {
              console.warn('[DualEmulator] BIOS appears empty in rust emulator');
            }
          } else {
            console.warn('[DualEmulator] Could not snapshot memory from rust emulator');
          }
        } else {
          console.warn('[DualEmulator] BIOS not loaded in rust emulator');
        }
      }
      
      if (!biosLoaded) {
        console.warn('[DualEmulator] BIOS loading failed - tests may not work correctly');
      }
      
      console.log('[DualEmulator] Both emulators initialized, BIOS loaded:', biosLoaded);
    } catch (error) {
      console.error('[DualEmulator] Initialization failed:', error);
    }
  }, []);

  // Run a comparison test
  const runTest = useCallback(async (testName: string) => {
    if (!rustCore.current || !jsvecxCore.current) {
      await initEmulators();
      return;
    }

    setIsRunning(true);
    const startTime = Date.now();

    try {
      // Reset both emulators
      console.log('[DualEmulator] Resetting both emulators...');
      rustCore.current?.reset();
      jsvecxCore.current?.reset();
      
      // Check initial state after reset
      console.log('[DualEmulator] Post-reset state check...');

      // Enable tracing if requested
      if (enableTracing) {
        rustCore.current.enableTraceCapture?.(true, 1000);
        jsvecxCore.current.enableTraceCapture?.(true, 1000);
      }

      // Clear previous traces and segments
      rustCore.current.clearTrace?.();
      jsvecxCore.current.clearTrace?.();
      rustCore.current.getSegmentsShared();
      jsvecxCore.current.getSegmentsShared();

      // Load test program if needed
      if (testName !== 'bios-presentation') {
        const testProgram = getTestProgram(testName);
        if (testProgram) {
          rustCore.current.loadProgram(testProgram);
          jsvecxCore.current.loadProgram(testProgram);
        }
      }

      // Run for specified number of frames
      console.log('[DualEmulator] Running', maxFrames, 'frames...');
      for (let frame = 0; frame < maxFrames; frame++) {
        rustCore.current.runFrame();
        jsvecxCore.current.runFrame();
        
        // Log every 5 frames for debugging
        if (frame % 5 === 0) {
          console.log(`[DualEmulator] Frame ${frame} completed`);
        }
      }
      console.log('[DualEmulator] All frames completed');

      // Collect results
      console.log('[DualEmulator] Collecting results...');
      const rustSegments = rustCore.current.getSegmentsShared();
      const jsvecxSegments = jsvecxCore.current.getSegmentsShared();
      const rustRegisters = rustCore.current.registers();
      const jsvecxRegisters = jsvecxCore.current.registers();
      const rustMetrics = rustCore.current.metrics();
      const jsvecxMetrics = jsvecxCore.current.metrics();
      
      console.log('[DualEmulator] Final state:');
      console.log('  Rust - PC:', rustRegisters?.pc, 'Segments:', rustSegments?.length);
      console.log('  jsvecx - PC:', jsvecxRegisters?.pc, 'Segments:', jsvecxSegments?.length);
      
      // Collect traces if enabled
      const rustTrace = enableTracing ? rustCore.current.traceLog?.() : undefined;
      const jsvecxTrace = enableTracing ? jsvecxCore.current.traceLog?.() : undefined;

      // Analyze divergences
      const divergences = analyzeDivergences(
        { segments: rustSegments, registers: rustRegisters, metrics: rustMetrics },
        { segments: jsvecxSegments, registers: jsvecxRegisters, metrics: jsvecxMetrics }
      );

      const result: ComparisonResult = {
        testName,
        rustSegments,
        jsvecxSegments,
        rustRegisters,
        jsvecxRegisters,
        rustMetrics,
        jsvecxMetrics,
        rustTrace,
        jsvecxTrace,
        divergences,
        timestamp: startTime
      };

      setResults(prev => [result, ...prev.slice(0, 9)]); // Keep last 10 results
      
    } catch (error) {
      console.error('[DualEmulator] Test failed:', error);
    } finally {
      setIsRunning(false);
    }
  }, [maxFrames, initEmulators]);

  // Analyze differences between emulator outputs
  const analyzeDivergences = (rust: any, jsvecx: any): string[] => {
    const divergences: string[] = [];

    // Compare segment counts
    if (rust.segments.length !== jsvecx.segments.length) {
      divergences.push(`Segment count: Rust=${rust.segments.length}, jsvecx=${jsvecx.segments.length}`);
    }

    // Compare register states
    if (rust.registers && jsvecx.registers) {
      const regs = ['pc', 'a', 'b', 'x', 'y', 'u', 's', 'dp'];
      for (const reg of regs) {
        if (rust.registers[reg] !== jsvecx.registers[reg]) {
          divergences.push(`Register ${reg}: Rust=0x${rust.registers[reg]?.toString(16)}, jsvecx=0x${jsvecx.registers[reg]?.toString(16)}`);
        }
      }
    }

    // Compare frame counts
    if (rust.metrics?.frames !== jsvecx.metrics?.frames) {
      divergences.push(`Frame count: Rust=${rust.metrics?.frames}, jsvecx=${jsvecx.metrics?.frames}`);
    }

    // Detailed vector analysis
    if (rust.segments.length > 0 || jsvecx.segments.length > 0) {
      // Count horizontal vectors (Y difference < 0.1)
      const rustHorizontal = rust.segments.filter((s: Segment) => Math.abs(s.y0 - s.y1) < 0.1).length;
      const jsvecxHorizontal = jsvecx.segments.filter((s: Segment) => Math.abs(s.y0 - s.y1) < 0.1).length;
      
      // Count vertical vectors (X difference < 0.1)
      const rustVertical = rust.segments.filter((s: Segment) => Math.abs(s.x0 - s.x1) < 0.1).length;
      const jsvecxVertical = jsvecx.segments.filter((s: Segment) => Math.abs(s.x0 - s.x1) < 0.1).length;
      
      // Count diagonal vectors
      const rustDiagonal = rust.segments.filter((s: Segment) => 
        Math.abs(s.x0 - s.x1) > 0.1 && Math.abs(s.y0 - s.y1) > 0.1).length;
      const jsvecxDiagonal = jsvecx.segments.filter((s: Segment) => 
        Math.abs(s.x0 - s.x1) > 0.1 && Math.abs(s.y0 - s.y1) > 0.1).length;
      
      if (rustHorizontal !== jsvecxHorizontal) {
        divergences.push(`Horizontal vectors: Rust=${rustHorizontal}, jsvecx=${jsvecxHorizontal}`);
      }
      if (rustVertical !== jsvecxVertical) {
        divergences.push(`Vertical vectors: Rust=${rustVertical}, jsvecx=${jsvecxVertical}`);
      }
      if (rustDiagonal !== jsvecxDiagonal) {
        divergences.push(`Diagonal vectors: Rust=${rustDiagonal}, jsvecx=${jsvecxDiagonal}`);
      }
      
      // Analyze vector positions
      if (rust.segments.length > 0 && jsvecx.segments.length > 0) {
        const rustAvgX = rust.segments.reduce((sum: number, s: Segment) => sum + (s.x0 + s.x1) / 2, 0) / rust.segments.length;
        const jsvecxAvgX = jsvecx.segments.reduce((sum: number, s: Segment) => sum + (s.x0 + s.x1) / 2, 0) / jsvecx.segments.length;
        const rustAvgY = rust.segments.reduce((sum: number, s: Segment) => sum + (s.y0 + s.y1) / 2, 0) / rust.segments.length;
        const jsvecxAvgY = jsvecx.segments.reduce((sum: number, s: Segment) => sum + (s.y0 + s.y1) / 2, 0) / jsvecx.segments.length;
        
        if (Math.abs(rustAvgX - jsvecxAvgX) > 0.2) {
          divergences.push(`Avg X position: Rust=${rustAvgX.toFixed(2)}, jsvecx=${jsvecxAvgX.toFixed(2)}`);
        }
        if (Math.abs(rustAvgY - jsvecxAvgY) > 0.2) {
          divergences.push(`Avg Y position: Rust=${rustAvgY.toFixed(2)}, jsvecx=${jsvecxAvgY.toFixed(2)}`);
        }
      }
    }

    return divergences;
  };

  // Get test program bytes
  const getTestProgram = (testName: string): Uint8Array | null => {
    switch (testName) {
      case 'simple-line':
        // Simple line drawing program that should create a diagonal line
        return new Uint8Array([
          // Reset and setup
          0x8E, 0x00, 0x00,       // LDX #$0000 (center position)
          0x10, 0x8E, 0x00, 0x00, // LDY #$0000 (center position)
          0x86, 0x7F,             // LDA #$7F (intensity)
          0xCC, 0xD0, 0x0A,       // LDD #$D00A (VIA port B - intensity)
          0xE7, 0x84,             // STB ,X (write intensity)
          
          // Draw diagonal line by setting DAC values
          0x8E, 0x20, 0x00,       // LDX #$2000 (positive X)
          0x10, 0x8E, 0x20, 0x00, // LDY #$2000 (positive Y)
          0xCC, 0xD0, 0x08,       // LDD #$D008 (VIA port A - X DAC)
          0xED, 0x84,             // STD ,X
          0xCC, 0xD0, 0x09,       // LDD #$D009 (VIA port A - Y DAC)  
          0xED, 0x84,             // STD ,X
          
          0x39,                   // RTS
        ]);
      case 'vector-test':
        // More comprehensive vector test
        return new Uint8Array([
          // Test pattern: horizontal, vertical, and diagonal lines
          0x8E, 0x00, 0x00,       // LDX #$0000 (start at center)
          0x10, 0x8E, 0x00, 0x00, // LDY #$0000
          
          // Horizontal line
          0x8E, 0x10, 0x00,       // LDX #$1000 (right)
          0x10, 0x8E, 0x00, 0x00, // LDY #$0000 (same Y)
          
          // Vertical line  
          0x8E, 0x00, 0x00,       // LDX #$0000 (back to center)
          0x10, 0x8E, 0x10, 0x00, // LDY #$1000 (up)
          
          // Diagonal line
          0x8E, 0x10, 0x00,       // LDX #$1000 (right)
          0x10, 0x8E, 0x10, 0x00, // LDY #$1000 (up)
          
          0x39,                   // RTS
        ]);
      default:
        return null;
    }
  };

  const formatSegment = (seg: Segment) => 
    `(${seg.x0.toFixed(2)},${seg.y0.toFixed(2)}) → (${seg.x1.toFixed(2)},${seg.y1.toFixed(2)}) I:${seg.intensity}`;

  // Inicializar automáticamente los emuladores cuando se monta el componente
  useEffect(() => {
    initEmulators();
  }, []);

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', fontSize:12}}>
      <div style={{padding:'8px', borderBottom:'1px solid #333', display:'flex', alignItems:'center', gap:12}}>
        <strong>Dual Emulator Comparison</strong>
        <select 
          value={selectedTest} 
          onChange={e => setSelectedTest(e.target.value)}
          style={{background:'#111', color:'#ddd', border:'1px solid #333'}}
        >
          <option value="bios-presentation">BIOS Presentation</option>
          <option value="simple-line">Simple Line</option>
          <option value="vector-test">Vector Test</option>
        </select>
        <label>
          Frames: 
          <input 
            type="number" 
            value={maxFrames} 
            onChange={e => setMaxFrames(Math.max(1, parseInt(e.target.value) || 1))}
            style={{width:50, marginLeft:4, background:'#111', color:'#ddd', border:'1px solid #333'}}
            min={1}
            max={100}
          />
        </label>
        <label style={{display:'flex', alignItems:'center', gap:4}}>
          <input 
            type="checkbox" 
            checked={enableTracing} 
            onChange={e => setEnableTracing(e.target.checked)} 
          />
          Trace
        </label>
        <button 
          onClick={() => runTest(selectedTest)}
          disabled={isRunning || !biosLoaded}
          style={{
            padding:'4px 12px', 
            background: isRunning ? '#666' : '#0066cc', 
            color:'white', 
            border:'none', 
            borderRadius:4,
            cursor: isRunning ? 'not-allowed' : 'pointer'
          }}
        >
          {isRunning ? 'Running...' : 'Run Test'}
        </button>
        <button 
          onClick={initEmulators}
          style={{
            padding:'4px 12px', 
            background:'#666', 
            color:'white', 
            border:'none', 
            borderRadius:4,
            cursor: 'pointer'
          }}
        >
          Reload BIOS
        </button>
        <span style={{color: biosLoaded ? '#0a0' : '#a00'}}>
          BIOS: {biosLoaded ? 'Loaded' : 'Not loaded'}
        </span>
        <span style={{color:'#aaa', fontSize:10, marginLeft:8}}>
          Current: {currentBackend}
        </span>
        <button 
          onClick={() => {
            const newBackend = currentBackend === 'rust' ? 'jsvecx' : 'rust';
            persistPreference(newBackend);
            setCurrentBackend(newBackend);
            setTimeout(() => location.reload(), 100);
          }}
          style={{padding:'2px 8px', fontSize:10, background:'#444', color:'white', border:'none', borderRadius:3}}
        >
          Switch to {currentBackend === 'rust' ? 'jsvecx' : 'rust'}
        </button>
      </div>

      <div style={{flex:1, overflow:'auto', padding:8}}>
        {results.length === 0 ? (
          <div style={{textAlign:'center', padding:40, color:'#666'}}>
            No test results yet. Click "Run Test" to compare emulators.
          </div>
        ) : (
          <div>
            {results.map((result, idx) => (
              <div key={result.timestamp} style={{
                marginBottom:20, 
                padding:12, 
                border:'1px solid #333', 
                borderRadius:4,
                background: idx === 0 ? '#001122' : '#111'
              }}>
                <div style={{fontWeight:'bold', marginBottom:8}}>
                  {result.testName} ({new Date(result.timestamp).toLocaleTimeString()})
                </div>
                
                {result.divergences.length > 0 && (
                  <div style={{marginBottom:8}}>
                    <strong style={{color:'#ff6666'}}>Divergences found:</strong>
                    <ul style={{margin:'4px 0', paddingLeft:20}}>
                      {result.divergences.map((div, i) => (
                        <li key={i} style={{color:'#ffaa66'}}>{div}</li>
                      ))}
                    </ul>
                  </div>
                )}

                <div style={{display:'flex', gap:20}}>
                  <div style={{flex:1}}>
                    <strong>Rust Emulator:</strong>
                    <div>Segments: {result.rustSegments.length}</div>
                    <div>PC: 0x{result.rustRegisters?.pc.toString(16).padStart(4, '0')}</div>
                    <div>Frames: {result.rustMetrics?.frames}</div>
                    {result.rustTrace && <div>Trace entries: {result.rustTrace.length}</div>}
                    {result.rustSegments.slice(0, 3).map((seg, i) => (
                      <div key={i} style={{fontSize:10, color:'#aaa'}}>{formatSegment(seg)}</div>
                    ))}
                  </div>
                  
                  <div style={{flex:1}}>
                    <strong>jsvecx Emulator:</strong>
                    <div>Segments: {result.jsvecxSegments.length}</div>
                    <div>PC: 0x{result.jsvecxRegisters?.pc.toString(16).padStart(4, '0')}</div>
                    <div>Frames: {result.jsvecxMetrics?.frames}</div>
                    {result.jsvecxTrace && <div>Trace entries: {result.jsvecxTrace.length}</div>}
                    {result.jsvecxSegments.slice(0, 3).map((seg, i) => (
                      <div key={i} style={{fontSize:10, color:'#aaa'}}>{formatSegment(seg)}</div>
                    ))}
                  </div>
                </div>
                
                {(result.rustTrace || result.jsvecxTrace) && (
                  <details style={{marginTop:8}}>
                    <summary style={{cursor:'pointer', color:'#66aaff'}}>Execution Traces</summary>
                    <div style={{display:'flex', gap:20, marginTop:8, fontSize:10}}>
                      {result.rustTrace && (
                        <div style={{flex:1}}>
                          <strong>Rust Trace (first 10):</strong>
                          {result.rustTrace.slice(0, 10).map((entry: any, i: number) => (
                            <div key={i} style={{fontFamily:'monospace', color:'#ccc'}}>
                              {JSON.stringify(entry).slice(0, 80)}...
                            </div>
                          ))}
                        </div>
                      )}
                      {result.jsvecxTrace && (
                        <div style={{flex:1}}>
                          <strong>jsvecx Trace (first 10):</strong>
                          {result.jsvecxTrace.slice(0, 10).map((entry: any, i: number) => (
                            <div key={i} style={{fontFamily:'monospace', color:'#ccc'}}>
                              {JSON.stringify(entry).slice(0, 80)}...
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  </details>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}