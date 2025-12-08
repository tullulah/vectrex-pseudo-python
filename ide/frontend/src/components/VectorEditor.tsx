/**
 * VectorEditor - Visual editor for .vec vector resources
 * 
 * A canvas-based editor for creating and editing Vectrex vector graphics.
 * Features:
 * - Background image layer for tracing
 * - Automatic edge detection to generate vectors from images
 * - Drawing tools: Select, Pen
 * - Layers panel with visibility toggles
 */

import React, { useRef, useEffect, useState, useCallback } from 'react';

// Types from the .vec format
interface Point {
  x: number;
  y: number;
}

interface VecPath {
  name: string;
  intensity: number;
  closed: boolean;
  points: Point[];
}

interface Layer {
  name: string;
  visible: boolean;
  paths: VecPath[];
}

interface VecResource {
  version: string;
  name: string;
  author: string;
  created: string;
  canvas: {
    width: number;
    height: number;
    origin: string;
  };
  layers: Layer[];
  animations: any[];
  metadata: {
    hitbox: { x: number; y: number; w: number; h: number } | null;
    origin: Point | null;
    tags: string[];
  };
  // Background image stored as base64 data URL
  backgroundImage?: string;
}

interface VectorEditorProps {
  /** Initial resource to edit */
  resource?: VecResource;
  /** Callback when resource changes */
  onChange?: (resource: VecResource) => void;
  /** Width of the editor */
  width?: number;
  /** Height of the editor */
  height?: number;
}

type Tool = 'select' | 'pen' | 'line' | 'polygon' | 'pan';

const defaultResource: VecResource = {
  version: '1.0',
  name: 'untitled',
  author: '',
  created: '',
  canvas: { width: 256, height: 256, origin: 'center' },
  layers: [{ name: 'default', visible: true, paths: [] }],
  animations: [],
  metadata: { hitbox: null, origin: null, tags: [] },
};

// ============================================
// Edge Detection Algorithm (Canny-like)
// ============================================

interface EdgeDetectionOptions {
  lowThreshold: number;
  highThreshold: number;
  simplifyTolerance: number;
  minPathLength: number;
  useBlur: boolean; // Whether to apply Gaussian blur (disable for thin lines)
}

const defaultEdgeOptions: EdgeDetectionOptions = {
  lowThreshold: 20,
  highThreshold: 60,
  simplifyTolerance: 2.0,
  minPathLength: 4,
  useBlur: false, // Disabled by default - better for pixel art / thin lines
};

/**
 * Apply Gaussian blur for noise reduction
 */
function gaussianBlur(imageData: ImageData): ImageData {
  const width = imageData.width;
  const height = imageData.height;
  const src = imageData.data;
  const output = new ImageData(width, height);
  const dst = output.data;

  // 5x5 Gaussian kernel (sigma ~1.4)
  const kernel = [
    1, 4, 6, 4, 1,
    4, 16, 24, 16, 4,
    6, 24, 36, 24, 6,
    4, 16, 24, 16, 4,
    1, 4, 6, 4, 1
  ];
  const kernelSum = 256;

  for (let y = 2; y < height - 2; y++) {
    for (let x = 2; x < width - 2; x++) {
      let r = 0, g = 0, b = 0;
      for (let ky = -2; ky <= 2; ky++) {
        for (let kx = -2; kx <= 2; kx++) {
          const idx = ((y + ky) * width + (x + kx)) * 4;
          const k = kernel[(ky + 2) * 5 + (kx + 2)];
          r += src[idx] * k;
          g += src[idx + 1] * k;
          b += src[idx + 2] * k;
        }
      }
      const dstIdx = (y * width + x) * 4;
      dst[dstIdx] = r / kernelSum;
      dst[dstIdx + 1] = g / kernelSum;
      dst[dstIdx + 2] = b / kernelSum;
      dst[dstIdx + 3] = 255;
    }
  }
  return output;
}

/**
 * Apply Sobel edge detection with gradient direction
 */
function sobelEdgeDetection(imageData: ImageData): { magnitude: Float32Array; direction: Float32Array; width: number; height: number } {
  const width = imageData.width;
  const height = imageData.height;
  const src = imageData.data;
  const magnitude = new Float32Array(width * height);
  const direction = new Float32Array(width * height);

  // Sobel kernels
  const sobelX = [-1, 0, 1, -2, 0, 2, -1, 0, 1];
  const sobelY = [-1, -2, -1, 0, 0, 0, 1, 2, 1];

  for (let y = 1; y < height - 1; y++) {
    for (let x = 1; x < width - 1; x++) {
      let gx = 0, gy = 0;
      
      for (let ky = -1; ky <= 1; ky++) {
        for (let kx = -1; kx <= 1; kx++) {
          const idx = ((y + ky) * width + (x + kx)) * 4;
          const gray = src[idx] * 0.299 + src[idx + 1] * 0.587 + src[idx + 2] * 0.114;
          const kernelIdx = (ky + 1) * 3 + (kx + 1);
          gx += gray * sobelX[kernelIdx];
          gy += gray * sobelY[kernelIdx];
        }
      }
      
      const idx = y * width + x;
      magnitude[idx] = Math.sqrt(gx * gx + gy * gy);
      direction[idx] = Math.atan2(gy, gx);
    }
  }

  return { magnitude, direction, width, height };
}

/**
 * Non-Maximum Suppression - thin edges to single pixel width
 */
