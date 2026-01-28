/**
 * AnimationEditor - Visual editor for .vanim animation resources
 * 
 * Features:
 * - Frame management (add, edit, delete, reorder)
 * - State machine editor (idle, walking, attacking, etc.)
 * - Real-time preview with playback controls
 * - Timeline view for frame durations
 * - Integration with .vec assets
 */

import React, { useRef, useEffect, useState, useCallback, useMemo } from 'react';
import { useProjectStore } from '../state/projectStore';
import { useEditorStore } from '../state/editorStore';
import type { FileNode } from '../types/models';

// ============================================
// Types from .vanim format
// ============================================

interface AnimFrame {
  id: string;
  vectorName: string;
  duration: number;
  intensity: number;
  offset_x: number;
  offset_y: number;
  mirror: 0 | 1 | 2 | 3;
}

interface AnimState {
  name: string;
  frames: string[];  // Array of frame IDs
  loop_state: boolean;
}

interface AnimResource {
  version: string;
  name: string;
  frames: AnimFrame[];
  states: { [key: string]: AnimState };
}

interface AnimationEditorProps {
  resource?: AnimResource;
  onChange?: (resource: AnimResource) => void;
  width?: number;
  height?: number;
}

const defaultResource: AnimResource = {
  version: '1.0',
  name: 'untitled',
  frames: [],
  states: {},
};

const MIRROR_MODES = [
  { value: 0, label: 'Normal' },
  { value: 1, label: 'X-Flip (Mirror Left-Right)' },
  { value: 2, label: 'Y-Flip (Mirror Up-Down)' },
  { value: 3, label: 'XY-Flip (180° Rotation)' },
];

// ============================================
// Main Component
// ============================================

