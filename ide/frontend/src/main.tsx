import React, { useEffect, useRef, useState, useCallback } from 'react';
import { createRoot } from 'react-dom/client';
import './i18n';
import './global.css';
import { useTranslation } from 'react-i18next';
// (import eliminado duplicado) 
import { initLsp, lspClient } from './lspClient';
import { DockWorkspace } from './components/DockWorkspace';
import { restoreEditorState, ensureEditorPersistence } from './state/editorPersistence';
import { deriveBinaryName } from './utils';
import { toggleComponent, resetLayout, ensureComponent } from './state/dockBus';
import { useEditorStore } from './state/editorStore';

function App() {
  const { t, i18n } = useTranslation(['common']);
  // IMPORTANT: Avoid grouping multiple fields into a new object each render (React 19 strict external store snapshot loop)
  const documents = useEditorStore(s => s.documents);
  const openDocument = useEditorStore(s => s.openDocument);
  const allDiagnostics = useEditorStore(s => s.allDiagnostics);
  const setDiagnostics = useEditorStore(s => s.setDiagnostics);

  const initializedRef = useRef(false);

  // Optional auto-open demo disabled: show Welcome when no docs. Uncomment block below if you want the sample on fresh start.
  /*useEffect(() => {
    if (documents.length === 0 && process.env.VPY_AUTO_DEMO === '1') {
      const content = '...'; // trimmed for disabled path
      openDocument({ uri: 'inmemory://demo.vpy', language: 'vpy', content, dirty:false, diagnostics: [] });
      const w: any = typeof window !== 'undefined' ? window : undefined;
      const isElectron = !!(w && w.electronAPI);
      if (isElectron) { initLsp(i18n.language || 'en', 'inmemory://demo.vpy', content).catch(e=>console.error(e)); }
    }
  }, [documents.length, openDocument, i18n.language]);*/

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
    { key: 'files', label: t('panel.files'), component: 'files' },
    { key: 'emulator', label: t('panel.emulator'), component: 'emulator' },
    { key: 'debug', label: t('panel.debug'), component: 'debug' },
    { key: 'errors', label: t('panel.errors'), component: 'errors', badge: (errCount+warnCount>0) ? (errCount>0? `${errCount}E` : `${warnCount}W`) : undefined },
  ];

  // Detect visibility via flexlayout model only
  const isComponentVisible = (comp: string) => {
    const model: any = (window as any).__vpyDockModel;
    if (!model) return false;
    let found = false;
    model.visitNodes((n: any) => {
      try {
        if (typeof n.getComponent === 'function') {
          if (n.getComponent() === comp) found = true;
        } else if (n?._attributes?.component === comp) {
          found = true;
        }
      } catch {}
    });
    return found;
  };

  const toggleFromView = (compKey: string) => {
    if (compKey === 'editor') { setOpenMenu(null); return; }
    // Real toggle: hide if present (state captured in DockWorkspace), restore if absent
    toggleComponent(compKey as any);
    setOpenMenu(null);
  };

  // --- Command / Action layer (placeholder implementations) ---
  const activeDoc = documents.find(d => d.uri === useEditorStore.getState().active);
  const activeUri = activeDoc?.uri;
  const activeBinName = activeUri ? deriveBinaryName(activeUri) : 'output.bin';

  const commandExec = useCallback((id: string) => {
    const apiFiles: any = (window as any).files;
    switch (id) {
      case 'file.new': {
        const idx = documents.filter(d => d.uri.startsWith('inmemory://untitled')).length + 1;
        const uri = `inmemory://untitled${idx}.vpy`;
        openDocument({ uri, language: 'vpy', content: '', dirty: false, diagnostics: [] });
        // If LSP not initialized yet, defer; the init effect will pick it up. If initialized, send didOpen.
        try {
          if ((window as any)._lspInit) {
            lspClient.didOpen(uri, 'vpy', '');
          }
        } catch {}
        break; }
      case 'file.open': {
        if (!apiFiles?.openFile) { console.warn('files API missing'); break; }
        apiFiles.openFile().then((res: any) => {
          if (!res || res.error) return;
            const { path, content, mtime } = res;
            const normPath = path.replace(/\\/g,'/');
            // Ensure triple-slash file URI + uppercase drive letter normalized the same way Monaco does (file:///C:/...)
            const uri = normPath.match(/^[A-Za-z]:\//) ? `file:///${normPath}` : `file://${normPath}`;
            openDocument({ uri, language: 'vpy', content, dirty: false, diagnostics: [], diskPath: path, mtime, lastSavedContent: content });
            // If already initialized, notify didOpen immediately; else init effect will do first doc.
            try { if ((window as any)._lspInit) { lspClient.didOpen(uri, 'vpy', content); } } catch {}
        });
        break; }
      case 'file.save': {
        const st = useEditorStore.getState();
        const active = st.documents.find(d => d.uri === st.active);
        if (!active) break;
        const path = active.diskPath;
        const content = active.content;
        if (!apiFiles?.saveFile || !path) { // fallback to Save As if no diskPath
          commandExec('file.saveAs');
          break;
        }
        apiFiles.saveFile({ path, content, expectedMTime: active.mtime }).then((res: any) => {
          if (!res) return;
          if (res.conflict) {
            // Simple strategy: prompt reload (placeholder)
            console.warn('Save conflict, reload strategy not implemented yet', res);
            return;
          }
          if (res.error) { console.error('Save error', res.error); return; }
          useEditorStore.getState().markSaved(active.uri, res.mtime);
        });
        break; }
      case 'file.saveAs': {
        const st = useEditorStore.getState();
        const active = st.documents.find(d => d.uri === st.active);
        if (!active) break;
        if (!apiFiles?.saveFileAs) break;
        apiFiles.saveFileAs({ suggestedName: active.diskPath ? undefined : 'untitled.vpy', content: active.content }).then((res: any) => {
          if (!res || res.canceled || res.error) return;
          const { path, mtime, name } = res;
          const normPath = path.replace(/\\/g,'/');
          const uri = normPath.match(/^[A-Za-z]:\//) ? `file:///${normPath}` : `file://${normPath}`;
          // Replace existing doc entry
          useEditorStore.setState((s) => ({
            documents: s.documents.map(d => d.uri === active.uri ? { ...d, uri, diskPath: path, mtime, lastSavedContent: d.content, dirty: false } : d),
            active: uri
          }));
        });
        break; }
      case 'file.close': {
        const st = useEditorStore.getState();
        if (st.active) st.closeDocument(st.active);
        break; }
      case 'build.build':
        console.log('[command] build (stub) ->', activeBinName);
        break;
      case 'build.run':
        console.log('[command] build & run (stub) ->', activeBinName);
        break;
      case 'build.clean':
        console.log('[command] clean build artifacts (stub)');
        break;
      case 'debug.start':
        console.log('[command] start debug (stub)');
        break;
      case 'debug.stop':
        console.log('[command] stop debug (stub)');
        break;
      case 'debug.stepOver':
        console.log('[command] step over (stub)');
        break;
      case 'debug.stepInto':
        console.log('[command] step into (stub)');
        break;
      case 'debug.stepOut':
        console.log('[command] step out (stub)');
        break;
      case 'debug.toggleBreakpoint':
        console.log('[command] toggle breakpoint (stub)');
        break;
      default:
        console.warn('[command] unknown', id);
    }
  }, [documents, openDocument, activeBinName]);

  // Keyboard shortcuts mapping (similar to VS conventions)
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const ctrl = e.ctrlKey || e.metaKey; // meta for mac future
      // File
      if (ctrl && e.key.toLowerCase() === 's' && !e.shiftKey) { e.preventDefault(); commandExec('file.save'); }
      else if (ctrl && e.key.toLowerCase() === 's' && e.shiftKey) { e.preventDefault(); commandExec('file.saveAs'); }
      else if (ctrl && e.key.toLowerCase() === 'o') { e.preventDefault(); commandExec('file.open'); }
      else if (ctrl && e.key.toLowerCase() === 'n') { e.preventDefault(); commandExec('file.new'); }
      // Build / Run
      else if (e.key === 'F7') { e.preventDefault(); commandExec('build.build'); }
      else if (e.key === 'F5' && !ctrl) { e.preventDefault(); commandExec('build.run'); }
      // Debug
      else if (ctrl && e.key === 'F5') { e.preventDefault(); commandExec('debug.start'); }
      else if (e.key === 'F9') { e.preventDefault(); commandExec('debug.toggleBreakpoint'); }
      else if (e.key === 'F10') { e.preventDefault(); commandExec('debug.stepOver'); }
      else if (e.key === 'F11' && !e.shiftKey) { e.preventDefault(); commandExec('debug.stepInto'); }
      else if (e.key === 'F11' && e.shiftKey) { e.preventDefault(); commandExec('debug.stepOut'); }
      else if (e.key === 'F5' && e.shiftKey) { e.preventDefault(); commandExec('debug.stop'); }
    };
    window.addEventListener('keydown', handler, { capture: true });
    return () => window.removeEventListener('keydown', handler, { capture: true } as any);
  }, [commandExec]);

  // Auto-initialize LSP once when first document becomes available (or language changes with no init yet)
  useEffect(() => {
    if (!(window as any).electronAPI) return; // no backend in web build
    if ((window as any)._lspInit) return;
    if (documents.length === 0) return;
    const first = documents[0];
    (async () => {
      try {
        await initLsp(i18n.language || 'en', first.uri, first.content);
        (window as any)._lspInit = true;
      } catch (e) { console.error('[LSP] init failed', e); }
    })();
  }, [documents.length, i18n.language]);

  return (
    <div style={{display:'flex', flexDirection:'column', height:'100vh', fontFamily:'sans-serif'}}>
      <header style={{padding:'2px 8px', background:'#222', color:'#eee', display:'flex', alignItems:'stretch', userSelect:'none'}}
        onMouseLeave={()=>setOpenMenu(null)}>
        <div style={{display:'flex', gap:0}}>
          {/* File menu */}
          <MenuRoot label={t('menu.file')} open={openMenu==='file'} setOpen={()=>setOpenMenu(openMenu==='file'?null:'file')}>
            <MenuItem label={`${t('file.new', 'New')}	Ctrl+N`} onClick={()=>{ commandExec('file.new'); setOpenMenu(null); }} />
            <MenuItem label={`${t('file.open', 'Open...')}	Ctrl+O`} onClick={()=>{ commandExec('file.open'); setOpenMenu(null); }} />
            <MenuSeparator />
            <MenuItem label={activeDoc?.dirty? `${t('file.save', 'Save')} *	Ctrl+S` : `${t('file.save', 'Save')}	Ctrl+S`} disabled={!activeDoc} onClick={()=>{ commandExec('file.save'); setOpenMenu(null); }} />
            <MenuItem label={`${t('file.saveAs', 'Save As...')}	Ctrl+Shift+S`} disabled={!activeDoc} onClick={()=>{ commandExec('file.saveAs'); setOpenMenu(null); }} />
            <MenuItem label={t('file.close', 'Close File')} disabled={!activeDoc} onClick={()=>{ commandExec('file.close'); setOpenMenu(null); }} />
            <MenuSeparator />
            <MenuItem label={t('file.recent.placeholder', 'Recent Files (coming soon)')} disabled />
            {/* Future: dynamically inject recent file entries here using recents.load() */}
            <MenuSeparator />
            <MenuItem label={t('layout.reset', 'Reset Layout')} onClick={()=>{ resetLayout(); setOpenMenu(null); }} />
            <MenuSeparator />
            <MenuItem label={t('app.exit', 'Exit')} onClick={()=>{ window.close(); }} />
          </MenuRoot>
          {/* Edit menu */}
          <MenuRoot label={t('menu.edit')} open={openMenu==='edit'} setOpen={()=>setOpenMenu(openMenu==='edit'?null:'edit')}>
            <MenuItem label={`${t('edit.undo', 'Undo')}	Ctrl+Z`} disabled />
            <MenuItem label={`${t('edit.redo', 'Redo')}	Ctrl+Y`} disabled />
            <MenuSeparator />
            <MenuItem label={`${t('edit.cut', 'Cut')}	Ctrl+X`} disabled />
            <MenuItem label={`${t('edit.copy', 'Copy')}	Ctrl+C`} disabled />
            <MenuItem label={`${t('edit.paste', 'Paste')}	Ctrl+V`} disabled />
            <MenuSeparator />
            <MenuItem label={`${t('edit.selectAll', 'Select All')}	Ctrl+A`} disabled />
            <MenuSeparator />
            <MenuItem label={`${t('edit.toggleComment', 'Toggle Comment')}	Ctrl+/`} disabled />
            <MenuItem label={`${t('edit.format', 'Format Document')}	Shift+Alt+F`} disabled />
          </MenuRoot>
          {/* Build menu */}
          <MenuRoot label={t('menu.build', 'Build')} open={openMenu==='build'} setOpen={()=>setOpenMenu(openMenu==='build'?null:'build')}>
            <MenuItem label={`${t('build.build', 'Build')}	F7`} onClick={()=>{ commandExec('build.build'); setOpenMenu(null); }} />
            <MenuItem label={`${t('build.buildAndRun', 'Build && Run')}	F5`} onClick={()=>{ commandExec('build.run'); setOpenMenu(null); }} />
            <MenuItem label={t('build.clean', 'Clean')} onClick={()=>{ commandExec('build.clean'); setOpenMenu(null); }} />
            <MenuSeparator />
            <MenuItem label={`${t('build.targetBinary', 'Target Binary')}: ${activeBinName}`} disabled />
          </MenuRoot>
          {/* Debug menu */}
            <MenuRoot label={t('menu.debug', 'Debug')} open={openMenu==='debug'} setOpen={()=>setOpenMenu(openMenu==='debug'?null:'debug')}>
              <MenuItem label={`${t('debug.start', 'Start Debugging')}	Ctrl+F5`} onClick={()=>{ commandExec('debug.start'); setOpenMenu(null); }} />
              <MenuItem label={`${t('debug.stop', 'Stop Debugging')}	Shift+F5`} onClick={()=>{ commandExec('debug.stop'); setOpenMenu(null); }} />
              <MenuSeparator />
              <MenuItem label={`${t('debug.stepOver', 'Step Over')}	F10`} onClick={()=>{ commandExec('debug.stepOver'); setOpenMenu(null); }} />
              <MenuItem label={`${t('debug.stepInto', 'Step Into')}	F11`} onClick={()=>{ commandExec('debug.stepInto'); setOpenMenu(null); }} />
              <MenuItem label={`${t('debug.stepOut', 'Step Out')}	Shift+F11`} onClick={()=>{ commandExec('debug.stepOut'); setOpenMenu(null); }} />
              <MenuSeparator />
              <MenuItem label={`${t('debug.toggleBreakpoint', 'Toggle Breakpoint')}	F9`} onClick={()=>{ commandExec('debug.toggleBreakpoint'); setOpenMenu(null); }} />
            </MenuRoot>
          {/* View menu */}
          <MenuRoot label={t('menu.view')} open={openMenu==='view'} setOpen={()=>setOpenMenu(openMenu==='view'?null:'view')}>
            {viewItems.map(it => (
              <MenuCheckItem key={it.key}
                label={it.label}
                badge={it.badge}
                checked={isComponentVisible(it.key)}
                onClick={()=>toggleFromView(it.key)} />
            ))}
            <MenuSeparator />
            <MenuItem label={t('panel.hideActive', 'Hide Active Panel')} onClick={()=>{
              // Determine active panel by scanning selected tab that matches our panels
              const mdl: any = (window as any).__vpyDockModel; let activeComp: string | undefined;
              try {
                mdl.visitNodes((n:any) => {
                  if (activeComp) return;
                  if (n.getType && n.getType()==='tabset') {
                    const selected = n.getSelectedNode?.();
                    if (selected) {
                      const c = typeof selected.getComponent === 'function' ? selected.getComponent() : selected?._attributes?.component;
                      if (['files','emulator','debug','errors'].includes(c)) activeComp = c;
                    }
                  }
                });
              } catch {}
              if (activeComp) { toggleComponent(activeComp as any); }
              setOpenMenu(null);
            }} />
            <MenuItem label={t('panel.togglePinActive', 'Pin/Unpin Active Panel')} onClick={()=>{
              const pnlRef: any = (window as any).__pinnedPanelsRef; const mdl: any = (window as any).__vpyDockModel; let activeComp: string | undefined;
              try {
                mdl.visitNodes((n:any) => {
                  if (activeComp) return;
                  if (n.getType && n.getType()==='tabset') {
                    const selected = n.getSelectedNode?.();
                    if (selected) {
                      const c = typeof selected.getComponent === 'function' ? selected.getComponent() : selected?._attributes?.component;
                      if (['files','emulator','debug','errors'].includes(c)) activeComp = c;
                    }
                  }
                });
              } catch {}
              if (activeComp && pnlRef?.current) {
                if (pnlRef.current.has(activeComp)) {
                  pnlRef.current.delete(activeComp); // unpin -> hide
                  toggleComponent(activeComp as any); // will remove
                } else {
                  pnlRef.current.add(activeComp);
                }
                try { const arr = Array.from(pnlRef.current.values()); if (arr.length) localStorage.setItem('vpy_pinned_panels_v1', JSON.stringify(arr)); else localStorage.removeItem('vpy_pinned_panels_v1'); } catch {}
              }
              setOpenMenu(null);
            }} />
            <MenuSeparator />
            <MenuItem label={t('menu.languageToggle', 'Language: EN/ES')} onClick={()=>setOpenMenu(null)} />
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

// Restore persisted editor state before first render
try { restoreEditorState(); } catch (e) { console.warn('restore failed', e); }
// Start persistence subscription
try { ensureEditorPersistence(); } catch (e) { console.warn('persist init failed', e); }

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
