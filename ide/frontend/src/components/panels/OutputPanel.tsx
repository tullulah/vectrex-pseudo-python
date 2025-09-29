import React, { useEffect, useState, useRef } from 'react';

// Tipos simples para JSVecX
interface VecxMetrics {
  totalCycles: number;
  instructionCount: number;
  frameCount: number;
  running: boolean;
}

interface VecxRegs {
  PC: number;
  A: number; B: number;
  X: number; Y: number; U: number; S: number;
  DP: number; CC: number;
}

// Componente compacto para gráficas de barras horizontales con historial
const PerformanceChart: React.FC<{ 
  label: string; 
  data: number[];
  max: number; 
  color: string; 
  dangerZone?: number;
  unit?: string;
}> = ({ label, data, max, color, dangerZone, unit = '' }) => {
  const current = data[data.length - 1] || 0;
  const percentage = Math.min((current / max) * 100, 100);
  const isDanger = dangerZone && current >= dangerZone;
  const dangerPercentage = dangerZone ? (dangerZone / max) * 100 : 0;
  
  return (
    <div style={{ flex: 1, minWidth: '200px' }}>
      <div style={{
        fontSize: '11px',
        marginBottom: '4px',
        color: isDanger ? '#ff6666' : '#ddd',
        fontWeight: 'bold',
        textAlign: 'center'
      }}>
        {label} {isDanger ? '⚠️' : ''}: {current.toLocaleString()}{unit}
      </div>
      
      {/* Barra de progreso compacta */}
      <div style={{
        width: '100%',
        height: '16px',
        background: '#2a2a2a',
        borderRadius: '8px',
        overflow: 'hidden',
        border: '1px solid #444',
        position: 'relative',
        marginBottom: '6px'
      }}>
        {/* Zona de peligro de fondo */}
        {dangerZone && (
          <div style={{
            position: 'absolute',
            left: `${dangerPercentage}%`,
            width: `${100 - dangerPercentage}%`,
            height: '100%',
            background: 'rgba(255, 68, 68, 0.2)',
            zIndex: 1
          }} />
        )}
        
        {/* Barra de progreso principal */}
        <div style={{
          width: `${percentage}%`,
          height: '100%',
          background: isDanger ? 
            'linear-gradient(90deg, #ff4444, #ff6666)' :
            `linear-gradient(90deg, ${color}, ${color}aa)`,
          transition: 'width 0.3s ease-out',
          borderRadius: '8px',
          zIndex: 2,
          position: 'relative',
          boxShadow: isDanger ? '0 0 8px rgba(255, 68, 68, 0.6)' : `0 0 6px ${color}44`
        }} />
        
        {/* Línea marcadora de zona peligro */}
        {dangerZone && (
          <div style={{
            position: 'absolute',
            left: `${dangerPercentage}%`,
            width: '2px',
            height: '100%',
            background: '#ff4444',
            zIndex: 3,
            boxShadow: '0 0 4px #ff4444'
          }} />
        )}
      </div>
      
      {/* Mini gráfica de historial */}
      <div style={{
        display: 'flex',
        height: '24px',
        alignItems: 'end',
        background: '#1a1a1a',
        padding: '2px',
        borderRadius: '4px',
        border: '1px solid #333',
        overflow: 'hidden'
      }}>
        {data.map((value, index) => {
          const barHeight = Math.max(2, (value / max) * 20);
          const barColor = dangerZone && value >= dangerZone ? '#ff4444' : color;
          return (
            <div
              key={index}
              style={{
                flex: 1,
                minWidth: '1px',
                height: `${barHeight}px`,
                background: barColor,
                opacity: 0.6 + (index / Math.max(data.length, 1)) * 0.4,
                transition: 'height 0.2s ease-out',
                marginRight: index < data.length - 1 ? '1px' : '0'
              }}
            />
          );
        })}
      </div>
    </div>
  );
};

