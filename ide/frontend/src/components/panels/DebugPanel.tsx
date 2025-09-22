import React, { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';

// Interfaz para la información de debug de vectores
interface VectorDebugInfo {
  x0: number;
  y0: number;
  x1: number;
  y1: number;
  intensity: number;
  color: string;
  colorName: string;
}

interface DebugRenderInfo {
  lastCanvasClear: string;
  vectorsDrawn: VectorDebugInfo[];
  summary: { rojos: number; verdes: number; total: number };
}

export const DebugPanel: React.FC = () => {
  const { t } = useTranslation(['common']);
  const [debugInfo, setDebugInfo] = useState<DebugRenderInfo | null>(null);

  // Escuchar eventos de debug del renderizado desde el EmulatorPanel
  useEffect(() => {
    const handleDebugUpdate = (event: CustomEvent<DebugRenderInfo>) => {
      setDebugInfo(event.detail);
    };

    window.addEventListener('debugRenderUpdate', handleDebugUpdate as EventListener);
    return () => {
      window.removeEventListener('debugRenderUpdate', handleDebugUpdate as EventListener);
    };
  }, []);

  return (
    <div style={{padding:8, fontFamily:'monospace', fontSize:12}}>
      <strong>{t('panel.debug')}</strong>
      
      {debugInfo ? (
        <div style={{marginTop:8}}>
          <div style={{marginBottom:8, padding:4, background:'#1a1a1a', border:'1px solid #333'}}>
            <div style={{color:'#0f0', marginBottom:4}}>🧹 Canvas limpiado: {debugInfo.lastCanvasClear}</div>
            <div style={{color:'#ff0'}}>
              📊 Resumen: {debugInfo.summary.rojos} ROJOS + {debugInfo.summary.verdes} VERDES = {debugInfo.summary.total} total
            </div>
          </div>

          <div style={{color:'#ccc', marginBottom:4}}>Vectores dibujados (últimos 10):</div>
          <div style={{maxHeight:200, overflow:'auto', border:'1px solid #444', background:'#0a0a0a'}}>
            {debugInfo.vectorsDrawn.slice(-10).map((vector, index) => (
              <div key={index} style={{
                padding:'2px 4px', 
                borderBottom:'1px solid #222',
                color: vector.intensity === 0 ? '#ff6666' : '#66ff66'
              }}>
                🎨 ({vector.x0.toFixed(1)},{vector.y0.toFixed(1)}) → ({vector.x1.toFixed(1)},{vector.y1.toFixed(1)}) 
                <span style={{marginLeft:8, fontWeight:'bold'}}>
                  intensidad={vector.intensity} {vector.colorName}
                </span>
              </div>
            ))}
            {debugInfo.vectorsDrawn.length === 0 && (
              <div style={{padding:8, color:'#666', textAlign:'center'}}>No hay vectores dibujados</div>
            )}
          </div>
        </div>
      ) : (
        <div style={{marginTop:8, color:'#888'}}>Esperando información de debug...</div>
      )}
    </div>
  );
};
