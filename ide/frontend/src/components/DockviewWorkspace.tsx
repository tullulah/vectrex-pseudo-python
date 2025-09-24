import React, { useCallback, useEffect, useRef, useState } from 'react';
import 'dockview/dist/styles/dockview.css';
import { FileTreePanel } from './panels/FileTreePanel';
import { EditorPanel } from './panels/EditorPanel';
import { EmulatorPanel } from './panels/EmulatorPanel';
import { DebugPanel } from './panels/DebugPanel';
import { ErrorsPanel } from './panels/ErrorsPanel';
import { useEditorStore } from '../state/editorStore';
// Scaffold for Dockview-based layout (experimental). Install dependency 'dockview' before enabling.
// This file allows running the IDE with a query param ?layout=dockview without breaking existing flexlayout usage.
// Pending: actual dockview import once dependency added: import { DockviewReact, DockviewReadyEvent, PanelApi } from 'dockview';
// For now we type minimal placeholders to avoid TS errors until the library is installed.

// ---- Temporary type shims (remove after installing dockview) ----
// eslint-disable-next-line @typescript-eslint/no-unused-vars
interface DockviewReadyEvent { api: any; }

const STORAGE_KEY = 'vpy_dockview_layout_v1';

export const DockviewWorkspace: React.FC = () => {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const [ready, setReady] = useState(false);
  const apiRef = useRef<any>(null);
  const { documents } = useEditorStore(s=>({ documents: (s as any).documents }));

  const persistLayout = useCallback(() => {
    if (!apiRef.current) return;
    try {
      const json = apiRef.current.toJSON?.();
      if (json) localStorage.setItem(STORAGE_KEY, JSON.stringify(json));
    } catch(e){ console.warn('[Dockview] persist failed', e); }
  }, []);

  const restoreLayout = useCallback((api:any) => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        api.fromJSON?.(JSON.parse(stored));
        return true;
      }
    } catch(e){ console.warn('[Dockview] restore failed', e); }
    return false;
  }, []);

  useEffect(() => {
    // Lazy dynamic import so we don't break build before adding dependency
    (async () => {
      try {
        // @ts-ignore dynamic import placeholder
  const mod = await import(/* @vite-ignore */ 'dockview');
        const DockviewReact = mod.DockviewReact;
        // Render runtime instance
        // We create a temporary div root for the DockviewReact component
        if (containerRef.current) {
          const rootEl = document.createElement('div');
          rootEl.style.position='absolute';
          rootEl.style.inset='0';
          containerRef.current.appendChild(rootEl);
          const ReactDOM = await import('react-dom');
          const r = (ReactDOM as any).createRoot(rootEl);
          r.render(<DockviewReact components={{
            files: (p:any)=> <FileTreePanel />,
            editor: (p:any)=> <EditorPanel />,
            emulator: (p:any)=> <EmulatorPanel />,
            debug: (p:any)=> <DebugPanel />,
            errors: (p:any)=> <ErrorsPanel />,
          }} onReady={(e:DockviewReadyEvent)=>{
            try {
              const api = (e as any).api;
              apiRef.current = api;
              (window as any).__vpyDockviewApi = api;
              // Try restore
              const restored = restoreLayout(api);
              if (!restored) {
                api.addPanel({ id:'editor', component:'editor', position: { direction:'left' }, title:'Editor' });
                api.addPanel({ id:'files', component:'files', position: { referencePanel:'editor', direction:'left' }, title:'Files' });
                api.addPanel({ id:'emulator', component:'emulator', position: { referencePanel:'editor', direction:'right' }, title:'Emulator' });
                api.addPanel({ id:'debug', component:'debug', position: { referencePanel:'editor', direction:'below' }, title:'Debug' });
                api.addPanel({ id:'errors', component:'errors', position: { referencePanel:'debug', direction:'right' }, title:'Errors' });
              }
              setReady(true);
              // Subscribe to layout changes
              api.onDidLayoutChange?.(() => persistLayout());
              api.onDidAddPanel?.(() => persistLayout());
              api.onDidRemovePanel?.(() => persistLayout());
            } catch(err) {
              console.warn('[Dockview] init failed', err);
            }
          }} />);
        }
      } catch (e) {
        console.warn('[Dockview] dynamic import failed (install dependency?)', e);
      }
    })();
  }, []);

  return <div ref={containerRef} style={{position:'absolute', inset:0}}>
    {!ready && <div style={{position:'absolute', inset:0, display:'flex', alignItems:'center', justifyContent:'center', color:'#888', fontSize:12}}>Initializing Dockview layout...</div>}
  </div>;
};
