import React, { useEffect, useRef } from 'react';
import { createRoot } from 'react-dom/client';
import './i18n';
import './global.css';
import { useTranslation } from 'react-i18next';
import { useEditorStore } from './state/editorStore';
import { initLsp, lspClient } from './lspClient';
import { DockWorkspace } from './components/DockWorkspace';
import { toggleComponent, resetLayout } from './state/dockBus';

function App() {
  const { t, i18n } = useTranslation(['common']);
  const { documents, openDocument, updateContent } = useEditorStore(s => ({
    documents: s.documents,
    openDocument: s.openDocument,
    updateContent: s.updateContent
  }));

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
      // Start LSP only inside Tauri runtime
      if (typeof window !== 'undefined' && (window as any).__TAURI_IPC__) {
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

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100vh', fontFamily:'sans-serif'}}>
      <header style={{padding:'4px 8px', background:'#222', color:'#eee', display:'flex', gap:12, alignItems:'center'}}>
        <strong>{t('app.title')}</strong>
        <div style={{display:'flex', gap:4}}>
          <button onClick={()=>toggleComponent('files')} style={{fontSize:12}}>Files</button>
          <button onClick={()=>toggleComponent('editor')} style={{fontSize:12}}>Editor</button>
          <button onClick={()=>toggleComponent('emulator')} style={{fontSize:12}}>Emu</button>
          <button onClick={()=>toggleComponent('debug')} style={{fontSize:12}}>Debug</button>
          <button onClick={()=>resetLayout()} style={{fontSize:12}}>Reset</button>
        </div>
        <div style={{marginLeft:'auto'}}>
          <select value={i18n.language} onChange={e=>i18n.changeLanguage(e.target.value)}> 
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