export const OutputPanel: React.FC = () => {
  const [metrics, setMetrics] = useState<VecxMetrics | null>(null);
  const auto = true; // Always auto-update
  const [cpuData, setCpuData] = useState<number[]>([]);
  const [ramData, setRamData] = useState<number[]>([]);
  const [vectorData, setVectorData] = useState<number[]>([]);
  const timerRef = useRef<number|null>(null);

  const fetchStats = () => {
    try {
      const vecx = (window as any).vecx;
      if (!vecx) {
        setMetrics(null);
        return;
      }
      
      const fetchedMetrics = vecx.getMetrics && vecx.getMetrics();
      setMetrics(fetchedMetrics || null);
      
      // Generar datos simulados basados en el estado del emulador
      if (fetchedMetrics && fetchedMetrics.running) {
        const currentTime = Date.now();
        
        // CPU Usage simulado (0-100%)
        const baseCpu = 45;
        const cpuVariation = Math.sin(currentTime / 3000) * 25 + Math.random() * 15;
        const newCpuUsage = Math.max(5, Math.min(95, baseCpu + cpuVariation));
        
        // RAM Usage simulado (256-800 bytes)
        const baseRam = 400;
        const ramVariation = Math.sin(currentTime / 5000) * 150 + Math.random() * 100;
        const newRamUsage = Math.max(256, Math.min(800, baseRam + ramVariation));
        
        // Vector count simulado (50-180 vectores, zona peligro en 150+)
        const baseVectors = 85;
        const vectorVariation = Math.sin(currentTime / 2000) * 40;
        const randomBurst = Math.random() > 0.9 ? Math.random() * 50 : 0; // Picos ocasionales
        const newVectorCount = Math.max(20, Math.min(180, baseVectors + vectorVariation + randomBurst));
        
        setCpuData(prev => [...prev.slice(-39), newCpuUsage]);
        setRamData(prev => [...prev.slice(-39), newRamUsage]);
        setVectorData(prev => [...prev.slice(-39), newVectorCount]);
      } else {
        // Emulador detenido
        setCpuData(prev => [...prev.slice(-39), 0]);
        setRamData(prev => [...prev.slice(-39), 256]);
        setVectorData(prev => [...prev.slice(-39), 0]);
      }
    } catch (e) {
      setMetrics(null);
    }
  };

  useEffect(() => { fetchStats(); }, []);
  useEffect(() => {
    if (auto) {
      timerRef.current = window.setInterval(fetchStats, 500); // Más frecuente para gráficas suaves
    } else if (timerRef.current) {
      clearInterval(timerRef.current); timerRef.current=null;
    }
    return () => { if (timerRef.current) clearInterval(timerRef.current); };
  }, [auto]);

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100%', fontSize:12}}>
      <div style={{padding:'8px 12px', borderBottom:'1px solid #333', display:'flex', alignItems:'center', gap:12}}>
        <span style={{marginLeft:'auto', opacity:0.7}}>
          Status: {metrics?.running ? '🟢 Running' : '🔴 Stopped'}
        </span>
      </div>
      
      <div style={{
        padding: '16px',
        flex: 1,
        overflow: 'auto'
      }}>
        {/* Tres gráficas en una sola línea horizontal */}
        <div style={{
          display: 'flex',
          gap: '16px',
          width: '100%'
        }}>
          <PerformanceChart 
            label="CPU Usage"
            data={cpuData}
            max={100}
            color="#00ff88"
            unit="%"
          />
          
          <PerformanceChart 
            label="RAM Usage"
            data={ramData}
            max={1024}
            color="#4488ff"
            unit=" bytes"
          />
          
          <PerformanceChart 
            label="Vectors per Frame"
            data={vectorData}
            max={200}
            color="#ffaa00"
            dangerZone={150} // Zona roja donde empiezan los parpadeos
            unit=" vectors"
          />
        </div>
      </div>
    </div>
  );
};

const btnStyle: React.CSSProperties = { background:'#1e1e1e', color:'#ddd', border:'1px solid #333', padding:'2px 6px', cursor:'pointer', fontSize:11 };
const th: React.CSSProperties = { padding:'4px 6px' };
const td: React.CSSProperties = { padding:'2px 6px', fontFamily:'monospace' };
function hex8(v:any){ if (typeof v!=='number') return '--'; return '0x'+(v&0xFF).toString(16).padStart(2,'0'); }
function hex16(v:any){ if (typeof v!=='number') return '--'; return '0x'+(v&0xFFFF).toString(16).padStart(4,'0'); }
