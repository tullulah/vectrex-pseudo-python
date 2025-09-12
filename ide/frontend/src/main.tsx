import React, { useEffect, useRef } from 'react';
import { createRoot } from 'react-dom/client';
import './i18n';
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
      const content = '# demo vpy file\nPLOT 10,10\nLINE 0,0, 100,100\n';
      openDocument({
        uri: 'inmemory://demo.vpy',
        language: 'vpy',
        content,
        dirty: false,
        diagnostics: []
      });
      // Start LSP after opening demo
      (async () => {
        try {
          await initLsp(i18n.language || 'en', 'inmemory://demo.vpy', content);
          initializedRef.current = true;
        } catch (e) {
          console.error('Failed init LSP', e);
        }
      })();
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