function nonMaximumSuppression(
  magnitude: Float32Array,
  direction: Float32Array,
  width: number,
  height: number
): Float32Array {
  const output = new Float32Array(width * height);

  for (let y = 1; y < height - 1; y++) {
    for (let x = 1; x < width - 1; x++) {
      const idx = y * width + x;
      const mag = magnitude[idx];
      const angle = direction[idx];
      
      // Determine gradient direction (0, 45, 90, 135 degrees)
      let neighbor1 = 0, neighbor2 = 0;
      const absAngle = Math.abs(angle);
      
      if (absAngle < Math.PI / 8 || absAngle > 7 * Math.PI / 8) {
        // Horizontal edge - compare with left/right
        neighbor1 = magnitude[idx - 1];
        neighbor2 = magnitude[idx + 1];
      } else if (absAngle < 3 * Math.PI / 8) {
        // Diagonal edge (45 or -45)
        if (angle > 0) {
          neighbor1 = magnitude[(y - 1) * width + (x + 1)];
          neighbor2 = magnitude[(y + 1) * width + (x - 1)];
        } else {
          neighbor1 = magnitude[(y - 1) * width + (x - 1)];
          neighbor2 = magnitude[(y + 1) * width + (x + 1)];
        }
      } else if (absAngle < 5 * Math.PI / 8) {
        // Vertical edge - compare with top/bottom
        neighbor1 = magnitude[(y - 1) * width + x];
        neighbor2 = magnitude[(y + 1) * width + x];
      } else {
        // Other diagonal
        if (angle > 0) {
          neighbor1 = magnitude[(y - 1) * width + (x - 1)];
          neighbor2 = magnitude[(y + 1) * width + (x + 1)];
        } else {
          neighbor1 = magnitude[(y - 1) * width + (x + 1)];
          neighbor2 = magnitude[(y + 1) * width + (x - 1)];
        }
      }
      
      // Keep pixel only if it's a local maximum
      if (mag >= neighbor1 && mag >= neighbor2) {
        output[idx] = mag;
      }
    }
  }

  return output;
}

/**
 * Double threshold and hysteresis
 */
function hysteresisThreshold(
  magnitude: Float32Array,
  width: number,
  height: number,
  lowThreshold: number,
  highThreshold: number
): boolean[][] {
  const edges: boolean[][] = Array(height).fill(null).map(() => Array(width).fill(false));
  const strong: boolean[][] = Array(height).fill(null).map(() => Array(width).fill(false));
  
  // Mark strong and weak edges
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const mag = magnitude[y * width + x];
      if (mag >= highThreshold) {
        strong[y][x] = true;
        edges[y][x] = true;
      }
    }
  }
  
  // Hysteresis - connect weak edges to strong edges
  let changed = true;
  while (changed) {
    changed = false;
    for (let y = 1; y < height - 1; y++) {
      for (let x = 1; x < width - 1; x++) {
        if (edges[y][x]) continue;
        
        const mag = magnitude[y * width + x];
        if (mag < lowThreshold) continue;
        
        // Check if connected to an edge
        for (let dy = -1; dy <= 1; dy++) {
          for (let dx = -1; dx <= 1; dx++) {
            if (edges[y + dy][x + dx]) {
              edges[y][x] = true;
              changed = true;
              break;
            }
          }
          if (edges[y][x]) break;
        }
      }
    }
  }

  return edges;
}

/**
 * Trace edge pixels into paths with better connectivity
 */
function traceEdgesToPaths(
  edges: boolean[][],
  imgWidth: number,
  imgHeight: number,
  imgDrawX: number,
  imgDrawY: number,
  imgDrawWidth: number,
  imgDrawHeight: number,
  canvasWidth: number,
  canvasHeight: number,
  resourceWidth: number,
  resourceHeight: number
): VecPath[] {
  const height = edges.length;
  const width = edges[0].length;
  const visited: boolean[][] = Array(height).fill(null).map(() => Array(width).fill(false));
  const paths: VecPath[] = [];

  // Direction vectors for 8-connectivity - prioritize straight directions
  const dx = [1, 0, -1, 0, 1, 1, -1, -1];
  const dy = [0, 1, 0, -1, 1, -1, 1, -1];

  // Convert pixel coordinates to resource coordinates
  const pixelToResource = (px: number, py: number): Point => {
    // First, map from edge image coords to canvas coords
    const canvasX = imgDrawX + (px / width) * imgDrawWidth;
    const canvasY = imgDrawY + (py / height) * imgDrawHeight;
    
    // Then map from canvas coords to resource coords (centered)
    const centerX = canvasWidth / 2;
    const centerY = canvasHeight / 2;
    const scale = Math.min(canvasWidth, canvasHeight) / resourceWidth;
    
    return {
      x: Math.round((canvasX - centerX) / scale),
      y: Math.round((centerY - canvasY) / scale),
    };
  };

  for (let startY = 0; startY < height; startY++) {
    for (let startX = 0; startX < width; startX++) {
      if (!edges[startY][startX] || visited[startY][startX]) continue;

      // Start a new path
      const pathPixels: Array<{x: number, y: number}> = [];
      let x = startX, y = startY;

      while (true) {
        visited[y][x] = true;
        pathPixels.push({ x, y });

        // Find next unvisited edge pixel
        let found = false;
        for (let d = 0; d < 8; d++) {
          const nx = x + dx[d];
          const ny = y + dy[d];
          if (nx >= 0 && nx < width && ny >= 0 && ny < height &&
              edges[ny][nx] && !visited[ny][nx]) {
            x = nx;
            y = ny;
            found = true;
            break;
          }
        }

        if (!found) break;
      }

      // Only keep paths with enough pixels
      if (pathPixels.length >= 4) {
        // Sample every Nth pixel to reduce noise and improve performance
        const sampleRate = Math.max(1, Math.floor(pathPixels.length / 100));
        const sampledPoints: Point[] = [];
        for (let i = 0; i < pathPixels.length; i += sampleRate) {
          sampledPoints.push(pixelToResource(pathPixels[i].x, pathPixels[i].y));
        }
        // Always include last point
        if (pathPixels.length > 0) {
          const last = pathPixels[pathPixels.length - 1];
          const lastPoint = pixelToResource(last.x, last.y);
          if (sampledPoints.length > 0) {
            const prevLast = sampledPoints[sampledPoints.length - 1];
            if (prevLast.x !== lastPoint.x || prevLast.y !== lastPoint.y) {
              sampledPoints.push(lastPoint);
            }
          }
        }
        
        if (sampledPoints.length >= 2) {
          paths.push({
            name: `traced_${paths.length}`,
            intensity: 127,
            closed: false,
            points: sampledPoints,
          });
        }
      }
    }
  }

  return paths;
}

/**
 * Simplify a path using Ramer-Douglas-Peucker algorithm
 */
