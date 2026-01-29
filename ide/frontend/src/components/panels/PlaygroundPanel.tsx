import React, { useState, useEffect, useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { useProjectStore } from '../../state/projectStore';
import { VPlayLevel, VPlayObject, VPlayValidator, DEFAULT_LEVEL, VPLAY_VERSION } from '../../types/vplay-schema';

interface VecPath {
  name: string;
  points: { x: number; y: number }[];
  intensity: number;
  closed: boolean;
}

interface VecVector {
  version: string;
  name: string;
  canvas: { width: number; height: number; origin: string };
  layers: {
    name: string;
    visible: boolean;
    paths: VecPath[];
  }[];
}

interface SceneObject {
  id: string;
  type: 'background' | 'enemy' | 'player' | 'projectile';
  vectorName?: string; // Optional - for static vectors
  animationName?: string; // Optional - for animated objects
  layer?: 'background' | 'gameplay' | 'foreground'; // NUEVO: Layer del nivel
  x: number;
  y: number;
  rotation: number;
  scale: number;
  velocity?: { x: number; y: number };
  physicsEnabled?: boolean;
  collidable?: boolean;
  gravity?: number;
  bounceDamping?: number;
  physicsType?: 'gravity' | 'bounce' | 'projectile' | 'static';
  radius?: number; // Calculated from vector bounds
}

export function PlaygroundPanel() {
  const { t } = useTranslation();
  const { vpyProject } = useProjectStore();
  const canvasRef = useRef<SVGSVGElement>(null);
  const [objects, setObjects] = useState<SceneObject[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [availableVectors, setAvailableVectors] = useState<string[]>([]);
  const [availableAnimations, setAvailableAnimations] = useState<string[]>([]);
  const [loadedVectors, setLoadedVectors] = useState<Map<string, VecVector>>(new Map());
  const [draggedVector, setDraggedVector] = useState<string | null>(null);
  const [draggedAnimation, setDraggedAnimation] = useState<string | null>(null);
  const [draggingObjectId, setDraggingObjectId] = useState<string | null>(null);
  const [dragOffset, setDragOffset] = useState<{ x: number; y: number } | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [savedScene, setSavedScene] = useState<SceneObject[] | null>(null);
  const [editingVelocity, setEditingVelocity] = useState(false);
  const animationFrameRef = useRef<number | null>(null);
  const [showSaveLoadModal, setShowSaveLoadModal] = useState(false);
  const [modalMode, setModalMode] = useState<'save' | 'load'>('save');
  const [sceneName, setSceneName] = useState('');
  const [availableScenes, setAvailableScenes] = useState<string[]>([]);
  const [draggingVelocity, setDraggingVelocity] = useState(false);
  const [velocityMagnitude, setVelocityMagnitude] = useState(10);
  const [velocityAngle, setVelocityAngle] = useState(-90); // -90 = up
  const [toasts, setToasts] = useState<Array<{id: number; message: string; type: 'success' | 'error'}>>([]);

  // Toast helper
  const showToast = (message: string, type: 'success' | 'error' = 'success') => {
    const id = Date.now();
    setToasts(prev => [...prev, { id, message, type }]);
    setTimeout(() => {
      setToasts(prev => prev.filter(t => t.id !== id));
    }, 3000);
  };

  // Vectrex coordinate system: 3:4 aspect ratio (192x256)
  const VECTREX_WIDTH = 192;
  const VECTREX_HEIGHT = 256;
  const VECTREX_X_MIN = -96;
  const VECTREX_X_MAX = 95;
  const VECTREX_Y_MIN = -128;
  const VECTREX_Y_MAX = 127;

  // Load available vectors from project config
  useEffect(() => {
    const loadVectors = async () => {
      if (!vpyProject) {
        console.log('[Playground] No project loaded');
        return;
      }

      try {
        const filesAPI = (window as any).files;
        if (!filesAPI) {
          console.error('[Playground] No files API available');
          return;
        }

        // Get vectors from project config
        const vectorResources = vpyProject.config.resources?.vectors;
        console.log('[Playground] Vector resources:', vectorResources);
        
        if (!vectorResources || vectorResources.length === 0) {
          console.log('[Playground] No vectors in project');
          return;
        }

        const projectPath = vpyProject.rootDir;

        // Expand glob patterns
        const vecFiles: string[] = [];
        const vecPaths = new Map<string, string>();
        
        for (const pattern of vectorResources) {
          if (pattern.includes('*')) {
            const dirPath = pattern.substring(0, pattern.lastIndexOf('/'));
            const fullDirPath = `${projectPath}/${dirPath}`;
            
            const result = await filesAPI.readDirectory(fullDirPath);
            if (!result.error && result.files) {
              const matchedFiles = result.files
                .filter((f: any) => !f.isDir && f.name.endsWith('.vec'))
                .map((f: any) => {
                  const name = f.name.replace('.vec', '');
                  vecPaths.set(name, `${fullDirPath}/${f.name}`);
                  return name;
                });
              vecFiles.push(...matchedFiles);
            }
          } else {
            const name = pattern.split('/').pop()?.replace('.vec', '') || '';
            vecFiles.push(name);
            vecPaths.set(name, `${projectPath}/${pattern}`);
          }
        }
        
        console.log('[Playground] Found vectors:', vecFiles.length);
        setAvailableVectors(vecFiles);

        // Load vector files using file:read IPC
        const vectors = new Map<string, VecVector>();
        for (const vecName of vecFiles) {
          try {
            const vecPath = vecPaths.get(vecName);
            if (!vecPath) continue;
            
            const result = await filesAPI.readFile(vecPath);
            if (!result.error && result.content) {
              const vecData: VecVector = JSON.parse(result.content);
              vectors.set(vecName, vecData);
            } else {
              console.error(`[Playground] Error reading ${vecName}:`, result.error);
            }
          } catch (err) {
            console.error(`[Playground] Failed to parse ${vecName}:`, err);
          }
        }
        
        setLoadedVectors(vectors);
        console.log('[Playground] Loaded', vectors.size, 'vectors');
      } catch (error) {
        console.error('[Playground] Error:', error);
      }
    };

    loadVectors();
  }, [vpyProject]);

  // Load available animations from project
  useEffect(() => {
    const loadAnimations = async () => {
      if (!vpyProject) {
        console.log('[Playground] No project loaded for animations');
        return;
      }

      try {
        const filesAPI = (window as any).files;
        if (!filesAPI) {
          console.error('[Playground] No files API available');
          return;
        }

        const projectPath = vpyProject.rootDir;
        const animPath = `${projectPath}/assets/animations`;

        const result = await filesAPI.readDirectory(animPath);
        if (!result.error && result.files) {
          const animFiles = result.files
            .filter((f: any) => !f.isDir && f.name.endsWith('.vanim'))
            .map((f: any) => f.name.replace('.vanim', ''));
          
          console.log('[Playground] Found animations:', animFiles.length);
          setAvailableAnimations(animFiles);
        }
      } catch (error) {
        console.log('[Playground] No animations directory');
      }
    };

    loadAnimations();
  }, [vpyProject]);

  // Load available .vplay scenes
  useEffect(() => {
    const loadScenes = async () => {
      if (!vpyProject?.rootDir) return;
      
      try {
        const projectRoot = vpyProject.rootDir;
        const levelsPath = `${projectRoot}/assets/levels`;
        
        const result = await (window as any).files.readDirectory(levelsPath);
        const scenes = result.files
          .filter((f: any) => f.name.endsWith('.vplay'))
          .map((f: any) => f.name.replace('.vplay', ''));
        setAvailableScenes(scenes);
        console.log('[Playground] Found', scenes.length, 'scenes');
      } catch (error) {
        console.log('[Playground] No scenes directory yet');
      }
    };

    loadScenes();
  }, [vpyProject]);

  // Listen for scene load requests from FileTree
  useEffect(() => {
    const handleLoadSceneRequest = (event: CustomEvent) => {
      const sceneName = event.detail.sceneName;
      console.log('[Playground] Load scene request:', sceneName);
      handleLoadScene(sceneName);
    };

    window.addEventListener('playground:loadScene' as any, handleLoadSceneRequest);
    return () => {
      window.removeEventListener('playground:loadScene' as any, handleLoadSceneRequest);
    };
  }, [vpyProject]); // handleLoadScene uses vpyProject, so include it in deps

  // Physics simulation loop
  useEffect(() => {
    if (!isPlaying) {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
        animationFrameRef.current = null;
      }
      return;
    }

    const updatePhysics = () => {
      setObjects(prevObjects => {
        const newObjects = prevObjects.map(obj => {
          if (!obj.physicsEnabled) return obj;

          const physicsType = obj.physicsType || 'gravity';
          const objGravity = obj.gravity ?? 1;
          const objBounce = obj.bounceDamping ?? 0.85;
          const objRadius = (obj.radius ?? 10) * obj.scale;

          let newVelY = obj.velocity?.y || 0;
          let newVelX = obj.velocity?.x || 0;
          
          // Apply physics based on type
          if (physicsType === 'gravity') {
            // Standard gravity physics
            newVelY = newVelY - objGravity;
          } else if (physicsType === 'bounce') {
            // Perpetual bounce - no gravity, constant speed
            // Velocity stays the same until collision
          } else if (physicsType === 'projectile') {
            // Projectile with gravity but no bouncing
            newVelY = newVelY - objGravity;
          }
          // 'static' type doesn't move
          
          // Limit max velocity (Pang-style slow movement)
          const MAX_VEL = 15;
          newVelX = Math.max(-MAX_VEL, Math.min(MAX_VEL, newVelX));
          newVelY = Math.max(-MAX_VEL, Math.min(MAX_VEL, newVelY));
          
          let newX = obj.x + newVelX;
          let newY = obj.y + newVelY;
          let velX = newVelX;
          let velY = newVelY;

          // Boundary collisions based on physics type (with radius)
          if (physicsType !== 'static') {
            // Horizontal walls (left/right)
            if (newX - objRadius <= VECTREX_X_MIN) {
              newX = VECTREX_X_MIN + objRadius;
              if (physicsType === 'bounce') {
                velX = -velX; // Perfect bounce
              } else if (physicsType === 'gravity') {
                velX = -velX * objBounce; // Energy loss
              } else if (physicsType === 'projectile') {
                velX = 0; // Stop
              }
            } else if (newX + objRadius >= VECTREX_X_MAX) {
              newX = VECTREX_X_MAX - objRadius;
              if (physicsType === 'bounce') {
                velX = -velX;
              } else if (physicsType === 'gravity') {
                velX = -velX * objBounce;
              } else if (physicsType === 'projectile') {
                velX = 0;
              }
            }

            // Vertical walls (floor/ceiling)
            if (newY - objRadius <= VECTREX_Y_MIN) {
              newY = VECTREX_Y_MIN + objRadius + 0.5; // Keep object fully visible + safety margin
              if (physicsType === 'bounce') {
                velY = -velY; // Perfect bounce - maintains speed
              } else if (physicsType === 'gravity') {
                velY = -velY * objBounce; // Energy loss
                if (Math.abs(velY) < 2) velY = 0;
              } else if (physicsType === 'projectile') {
                velY = 0; // Stop on ground
              }
            } else if (newY + objRadius >= VECTREX_Y_MAX) {
              newY = VECTREX_Y_MAX - objRadius - 0.5; // Keep object fully visible + safety margin
              if (physicsType === 'bounce') {
                velY = -velY; // Perfect bounce from ceiling
              } else if (physicsType === 'gravity') {
                velY = -velY * objBounce;
              } else if (physicsType === 'projectile') {
                velY = 0;
              }
            }
          }

          return {
            ...obj,
            x: newX,
            y: newY,
            velocity: { x: velX, y: velY },
          };
        });

        // Object-to-object collisions
        const finalObjects = newObjects.map((obj, i) => {
          if (!obj.physicsEnabled) return obj;
          // Skip if current object is not collidable
          if (!obj.collidable) return obj;

          for (let j = 0; j < newObjects.length; j++) {
            if (i === j) continue;
            
            const other = newObjects[j];
            // Skip if other object is not collidable
            if (!other.collidable) continue;

            const dx = obj.x - other.x;
            const dy = obj.y - other.y;
            const distance = Math.sqrt(dx * dx + dy * dy);
            
            // Use actual radius from vector bounds
            const objRadius = (obj.radius ?? 10) * obj.scale;
            const otherRadius = (other.radius ?? 10) * other.scale;
            const minDist = objRadius + otherRadius;

            if (distance < minDist && distance > 0) {
              // Collision detected - bounce away
              const angle = Math.atan2(dy, dx);
              const targetX = other.x + Math.cos(angle) * minDist;
              const targetY = other.y + Math.sin(angle) * minDist;
              
              const velMag = Math.sqrt(
                (obj.velocity?.x || 0) ** 2 + (obj.velocity?.y || 0) ** 2
              );
              const objBounce = obj.bounceDamping ?? 0.85;
              
              return {
                ...obj,
                x: targetX,
                y: targetY,
                velocity: {
                  x: Math.cos(angle) * velMag * objBounce,
                  y: Math.sin(angle) * velMag * objBounce,
                },
              };
            }
          }
          return obj;
        });

        return finalObjects;
      });

      animationFrameRef.current = requestAnimationFrame(updatePhysics);
    };

    animationFrameRef.current = requestAnimationFrame(updatePhysics);

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [isPlaying]);

  // Save/Load scene functions
  const handleSaveScene = async () => {
    if (!sceneName.trim()) {
      showToast('Please enter a scene name', 'error');
      return;
    }

    try {
      const projectRoot = vpyProject?.rootDir;
      if (!projectRoot) {
        showToast('No project loaded', 'error');
        return;
      }

      const sceneData: VPlayLevel = {
        version: VPLAY_VERSION,
        type: 'level',
        metadata: {
          name: sceneName,
          author: '',
          difficulty: 'medium',
          timeLimit: 0,
          targetScore: 0,
          description: `Created in Playground - ${new Date().toISOString().split('T')[0]}`
        },
        worldBounds: {
          xMin: -96,
          xMax: 95,
          yMin: -128,
          yMax: 127
        },
        layers: {
          background: objects.filter(obj => obj.layer === 'background').map(obj => ({
            ...obj,
            x: Math.round(obj.x),
            y: Math.round(obj.y),
            velocity: obj.velocity ? { 
              x: Math.round(obj.velocity.x), 
              y: Math.round(obj.velocity.y) 
            } : { x: 0, y: 0 },
            layer: 'background' as const
          })) as VPlayObject[],
          gameplay: objects.filter(obj => !obj.layer || obj.layer === 'gameplay').map(obj => ({
            ...obj,
            x: Math.round(obj.x),
            y: Math.round(obj.y),
            velocity: obj.velocity ? { 
              x: Math.round(obj.velocity.x), 
              y: Math.round(obj.velocity.y) 
            } : { x: 0, y: 0 },
            layer: 'gameplay' as const
          })) as VPlayObject[],
          foreground: objects.filter(obj => obj.layer === 'foreground').map(obj => ({
            ...obj,
            x: Math.round(obj.x),
            y: Math.round(obj.y),
            velocity: obj.velocity ? { 
              x: Math.round(obj.velocity.x), 
              y: Math.round(obj.velocity.y) 
            } : { x: 0, y: 0 },
            layer: 'foreground' as const
          })) as VPlayObject[]
        },
        spawnPoints: {
          player: { x: 0, y: -100 }
        }
      };

      // Validate before saving
      const validation = VPlayValidator.validate(sceneData);
      if (!validation.valid) {
        console.error('[Playground] Validation errors:', validation.errors);
        showToast(`Validation failed: ${validation.errors[0]}`, 'error');
        return;
      }

      const filePath = `${projectRoot}/assets/levels/${sceneName}.vplay`;
      await (window as any).files.saveFile({
        path: filePath,
        content: JSON.stringify(sceneData, null, 2),
      });
      
      console.log('[Playground] Saved scene:', filePath);
      showToast(`Scene "${sceneName}" saved!`, 'success');
      setShowSaveLoadModal(false);
      // Keep sceneName so we can quick-save next time
      
      // Refresh scene list
      const result = await (window as any).files.readDirectory(`${projectRoot}/assets/playground`);
      const scenes = result.files
        .filter((f: any) => f.name.endsWith('.vplay'))
        .map((f: any) => f.name.replace('.vplay', ''));
      setAvailableScenes(scenes);
    } catch (error) {
      console.error('[Playground] Save error:', error);
      showToast('Failed to save scene', 'error');
    }
  };

  const handleLoadScene = async (name: string) => {
    try {
      const projectRoot = vpyProject?.rootDir;
      if (!projectRoot) return;

      const filePath = `${projectRoot}/assets/levels/${name}.vplay`;
      const result = await (window as any).files.readFile(filePath);
      let sceneData = JSON.parse(result.content);
      
      // Auto-migrate v1.0 to v2.0
      if (sceneData.version === '1.0') {
        console.log('[Playground] Migrating v1.0 level to v2.0');
        sceneData = VPlayValidator.migrateV1toV2(sceneData);
        showToast(`Migrated "${name}" from v1.0 to v2.0`, 'success');
      }
      
      // Validate loaded data
      const validation = VPlayValidator.validate(sceneData);
      if (!validation.valid) {
        console.error('[Playground] Validation errors:', validation.errors);
        showToast(`Invalid level: ${validation.errors[0]}`, 'error');
        return;
      }
      
      // Extract objects from layers or legacy objects array
      const loadedObjects: SceneObject[] = [];
      if (sceneData.layers) {
        loadedObjects.push(...(sceneData.layers.background || []));
        loadedObjects.push(...(sceneData.layers.gameplay || []));
        loadedObjects.push(...(sceneData.layers.foreground || []));
      } else if (sceneData.objects) {
        loadedObjects.push(...sceneData.objects);
      }
      
      setObjects(loadedObjects);
      setSelectedId(null);
      setSceneName(name); // Remember the scene name for future saves
      console.log('[Playground] Loaded scene:', name, `(${loadedObjects.length} objects)`);
      showToast(`Scene "${name}" loaded!`, 'success');
      setShowSaveLoadModal(false);
    } catch (error) {
      console.error('[Playground] Load error:', error);
      showToast('Failed to load scene', 'error');
    }
  };

  const handleDeleteScene = async (name: string) => {
    if (!confirm(`Delete scene "${name}"?`)) return;

    try {
      const projectRoot = vpyProject?.rootDir;
      if (!projectRoot) return;

      const filePath = `${projectRoot}/assets/levels/${name}.vplay`;
      await (window as any).files.deleteFile(filePath);
      
      console.log('[Playground] Deleted scene:', name);
      
      // Refresh scene list
      const result = await (window as any).files.readDirectory(`${projectRoot}/assets/playground`);
      const scenes = result.files
        .filter((f: any) => f.name.endsWith('.vplay'))
        .map((f: any) => f.name.replace('.vplay', ''));
      setAvailableScenes(scenes);
    } catch (error) {
      console.error('[Playground] Delete error:', error);
      showToast('Failed to delete scene', 'error');
    }
  };

  // Handle drag and drop to add objects
  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    if ((!draggedVector && !draggedAnimation) || !canvasRef.current) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    // Convert screen coordinates to Vectrex coordinates
    const vecX = Math.round((x / rect.width) * VECTREX_WIDTH + VECTREX_X_MIN);
    const vecY = Math.round(VECTREX_Y_MAX - (y / rect.height) * VECTREX_HEIGHT);

    const newObject: SceneObject = {
      id: `obj_${Date.now()}`,
      type: 'enemy',
      vectorName: draggedVector || undefined,
      animationName: draggedAnimation || undefined,
      layer: 'gameplay', // Por defecto gameplay
      x: vecX,
      y: vecY,
      rotation: 0,
      scale: 1,
      velocity: { x: 0, y: 0 },
      physicsEnabled: false,
      collidable: true,
      gravity: 1,
      bounceDamping: 0.85,
      physicsType: 'gravity',
    };

    setObjects([...objects, newObject]);
    setDraggedVector(null);
    setDraggedAnimation(null);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  // Object dragging handlers
  const handleObjectMouseDown = (e: React.MouseEvent, objId: string) => {
    e.stopPropagation();
    if (!canvasRef.current) return;

    const obj = objects.find(o => o.id === objId);
    if (!obj) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    // Convert to Vectrex coordinates
    const vecX = Math.round((mouseX / rect.width) * VECTREX_WIDTH + VECTREX_X_MIN);
    const vecY = Math.round(VECTREX_Y_MAX - (mouseY / rect.height) * VECTREX_HEIGHT);

    setDraggingObjectId(objId);
    setDragOffset({ x: vecX - obj.x, y: vecY - obj.y });
    setSelectedId(objId);
  };

  const handleVelocityArrowMouseDown = (e: React.MouseEvent) => {
    if (editingVelocity && selectedId) {
      e.stopPropagation();
      setDraggingVelocity(true);
    }
  };

  const handleVelocityArrowDrag = (e: React.MouseEvent) => {
    if (!draggingVelocity || !selectedId || !canvasRef.current) return;

    const selectedObj = objects.find(o => o.id === selectedId);
    if (!selectedObj) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    // Convert to Vectrex coordinates
    const vecX = ((mouseX / rect.width) * (VECTREX_X_MAX - VECTREX_X_MIN)) + VECTREX_X_MIN;
    const vecY = VECTREX_Y_MAX - ((mouseY / rect.height) * (VECTREX_Y_MAX - VECTREX_Y_MIN));

    // Calculate velocity from mouse position relative to object
    const dx = vecX - selectedObj.x;
    const dy = vecY - selectedObj.y;
    
    // Scale down (arrow is 3x size) and clamp to max velocity
    const velX = Math.max(-15, Math.min(15, dx / 3));
    const velY = Math.max(-15, Math.min(15, dy / 3));
    
    setObjects(objects.map(o =>
      o.id === selectedId ? { ...o, velocity: { x: velX, y: velY } } : o
    ));
  };

  const handleCanvasMouseMove = (e: React.MouseEvent) => {
    if (draggingVelocity) {
      handleVelocityArrowDrag(e);
      return;
    }
    
    if (!draggingObjectId || !dragOffset || !canvasRef.current) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    // Convert to Vectrex coordinates
    const vecX = Math.round((mouseX / rect.width) * VECTREX_WIDTH + VECTREX_X_MIN);
    const vecY = Math.round(VECTREX_Y_MAX - (mouseY / rect.height) * VECTREX_HEIGHT);

    // Update object position
    setObjects(objects.map(obj => 
      obj.id === draggingObjectId 
        ? { ...obj, x: vecX - dragOffset.x, y: vecY - dragOffset.y }
        : obj
    ));
  };

  const handleCanvasMouseUp = () => {
    setDraggingObjectId(null);
    setDragOffset(null);
    setDraggingVelocity(false);
  };

  // Convert Vectrex coordinates to SVG viewport coordinates
  const vecToSvg = (x: number, y: number) => {
    return {
      x: x - VECTREX_X_MIN,
      y: VECTREX_Y_MAX - y,
    };
  };

  // Render a vector sprite from loaded .vec data
  const renderVector = (obj: SceneObject) => {
    // Solo renderizar si es un vector (no una animación)
    if (!obj.vectorName) return null;
    
    const vecData = loadedVectors.get(obj.vectorName);
    if (!vecData) return null;

    const svgPos = vecToSvg(obj.x, obj.y);
    const isSelected = selectedId === obj.id;

    return (
      <g
        key={obj.id}
        transform={`translate(${svgPos.x}, ${svgPos.y}) scale(${obj.scale}) rotate(${obj.rotation})`}
        onMouseDown={(e) => handleObjectMouseDown(e, obj.id)}
        style={{ cursor: draggingObjectId === obj.id ? 'grabbing' : 'grab' }}
      >
        {vecData.layers.map((layer, layerIdx) =>
          layer.paths.map((path, pathIdx) => (
            <polyline
              key={`${layerIdx}-${pathIdx}`}
              points={path.points.map(p => `${p.x},${-p.y}`).join(' ')}
              fill="none"
              stroke={isSelected ? '#ffff00' : '#00ff00'}
              strokeWidth="1.5"
              strokeOpacity={path.intensity / 255}
            />
          ))
        )}
        {isSelected && (
          <circle
            cx="0"
            cy="0"
            r="5"
            fill="none"
            stroke="#ffff00"
            strokeWidth="2"
          />
        )}
      </g>
    );
  };

  // Render animation (as placeholder icon for now)
  const renderAnimation = (obj: SceneObject) => {
    if (!obj.animationName) return null;

    const svgPos = vecToSvg(obj.x, obj.y);
    const isSelected = selectedId === obj.id;

    return (
      <g
        key={obj.id}
        transform={`translate(${svgPos.x}, ${svgPos.y})`}
        onMouseDown={(e) => handleObjectMouseDown(e, obj.id)}
        style={{ cursor: draggingObjectId === obj.id ? 'grabbing' : 'grab' }}
      >
        {/* Film reel icon */}
        <circle cx="0" cy="0" r="8" fill="none" stroke={isSelected ? '#ffff00' : '#00ffff'} strokeWidth="2" />
        <line x1="-4" y1="0" x2="4" y2="0" stroke={isSelected ? '#ffff00' : '#00ffff'} strokeWidth="1.5" />
        <line x1="0" y1="-4" x2="0" y2="4" stroke={isSelected ? '#ffff00' : '#00ffff'} strokeWidth="1.5" />
        {/* Animation name label */}
        <text
          x="12"
          y="4"
          fill={isSelected ? '#ffff00' : '#00ffff'}
          fontSize="6"
          fontFamily="monospace"
        >
          {obj.animationName}
        </text>
      </g>
    );
  };

  // Render object (vector or animation)
  const renderObject = (obj: SceneObject) => {
    if (obj.animationName) {
      return renderAnimation(obj);
    } else if (obj.vectorName) {
      return renderVector(obj);
    }
    return null;
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
        {!isPlaying ? (
          <button
            onClick={() => {
              setSavedScene(JSON.parse(JSON.stringify(objects)));
              setIsPlaying(true);
            }}
            style={{
              padding: '4px 12px',
              backgroundColor: '#40a040',
              border: '1px solid #50b050',
              borderRadius: '4px',
              color: '#fff',
              cursor: 'pointer',
              fontWeight: 600,
            }}
          >
            ▶ Play
          </button>
        ) : (
          <button
            onClick={() => {
              setIsPlaying(false);
              if (savedScene) {
                setObjects(savedScene);
                setSavedScene(null);
              }
            }}
            style={{
              padding: '4px 12px',
              backgroundColor: '#c04040',
              border: '1px solid #d05050',
              borderRadius: '4px',
              color: '#fff',
              cursor: 'pointer',
              fontWeight: 600,
            }}
          >
            ⏹ Stop
          </button>
        )}
        <div style={{ borderLeft: '1px solid #555', height: '24px', margin: '0 4px' }} />
        <button
          onClick={() => setEditingVelocity(!editingVelocity)}
          style={{
            padding: '4px 12px',
            backgroundColor: editingVelocity ? '#6060a0' : '#333',
            border: '1px solid ' + (editingVelocity ? '#7070b0' : '#555'),
            borderRadius: '4px',
            color: '#fff',
            cursor: 'pointer',
            fontSize: '11px',
          }}
        >
          {editingVelocity ? '🎯 Editing Velocity' : '➡ Set Velocity'}
        </button>
        <div style={{ flex: 1 }} />
        <button
          onClick={() => {
            // Quick save: if we already have a name, save directly
            if (sceneName.trim()) {
              handleSaveScene();
            } else {
              // No name yet, open modal to ask for it
              setModalMode('save');
              setShowSaveLoadModal(true);
            }
          }}
          style={{
            padding: '4px 12px',
            backgroundColor: '#333',
            border: '1px solid #555',
            borderRadius: '4px',
            color: '#d4d4d4',
            cursor: 'pointer',
            fontSize: '11px',
          }}
        >
          💾 Save
        </button>
        <button
          onClick={() => {
            setModalMode('load');
            setShowSaveLoadModal(true);
          }}
          style={{
            padding: '4px 12px',
            backgroundColor: '#333',
            border: '1px solid #555',
            borderRadius: '4px',
            color: '#d4d4d4',
            cursor: 'pointer',
            fontSize: '11px',
          }}
        >
          📁 Load
        </button>
        <button
          onClick={() => {
            setObjects([]);
            setSelectedId(null);
          }}
          style={{
            padding: '4px 12px',
            backgroundColor: '#333',
            border: '1px solid #555',
            borderRadius: '4px',
            color: '#d4d4d4',
            cursor: 'pointer',
            fontSize: '11px',
          }}
        >
          🗑️ Clear
        </button>
      </div>

      <div style={{ display: 'flex', flex: 1, overflow: 'hidden', minHeight: 0 }}>
        {/* Asset Palette */}
        <div style={{
          width: '200px',
          borderRight: '1px solid #3e3e3e',
          display: 'flex',
          flexDirection: 'column',
          minHeight: 0,
        }}>
          <h3 style={{ fontSize: '12px', margin: '12px 12px 8px 12px', color: '#888', flexShrink: 0 }}>
            VECTORS
          </h3>
          <div style={{
            flex: 1,
            overflowY: 'auto',
            overflowX: 'hidden',
            padding: '0 12px 12px 12px',
            minHeight: 0,
          }}>
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
          
          <h3 style={{ fontSize: '12px', margin: '12px 12px 8px 12px', color: '#888', flexShrink: 0 }}>
            ANIMATIONS
          </h3>
          <div style={{
            flex: 1,
            overflowY: 'auto',
            overflowX: 'hidden',
            padding: '0 12px 12px 12px',
            minHeight: 0,
          }}>
            {availableAnimations.length === 0 && (
              <div style={{ fontSize: '11px', color: '#666', fontStyle: 'italic' }}>
                No .vanim files found in assets/animations/
              </div>
            )}
            {availableAnimations.map(anim => (
              <div
                key={anim}
                draggable
                onDragStart={() => setDraggedAnimation(anim)}
                style={{
                  padding: '8px',
                  marginBottom: '4px',
                  backgroundColor: '#2d3a2d',
                  border: '1px solid #4a6a4a',
                  borderRadius: '4px',
                  cursor: 'grab',
                  fontSize: '11px',
                  fontFamily: 'monospace',
                }}
              >
                🎬 {anim}
              </div>
            ))}
          </div>
        </div>

        {/* Canvas Area */}
        <div style={{
          flex: 1,
          display: 'flex',
          alignItems: 'flex-start',
          justifyContent: 'center',
          padding: '20px',
          overflow: 'hidden',
          minWidth: 0,
          minHeight: 0,
        }}>
          <div style={{
            width: '100%',
            maxWidth: '600px',
            display: 'flex',
            alignItems: 'flex-start',
            justifyContent: 'center',
          }}>
            <svg
              ref={canvasRef}
              onDrop={handleDrop}
              onDragOver={handleDragOver}
              onMouseMove={handleCanvasMouseMove}
              onMouseUp={handleCanvasMouseUp}
              onMouseLeave={handleCanvasMouseUp}
              viewBox={`0 0 ${VECTREX_WIDTH} ${VECTREX_HEIGHT}`}
              preserveAspectRatio="xMidYMid meet"
              style={{
                width: '100%',
                height: 'auto',
                aspectRatio: '3 / 4',
                backgroundColor: '#000',
                border: '2px solid #00ff00',
              }}
            >
            {/* Grid */}
            <defs>
              <pattern id="grid" width="32" height="32" patternUnits="userSpaceOnUse">
                <path d="M 32 0 L 0 0 0 32" fill="none" stroke="#1a3319" strokeWidth="0.5" />
              </pattern>
              <marker id="arrowhead" markerWidth="10" markerHeight="10" refX="8" refY="3" orient="auto">
                <polygon points="0 0, 10 3, 0 6" fill="#ff8000" />
              </marker>
            </defs>
            <rect width={VECTREX_WIDTH} height={VECTREX_HEIGHT} fill="url(#grid)" />

            {/* Axes */}
            <line x1="0" y1={VECTREX_HEIGHT / 2} x2={VECTREX_WIDTH} y2={VECTREX_HEIGHT / 2} stroke="#00ff0040" strokeWidth="1" />
            <line x1={VECTREX_WIDTH / 2} y1="0" x2={VECTREX_WIDTH / 2} y2={VECTREX_HEIGHT} stroke="#00ff0040" strokeWidth="1" />

            {/* Center marker */}
            <circle cx={VECTREX_WIDTH / 2} cy={VECTREX_HEIGHT / 2} r="3" fill="#00ff00" opacity="0.5" />

            {/* Objects - render actual vectors and animations */}
            {objects.map(obj => renderObject(obj))}

            {/* Coordinates display */}
            <text x="5" y="15" fill="#00ff00" fontSize="8" fontFamily="monospace">
              {objects.length} objects | {loadedVectors.size} vectors loaded
            </text>
          </svg>
          </div>
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
                    {obj.vectorName && (
                      <div>
                        <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                          Vector
                        </label>
                        <div style={{ color: '#d4d4d4', fontFamily: 'monospace' }}>
                          📦 {obj.vectorName}
                        </div>
                      </div>
                    )}
                    {obj.animationName && (
                      <div>
                        <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                          Animation
                        </label>
                        <div style={{ color: '#6d6', fontFamily: 'monospace' }}>
                          🎬 {obj.animationName}
                        </div>
                      </div>
                    )}
                    <div>
                      <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                        Layer
                      </label>
                      <select
                        value={obj.layer || 'gameplay'}
                        onChange={(e) => {
                          const newObjects = objects.map(o =>
                            o.id === selectedId ? { ...o, layer: e.target.value as any } : o
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
                      >
                        <option value="background">🏔️ Background (fondo, detrás)</option>
                        <option value="gameplay">⚡ Gameplay (jugable, medio)</option>
                        <option value="foreground">🌟 Foreground (frente, adelante)</option>
                      </select>
                      <div style={{ fontSize: '10px', color: '#666', marginTop: '4px' }}>
                        {obj.layer === 'background' && '🏔️ Dibuja primero - fondo del nivel'}
                        {obj.layer === 'foreground' && '🌟 Dibuja último - frente del nivel'}
                        {(!obj.layer || obj.layer === 'gameplay') && '⚡ Capa principal - objetos jugables'}
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
                    <div>
                      <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                        Scale
                      </label>
                      <input
                        type="number"
                        step="0.1"
                        min="0.1"
                        max="5"
                        value={obj.scale}
                        onChange={(e) => {
                          const newObjects = objects.map(o =>
                            o.id === selectedId ? { ...o, scale: parseFloat(e.target.value) } : o
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
                    </div>
                    <div>
                      <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                        Rotation (degrees)
                      </label>
                      <input
                        type="number"
                        step="15"
                        min="0"
                        max="360"
                        value={obj.rotation}
                        onChange={(e) => {
                          const newObjects = objects.map(o =>
                            o.id === selectedId ? { ...o, rotation: parseInt(e.target.value) } : o
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
                    </div>
                    <div>
                      <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                        Initial Velocity
                      </label>
                      <div style={{ display: 'flex', gap: '4px', marginBottom: '4px' }}>
                        <div style={{ flex: 1 }}>
                          <label style={{ fontSize: '10px', color: '#666' }}>X</label>
                          <input
                            type="number"
                            step="0.5"
                            value={(obj.velocity?.x || 0).toFixed(1)}
                            onChange={(e) => {
                              const x = parseFloat(e.target.value) || 0;
                              setObjects(objects.map(o =>
                                o.id === selectedId ? { ...o, velocity: { ...o.velocity!, x, y: o.velocity?.y || 0 } } : o
                              ));
                            }}
                            style={{
                              width: '100%',
                              padding: '4px',
                              backgroundColor: '#2d2d2d',
                              border: '1px solid #444',
                              color: '#d4d4d4',
                              borderRadius: '2px',
                              fontSize: '11px',
                            }}
                          />
                        </div>
                        <div style={{ flex: 1 }}>
                          <label style={{ fontSize: '10px', color: '#666' }}>Y</label>
                          <input
                            type="number"
                            step="0.5"
                            value={(obj.velocity?.y || 0).toFixed(1)}
                            onChange={(e) => {
                              const y = parseFloat(e.target.value) || 0;
                              setObjects(objects.map(o =>
                                o.id === selectedId ? { ...o, velocity: { x: o.velocity?.x || 0, y } } : o
                              ));
                            }}
                            style={{
                              width: '100%',
                              padding: '4px',
                              backgroundColor: '#2d2d2d',
                              border: '1px solid #444',
                              color: '#d4d4d4',
                              borderRadius: '2px',
                              fontSize: '11px',
                            }}
                          />
                        </div>
                      </div>
                      <div style={{ fontSize: '10px', color: '#888', marginTop: '4px' }}>
                        {(() => {
                          const vx = obj.velocity?.x || 0;
                          const vy = obj.velocity?.y || 0;
                          const mag = Math.sqrt(vx * vx + vy * vy);
                          const angle = Math.atan2(vy, vx) * (180 / Math.PI);
                          return `📐 ${angle.toFixed(0)}° | ⚡ ${mag.toFixed(1)} px/frame`;
                        })()}
                      </div>
                      <div style={{ fontSize: '10px', color: '#666', marginTop: '4px' }}>
                        💡 Activa "Set Velocity" y arrastra la flecha naranja
                      </div>
                    </div>
                    {obj.physicsEnabled && (obj.physicsType === 'gravity' || !obj.physicsType) && (
                      <div>
                        <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                          Gravity
                        </label>
                        <input
                          type="range"
                          min="0"
                          max="3"
                          step="0.1"
                          value={obj.gravity ?? 1}
                          onChange={(e) => {
                            const newObjects = objects.map(o =>
                              o.id === selectedId ? { ...o, gravity: parseFloat(e.target.value) } : o
                            );
                            setObjects(newObjects);
                          }}
                          style={{ width: '100%' }}
                        />
                        <span style={{ fontSize: '10px', color: '#d4d4d4' }}>{(obj.gravity ?? 1).toFixed(1)}</span>
                      </div>
                    )}
                    {obj.physicsEnabled && (obj.physicsType === 'gravity' || !obj.physicsType) && (
                      <div>
                        <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                          Bounce Damping
                        </label>
                        <input
                          type="range"
                          min="0"
                          max="1"
                          step="0.05"
                          value={obj.bounceDamping ?? 0.85}
                          onChange={(e) => {
                            const newObjects = objects.map(o =>
                              o.id === selectedId ? { ...o, bounceDamping: parseFloat(e.target.value) } : o
                            );
                            setObjects(newObjects);
                          }}
                          style={{ width: '100%' }}
                        />
                        <span style={{ fontSize: '10px', color: '#d4d4d4' }}>{((obj.bounceDamping ?? 0.85) * 100).toFixed(0)}%</span>
                      </div>
                    )}
                    <div>
                      <label style={{ color: '#888', display: 'flex', alignItems: 'center', gap: '8px', cursor: 'pointer' }}>
                        <input
                          type="checkbox"
                          checked={obj.physicsEnabled ?? false}
                          onChange={(e) => {
                            const newObjects = objects.map(o =>
                              o.id === selectedId ? { ...o, physicsEnabled: e.target.checked } : o
                            );
                            setObjects(newObjects);
                          }}
                          style={{
                            width: '16px',
                            height: '16px',
                            cursor: 'pointer',
                          }}
                        />
                        Physics Enabled
                      </label>
                      <div style={{ fontSize: '10px', color: '#666', marginTop: '4px', marginLeft: '24px' }}>
                        {obj.physicsEnabled ? '⚡ Aplica física' : '🔒 Estático'}
                      </div>
                    </div>
                    {obj.physicsEnabled && (
                      <div>
                        <label style={{ color: '#888', display: 'block', marginBottom: '4px' }}>
                          Physics Type
                        </label>
                        <select
                          value={obj.physicsType || 'gravity'}
                          onChange={(e) => {
                            const newObjects = objects.map(o =>
                              o.id === selectedId ? { ...o, physicsType: e.target.value as any } : o
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
                        >
                          <option value="gravity">🌍 Gravity (cae, pierde energía)</option>
                          <option value="bounce">⚡ Bounce (rebote perpetuo)</option>
                          <option value="projectile">🎯 Projectile (parábola, no rebota)</option>
                          <option value="static">🔒 Static (sin movimiento)</option>
                        </select>
                      </div>
                    )}
                    <div>
                      <label style={{ color: '#888', display: 'flex', alignItems: 'center', gap: '8px', cursor: 'pointer' }}>
                        <input
                          type="checkbox"
                          checked={obj.collidable ?? true}
                          onChange={(e) => {
                            const newObjects = objects.map(o =>
                              o.id === selectedId ? { ...o, collidable: e.target.checked } : o
                            );
                            setObjects(newObjects);
                          }}
                          style={{
                            width: '16px',
                            height: '16px',
                            cursor: 'pointer',
                          }}
                        />
                        Collidable (rebota)
                      </label>
                      <div style={{ fontSize: '10px', color: '#666', marginTop: '4px', marginLeft: '24px' }}>
                        {obj.collidable ? '🔷 Objeto sólido - los demás rebotan' : '⬜ Objeto atravesable - sin colisión'}
                      </div>
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

      {/* Save/Load Modal */}
      {showSaveLoadModal && (
        <div style={{
          position: 'fixed',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          backgroundColor: 'rgba(0,0,0,0.7)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 1000,
        }}>
          <div style={{
            backgroundColor: '#252526',
            border: '1px solid #444',
            borderRadius: '4px',
            padding: '20px',
            minWidth: '400px',
            maxWidth: '600px',
            maxHeight: '80vh',
            display: 'flex',
            flexDirection: 'column',
          }}>
            <h2 style={{ margin: '0 0 16px 0', fontSize: '16px', color: '#d4d4d4' }}>
              {modalMode === 'save' ? '💾 Save Scene' : '📁 Load Scene'}
            </h2>

            {modalMode === 'save' && (
              <div style={{ marginBottom: '16px' }}>
                <label style={{ display: 'block', marginBottom: '8px', fontSize: '12px', color: '#888' }}>
                  Scene Name:
                </label>
                <input
                  type="text"
                  value={sceneName}
                  onChange={(e) => setSceneName(e.target.value)}
                  placeholder="my_scene"
                  style={{
                    width: '100%',
                    padding: '8px',
                    fontSize: '12px',
                    backgroundColor: '#1e1e1e',
                    border: '1px solid #444',
                    borderRadius: '2px',
                    color: '#d4d4d4',
                  }}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') handleSaveScene();
                  }}
                />
              </div>
            )}

            {modalMode === 'load' && (
              <div style={{
                flex: 1,
                overflowY: 'auto',
                marginBottom: '16px',
                border: '1px solid #444',
                borderRadius: '2px',
              }}>
                {availableScenes.length === 0 ? (
                  <div style={{ padding: '20px', textAlign: 'center', color: '#666', fontSize: '12px' }}>
                    No saved scenes found
                  </div>
                ) : (
                  availableScenes.map(scene => (
                    <div
                      key={scene}
                      style={{
                        padding: '12px',
                        borderBottom: '1px solid #333',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'space-between',
                      }}
                    >
                      <span style={{ fontSize: '12px', color: '#d4d4d4' }}>{scene}</span>
                      <div style={{ display: 'flex', gap: '8px' }}>
                        <button
                          onClick={() => handleLoadScene(scene)}
                          style={{
                            padding: '4px 12px',
                            fontSize: '11px',
                            border: '1px solid #0e639c',
                            backgroundColor: '#0e639c',
                            borderRadius: '2px',
                            color: '#fff',
                            cursor: 'pointer',
                          }}
                        >
                          Load
                        </button>
                        <button
                          onClick={() => handleDeleteScene(scene)}
                          style={{
                            padding: '4px 12px',
                            fontSize: '11px',
                            border: '1px solid #c72e0f',
                            backgroundColor: '#c72e0f',
                            borderRadius: '2px',
                            color: '#fff',
                            cursor: 'pointer',
                          }}
                        >
                          Delete
                        </button>
                      </div>
                    </div>
                  ))
                )}
              </div>
            )}

            <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
              {modalMode === 'save' && (
                <button
                  onClick={handleSaveScene}
                  style={{
                    padding: '8px 16px',
                    fontSize: '12px',
                    border: '1px solid #0e639c',
                    backgroundColor: '#0e639c',
                    borderRadius: '2px',
                    color: '#fff',
                    cursor: 'pointer',
                  }}
                >
                  Save
                </button>
              )}
              <button
                onClick={() => {
                  setShowSaveLoadModal(false);
                  setSceneName('');
                }}
                style={{
                  padding: '8px 16px',
                  fontSize: '12px',
                  border: '1px solid #444',
                  backgroundColor: '#3e3e3e',
                  borderRadius: '2px',
                  color: '#d4d4d4',
                  cursor: 'pointer',
                }}
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Toast notifications */}
      <div style={{
        position: 'fixed',
        bottom: '20px',
        right: '20px',
        display: 'flex',
        flexDirection: 'column',
        gap: '8px',
        zIndex: 10000,
      }}>
        {toasts.map(toast => (
          <div
            key={toast.id}
            style={{
              padding: '12px 16px',
              backgroundColor: toast.type === 'success' ? '#0e639c' : '#d32f2f',
              color: '#fff',
              borderRadius: '4px',
              boxShadow: '0 2px 8px rgba(0,0,0,0.3)',
              fontSize: '13px',
              minWidth: '200px',
              animation: 'slideIn 0.3s ease-out',
            }}
          >
            {toast.message}
          </div>
        ))}
      </div>
    </div>
  );
}
