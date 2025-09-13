import React, { useEffect, useRef, useState } from 'react';
import { createRoot } from 'react-dom/client';
import './i18n';
import './global.css';
import { useTranslation } from 'react-i18next';
// (import eliminado duplicado) 
import { initLsp, lspClient } from './lspClient';
import { DockWorkspace } from './components/DockWorkspace';
import { toggleComponent, resetLayout } from './state/dockBus';
import { useEditorStore } from './state/editorStore';

function App() {
  const { t, i18n } = useTranslation(['common']);
  // IMPORTANT: Avoid grouping multiple fields into a new object each render (React 19 strict external store snapshot loop)
  const documents = useEditorStore(s => s.documents);
  const openDocument = useEditorStore(s => s.openDocument);
  const allDiagnostics = useEditorStore(s => s.allDiagnostics);
  const setDiagnostics = useEditorStore(s => s.setDiagnostics);

  const initializedRef = useRef(false);

  // Open a demo document once
  useEffect(() => {
    if (documents.length === 0) {
      const content = `META TITLE = "SHAPE COMP"\nMETA COPYRIGHT = "g GCE 1998"\nMETA MUSIC = "music1"\n\n# Demo de composición de formas vectoriales usando macros:\n#  - Hexágono central\n#  - Triángulo superior como "flecha"\n#  - Cuadrado inferior\n#  - Círculo interior y anillo exterior segmentado\n#  - Arcos laterales decorativos\n#  - Espiral tipo galaxia a la izquierda\n#  - Texto de cabecera\n# Todas las figuras usan un solo Reset0Ref por figura (optimización en macros).\n\nconst I_FULL = 0x5F\nconst I_DIM  = 0x30\nconst I_LOW  = 0x18\n\n# Coordenadas base\nconst CX = 0\nconst CY = 0\n\ndef main():\n    PRINT_TEXT(-70, 70, "VECTOR DEMO")\n\n    # Hexágono central (radio ~28)\n    DRAW_POLYGON(6, I_FULL,  28, 0,  14, 24,  -14, 24,  -28, 0,  -14, -24, 14, -24)\n\n    # Triángulo superior (punta arriba)\n    DRAW_POLYGON(3, I_FULL, 0, 55, -25, 5, 25, 5)\n\n    # Cuadrado inferior (semilado 22)\n    DRAW_POLYGON(4, I_FULL, -22, -55, 22, -55, 22, -11, -22, -11)\n\n    # Círculo interior (diámetro 40) y anillo exterior segmentado (diámetro 80)\n    DRAW_CIRCLE(0, 0, 40, I_DIM)\n    DRAW_CIRCLE_SEG(48, 0, 0, 80, I_LOW)\n\n    # Arcos laterales (derecha e izquierda) 180° cada uno\n    DRAW_ARC(32, 90, 0, 35, -90, 180, I_FULL)   # derecha\n    DRAW_ARC(32, -90, 0, 35, 90, 180, I_FULL)   # izquierda\n\n    # Espiral (galaxia) a la izquierda superior\n    DRAW_SPIRAL(96, -110, 60, 5, 55, 2, I_DIM)\n\n    # Espiral pequeña decorativa abajo derecha\n    DRAW_SPIRAL(64, 100, -60, 4, 30, 1, I_DIM)\n\n    # Círculo pequeño resaltando el centro\n    DRAW_CIRCLE_SEG(24, 0, 0, 16, I_FULL)\n`;
      openDocument({
        uri: 'inmemory://demo.vpy',
        language: 'vpy',
        content,
        dirty: false,
        diagnostics: []
      });
      // Start LSP inside Electron runtime (Tauri removed)
      const w: any = typeof window !== 'undefined' ? window : undefined;
      const isElectron = !!(w && w.electronAPI);
      if (isElectron) {
        (async () => {
          try {
            await initLsp(i18n.language || 'en', 'inmemory://demo.vpy', content);
            initializedRef.current = true;
          } catch (e) {
            console.error('Failed init LSP', e);
          }
        })();
      }
    }
  }, [documents.length, openDocument, i18n.language]);

  // (Future) Hook to send didChange; currently Monaco wrapper should call updateContent, so we can observe changes here if needed.
  // Placeholder for future optimization.

  // Global LSP diagnostics listener (independiente de MonacoEditorWrapper) para poblar pestaña Errors aunque el editor no se haya montado
  useEffect(() => {
    const handler = (method: string, params: any) => {
      if (method === 'textDocument/publishDiagnostics') {
        const { uri, diagnostics } = params || {};
        if (!uri) return;
        const mapped = (diagnostics||[]).map((d: any) => ({
          message: d.message,
          severity: (d.severity === 1 ? 'error' : d.severity === 2 ? 'warning' : 'info'),
          line: d.range?.start?.line || 0,
          column: d.range?.start?.character || 0
        }));
        try { setDiagnostics(uri, mapped as any); } catch {}
      }
    };
    lspClient.onNotification(handler);
  }, [setDiagnostics]);

  // Track which menu is open
  const [openMenu, setOpenMenu] = useState<string | null>(null);
  const diags = allDiagnostics || [];
  const errCount = diags.filter((d: any)=>d.severity==='error').length;
  const warnCount = diags.filter((d: any)=>d.severity==='warning').length;

  const viewItems: Array<{ key: string; label: string; component?: any; disabled?: boolean; badge?: string; onClick?: () => void }> = [
    { key: 'files', label: 'Files', component: 'files' },
    { key: 'emulator', label: 'Emulator', component: 'emulator' },
    { key: 'debug', label: 'Debug', component: 'debug' },
    { key: 'errors', label: 'Errors', component: 'errors', badge: (errCount+warnCount>0) ? (errCount>0? `${errCount}E` : `${warnCount}W`) : undefined },
  ];

  // Detect visibility via flexlayout model only
  const isComponentVisible = (comp: string) => {
    const model: any = (window as any).__vpyDockModel;
    if (!model) return false;
    let found = false;
    model.visitNodes((n: any) => {
      if (n?._attributes?.component === comp) found = true;
    });
    return found;
  };

  const toggleFromView = (compKey: string) => {
    if (compKey === 'editor') { setOpenMenu(null); return; }
    toggleComponent(compKey as any);
    setOpenMenu(null);
  };

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100vh', fontFamily:'sans-serif'}}>
      <header style={{padding:'2px 8px', background:'#222', color:'#eee', display:'flex', alignItems:'stretch', userSelect:'none'}}
        onMouseLeave={()=>setOpenMenu(null)}>
        <div style={{display:'flex', gap:0}}>
          {/* File menu */}
          <MenuRoot label="File" open={openMenu==='file'} setOpen={()=>setOpenMenu(openMenu==='file'?null:'file')}>
            <MenuItem label="New (placeholder)" onClick={()=>setOpenMenu(null)} />
            <MenuItem label="Open (placeholder)" onClick={()=>setOpenMenu(null)} />
            <MenuSeparator />
            <MenuItem label="Reset Layout" onClick={()=>{ resetLayout(); setOpenMenu(null); }} />
            <MenuSeparator />
            <MenuItem label="Exit" onClick={()=>{ window.close(); }} />
          </MenuRoot>
          {/* View menu */}
          <MenuRoot label="View" open={openMenu==='view'} setOpen={()=>setOpenMenu(openMenu==='view'?null:'view')}>
            {viewItems.map(it => (
              <MenuCheckItem key={it.key}
                label={it.label}
                badge={it.badge}
                checked={isComponentVisible(it.key)}
                onClick={()=>toggleFromView(it.key)} />
            ))}
            <MenuSeparator />
            <MenuItem label="Language: EN/ES" onClick={()=>setOpenMenu(null)} />
          </MenuRoot>
        </div>
        <div style={{marginLeft:'auto', display:'flex', alignItems:'center', gap:8}}>
          <select value={i18n.language} onChange={e=>i18n.changeLanguage(e.target.value)} style={{background:'#333', color:'#fff', border:'1px solid #444'}}>
            <option value='en'>{t('lang.english')}</option>
            <option value='es'>{t('lang.spanish')}</option>
          </select>
        </div>
      </header>
      <div style={{flex:1, position:'relative'}}>
        <DockWorkspace />
      </div>
    </div>
  );
}

