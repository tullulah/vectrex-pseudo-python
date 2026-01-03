import React, { useState, useEffect, useRef } from 'react';
import { useTranslation } from 'react-i18next';

interface VecVector {
  name: string;
  paths: {
    points: { x: number; y: number }[];
    intensity: number;
    closed: boolean;
  }[];
}

interface SceneObject {
  id: string;
  type: 'background' | 'enemy' | 'player' | 'projectile';
  vectorName: string;
  x: number;
  y: number;
  rotation: number;
  scale: number;
  velocity?: { x: number; y: number };
  physicsEnabled?: boolean;
}

export function PlaygroundPanel() {
  const { t } = useTranslation();
  const canvasRef = useRef<SVGSVGElement>(null);
  const [objects, setObjects] = useState<SceneObject[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [availableVectors, setAvailableVectors] = useState<string[]>([]);
  const [draggedVector, setDraggedVector] = useState<string | null>(null);

  // Vectrex coordinate system: -128 to 127 (256x256)
  const VECTREX_SIZE = 256;
  const VECTREX_MIN = -128;
  const VECTREX_MAX = 127;

  // Load available vectors from assets
  useEffect(() => {
    const loadVectors = async () => {
      const electronAPI = (window as any).electronAPI;
      if (!electronAPI) return;

      try {
        // List .vec files from assets/vectors directory
        const projectPath = await electronAPI.getProjectPath();
        if (!projectPath) return;

        const assetsPath = `${projectPath}/assets/vectors`;
        const files = await electronAPI.listDirectory(assetsPath);
        
        if (!files.error) {
          const vecFiles = files
            .filter((f: any) => f.name.endsWith('.vec'))
            .map((f: any) => f.name.replace('.vec', ''));
          setAvailableVectors(vecFiles);
        }
      } catch (error) {
        console.error('Failed to load vectors:', error);
      }
    };

    loadVectors();
  }, []);

  // Handle drag and drop to add objects
  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    if (!draggedVector || !canvasRef.current) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    // Convert screen coordinates to Vectrex coordinates
    const vecX = Math.round((x / rect.width) * VECTREX_SIZE + VECTREX_MIN);
    const vecY = Math.round(VECTREX_MAX - (y / rect.height) * VECTREX_SIZE);

    const newObject: SceneObject = {
      id: `obj_${Date.now()}`,
      type: 'enemy',
      vectorName: draggedVector,
      x: vecX,
      y: vecY,
      rotation: 0,
      scale: 1,
      velocity: { x: 0, y: 0 },
      physicsEnabled: false,
    };

    setObjects([...objects, newObject]);
    setDraggedVector(null);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  // Convert Vectrex coordinates to SVG viewport coordinates
  const vecToSvg = (x: number, y: number) => {
    return {
      x: x - VECTREX_MIN,
      y: VECTREX_MAX - y,
    };
  };

  return (
    <div style={{
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      width: '100%',
      backgroundColor: '#1e1e1e',
      color: '#d4d4d4',
    }}>
      {/* Toolbar */}
      <div style={{
        padding: '8px',
        borderBottom: '1px solid #3e3e3e',
        display: 'flex',
        gap: '8px',
        alignItems: 'center',
      }}>
        <span style={{ fontSize: '14px', fontWeight: 600 }}>
          🎮 Playground
        </span>
        <div style={{ flex: 1 }} />
        <button
          onClick={() => setObjects([])}
          style={{
            padding: '4px 12px',
            backgroundColor: '#333',
            border: '1px solid #555',
            borderRadius: '4px',
            color: '#d4d4d4',
            cursor: 'pointer',
          }}
        >
          Clear Scene
        </button>
      </div>

      <div style={{ display: 'flex', flex: 1, overflow: 'hidden' }}>
        {/* Asset Palette */}
        <div style={{
          width: '200px',
          borderRight: '1px solid #3e3e3e',
          padding: '12px',
          overflowY: 'auto',
        }}>
          <h3 style={{ fontSize: '12px', margin: '0 0 8px 0', color: '#888' }}>
            VECTORS
          </h3>
          {availableVectors.length === 0 && (
            <div style={{ fontSize: '11px', color: '#666', fontStyle: 'italic' }}>
              No .vec files found in assets/vectors/
            </div>
          )}
          {availableVectors.map(vec => (
            <div
              key={vec}
              draggable
              onDragStart={() => setDraggedVector(vec)}
              style={{
                padding: '8px',
                marginBottom: '4px',
                backgroundColor: '#2d2d2d',
                border: '1px solid #444',
                borderRadius: '4px',
                cursor: 'grab',
                fontSize: '11px',
                fontFamily: 'monospace',
              }}
            >
              📦 {vec}
            </div>
          ))}
        </div>

        {/* Canvas Area */}
        <div style={{
          flex: 1,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          padding: '20px',
          overflow: 'hidden',
        }}>
          <svg
            ref={canvasRef}
            onDrop={handleDrop}
            onDragOver={handleDragOver}
            viewBox={`0 0 ${VECTREX_SIZE} ${VECTREX_SIZE}`}
            style={{
              width: '100%',
              height: '100%',
              maxWidth: '800px',
              maxHeight: '800px',
              backgroundColor: '#000',
              border: '2px solid #00ff00',
              aspectRatio: '1 / 1',
            }}
          >
            {/* Grid */}
            <defs>
              <pattern id="grid" width="32" height="32" patternUnits="userSpaceOnUse">
                <path d="M 32 0 L 0 0 0 32" fill="none" stroke="#1a3319" strokeWidth="0.5" />
              </pattern>
            </defs>
            <rect width={VECTREX_SIZE} height={VECTREX_SIZE} fill="url(#grid)" />

            {/* Axes */}
            <line x1="0" y1={VECTREX_SIZE / 2} x2={VECTREX_SIZE} y2={VECTREX_SIZE / 2} stroke="#00ff0040" strokeWidth="1" />
            <line x1={VECTREX_SIZE / 2} y1="0" x2={VECTREX_SIZE / 2} y2={VECTREX_SIZE} stroke="#00ff0040" strokeWidth="1" />

            {/* Center marker */}
            <circle cx={VECTREX_SIZE / 2} cy={VECTREX_SIZE / 2} r="3" fill="#00ff00" opacity="0.5" />

            {/* Objects */}
            {objects.map(obj => {
              const svgPos = vecToSvg(obj.x, obj.y);
              return (
                <g
                  key={obj.id}
                  transform={`translate(${svgPos.x}, ${svgPos.y})`}
                  onClick={() => setSelectedId(obj.id)}
                  style={{ cursor: 'pointer' }}
                >
                  {/* Placeholder: render vector name as text for now */}
                  <text
                    x="0"
                    y="0"
                    fill={selectedId === obj.id ? '#00ff00' : '#00ff0080'}
                    fontSize="8"
                    textAnchor="middle"
                    style={{ userSelect: 'none' }}
                  >
                    {obj.vectorName}
                  </text>
                  <circle
                    cx="0"
                    cy="0"
                    r="5"
                    fill="none"
                    stroke={selectedId === obj.id ? '#00ff00' : '#00ff0080'}
                    strokeWidth="1"
                  />
                </g>
              );
            })}

            {/* Coordinates display */}
            <text x="5" y="15" fill="#00ff00" fontSize="8" fontFamily="monospace">
              {objects.length} objects
            </text>
          </svg>
        </div>

        {/* Properties Panel */}
        <div style={{
          width: '250px',
          borderLeft: '1px solid #3e3e3e',
          padding: '12px',
          overflowY: 'auto',
        }}>
          <h3 style={{ fontSize: '12px', margin: '0 0 8px 0', color: '#888' }}>
            PROPERTIES
          </h3>
          {selectedId ? (
            <div style={{ fontSize: '11px' }}>
              {(() => {
                const obj = objects.find(o => o.id === selectedId);
                if (!obj) return null;

                return (
                  <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    <div>
                      <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                        Vector
                      </label>
                      <div style={{ color: '#d4d4d4', fontFamily: 'monospace' }}>
                        {obj.vectorName}
                      </div>
                    </div>
                    <div>
                      <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                        Position
                      </label>
                      <input
                        type="number"
                        value={obj.x}
                        onChange={(e) => {
                          const newObjects = objects.map(o =>
                            o.id === selectedId ? { ...o, x: parseInt(e.target.value) } : o
                          );
                          setObjects(newObjects);
                        }}
                        style={{
                          width: '100%',
                          padding: '4px',
                          backgroundColor: '#2d2d2d',
                          border: '1px solid #444',
                          color: '#d4d4d4',
                          borderRadius: '2px',
                        }}
                      />
                      <input
                        type="number"
                        value={obj.y}
                        onChange={(e) => {
                          const newObjects = objects.map(o =>
                            o.id === selectedId ? { ...o, y: parseInt(e.target.value) } : o
                          );
                          setObjects(newObjects);
                        }}
                        style={{
                          width: '100%',
                          padding: '4px',
                          marginTop: '4px',
                          backgroundColor: '#2d2d2d',
                          border: '1px solid #444',
                          color: '#d4d4d4',
                          borderRadius: '2px',
                        }}
                      />
                    </div>
                    <button
                      onClick={() => {
                        setObjects(objects.filter(o => o.id !== selectedId));
                        setSelectedId(null);
                      }}
                      style={{
                        padding: '4px 8px',
                        backgroundColor: '#a03030',
                        border: '1px solid #c04040',
                        borderRadius: '4px',
                        color: '#fff',
                        cursor: 'pointer',
                        fontSize: '11px',
                      }}
                    >
                      Delete
                    </button>
                  </div>
                );
              })()}
            </div>
          ) : (
            <div style={{ fontSize: '11px', color: '#666', fontStyle: 'italic' }}>
              No object selected
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