export const AnimationEditor: React.FC<AnimationEditorProps> = ({
  resource: initialResource,
  onChange,
  width = 800,
  height = 600,
}) => {
  const [resource, setResource] = useState<AnimResource>(initialResource || defaultResource);
  const [selectedFrame, setSelectedFrame] = useState<string | null>(null);
  const [selectedState, setSelectedState] = useState<string | null>(null);
  const [playing, setPlaying] = useState(false);
  const [currentFrame, setCurrentFrame] = useState(0);
  const [currentTick, setCurrentTick] = useState(0);
  const animationFrameRef = useRef<number | null>(null);
  const lastTimeRef = useRef<number>(0);
  const [previewMirror, setPreviewMirror] = useState<0 | 1 | 2 | 3>(0);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [vectorResources, setVectorResources] = useState<Map<string, any>>(new Map());
  const [globalBounds, setGlobalBounds] = useState<{ autoScale: number } | null>(null);
  
  // Get available .vec files from project
  const project = useProjectStore(s => s.project);
  const { documents } = useEditorStore();

  // Load .vec resource directly (no caching) - for initial bounds calculation
  const loadVectorResourceDirect = async (vectorName: string): Promise<any> => {
    if (!project?.rootPath) return null;

    // Try opened documents first
    const doc = documents.find(d => d.uri.includes(`${vectorName}.vec`));
    if (doc?.content) {
      try {
        return JSON.parse(doc.content);
      } catch (e) {
        console.error('[AnimEditor] Failed to parse .vec from document:', e);
        return null;
      }
    }
    
    // Try Electron API
    const absPath = `${project.rootPath}/assets/vectors/${vectorName}.vec`;
    const filesAPI = (window as any).files;
    
    if (filesAPI?.readFile) {
      try {
        const response = await filesAPI.readFile(absPath);
        if (response?.content) {
          return JSON.parse(response.content);
        }
      } catch (e) {
        console.error('[AnimEditor] Error loading vector:', vectorName, e);
      }
    }
    
    return null;
  };

  // Load .vec resource from editor (with caching for rendering)
  const loadVectorResource = useCallback(async (vectorName: string) => {
    if (!project?.rootPath) {
      console.warn('[AnimEditor] No project root path');
      return null;
    }
    
    // Check cache first
    if (vectorResources.has(vectorName)) {
      console.log('[AnimEditor] Using cached vector:', vectorName);
      return vectorResources.get(vectorName);
    }

    console.log('[AnimEditor] Loading vector:', vectorName, 'from root:', project.rootPath);

    // Strategy 1: Find in already-opened documents
    const doc = documents.find(d => d.uri.includes(`${vectorName}.vec`));
    if (doc?.content) {
      console.log('[AnimEditor] Found vector in open documents:', vectorName);
      try {
        const parsed = JSON.parse(doc.content);
        setVectorResources(prev => new Map(prev).set(vectorName, parsed));
        return parsed;
      } catch (e) {
        console.error('[AnimEditor] Failed to parse .vec from document:', vectorName, e);
        return null;
      }
    }
    
    // Strategy 2: Load from filesystem via electron API (if available)
    const absPath = `${project.rootPath}/assets/vectors/${vectorName}.vec`;
    const filesAPI = (window as any).files;
    
    if (filesAPI?.readFile) {
      console.log('[AnimEditor] Attempting Electron files.readFile:', absPath);
      try {
        const response = await filesAPI.readFile(absPath);
        console.log('[AnimEditor] Electron read response:', response?.error ? 'ERROR' : 'SUCCESS');
        
        if (response?.content) {
          const parsed = JSON.parse(response.content);
          console.log('[AnimEditor] Successfully loaded vector:', vectorName, 'layers:', parsed.layers?.length);
          setVectorResources(prev => new Map(prev).set(vectorName, parsed));
          return parsed;
        } else if (response?.error) {
          console.error('[AnimEditor] Electron read error:', response.error);
        }
      } catch (e) {
        console.error('[AnimEditor] Electron API error:', e);
      }
    } else {
      console.log('[AnimEditor] Electron files API not available');
    }
    
    // Strategy 3: Fallback to fetch() for dev mode (when running standalone frontend)
    // Try to open the .vec file directly via editor store (if already open)
    console.log('[AnimEditor] Electron API not available, trying to open file via editor');
    
    // Ask user to open the .vec file if not already open
    const vecFileName = `${vectorName}.vec`;
    console.log('[AnimEditor] Please open the file:', vecFileName, 'manually in the IDE');
    
    // Check if any document matches this vector name
    const vecDoc = documents.find(d => 
      d.uri.includes(vecFileName) || 
      d.uri.endsWith(vecFileName)
    );
    
    if (vecDoc?.content) {
      console.log('[AnimEditor] Found opened .vec file in documents!');
      try {
        const parsed = JSON.parse(vecDoc.content);
        console.log('[AnimEditor] Successfully parsed opened vector:', vectorName);
        setVectorResources(prev => new Map(prev).set(vectorName, parsed));
        return parsed;
      } catch (e) {
        console.error('[AnimEditor] Failed to parse opened .vec:', e);
      }
    }
    
    console.warn('[AnimEditor] Vector not found:', vectorName);
    return null;
  }, [project, documents, vectorResources]);
  const availableVectors = useMemo(() => {
    if (!project?.files) return [];
    
    const vecFiles: string[] = [];
    
    // Recursive function to traverse FileNode tree
    const findVecFiles = (nodes: FileNode[]) => {
      for (const node of nodes) {
        if (!node.isDir && node.name.endsWith('.vec')) {
          const name = node.name.replace('.vec', '');
          if (!vecFiles.includes(name)) {
            vecFiles.push(name);
          }
        } else if (node.isDir && node.children) {
          findVecFiles(node.children);
        }
      }
    };
    
    findVecFiles(project.files);
    
    return vecFiles;
  }, [project]);

  // Notify parent of changes
  useEffect(() => {
    if (onChange) {
      onChange(resource);
    }
  }, [resource, onChange]);

  // Animation playback loop (50 FPS = 20ms per frame)
  useEffect(() => {
    if (!playing || !selectedState) {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
      return;
    }

    const state = resource.states[selectedState];
    if (!state || state.frames.length === 0) return;

    const animate = (timestamp: number) => {
      if (!lastTimeRef.current) lastTimeRef.current = timestamp;
      const elapsed = timestamp - lastTimeRef.current;

      // 50 FPS = 20ms per tick
      if (elapsed >= 20) {
        lastTimeRef.current = timestamp;
        
        const frameId = state.frames[currentFrame];
        const frame = resource.frames.find(f => f.id === frameId);
        
        if (frame) {
          setCurrentTick(prev => {
            const next = prev + 1;
            if (next >= frame.duration) {
              // Move to next frame
              const nextFrame = currentFrame + 1;
              if (nextFrame >= state.frames.length) {
                if (state.loop_state) {
                  setCurrentFrame(0);
                  return 0;
                } else {
                  setPlaying(false);
                  return 0;
                }
              } else {
                setCurrentFrame(nextFrame);
                return 0;
              }
            }
            return next;
          });
        }
      }

      animationFrameRef.current = requestAnimationFrame(animate);
    };

    animationFrameRef.current = requestAnimationFrame(animate);

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [playing, selectedState, currentFrame, resource]);

  // ============================================
  // Frame Management
  // ============================================

  const addFrame = useCallback(() => {
    const newId = `frame_${Date.now()}`;
    const newFrame: AnimFrame = {
      id: newId,
      vectorName: availableVectors[0] || '',
      duration: 10,
      intensity: 127,
      offset_x: 0,
      offset_y: 0,
      mirror: 0,
    };
    
    setResource(prev => ({
      ...prev,
      frames: [...prev.frames, newFrame],
    }));
    setSelectedFrame(newId);
  }, [availableVectors]);

  const deleteFrame = useCallback((frameId: string) => {
    if (!confirm(`Delete frame "${frameId}"? This will remove it from all states.`)) return;
    
    setResource(prev => ({
      ...prev,
      frames: prev.frames.filter(f => f.id !== frameId),
      states: Object.fromEntries(
        Object.entries(prev.states).map(([key, state]) => [
          key,
          { ...state, frames: state.frames.filter(fid => fid !== frameId) },
        ])
      ),
    }));
    
    if (selectedFrame === frameId) {
      setSelectedFrame(null);
    }
  }, [selectedFrame]);

  const updateFrame = useCallback((frameId: string, updates: Partial<AnimFrame>) => {
    setResource(prev => ({
      ...prev,
      frames: prev.frames.map(f =>
        f.id === frameId ? { ...f, ...updates } : f
      ),
    }));
  }, []);

  const duplicateFrame = useCallback((frameId: string) => {
    const frame = resource.frames.find(f => f.id === frameId);
    if (!frame) return;
    
    const newId = `${frameId}_copy_${Date.now()}`;
    const newFrame: AnimFrame = {
      ...frame,
      id: newId,
    };
    
    setResource(prev => ({
      ...prev,
      frames: [...prev.frames, newFrame],
    }));
    setSelectedFrame(newId);
  }, [resource.frames]);

  // ============================================
  // State Management
  // ============================================

  const addState = useCallback(() => {
    const name = prompt('State name (e.g., idle, walking, attacking):');
    if (!name || name.trim() === '') return;
    
    if (resource.states[name]) {
      alert(`State "${name}" already exists`);
      return;
    }
    
    const newState: AnimState = {
      name: name.trim(),
      frames: [],
      loop_state: true,
    };
    
    setResource(prev => ({
      ...prev,
      states: {
        ...prev.states,
        [name.trim()]: newState,
      },
    }));
    setSelectedState(name.trim());
  }, [resource.states]);

  const deleteState = useCallback((stateName: string) => {
    if (!confirm(`Delete state "${stateName}"?`)) return;
    
    setResource(prev => {
      const newStates = { ...prev.states };
      delete newStates[stateName];
      return {
        ...prev,
        states: newStates,
      };
    });
    
    if (selectedState === stateName) {
      setSelectedState(null);
    }
  }, [selectedState]);

  const updateState = useCallback((stateName: string, updates: Partial<AnimState>) => {
    setResource(prev => ({
      ...prev,
      states: {
        ...prev.states,
        [stateName]: {
          ...prev.states[stateName],
          ...updates,
        },
      },
    }));
  }, []);

  const addFrameToState = useCallback((stateName: string, frameId: string) => {
    setResource(prev => ({
      ...prev,
      states: {
        ...prev.states,
        [stateName]: {
          ...prev.states[stateName],
          frames: [...prev.states[stateName].frames, frameId],
        },
      },
    }));
  }, []);

  const removeFrameFromState = useCallback((stateName: string, index: number) => {
    setResource(prev => ({
      ...prev,
      states: {
        ...prev.states,
        [stateName]: {
          ...prev.states[stateName],
          frames: prev.states[stateName].frames.filter((_, i) => i !== index),
        },
      },
    }));
  }, []);

  const moveFrameInState = useCallback((stateName: string, fromIndex: number, toIndex: number) => {
    setResource(prev => {
      const state = prev.states[stateName];
      const newFrames = [...state.frames];
      const [moved] = newFrames.splice(fromIndex, 1);
      newFrames.splice(toIndex, 0, moved);
      
      return {
        ...prev,
        states: {
          ...prev.states,
          [stateName]: {
            ...state,
            frames: newFrames,
          },
        },
      };
    });
  }, []);

  // ============================================
  // Preview Controls
  // ============================================

  // Calculate global bounding box for all frames in animation
  useEffect(() => {
    console.log('[AnimEditor] Calculating global bounds for', resource.frames.length, 'frames');
    
    const calculateGlobalBounds = async () => {
      // Collect all unique vector names from all frames
      const vectorNames = new Set<string>();
      for (const frame of resource.frames) {
        vectorNames.add(frame.vectorName);
      }

      console.log('[AnimEditor] Unique vectors in animation:', Array.from(vectorNames));

      // Load all vectors and calculate global bounding box
      let globalMinX = Infinity, globalMaxX = -Infinity;
      let globalMinY = Infinity, globalMaxY = -Infinity;
      let loadedCount = 0;

      for (const vectorName of vectorNames) {
        const vecResource = await loadVectorResourceDirect(vectorName);
        if (!vecResource?.layers) {
          console.warn('[AnimEditor] No layers in', vectorName);
          continue;
        }

        loadedCount++;
        console.log(`[AnimEditor] Processing vector ${loadedCount}/${vectorNames.size}:`, vectorName);

        for (const layer of vecResource.layers) {
          if (!layer.visible || !layer.paths) continue;
          for (const path of layer.paths) {
            if (!path.points) continue;
            for (const point of path.points) {
              globalMinX = Math.min(globalMinX, point.x);
              globalMaxX = Math.max(globalMaxX, point.x);
              globalMinY = Math.min(globalMinY, point.y);
              globalMaxY = Math.max(globalMaxY, point.y);
            }
          }
        }
      }

      console.log('[AnimEditor] Loaded', loadedCount, 'vectors for global bounds');

      if (isFinite(globalMinX) && isFinite(globalMaxX)) {
        const rangeX = globalMaxX - globalMinX;
        const rangeY = globalMaxY - globalMinY;
        const maxRange = Math.max(rangeX, rangeY);
        const targetSize = 200;
        const autoScale = maxRange > 0 ? targetSize / maxRange : 1;

        console.log('[AnimEditor] ✓ Global scale calculated:', {
          minX: globalMinX, maxX: globalMaxX,
          minY: globalMinY, maxY: globalMaxY,
          maxRange, autoScale,
          vectorCount: vectorNames.size
        });

        setGlobalBounds({ autoScale });
      } else {
        console.warn('[AnimEditor] Could not calculate finite global bounds');
      }
    };

    calculateGlobalBounds();
  }, [resource.frames, project, documents]); // Now includes dependencies for loadVectorResourceDirect

  const playState = useCallback(() => {
    if (!selectedState) {
      alert('Select a state to preview');
      return;
    }
    setCurrentFrame(0);
    setCurrentTick(0);
    setPlaying(true);
  }, [selectedState]);

  const stopPreview = useCallback(() => {
    setPlaying(false);
    setCurrentFrame(0);
    setCurrentTick(0);
  }, []);

  // Get current preview frame
  const previewFrame = useMemo(() => {
    if (!selectedState) return null;
    const state = resource.states[selectedState];
    if (!state || state.frames.length === 0) return null;
    const frameId = state.frames[currentFrame];
    return resource.frames.find(f => f.id === frameId) || null;
  }, [selectedState, currentFrame, resource]);

  // Selected frame for editing
  const editFrame = useMemo(() => {
    return resource.frames.find(f => f.id === selectedFrame) || null;
  }, [selectedFrame, resource.frames]);

  // Draw vector on canvas
  const drawVectorOnCanvas = useCallback((frame: AnimFrame, mirror: number) => {
    const canvas = canvasRef.current;
    if (!canvas) {
      console.warn('[AnimEditor] Canvas ref not available');
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      console.warn('[AnimEditor] Canvas context not available');
      return;
    }

    // Clear canvas
    ctx.fillStyle = '#1e1e2e';
    ctx.fillRect(0, 0, 256, 256);

    console.log('[AnimEditor] Drawing frame:', frame.vectorName);

    // Load vector resource
    loadVectorResource(frame.vectorName).then(vecResource => {
      if (!vecResource?.layers) {
        console.warn('[AnimEditor] No layers in vector resource:', frame.vectorName);
        // Show error text if vector not found
        ctx.fillStyle = '#f38ba8';
        ctx.font = '12px monospace';
        ctx.textAlign = 'center';
        ctx.fillText(`Vector "${frame.vectorName}" not found`, 128, 128);
        return;
      }

      console.log('[AnimEditor] Drawing vector with', vecResource.layers.length, 'layers');

      // Use global bounds if available for consistent scaling
      let autoScale = 1;
      
      if (globalBounds) {
        // Use pre-calculated global scale (consistent across all frames)
        autoScale = globalBounds.autoScale;
        console.log('[AnimEditor] Using global scale:', autoScale);
      } else {
        // Fallback: calculate scale for this frame only
        let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
        for (const layer of vecResource.layers) {
          if (!layer.visible || !layer.paths) continue;
          for (const path of layer.paths) {
            if (!path.points) continue;
            for (const point of path.points) {
              minX = Math.min(minX, point.x);
              maxX = Math.max(maxX, point.x);
              minY = Math.min(minY, point.y);
              maxY = Math.max(maxY, point.y);
            }
          }
        }
        
        const rangeX = maxX - minX;
        const rangeY = maxY - minY;
        const maxRange = Math.max(rangeX, rangeY);
        const targetSize = 200;
        autoScale = maxRange > 0 ? targetSize / maxRange : 1;
        console.log('[AnimEditor] Calculated per-frame scale:', autoScale);
      }

      // Apply transformations
      ctx.save();
      
      // Step 1: Move to center of canvas
      ctx.translate(128, 128);
      
      // Step 2: Apply user offset from frame
      ctx.translate(frame.offset_x || 0, -(frame.offset_y || 0));

      // Step 3: Apply mirror
      const scaleX = (mirror === 1 || mirror === 3) ? -1 : 1;
      const scaleY = (mirror === 2 || mirror === 3) ? -1 : 1;
      
      // Step 4: Apply scale (auto + mirror combined)
      ctx.scale(scaleX * autoScale, scaleY * autoScale);
      
      console.log('[AnimEditor] Transform applied:', {
        centerAt: [128, 128],
        userOffset: [frame.offset_x || 0, frame.offset_y || 0],
        mirror: [scaleX, scaleY],
        autoScale
      });

      let pathCount = 0;
      // Draw all paths from all layers
      for (const layer of vecResource.layers) {
        if (!layer.visible || !layer.paths) continue;

        for (const path of layer.paths) {
          if (!path.points || path.points.length === 0) continue;

          pathCount++;
          console.log('[AnimEditor] Drawing path', pathCount, 'with', path.points.length, 'points');

          // Calculate intensity-based color (same as VectorEditor)
          const intensity = (path.intensity || frame.intensity) / 127;
          const green = Math.floor(200 + 55 * intensity);
          ctx.strokeStyle = `rgb(${Math.floor(100 * intensity)}, ${green}, ${Math.floor(100 * intensity)})`;
          
          // Adjust line width inversely to scale (keep visual thickness consistent)
          ctx.lineWidth = 2 / autoScale;
          ctx.lineCap = 'round';
          ctx.lineJoin = 'round';

          ctx.beginPath();
          const firstPoint = path.points[0];
          ctx.moveTo(firstPoint.x, -firstPoint.y); // Y inverted for Vectrex coordinates

          for (let i = 1; i < path.points.length; i++) {
            const point = path.points[i];
            ctx.lineTo(point.x, -point.y);
          }

          // Close path if needed
          if (path.closed) {
            ctx.closePath();
          }

          ctx.stroke();
        }
      }

      console.log('[AnimEditor] Drew', pathCount, 'paths total');

      ctx.restore();

      // Draw info overlay
      ctx.fillStyle = 'rgba(30, 30, 46, 0.8)';
      ctx.fillRect(0, 0, 256, 30);
      ctx.fillStyle = '#89b4fa';
      ctx.font = '11px monospace';
      ctx.textAlign = 'left';
      ctx.fillText(`${frame.vectorName} • Frame ${currentFrame + 1} • Tick ${currentTick}/${frame.duration}`, 8, 18);
    }).catch(e => {
      console.error('[AnimEditor] Error drawing vector:', e);
    });
  }, [loadVectorResource, currentFrame, currentTick, globalBounds]);

  // Redraw canvas when preview frame or mirror changes
  useEffect(() => {
    if (previewFrame) {
      drawVectorOnCanvas(previewFrame, previewMirror);
    } else {
      // Clear canvas when no frame
      const canvas = canvasRef.current;
      if (canvas) {
        const ctx = canvas.getContext('2d');
        if (ctx) {
          ctx.fillStyle = '#1e1e2e';
          ctx.fillRect(0, 0, 256, 256);
        }
      }
    }
  }, [previewFrame, previewMirror, drawVectorOnCanvas]);

  // ============================================
  // Render
  // ============================================

  return (
    <div style={{ 
      display: 'flex', 
      width, 
      height, 
      background: '#1e1e2e',
      color: '#cdd6f4',
      fontFamily: 'system-ui, sans-serif',
      overflow: 'hidden',
    }}>
      {/* Left Panel: Frames List */}
      <div style={{
        width: 280,
        borderRight: '1px solid #313244',
        display: 'flex',
        flexDirection: 'column',
        overflow: 'hidden',
      }}>
        <div style={{
          padding: '12px',
          borderBottom: '1px solid #313244',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
        }}>
          <h3 style={{ margin: 0, fontSize: 14, fontWeight: 600 }}>Frames</h3>
          <button onClick={addFrame} style={buttonStyle}>+ Add</button>
        </div>
        
        <div style={{ flex: 1, overflow: 'auto', padding: '8px' }}>
          {resource.frames.length === 0 ? (
            <div style={{ padding: 16, textAlign: 'center', color: '#6c7086', fontSize: 12 }}>
              No frames. Click "+ Add" to create one.
            </div>
          ) : (
            resource.frames.map(frame => (
              <div
                key={frame.id}
                onClick={() => setSelectedFrame(frame.id)}
                style={{
                  padding: '8px 12px',
                  marginBottom: 4,
                  background: selectedFrame === frame.id ? '#45475a' : '#313244',
                  borderRadius: 4,
                  cursor: 'pointer',
                  fontSize: 12,
                  border: selectedFrame === frame.id ? '1px solid #89b4fa' : '1px solid transparent',
                }}
              >
                <div style={{ fontWeight: 600, marginBottom: 4 }}>{frame.id}</div>
                <div style={{ color: '#a6adc8', fontSize: 11 }}>
                  {frame.vectorName} • {frame.duration}t
                </div>
                <div style={{ marginTop: 4, display: 'flex', gap: 4 }}>
                  <button 
                    onClick={(e) => { e.stopPropagation(); duplicateFrame(frame.id); }}
                    style={{ ...smallButtonStyle }}
                  >
                    Duplicate
                  </button>
                  <button 
                    onClick={(e) => { e.stopPropagation(); deleteFrame(frame.id); }}
                    style={{ ...smallButtonStyle, color: '#f38ba8' }}
                  >
                    Delete
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      {/* Center Panel: Preview & Properties */}
      <div style={{
        flex: 1,
        display: 'flex',
        flexDirection: 'column',
        overflow: 'hidden',
      }}>
        {/* Preview Area */}
        <div style={{
          flex: 1,
          background: '#11111b',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          padding: 16,
          borderBottom: '1px solid #313244',
          position: 'relative',
        }}>
          <div style={{
            width: 256,
            height: 256,
            background: '#1e1e2e',
            border: '2px solid #45475a',
            borderRadius: 8,
            position: 'relative',
            overflow: 'hidden',
          }}>
            <canvas
              ref={canvasRef}
              width={256}
              height={256}
              style={{
                display: 'block',
                imageRendering: 'crisp-edges',
              }}
            />
            {!previewFrame && (
              <div style={{
                position: 'absolute',
                top: '50%',
                left: '50%',
                transform: 'translate(-50%, -50%)',
                color: '#6c7086',
                fontSize: 12,
                textAlign: 'center',
                pointerEvents: 'none',
              }}>
                Select a state and click Play
              </div>
            )}
          </div>

          {/* Playback Controls */}
          <div style={{
            marginTop: 16,
            display: 'flex',
            gap: 8,
            alignItems: 'center',
          }}>
            <button onClick={playState} disabled={playing} style={buttonStyle}>
              ▶ Play
            </button>
            <button onClick={stopPreview} disabled={!playing} style={buttonStyle}>
              ⏹ Stop
            </button>
            <div style={{ marginLeft: 16, fontSize: 12 }}>
              Mirror:
              <select 
                value={previewMirror} 
                onChange={(e) => setPreviewMirror(Number(e.target.value) as 0 | 1 | 2 | 3)}
                style={selectStyle}
              >
                {MIRROR_MODES.map(mode => (
                  <option key={mode.value} value={mode.value}>{mode.label}</option>
                ))}
              </select>
            </div>
          </div>
        </div>

        {/* Frame Properties Editor */}
        {editFrame && (
          <div style={{
            height: 'auto',
            maxHeight: 280,
            overflow: 'auto',
            padding: 16,
            background: '#181825',
          }}>
            <h3 style={{ margin: '0 0 12px 0', fontSize: 14, fontWeight: 600 }}>
              Frame Properties: {editFrame.id}
            </h3>
            
            <div style={{ display: 'grid', gridTemplateColumns: '120px 1fr', gap: '8px 12px', fontSize: 12 }}>
              <label>Vector Asset:</label>
              <select 
                value={editFrame.vectorName}
                onChange={(e) => updateFrame(editFrame.id, { vectorName: e.target.value })}
                style={selectStyle}
              >
                {availableVectors.length === 0 ? (
                  <option value="">No .vec files found</option>
                ) : (
                  availableVectors.map(vec => (
                    <option key={vec} value={vec}>{vec}</option>
                  ))
                )}
              </select>

              <label>Duration (ticks):</label>
              <input 
                type="number"
                value={editFrame.duration}
                onChange={(e) => updateFrame(editFrame.id, { duration: parseInt(e.target.value) || 1 })}
                min={1}
                max={100}
                style={inputStyle}
              />

              <label>Intensity (0-127):</label>
              <input 
                type="number"
                value={editFrame.intensity}
                onChange={(e) => updateFrame(editFrame.id, { intensity: parseInt(e.target.value) || 0 })}
                min={0}
                max={127}
                style={inputStyle}
              />

              <label>Offset X:</label>
              <input 
                type="number"
                value={editFrame.offset_x}
                onChange={(e) => updateFrame(editFrame.id, { offset_x: parseInt(e.target.value) || 0 })}
                min={-127}
                max={127}
                style={inputStyle}
              />

              <label>Offset Y:</label>
              <input 
                type="number"
                value={editFrame.offset_y}
                onChange={(e) => updateFrame(editFrame.id, { offset_y: parseInt(e.target.value) || 0 })}
                min={-127}
                max={127}
                style={inputStyle}
              />

              <label>Mirror Mode:</label>
              <select 
                value={editFrame.mirror}
                onChange={(e) => updateFrame(editFrame.id, { mirror: Number(e.target.value) as 0 | 1 | 2 | 3 })}
                style={selectStyle}
              >
                {MIRROR_MODES.map(mode => (
                  <option key={mode.value} value={mode.value}>{mode.label}</option>
                ))}
              </select>
            </div>
          </div>
        )}
      </div>

      {/* Right Panel: States */}
      <div style={{
        width: 300,
        borderLeft: '1px solid #313244',
        display: 'flex',
        flexDirection: 'column',
        overflow: 'hidden',
      }}>
        <div style={{
          padding: '12px',
          borderBottom: '1px solid #313244',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
        }}>
          <h3 style={{ margin: 0, fontSize: 14, fontWeight: 600 }}>States</h3>
          <button onClick={addState} style={buttonStyle}>+ Add</button>
        </div>

        <div style={{ flex: 1, overflow: 'auto', padding: '8px' }}>
          {Object.keys(resource.states).length === 0 ? (
            <div style={{ padding: 16, textAlign: 'center', color: '#6c7086', fontSize: 12 }}>
              No states. Click "+ Add" to create one.
            </div>
          ) : (
            Object.entries(resource.states).map(([key, state]) => (
              <div
                key={key}
                style={{
                  marginBottom: 12,
                  background: selectedState === key ? '#45475a' : '#313244',
                  borderRadius: 4,
                  border: selectedState === key ? '1px solid #89b4fa' : '1px solid transparent',
                }}
              >
                <div
                  onClick={() => setSelectedState(key)}
                  style={{
                    padding: '8px 12px',
                    cursor: 'pointer',
                    fontSize: 12,
                  }}
                >
                  <div style={{ fontWeight: 600, marginBottom: 4 }}>{key}</div>
                  <div style={{ color: '#a6adc8', fontSize: 11, marginBottom: 8 }}>
                    {state.frames.length} frames • {state.loop_state ? 'Loop' : 'Once'}
                  </div>
                  
                  <div style={{ marginBottom: 8 }}>
                    <label style={{ fontSize: 11, display: 'flex', alignItems: 'center', gap: 6 }}>
                      <input 
                        type="checkbox"
                        checked={state.loop_state}
                        onChange={(e) => {
                          e.stopPropagation();
                          updateState(key, { loop_state: e.target.checked });
                        }}
                      />
                      Loop animation
                    </label>
                  </div>

                  {/* Frame sequence */}
                  <div style={{ marginTop: 8, fontSize: 11 }}>
                    <div style={{ marginBottom: 4, fontWeight: 600 }}>Sequence:</div>
                    {state.frames.length === 0 ? (
                      <div style={{ color: '#6c7086', fontStyle: 'italic' }}>No frames</div>
                    ) : (
                      <div style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                        {state.frames.map((frameId, index) => (
                          <div key={`${frameId}-${index}`} style={{
                            display: 'flex',
                            alignItems: 'center',
                            gap: 4,
                            padding: '2px 4px',
                            background: '#1e1e2e',
                            borderRadius: 2,
                          }}>
                            <span style={{ flex: 1 }}>{index + 1}. {frameId}</span>
                            <button 
                              onClick={(e) => { 
                                e.stopPropagation(); 
                                if (index > 0) moveFrameInState(key, index, index - 1);
                              }}
                              disabled={index === 0}
                              style={{ ...tinyButtonStyle }}
                            >
                              ▲
                            </button>
                            <button 
                              onClick={(e) => { 
                                e.stopPropagation(); 
                                if (index < state.frames.length - 1) moveFrameInState(key, index, index + 1);
                              }}
                              disabled={index === state.frames.length - 1}
                              style={{ ...tinyButtonStyle }}
                            >
                              ▼
                            </button>
                            <button 
                              onClick={(e) => { 
                                e.stopPropagation(); 
                                removeFrameFromState(key, index);
                              }}
                              style={{ ...tinyButtonStyle, color: '#f38ba8' }}
                            >
                              ×
                            </button>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>

                  {/* Add frame to state */}
                  <div style={{ marginTop: 8 }}>
                    <select 
                      onChange={(e) => {
                        e.stopPropagation();
                        if (e.target.value) {
                          addFrameToState(key, e.target.value);
                          e.target.value = '';
                        }
                      }}
                      style={{ ...selectStyle, fontSize: 10, padding: '2px 4px' }}
                      onClick={(e) => e.stopPropagation()}
                    >
                      <option value="">+ Add frame to sequence</option>
                      {resource.frames.map(frame => (
                        <option key={frame.id} value={frame.id}>{frame.id}</option>
                      ))}
                    </select>
                  </div>

                  {/* Delete state */}
                  <div style={{ marginTop: 8 }}>
                    <button 
                      onClick={(e) => { 
                        e.stopPropagation(); 
                        deleteState(key);
                      }}
                      style={{ ...smallButtonStyle, color: '#f38ba8', width: '100%' }}
                    >
                      Delete State
                    </button>
                  </div>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
};

// ============================================
// Styles
// ============================================

const buttonStyle: React.CSSProperties = {
  background: '#89b4fa',
  color: '#1e1e2e',
  border: 'none',
  padding: '6px 12px',
  borderRadius: 4,
  fontSize: 12,
  fontWeight: 600,
  cursor: 'pointer',
};

const smallButtonStyle: React.CSSProperties = {
  background: '#313244',
  color: '#cdd6f4',
  border: '1px solid #45475a',
  padding: '3px 8px',
  borderRadius: 3,
  fontSize: 10,
  cursor: 'pointer',
};

const tinyButtonStyle: React.CSSProperties = {
  background: '#313244',
  color: '#cdd6f4',
  border: '1px solid #45475a',
  padding: '1px 4px',
  borderRadius: 2,
  fontSize: 9,
  cursor: 'pointer',
  lineHeight: 1,
};

const inputStyle: React.CSSProperties = {
  background: '#313244',
  color: '#cdd6f4',
  border: '1px solid #45475a',
  padding: '4px 8px',
  borderRadius: 4,
  fontSize: 12,
  width: '100%',
};

const selectStyle: React.CSSProperties = {
  background: '#313244',
  color: '#cdd6f4',
  border: '1px solid #45475a',
  padding: '4px 8px',
  borderRadius: 4,
  fontSize: 12,
  width: '100%',
};
