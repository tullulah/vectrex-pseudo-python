import React, { useCallback } from 'react';
import { useEditorStore } from '../state/editorStore';
import { WelcomeView } from './WelcomeView';
import { MonacoEditorWrapper } from './MonacoEditorWrapper';

// Basic custom tab bar replacing flexlayout doc:* logic.
// Phase 1: single group, order = documents array order.

export const EditorSurface: React.FC = () => {
  const documents = useEditorStore(s=>s.documents);
  const active = useEditorStore(s=>s.active);
  const setActive = useEditorStore(s=>s.setActive);
  const closeDocument = useEditorStore(s=>s.closeDocument);
  const visibleDocs = documents; // all docs now visible (no hide/pin)

  const onClose = useCallback((e: React.MouseEvent, uri: string) => {
    e.stopPropagation();
    const doc = documents.find(d=>d.uri===uri);
    if (doc?.dirty) {
      if (!window.confirm('Archivo con cambios sin guardar. ¿Cerrar de todos modos?')) return;
    }
    closeDocument(uri);
  }, [documents, closeDocument]);

  return (
    <div className="vpy-editor-surface">
      <div className="vpy-tab-bar">
        {visibleDocs.map(doc => {
          const title = deriveTitle(doc.uri);
          return (
            <div key={doc.uri}
              className={"vpy-tab" + (doc.uri===active?" active":"") + (doc.dirty?" dirty":"")}
              title={doc.uri}
              onClick={()=> setActive(doc.uri)}>
              <span className="title">{title}</span>
              <button className="close" onClick={(e)=>onClose(e, doc.uri)}>×</button>
            </div>
          );
        })}
      </div>
      <div className="vpy-editor-body">
        {active ? <MonacoEditorWrapper uri={active} /> : <WelcomeView />}
      </div>
      <style>{editorSurfaceStyles}</style>
    </div>
  );
};

function deriveTitle(uri: string): string {
  if (uri.startsWith('file://')) {
    const parts = uri.split('/');
    return parts[parts.length-1] || 'file';
  }
  if (uri.startsWith('inmemory://')) {
    return uri.split('/').pop() || 'untitled';
  }
  return uri;
}

const editorSurfaceStyles = `
.vpy-editor-surface { display:flex; flex-direction:column; height:100%; width:100%; }
.vpy-tab-bar { display:flex; align-items:stretch; background:#1f1f1f; border-bottom:1px solid #333; overflow-x:auto; scrollbar-width:none; position:relative; }
.vpy-tab-bar::-webkit-scrollbar { display:none; }
.vpy-tab { position:relative; display:flex; align-items:center; gap:8px; padding:4px 12px 4px 12px; font-size:12px; cursor:pointer; color:#bbb; user-select:none; border-right:1px solid #2a2a2a; }
.vpy-tab.active { background:#2b2b2b; color:#eee; }
.vpy-tab:hover { background:#262626; }
.vpy-tab.dirty .title::after { content:'*'; color:#d8846b; margin-left:2px; }
.vpy-tab .close { background:transparent; border:none; color:#888; font-size:12px; cursor:pointer; padding:0 2px; }
.vpy-tab .close:hover { color:#fff; }
.vpy-editor-body { flex:1; position:relative; min-height:0; }
`;