function simplifyPath(points: Point[], tolerance: number): Point[] {
  if (points.length <= 2) return points;

  // Find the point with the maximum distance
  let maxDist = 0;
  let maxIdx = 0;
  const start = points[0];
  const end = points[points.length - 1];

  for (let i = 1; i < points.length - 1; i++) {
    const dist = perpendicularDistance(points[i], start, end);
    if (dist > maxDist) {
      maxDist = dist;
      maxIdx = i;
    }
  }

  // If max distance is greater than tolerance, recursively simplify
  if (maxDist > tolerance) {
    const left = simplifyPath(points.slice(0, maxIdx + 1), tolerance);
    const right = simplifyPath(points.slice(maxIdx), tolerance);
    return [...left.slice(0, -1), ...right];
  } else {
    return [start, end];
  }
}

function perpendicularDistance(point: Point, lineStart: Point, lineEnd: Point): number {
  const dx = lineEnd.x - lineStart.x;
  const dy = lineEnd.y - lineStart.y;
  const len = Math.sqrt(dx * dx + dy * dy);
  
  if (len === 0) {
    return Math.sqrt((point.x - lineStart.x) ** 2 + (point.y - lineStart.y) ** 2);
  }
  
  return Math.abs(
    (dy * point.x - dx * point.y + lineEnd.x * lineStart.y - lineEnd.y * lineStart.x) / len
  );
}

/**
 * Detect edges in an image and convert to vector paths
 * Uses Canny-like algorithm: Blur -> Sobel -> NMS -> Hysteresis -> Trace
 */
function detectEdgesFromImage(
  img: HTMLImageElement,
  canvasWidth: number,
  canvasHeight: number,
  resourceWidth: number,
  resourceHeight: number,
  options: EdgeDetectionOptions = defaultEdgeOptions
): VecPath[] {
  // Calculate image draw position (same as in the draw function)
  const imgAspect = img.width / img.height;
  const canvasAspect = canvasWidth / canvasHeight;
  let drawWidth: number, drawHeight: number, drawX: number, drawY: number;
  
  if (imgAspect > canvasAspect) {
    drawWidth = canvasWidth;
    drawHeight = canvasWidth / imgAspect;
  } else {
    drawHeight = canvasHeight;
    drawWidth = canvasHeight * imgAspect;
  }
  drawX = (canvasWidth - drawWidth) / 2;
  drawY = (canvasHeight - drawHeight) / 2;
  
  // Process at a reasonable resolution for speed
  const processWidth = Math.min(400, img.width);
  const processHeight = Math.round(processWidth / imgAspect);
  
  // Create a temporary canvas to process the image
  const tempCanvas = document.createElement('canvas');
  tempCanvas.width = processWidth;
  tempCanvas.height = processHeight;
  const ctx = tempCanvas.getContext('2d')!;
  
  // Draw image to processing canvas
  ctx.drawImage(img, 0, 0, processWidth, processHeight);
  
  // Get image data
  let imageData = ctx.getImageData(0, 0, processWidth, processHeight);
  
  // Step 1: Optional Gaussian blur for noise reduction (skip for thin lines/pixel art)
  if (options.useBlur) {
    imageData = gaussianBlur(imageData);
  }
  
  // Step 2: Sobel edge detection with gradient direction
  const { magnitude, direction, width, height } = sobelEdgeDetection(imageData);
  
  // Step 3: Non-Maximum Suppression - thin edges to 1 pixel
  const thinMagnitude = nonMaximumSuppression(magnitude, direction, width, height);
  
  // Step 4: Double threshold and hysteresis
  const edges = hysteresisThreshold(thinMagnitude, width, height, options.lowThreshold, options.highThreshold);
  
  // Step 5: Trace edges to paths with correct coordinate mapping
  let paths = traceEdgesToPaths(
    edges, 
    processWidth, 
    processHeight, 
    drawX, 
    drawY, 
    drawWidth, 
    drawHeight,
    canvasWidth, 
    canvasHeight, 
    resourceWidth, 
    resourceHeight
  );
  
  // Step 6: Simplify paths
  paths = paths.map(path => ({
    ...path,
    points: simplifyPath(path.points, options.simplifyTolerance),
  }));
  
  // Step 7: Filter out short paths
  paths = paths.filter(path => path.points.length >= options.minPathLength);
  
  return paths;
}

// ============================================
// Main VectorEditor Component
// ============================================