const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(<App />);
}

// --- Menubar Components ---
interface MenuRootProps { label: string; open: boolean; setOpen: () => void; children: React.ReactNode; }
const MenuRoot: React.FC<MenuRootProps> = ({ label, open, setOpen, children }) => {
  return (
    <div style={{position:'relative'}}>
      <div onClick={setOpen} style={{padding:'4px 10px', cursor:'default', background: open? '#333':'transparent'}}>
        {label}
      </div>
      {open && (
        <div style={{position:'absolute', top:'100%', left:0, background:'#2d2d2d', border:'1px solid #444', minWidth:180, zIndex:1000, boxShadow:'0 2px 6px rgba(0,0,0,0.4)'}}>
          {children}
        </div>
      )}
    </div>
  );
};

interface MenuItemProps { label: string; onClick?: () => void; disabled?: boolean; }
const MenuItem: React.FC<MenuItemProps> = ({ label, onClick, disabled }) => (
  <div onClick={() => !disabled && onClick && onClick()} style={{
    padding:'4px 10px', fontSize:12, cursor: disabled? 'not-allowed':'default', color: disabled? '#666':'#eee', display:'flex', alignItems:'center', gap:8
  }}>{label}</div>
);

const MenuSeparator: React.FC = () => <div style={{borderTop:'1px solid #444', margin:'4px 0'}} />;

interface MenuCheckItemProps { label: string; checked?: boolean; onClick: () => void; badge?: string; }
const MenuCheckItem: React.FC<MenuCheckItemProps> = ({ label, checked, onClick, badge }) => (
  <div onClick={onClick} style={{padding:'4px 10px', fontSize:12, cursor:'default', display:'flex', alignItems:'center', gap:8}}>
    <span style={{width:14, textAlign:'center', color:'#bbb'}}>{checked ? '✓' : ''}</span>
    <span style={{flex:1}}>{label}</span>
    {badge && <span style={{background: badge.includes('E')? '#F14C4C':'#CCA700', color:'#fff', borderRadius:8, padding:'0 4px', fontSize:10}}>{badge}</span>}
  </div>
);
