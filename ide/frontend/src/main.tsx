import React, { useEffect } from 'react';
import { createRoot } from 'react-dom/client';
import './i18n';
import { useTranslation } from 'react-i18next';
import { useEditorStore } from './state/editorStore';
import { MonacoEditorWrapper } from './components/MonacoEditorWrapper';

function App() {
  const { t, i18n } = useTranslation(['common']);
  const { documents, openDocument } = useEditorStore(s => ({
    documents: s.documents,
    openDocument: s.openDocument
  }));

  // Open a demo document once
  useEffect(() => {
    if (documents.length === 0) {
      openDocument({
        uri: 'inmemory://demo.vpy',
        language: 'vpy',
        content: '# demo vpy file\nPLOT 10,10\nLINE 0,0, 100,100\n',
        dirty: false,
        diagnostics: []
      });
    }
  }, [documents.length, openDocument]);

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100vh', fontFamily:'sans-serif'}}>
      <header style={{padding:'4px 8px', background:'#222', color:'#eee', display:'flex', gap:8, alignItems:'center'}}>
        <strong>{t('app.title')}</strong>
        <div style={{marginLeft:'auto'}}>
          <select value={i18n.language} onChange={e=>i18n.changeLanguage(e.target.value)}> 
            <option value='en'>{t('lang.english')}</option>
            <option value='es'>{t('lang.spanish')}</option>
          </select>
        </div>
      </header>
      <div style={{flex:1, display:'flex'}}>
        <aside style={{width:240, borderRight:'1px solid #444', padding:8}}>{t('menu.file')} / tree (placeholder)</aside>
        <main style={{flex:1, display:'flex', flexDirection:'column'}}>
          <div style={{borderBottom:'1px solid #444', padding:'4px 8px'}}>Tabs (placeholder)</div>
          <div style={{flex:1, position:'relative'}}>
            <MonacoEditorWrapper />
          </div>
        </main>
        <section style={{width:280, borderLeft:'1px solid #444', padding:8}}>{t('panel.emulator')} placeholder</section>
      </div>
      <footer style={{height:160, borderTop:'1px solid #444', padding:8}}>{t('panel.debug')} panel placeholder</footer>
    </div>
  );
}

const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(<App />);
}