export const VectorEditor: React.FC<VectorEditorProps> = ({
  resource: initialResource,
  onChange,
  width = 512,
  height = 512,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  
  // Ensure resource has valid structure with visible layers
  const normalizeResource = (res: VecResource | undefined): VecResource => {
    if (!res) return defaultResource;
    const normalized = { ...res };
    // Ensure layers array exists
    if (!normalized.layers || normalized.layers.length === 0) {
      normalized.layers = [{ name: 'drawing', visible: true, paths: [] }];
    } else {
      // Ensure all layers have visible property
      normalized.layers = normalized.layers.map(layer => ({
        ...layer,
        visible: layer.visible !== false, // default to true
      }));
    }
    return normalized;
  };
  
  const [resource, setResource] = useState<VecResource>(() => normalizeResource(initialResource));
  const [currentTool, setCurrentTool] = useState<Tool>('pen');
  const [currentLayerIndex, setCurrentLayerIndex] = useState(0);
  const [currentPathIndex, setCurrentPathIndex] = useState(-1);
  const [selectedPointIndex, setSelectedPointIndex] = useState(-1);
  const [selectedPoints, setSelectedPoints] = useState<Set<string>>(new Set()); // "pathIdx-pointIdx" format
  const [zoom, setZoom] = useState(1);
  const [pan, setPan] = useState({ x: 0, y: 0 });
  const [isDrawing, setIsDrawing] = useState(false);
  const [tempPoints, setTempPoints] = useState<Point[]>([]);
  
  // Track if we're the source of changes to avoid loops
  const isInternalChange = useRef(false);
  
  // Sync with external resource changes (but not our own changes)
  useEffect(() => {
    if (isInternalChange.current) {
      isInternalChange.current = false;
      return;
    }
    if (initialResource) {
      setResource(normalizeResource(initialResource));
    }
  }, [initialResource]);
  
  // Wrapper to set resource and notify parent
  const updateResource = useCallback((newResource: VecResource) => {
    isInternalChange.current = true;
    setResource(newResource);
    onChange?.(newResource);
  }, [onChange]);
  
  // Box selection state
  const [isBoxSelecting, setIsBoxSelecting] = useState(false);
  const [boxStart, setBoxStart] = useState<{ x: number; y: number } | null>(null);
  const [boxEnd, setBoxEnd] = useState<{ x: number; y: number } | null>(null);
  
  // Background image state
  const [backgroundImage, setBackgroundImage] = useState<HTMLImageElement | null>(null);
  const [backgroundOpacity, setBackgroundOpacity] = useState(0.5);
  const [showBackground, setShowBackground] = useState(true);
  
  // Edge detection settings
  const [edgeOptions, setEdgeOptions] = useState<EdgeDetectionOptions>(defaultEdgeOptions);
  const [showEdgeSettings, setShowEdgeSettings] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [previewPaths, setPreviewPaths] = useState<VecPath[]>([]); // Preview paths before applying
  const [showPreview, setShowPreview] = useState(true);
  const previewTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Generate preview when edge settings change
  useEffect(() => {
    if (!showEdgeSettings || !backgroundImage) {
      setPreviewPaths([]);
      return;
    }
    
    // Debounce the preview generation
    if (previewTimeoutRef.current) {
      clearTimeout(previewTimeoutRef.current);
    }
    
    previewTimeoutRef.current = setTimeout(() => {
      try {
        const paths = detectEdgesFromImage(
          backgroundImage,
          width,
          height,
          resource.canvas.width,
          resource.canvas.height,
          edgeOptions
        );
        setPreviewPaths(paths);
      } catch (error) {
        console.error('Preview generation failed:', error);
        setPreviewPaths([]);
      }
    }, 150); // 150ms debounce
    
    return () => {
      if (previewTimeoutRef.current) {
        clearTimeout(previewTimeoutRef.current);
      }
    };
  }, [edgeOptions, backgroundImage, showEdgeSettings, width, height, resource.canvas.width, resource.canvas.height]);

  // Convert canvas coordinates to resource coordinates
  const canvasToResource = useCallback((canvasX: number, canvasY: number): Point => {
    const centerX = width / 2;
    const centerY = height / 2;
    const scale = Math.min(width, height) / resource.canvas.width;
    
    return {
      x: Math.round((canvasX - centerX - pan.x) / (scale * zoom)),
      y: Math.round((centerY - canvasY + pan.y) / (scale * zoom)),
    };
  }, [width, height, resource.canvas.width, pan, zoom]);

  // Convert resource coordinates to canvas coordinates
  const resourceToCanvas = useCallback((point: Point): { x: number; y: number } => {
    const centerX = width / 2;
    const centerY = height / 2;
    const scale = Math.min(width, height) / resource.canvas.width;
    
    return {
      x: centerX + point.x * scale * zoom + pan.x,
      y: centerY - point.y * scale * zoom + pan.y,
    };
  }, [width, height, resource.canvas.width, pan, zoom]);

  // Draw the canvas
  const draw = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear
    ctx.fillStyle = '#1a1a2e';
    ctx.fillRect(0, 0, width, height);

    // Draw background image if present
    if (backgroundImage && showBackground) {
      ctx.save();
      ctx.globalAlpha = backgroundOpacity;
      
      // Calculate image position and size
      const imgAspect = backgroundImage.width / backgroundImage.height;
      const canvasAspect = width / height;
      let drawWidth, drawHeight, drawX, drawY;
      
      if (imgAspect > canvasAspect) {
        drawWidth = width * zoom;
        drawHeight = (width / imgAspect) * zoom;
      } else {
        drawHeight = height * zoom;
        drawWidth = (height * imgAspect) * zoom;
      }
      
      drawX = (width - drawWidth) / 2 + pan.x;
      drawY = (height - drawHeight) / 2 + pan.y;
      
      ctx.drawImage(backgroundImage, drawX, drawY, drawWidth, drawHeight);
      ctx.restore();
    }

    // Draw grid
    ctx.strokeStyle = '#2a2a4e';
    ctx.lineWidth = 1;
    const gridSize = 16 * zoom;
    const centerX = width / 2 + pan.x;
    const centerY = height / 2 + pan.y;
    
    for (let x = centerX % gridSize; x < width; x += gridSize) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, height);
      ctx.stroke();
    }
    for (let y = centerY % gridSize; y < height; y += gridSize) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();
    }

    // Draw axes
    ctx.strokeStyle = '#4a4a6e';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(0, centerY);
    ctx.lineTo(width, centerY);
    ctx.moveTo(centerX, 0);
    ctx.lineTo(centerX, height);
    ctx.stroke();

    // Draw paths
    for (let layerIdx = 0; layerIdx < resource.layers.length; layerIdx++) {
      const layer = resource.layers[layerIdx];
      if (!layer.visible) continue;

      for (let pathIdx = 0; pathIdx < layer.paths.length; pathIdx++) {
        const path = layer.paths[pathIdx];
        if (path.points.length < 2) continue;

        const intensity = path.intensity / 127;
        const green = Math.floor(200 + 55 * intensity);
        ctx.strokeStyle = `rgb(${Math.floor(100 * intensity)}, ${green}, ${Math.floor(100 * intensity)})`;
        ctx.lineWidth = 2;

        ctx.beginPath();
        const start = resourceToCanvas(path.points[0]);
        ctx.moveTo(start.x, start.y);

        for (let i = 1; i < path.points.length; i++) {
          const pt = resourceToCanvas(path.points[i]);
          ctx.lineTo(pt.x, pt.y);
        }

        if (path.closed) {
          ctx.closePath();
        }
        ctx.stroke();

        if (layerIdx === currentLayerIndex && pathIdx === currentPathIndex) {
          ctx.fillStyle = '#ffff00';
          for (let i = 0; i < path.points.length; i++) {
            const pt = resourceToCanvas(path.points[i]);
            const isSelected = selectedPoints.has(`${pathIdx}-${i}`);
            ctx.beginPath();
            ctx.arc(pt.x, pt.y, i === selectedPointIndex || isSelected ? 6 : 4, 0, Math.PI * 2);
            ctx.fill();
          }
        }
        
        // Draw selected points from multi-selection
        if (layerIdx === currentLayerIndex) {
          for (let i = 0; i < path.points.length; i++) {
            if (selectedPoints.has(`${pathIdx}-${i}`)) {
              const pt = resourceToCanvas(path.points[i]);
              ctx.fillStyle = '#ff6600';
              ctx.beginPath();
              ctx.arc(pt.x, pt.y, 6, 0, Math.PI * 2);
              ctx.fill();
            }
          }
        }
      }
    }

    // Draw temporary points while drawing
    if (tempPoints.length > 0) {
      ctx.strokeStyle = '#00ffff';
      ctx.lineWidth = 2;
      ctx.setLineDash([5, 5]);
      ctx.beginPath();
      const start = resourceToCanvas(tempPoints[0]);
      ctx.moveTo(start.x, start.y);
      for (let i = 1; i < tempPoints.length; i++) {
        const pt = resourceToCanvas(tempPoints[i]);
        ctx.lineTo(pt.x, pt.y);
      }
      ctx.stroke();
      ctx.setLineDash([]);

      ctx.fillStyle = '#00ffff';
      for (const point of tempPoints) {
        const pt = resourceToCanvas(point);
        ctx.beginPath();
        ctx.arc(pt.x, pt.y, 4, 0, Math.PI * 2);
        ctx.fill();
      }
    }
    
    // Draw box selection rectangle
    if (isBoxSelecting && boxStart && boxEnd) {
      ctx.strokeStyle = '#00aaff';
      ctx.lineWidth = 1;
      ctx.setLineDash([4, 4]);
      ctx.fillStyle = 'rgba(0, 170, 255, 0.1)';
      const x = Math.min(boxStart.x, boxEnd.x);
      const y = Math.min(boxStart.y, boxEnd.y);
      const w = Math.abs(boxEnd.x - boxStart.x);
      const h = Math.abs(boxEnd.y - boxStart.y);
      ctx.fillRect(x, y, w, h);
      ctx.strokeRect(x, y, w, h);
      ctx.setLineDash([]);
    }
    
    // Draw preview paths (edge detection preview)
    if (showPreview && previewPaths.length > 0 && showEdgeSettings) {
      ctx.strokeStyle = '#ff00ff';
      ctx.lineWidth = 1.5;
      ctx.setLineDash([3, 3]);
      ctx.globalAlpha = 0.7;
      
      for (const path of previewPaths) {
        if (path.points.length < 2) continue;
        
        ctx.beginPath();
        const start = resourceToCanvas(path.points[0]);
        ctx.moveTo(start.x, start.y);
        
        for (let i = 1; i < path.points.length; i++) {
          const pt = resourceToCanvas(path.points[i]);
          ctx.lineTo(pt.x, pt.y);
        }
        
        if (path.closed) {
          ctx.closePath();
        }
        ctx.stroke();
      }
      
      ctx.setLineDash([]);
      ctx.globalAlpha = 1;
    }
  }, [resource, currentLayerIndex, currentPathIndex, selectedPointIndex, selectedPoints, tempPoints, pan, zoom, width, height, resourceToCanvas, backgroundImage, backgroundOpacity, showBackground, isBoxSelecting, boxStart, boxEnd, showPreview, previewPaths, showEdgeSettings]);

  useEffect(() => {
    draw();
  }, [draw]);

  // Handle image upload
  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const dataUrl = event.target?.result as string;
      const img = new Image();
      img.onload = () => {
        setBackgroundImage(img);
        setShowBackground(true);
        
        // Save the image data URL in the resource so it persists
        const newResource = { ...resource, backgroundImage: dataUrl };
        updateResource(newResource);
      };
      img.src = dataUrl;
    };
    reader.readAsDataURL(file);
  };
  
  // Restore background image from resource on load
  useEffect(() => {
    if (resource.backgroundImage && !backgroundImage) {
      const img = new Image();
      img.onload = () => {
        setBackgroundImage(img);
        setShowBackground(true);
      };
      img.src = resource.backgroundImage;
    }
  }, [resource.backgroundImage]);

  // Auto-detect edges and create vectors
  const handleAutoDetect = async () => {
    if (!backgroundImage) return;
    
    setIsProcessing(true);
    
    // Use requestAnimationFrame to avoid blocking UI
    await new Promise(resolve => requestAnimationFrame(resolve));
    
    try {
      const paths = detectEdgesFromImage(
        backgroundImage,
        width,
        height,
        resource.canvas.width,
        resource.canvas.height,
        edgeOptions
      );
      
      if (paths.length > 0) {
        // Add detected paths to the main drawing layer (index 0)
        const newResource = { ...resource };
        // Ensure layer 0 exists
        if (!newResource.layers[0]) {
          newResource.layers[0] = { name: 'drawing', visible: true, paths: [] };
        }
        // Add traced paths to the main layer
        newResource.layers[0].paths.push(...paths);
        updateResource(newResource);
      } else {
        alert('No edges detected. Try adjusting the threshold values.');
      }
    } catch (error) {
      console.error('Edge detection failed:', error);
      alert('Edge detection failed. Please try again with different settings.');
    } finally {
      setIsProcessing(false);
    }
  };

  // Mouse event handlers
  const handleMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;

    const canvasX = e.clientX - rect.left;
    const canvasY = e.clientY - rect.top;
    const point = canvasToResource(canvasX, canvasY);

    if (currentTool === 'pen') {
      setTempPoints([...tempPoints, point]);
      setIsDrawing(true);
    } else if (currentTool === 'select') {
      // Check if clicking on a point
      let closestDist = Infinity;
      let closestPath = -1;
      let closestPoint = -1;

      const layer = resource.layers[currentLayerIndex];
      for (let pathIdx = 0; pathIdx < layer.paths.length; pathIdx++) {
        const path = layer.paths[pathIdx];
        for (let pointIdx = 0; pointIdx < path.points.length; pointIdx++) {
          const pt = resourceToCanvas(path.points[pointIdx]);
          const dist = Math.sqrt((pt.x - canvasX) ** 2 + (pt.y - canvasY) ** 2);
          if (dist < closestDist && dist < 10) {
            closestDist = dist;
            closestPath = pathIdx;
            closestPoint = pointIdx;
          }
        }
      }

      if (closestPath >= 0) {
        // Clicked on a point - select it
        setCurrentPathIndex(closestPath);
        setSelectedPointIndex(closestPoint);
        const key = `${closestPath}-${closestPoint}`;
        if (e.shiftKey) {
          // Add to selection
          setSelectedPoints(prev => new Set([...prev, key]));
        } else {
          setSelectedPoints(new Set([key]));
        }
        setIsDrawing(true); // Enable dragging
      } else {
        // Start box selection
        setIsBoxSelecting(true);
        setBoxStart({ x: canvasX, y: canvasY });
        setBoxEnd({ x: canvasX, y: canvasY });
        if (!e.shiftKey) {
          setSelectedPoints(new Set());
          setSelectedPointIndex(-1);
          setCurrentPathIndex(-1);
        }
      }
    }
  };

  const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;

    const canvasX = e.clientX - rect.left;
    const canvasY = e.clientY - rect.top;
    
    if (isBoxSelecting && currentTool === 'select') {
      // Update box selection
      setBoxEnd({ x: canvasX, y: canvasY });
      return;
    }
    
    if (!isDrawing || currentTool !== 'select') return;

    const point = canvasToResource(canvasX, canvasY);

    if (selectedPointIndex >= 0 && currentPathIndex >= 0) {
      const newResource = { ...resource };
      newResource.layers[currentLayerIndex].paths[currentPathIndex].points[selectedPointIndex] = point;
      updateResource(newResource);
    }
  };

  const handleMouseUp = () => {
    // Complete box selection
    if (isBoxSelecting && boxStart && boxEnd) {
      const minX = Math.min(boxStart.x, boxEnd.x);
      const maxX = Math.max(boxStart.x, boxEnd.x);
      const minY = Math.min(boxStart.y, boxEnd.y);
      const maxY = Math.max(boxStart.y, boxEnd.y);
      
      // Find all points within the box
      const newSelection = new Set(selectedPoints);
      const layer = resource.layers[currentLayerIndex];
      
      for (let pathIdx = 0; pathIdx < layer.paths.length; pathIdx++) {
        const path = layer.paths[pathIdx];
        for (let pointIdx = 0; pointIdx < path.points.length; pointIdx++) {
          const canvasPt = resourceToCanvas(path.points[pointIdx]);
          if (canvasPt.x >= minX && canvasPt.x <= maxX && 
              canvasPt.y >= minY && canvasPt.y <= maxY) {
            newSelection.add(`${pathIdx}-${pointIdx}`);
          }
        }
      }
      
      setSelectedPoints(newSelection);
      setIsBoxSelecting(false);
      setBoxStart(null);
      setBoxEnd(null);
    }
    
    setIsDrawing(false);
  };
  
  // Delete selected points
  const handleDeleteSelected = useCallback(() => {
    if (selectedPoints.size === 0) return;
    
    const newResource = { ...resource };
    const layer = newResource.layers[currentLayerIndex];
    
    // Group selections by path and sort in reverse order to delete from end
    const pointsByPath: Map<number, number[]> = new Map();
    selectedPoints.forEach(key => {
      const [pathIdx, pointIdx] = key.split('-').map(Number);
      if (!pointsByPath.has(pathIdx)) {
        pointsByPath.set(pathIdx, []);
      }
      pointsByPath.get(pathIdx)!.push(pointIdx);
    });
    
    // Delete points in reverse order to maintain indices
    pointsByPath.forEach((pointIndices, pathIdx) => {
      pointIndices.sort((a, b) => b - a); // Sort descending
      pointIndices.forEach(pointIdx => {
        layer.paths[pathIdx].points.splice(pointIdx, 1);
      });
    });
    
    // Remove empty paths
    layer.paths = layer.paths.filter(p => p.points.length > 0);
    
    updateResource(newResource);
    setSelectedPoints(new Set());
    setSelectedPointIndex(-1);
    setCurrentPathIndex(-1);
  }, [resource, currentLayerIndex, selectedPoints, updateResource]);

  const handleDoubleClick = () => {
    if (currentTool === 'pen' && tempPoints.length >= 2) {
      const newPath: VecPath = {
        name: `path_${Date.now()}`,
        intensity: 127,
        closed: false,
        points: [...tempPoints],
      };

      const newResource = { ...resource };
      newResource.layers[currentLayerIndex].paths.push(newPath);
      updateResource(newResource);
      setTempPoints([]);
      setCurrentPathIndex(newResource.layers[currentLayerIndex].paths.length - 1);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      setTempPoints([]);
      setSelectedPointIndex(-1);
      setSelectedPoints(new Set());
      setIsBoxSelecting(false);
      setBoxStart(null);
      setBoxEnd(null);
    } else if (e.key === 'Delete' || e.key === 'Backspace') {
      if (selectedPoints.size > 0) {
        handleDeleteSelected();
      } else if (selectedPointIndex >= 0 && currentPathIndex >= 0) {
        const newResource = { ...resource };
        newResource.layers[currentLayerIndex].paths[currentPathIndex].points.splice(selectedPointIndex, 1);
        updateResource(newResource);
        setSelectedPointIndex(-1);
      }
    }
  };

  // UI Components
  const Toolbar = () => (
    <div style={{ display: 'flex', gap: '4px', marginBottom: '8px', padding: '4px', background: '#2a2a4e', borderRadius: '4px', flexWrap: 'wrap', alignItems: 'center' }}>
      <button
        onClick={() => setCurrentTool('select')}
        style={{
          padding: '8px 12px',
          background: currentTool === 'select' ? '#4a4a8e' : '#3a3a5e',
          color: 'white',
          border: 'none',
          borderRadius: '4px',
          cursor: 'pointer',
        }}
        title="Select tool - click to select, drag to box select"
      >
        ⬚ Select
      </button>
      <button
        onClick={() => setCurrentTool('pen')}
        style={{
          padding: '8px 12px',
          background: currentTool === 'pen' ? '#4a4a8e' : '#3a3a5e',
          color: 'white',
          border: 'none',
          borderRadius: '4px',
          cursor: 'pointer',
        }}
        title="Pen tool - click to add points, double-click to finish path"
      >
        ✏️ Pen
      </button>
      
      <div style={{ width: '1px', background: '#4a4a6e', margin: '0 8px' }} />
      
      {/* Delete button */}
      <button
        onClick={handleDeleteSelected}
        disabled={selectedPoints.size === 0 && selectedPointIndex < 0}
        style={{ 
          padding: '8px 12px', 
          background: (selectedPoints.size > 0 || selectedPointIndex >= 0) ? '#8a3a3e' : '#4a4a5e', 
          color: 'white', 
          border: 'none', 
          borderRadius: '4px', 
          cursor: (selectedPoints.size > 0 || selectedPointIndex >= 0) ? 'pointer' : 'not-allowed',
          opacity: (selectedPoints.size > 0 || selectedPointIndex >= 0) ? 1 : 0.5,
        }}
        title="Delete selected points (Delete key)"
      >
        🗑️ Delete {selectedPoints.size > 0 ? `(${selectedPoints.size})` : ''}
      </button>
      
      <div style={{ width: '1px', background: '#4a4a6e', margin: '0 8px' }} />
      
      <button
        onClick={() => fileInputRef.current?.click()}
        style={{ padding: '8px 12px', background: '#3a5a3e', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
      >
        📷 Load Image
      </button>
      <input
        ref={fileInputRef}
        type="file"
        accept="image/*"
        onChange={handleImageUpload}
        style={{ display: 'none' }}
      />
      
      {backgroundImage && (
        <>
          <button
            onClick={() => setShowBackground(!showBackground)}
            style={{
              padding: '8px 12px',
              background: showBackground ? '#4a4a8e' : '#3a3a5e',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
            }}
          >
            {showBackground ? '👁 Hide' : '👁 Show'}
          </button>
          <button
            onClick={handleAutoDetect}
            disabled={isProcessing}
            style={{
              padding: '8px 12px',
              background: isProcessing ? '#666' : '#5a3a8e',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: isProcessing ? 'wait' : 'pointer',
            }}
          >
            {isProcessing ? '⏳ Processing...' : '✨ Auto-Trace'}
          </button>
          <button
            onClick={() => setShowEdgeSettings(!showEdgeSettings)}
            style={{
              padding: '8px 12px',
              background: showEdgeSettings ? '#4a4a8e' : '#3a3a5e',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
            }}
          >
            ⚙️
          </button>
        </>
      )}
      
      <div style={{ flex: 1 }} />
      
      <button
        onClick={() => setZoom(z => Math.min(z * 1.2, 4))}
        style={{ padding: '8px 12px', background: '#3a3a5e', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
      >
        +
      </button>
      <button
        onClick={() => setZoom(z => Math.max(z / 1.2, 0.25))}
        style={{ padding: '8px 12px', background: '#3a3a5e', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
      >
        -
      </button>
      <span style={{ color: '#aaa', padding: '8px' }}>{Math.round(zoom * 100)}%</span>
    </div>
  );

  const EdgeSettingsPanel = () => {
    const previewPointCount = previewPaths.reduce((sum, p) => sum + p.points.length, 0);
    
    return showEdgeSettings && backgroundImage ? (
      <div style={{ background: '#2a2a4e', padding: '10px', borderRadius: '4px' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
          <div style={{ color: '#aaa', fontSize: '11px', fontWeight: 'bold' }}>Auto-Trace</div>
          <label style={{ display: 'flex', alignItems: 'center', gap: '4px', color: '#888', fontSize: '10px', cursor: 'pointer' }}>
            <input
              type="checkbox"
              checked={showPreview}
              onChange={(e) => setShowPreview(e.target.checked)}
              style={{ margin: 0 }}
            />
            Preview
          </label>
        </div>
        
        {/* Preview stats */}
        {showPreview && previewPaths.length > 0 && (
          <div style={{ 
            background: '#3a3a5e', 
            padding: '8px', 
            borderRadius: '4px', 
            marginBottom: '12px',
            color: '#ff88ff',
            fontSize: '11px',
            display: 'flex',
            justifyContent: 'space-between'
          }}>
            <span>📊 {previewPaths.length} paths</span>
            <span>{previewPointCount} points</span>
          </div>
        )}
        
        <div style={{ marginBottom: '8px' }}>
          <label style={{ color: '#888', fontSize: '10px', display: 'block', marginBottom: '2px' }}>
            Opacity: {Math.round(backgroundOpacity * 100)}%
          </label>
          <input
            type="range"
            min="0"
            max="100"
            step="5"
            value={backgroundOpacity * 100}
            onChange={(e) => setBackgroundOpacity(parseInt(e.target.value) / 100)}
            style={{ width: '100%', cursor: 'pointer', height: '16px' }}
          />
        </div>
        
        <div style={{ marginBottom: '8px' }}>
          <label style={{ color: '#888', fontSize: '10px', display: 'block', marginBottom: '2px' }}>
            Low: {edgeOptions.lowThreshold}
          </label>
          <input
            type="range"
            min="5"
            max="100"
            step="5"
            value={edgeOptions.lowThreshold}
            onChange={(e) => setEdgeOptions({ ...edgeOptions, lowThreshold: parseInt(e.target.value) })}
            style={{ width: '100%', cursor: 'pointer', height: '16px' }}
          />
        </div>
        
        <div style={{ marginBottom: '8px' }}>
          <label style={{ color: '#888', fontSize: '10px', display: 'block', marginBottom: '2px' }}>
            High: {edgeOptions.highThreshold}
          </label>
          <input
            type="range"
            min="20"
            max="200"
            step="5"
            value={edgeOptions.highThreshold}
            onChange={(e) => setEdgeOptions({ ...edgeOptions, highThreshold: parseInt(e.target.value) })}
            style={{ width: '100%', cursor: 'pointer', height: '16px' }}
          />
        </div>
        
        <div style={{ marginBottom: '8px' }}>
          <label style={{ color: '#888', fontSize: '10px', display: 'block', marginBottom: '2px' }}>
            Simplify: {edgeOptions.simplifyTolerance}
          </label>
          <input
            type="range"
            min="1"
            max="20"
            step="1"
            value={edgeOptions.simplifyTolerance}
            onChange={(e) => setEdgeOptions({ ...edgeOptions, simplifyTolerance: parseInt(e.target.value) })}
            style={{ width: '100%', cursor: 'pointer', height: '16px' }}
          />
        </div>
        
        <div style={{ marginBottom: '8px' }}>
          <label style={{ color: '#888', fontSize: '10px', display: 'block', marginBottom: '2px' }}>
            Min Length: {edgeOptions.minPathLength}
          </label>
          <input
            type="range"
            min="2"
            max="30"
            step="2"
            value={edgeOptions.minPathLength}
            onChange={(e) => setEdgeOptions({ ...edgeOptions, minPathLength: parseInt(e.target.value) })}
            style={{ width: '100%', cursor: 'pointer', height: '16px' }}
          />
        </div>
        
        {/* Blur toggle */}
        <div style={{ marginBottom: '8px' }}>
          <label style={{ display: 'flex', alignItems: 'center', gap: '6px', color: '#888', fontSize: '10px', cursor: 'pointer' }}>
            <input
              type="checkbox"
              checked={edgeOptions.useBlur}
              onChange={(e) => setEdgeOptions({ ...edgeOptions, useBlur: e.target.checked })}
              style={{ margin: 0 }}
            />
            Blur (photos)
          </label>
        </div>
        
        {/* Buttons row */}
        <div style={{ display: 'flex', gap: '6px' }}>
          {/* Reset button */}
          <button
            onClick={() => setEdgeOptions(defaultEdgeOptions)}
            style={{
              flex: 1,
              padding: '6px',
              background: '#5a4a3e',
              color: '#fa8',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
              fontSize: '10px',
            }}
            title="Reset to default values"
          >
            ↺ Reset
          </button>
          
          {/* Apply button */}
          <button
            onClick={handleAutoDetect}
            disabled={isProcessing || previewPaths.length === 0}
            style={{
              flex: 2,
              padding: '6px',
              background: previewPaths.length > 0 ? '#4a8a4e' : '#4a4a5e',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: previewPaths.length > 0 ? 'pointer' : 'not-allowed',
              fontSize: '10px',
              fontWeight: 'bold',
            }}
          >
            {isProcessing ? '⏳...' : `✓ (${previewPaths.length})`}
          </button>
        </div>
      </div>
    ) : null;
  };

  const LayersPanel = () => {
    // Only show the main drawing layer, background is handled separately
    const mainLayer = resource.layers[0];
    const pathCount = mainLayer?.paths.length || 0;
    const pointCount = mainLayer?.paths.reduce((sum, p) => sum + p.points.length, 0) || 0;
    
    return (
      <div style={{ background: '#2a2a4e', padding: '8px', borderRadius: '4px' }}>
        <div style={{ color: '#aaa', marginBottom: '8px', fontSize: '12px', fontWeight: 'bold' }}>Layers</div>
        
        {/* Background image layer - only shown when image is loaded */}
        {backgroundImage && (
          <div style={{
            padding: '6px 8px',
            background: '#3a4a3e',
            color: '#8f8',
            borderRadius: '4px',
            marginBottom: '8px',
            fontSize: '11px',
          }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
              <input
                type="checkbox"
                checked={showBackground}
                onChange={(e) => setShowBackground(e.target.checked)}
                style={{ margin: 0 }}
              />
              <span>📷 Background</span>
              <button
                onClick={() => {
                  if (window.confirm('Remove background image?')) {
                    setBackgroundImage(null);
                    setShowBackground(false);
                    setShowEdgeSettings(false);
                    // Remove from resource
                    const newResource = { ...resource };
                    delete newResource.backgroundImage;
                    updateResource(newResource);
                  }
                }}
                style={{
                  marginLeft: 'auto',
                  background: 'transparent',
                  border: 'none',
                  color: '#a66',
                  cursor: 'pointer',
                  fontSize: '12px',
                  padding: '2px 4px',
                }}
                title="Remove background image"
              >
                ✕
              </button>
            </div>
          </div>
        )}
        
        {/* Main drawing layer - always visible and active */}
        <div style={{
          padding: '6px 8px',
          background: '#4a4a8e',
          color: 'white',
          borderRadius: '4px',
          fontSize: '12px',
        }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '4px' }}>
            <span>✏️ Drawing</span>
            <span style={{ marginLeft: 'auto', color: '#aaa', fontSize: '10px' }}>active</span>
          </div>
          <div style={{ color: '#888', fontSize: '10px' }}>
            {pathCount} path{pathCount !== 1 ? 's' : ''} · {pointCount} point{pointCount !== 1 ? 's' : ''}
          </div>
        </div>
        
        {/* Selection info */}
        {selectedPoints.size > 0 && (
          <div style={{
            marginTop: '8px',
            padding: '6px 8px',
            background: '#5a3a3e',
            color: '#faa',
            borderRadius: '4px',
            fontSize: '11px',
          }}>
            {selectedPoints.size} point{selectedPoints.size !== 1 ? 's' : ''} selected
          </div>
        )}
      </div>
    );
  };

  // Right side panel - combines layers and edge settings
  const RightPanel = () => (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', width: '200px' }}>
      <LayersPanel />
      <EdgeSettingsPanel />
    </div>
  );

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
      <Toolbar />
      <div style={{ display: 'flex', gap: '8px' }}>
        <canvas
          ref={canvasRef}
          width={width}
          height={height}
          tabIndex={0}
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onDoubleClick={handleDoubleClick}
          onKeyDown={handleKeyDown}
          style={{
            border: '2px solid #4a4a8e',
            borderRadius: '4px',
            cursor: currentTool === 'pen' ? 'crosshair' : 'default',
          }}
        />
        <RightPanel />
      </div>
      <div style={{ color: '#888', fontSize: '12px' }}>
        {currentTool === 'pen' && 'Click to add points. Double-click to finish path.'}
        {currentTool === 'select' && 'Click to select points. Drag to move.'}
        {backgroundImage && ' | 📷 Background image loaded - use Auto-Trace to detect edges.'}
      </div>
    </div>
  );
};

export default VectorEditor;
