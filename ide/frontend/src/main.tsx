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
import { useProjectStore } from './state/projectStore';
import { useDebugStore } from './state/debugStore';
import { MenuRoot, MenuItem, MenuSeparator, MenuCheckItem } from './components/MenuComponents';
import { initLoggerWithRefreshDetection, logger, detectRefresh } from './utils/logger';

function App() {
  const { t, i18n } = useTranslation(['common']);
  // IMPORTANT: Avoid grouping multiple fields into a new object each render (React 19 strict external store snapshot loop)
  const documents = useEditorStore(s => s.documents);
  const openDocument = useEditorStore(s => s.openDocument);
  const allDiagnostics = useEditorStore(s => s.allDiagnostics);
  const setDiagnosticsBySource = useEditorStore(s => s.setDiagnosticsBySource);

  const initializedRef = useRef(false);

  // Optional auto-open demo disabled: show Welcome when no docs. Uncomment block below if you want the sample on fresh start.
  /*useEffect(() => {
    if (documents.length === 0 && process.env.VPY_AUTO_DEMO === '1') {
      const content = '...'; // trimmed for disabled path
      openDocument({ uri: 'inmemory://demo.vpy', language: 'vpy', content, dirty:false, diagnostics: [] });
      const w: any = typeof window !== 'undefined' ? window : undefined;
      const isElectron = !!(w && w.electronAPI);
      if (isElectron) { initLsp(i18n.language || 'en', 'inmemory://demo.vpy', content).catch(e=>logger.error('LSP', 'Init error:', e)); }
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
        
        logger.debug('LSP', `Received ${(diagnostics||[]).length} diagnostics for URI:`, uri);
        
        // Decode URI to handle URL encoding (e.g., %3A -> :)
        let decodedUri: string;
        try {
          decodedUri = decodeURIComponent(uri);
          logger.verbose('LSP', 'Decoded URI:', decodedUri);
        } catch (error) {
          logger.warn('LSP', 'Failed to decode URI, using original:', uri);
          decodedUri = uri;
        }
        
        const mapped = (diagnostics||[]).map((d: any) => ({
          message: d.message,
          severity: (d.severity === 1 ? 'error' : d.severity === 2 ? 'warning' : 'info'),
          line: d.range?.start?.line || 0,
          column: d.range?.start?.character || 0
        }));
        
        try { 
          setDiagnosticsBySource(decodedUri, 'lsp', mapped as any);
          const errorCount = mapped.filter((d: any) => d.severity === 'error').length;
          if (errorCount > 0) {
            logger.info('LSP', `Set ${errorCount} errors for ${decodedUri.split('/').pop()}`);
          }
        } catch (error) {
          logger.error('LSP', 'Error calling setDiagnosticsBySource:', error);
        }
      }
    };
    lspClient.onNotification(handler);
  }, [setDiagnosticsBySource, documents]);

  // Listen for compilation diagnostics from Electron backend (run://diagnostics)
  useEffect(() => {
    const electronAPI = (window as any).electronAPI;
    if (!electronAPI?.onRunDiagnostics) return;
    
    const handler = (diags: Array<{ file: string; line: number; col: number; message: string }>) => {
      if (diags.length > 0) {
        logger.info('Compilation', `Received ${diags.length} compilation errors`);
      }
      
      // Group diagnostics by file and convert to store format
      const diagsByFile: Record<string, any[]> = {};
      
      diags.forEach((diag) => {
        const { file, line, col, message } = diag;
        
        // Convert file path to proper URI format
        let uri = file;
        if (file && !file.startsWith('file://')) {
          const normPath = file.replace(/\\/g, '/');
          uri = normPath.match(/^[A-Za-z]:\//) ? `file:///${normPath}` : `file://${normPath}`;
        }
        
        if (!diagsByFile[uri]) {
          diagsByFile[uri] = [];
        }
        
        diagsByFile[uri].push({
          line: Math.max(0, line),
          column: Math.max(0, col),
          severity: 'error' as const,
          message: message
        });
      });
      
      // Set diagnostics for each file
      Object.entries(diagsByFile).forEach(([uri, fileDiags]) => {
        try { 
          setDiagnosticsBySource(uri, 'compiler', fileDiags as any); 
          const fileName = uri.split('/').pop() || uri;
          logger.debug('Compilation', `Set ${fileDiags.length} errors for ${fileName}`);
        } catch (e) {
          logger.error('Compilation', 'Failed to set diagnostics for', uri, e);
        }
      });
    };
    
    electronAPI.onRunDiagnostics(handler);
    
    // Cleanup function
    return () => {
      // Note: electron doesn't provide an off method, so we rely on component unmount
    };
  }, [setDiagnosticsBySource]);

  // Auto-restore last workspace on app startup
  const restoreLastWorkspace = useProjectStore(s => s.restoreLastWorkspace);
  const hasWorkspace = useProjectStore(s => s.hasWorkspace);
  
  useEffect(() => {
    if (!initializedRef.current && !hasWorkspace()) {
      logger.debug('App', 'Auto-restoring last workspace on startup');
      restoreLastWorkspace();
      initializedRef.current = true;
    }
  }, [restoreLastWorkspace, hasWorkspace]);

  // Track which menu is open
  const [openMenu, setOpenMenu] = useState<string | null>(null);
  const diags = allDiagnostics || [];
  const errCount = diags.filter((d: any)=>d.severity==='error').length;
  const warnCount = diags.filter((d: any)=>d.severity==='warning').length;

  const viewItems: Array<{ key: string; label: string; component?: any; disabled?: boolean; badge?: string; onClick?: () => void }> = [
    { key: 'files', label: t('panel.files'), component: 'files' },
    { key: 'emulator', label: t('panel.emulator'), component: 'emulator' },
    { key: 'dual-emulator', label: 'Dual Test', component: 'dual-emulator' },
    { key: 'debug', label: t('panel.debug'), component: 'debug' },
    { key: 'errors', label: t('panel.errors'), component: 'errors', badge: (errCount+warnCount>0) ? (errCount>0? `${errCount}E` : `${warnCount}W`) : undefined },
    { key: 'output', label: t('panel.output','Output'), component: 'output' },
    { key: 'memory', label: t('panel.memory','Memory'), component: 'memory' },
    { key: 'trace', label: t('panel.trace','Trace'), component: 'trace' },
    { key: 'ai-assistant', label: t('panel.ai','PyPilot'), component: 'ai-assistant' },
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

  // --- Command / Action layer ---
  const activeDoc = documents.find(d => d.uri === useEditorStore.getState().active);
  const activeUri = activeDoc?.uri;
  const activeBinName = activeUri ? deriveBinaryName(activeUri) : 'output.bin';

  // Función para manejar build y run
  const handleBuild = useCallback(async (autoRun: boolean = false) => {
    const electronAPI: any = (window as any).electronAPI;
    if (!electronAPI?.runCompile) {
      logger.warn('Build', 'electronAPI.runCompile not available');
      return;
    }

    const editorState = useEditorStore.getState();
    const activeDoc = documents.find(d => d.uri === editorState.active);
    if (!activeDoc) {
      logger.warn('Build', 'No active document to build');
      return;
    }

    if (!activeDoc.uri.endsWith('.vpy')) {
      logger.warn('Build', 'Active document is not a .vpy file:', activeDoc.uri);
      return;
    }

    const fileName = activeDoc.uri.split('/').pop() || activeDoc.uri;
    logger.info('Build', `${autoRun ? 'Build & Run' : 'Build'} starting: ${fileName}`);

    try {
      // Preparar argumentos para runCompile
      // Use diskPath (real file system path) instead of uri (which is a file:// URI)
      const filePath = activeDoc.diskPath || activeDoc.uri;
      
      if (filePath.startsWith('file://')) {
        logger.error('Build', 'Document has no diskPath, cannot compile:', activeDoc.uri);
        return;
      }
      
      logger.debug('Build', 'Using file path:', filePath);
      
      const args: any = {
        path: filePath,
        autoStart: autoRun
      };

      // Si el documento está sucio, enviarlo para que se guarde antes de compilar
      if (activeDoc.dirty) {
        args.saveIfDirty = {
          content: activeDoc.content,
          expectedMTime: activeDoc.mtime
        };
      }

      // Ejecutar compilación
      const result = await electronAPI.runCompile(args);
      
      if (result.error) {
        logger.error('Build', 'Compilation failed:', result.error, result.detail || '');
        return;
      }

      if (result.conflict) {
        // File was modified externally during build - automatically force overwrite
        logger.info('Build', 'File conflict detected, auto-overwriting...');
        try {
          const forceArgs = { ...args, saveIfDirty: { ...args.saveIfDirty, expectedMTime: null } };
          const forceResult = await electronAPI.runCompile(forceArgs);
          if (forceResult.error) {
            logger.error('Build', 'Force compilation failed:', forceResult.error);
            return;
          }
          useEditorStore.getState().markSaved(activeDoc.uri, forceResult.savedMTime);
          logger.info('Build', 'Force compilation successful:', forceResult.binPath, `(${forceResult.size} bytes)`);
          if (autoRun) {
            logger.debug('Build', 'Auto-run enabled - emulator should load the binary automatically');
          }
        } catch (forceError) {
          logger.error('Build', 'Failed to force compile during conflict:', forceError);
        }
        return;
      }

      logger.info('Build', 'Compilation successful:', result.binPath, `(${result.size} bytes)`);
      
      // Si el archivo fue guardado durante la compilación, actualizar el estado del editor
      if (activeDoc.dirty && result.savedMTime) {
        useEditorStore.getState().markSaved(activeDoc.uri, result.savedMTime);
        logger.debug('Build', 'File saved during compilation, tab marked as clean');
      }
      
      if (autoRun) {
        logger.debug('Build', 'Auto-run enabled - emulator should load the binary automatically');
      }
    } catch (error) {
      logger.error('Build', 'Build process failed:', error);
    }
  }, [documents]);

  const commandExec = useCallback(async (id: string) => {
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
        if (!apiFiles?.openFile) { logger.warn('File', 'files API missing'); break; }
        apiFiles.openFile().then((res: any) => {
          if (!res || res.error) return;
            const { path, content, mtime } = res;
            const normPath = path.replace(/\\/g,'/');
            // Ensure triple-slash file URI + uppercase drive letter normalized the same way Monaco does (file:///C:/...)
            let uri: string;
            if (normPath.match(/^[A-Za-z]:\//)) {
              // Windows absolute path like C:/path/file.ext
              uri = `file:///${normPath}`;
            } else if (normPath.startsWith('/')) {
              // Unix absolute path like /path/file.ext  
              uri = `file://${normPath}`;
            } else {
              // Relative path - should not happen normally but handle it
              uri = `file://${normPath}`;
            }
            logger.debug('File', 'Opening file with path:', path, 'normPath:', normPath, 'uri:', uri);
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
            // File was modified externally - automatically force overwrite
            logger.info('Save', 'File conflict detected, auto-overwriting...');
            apiFiles.saveFile({ path, content, expectedMTime: null }).then((forceRes: any) => {
              if (forceRes?.error) { 
                logger.error('Save', 'Force save error:', forceRes.error); 
                return; 
              }
              useEditorStore.getState().markSaved(active.uri, forceRes.mtime);
              logger.debug('Save', 'Overwrote external changes');
            });
            return;
          }
          if (res.error) { logger.error('Save', 'Save error:', res.error); return; }
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
        await handleBuild(false); // Solo compilar
        break;
      case 'build.run':
        await handleBuild(true); // Compilar y ejecutar
        break;
      case 'build.clean':
  logger.debug('App', 'clean build artifacts (pending implementation)');
        break;
      case 'debug.start': {
        // Phase 2: Implementar debug.start
        logger.info('Debug', 'Starting debug session...');
        
        try {
          // 0. Activar flag ANTES de compilar para que EmulatorPanel no auto-inicie
          useDebugStore.getState().setLoadingForDebug(true);
          useDebugStore.getState().setState('paused'); // Set to paused so F5 will continue
          
          // 1. Compilar sin auto-run (necesitamos el binario pero no ejecutarlo automáticamente)
          const editorState = useEditorStore.getState();
          const activeDoc = documents.find(d => d.uri === editorState.active);
          
          if (!activeDoc) {
            logger.error('Debug', 'No active document to debug');
            break;
          }

          if (!activeDoc.uri.endsWith('.vpy')) {
            logger.error('Debug', 'Active document is not a .vpy file:', activeDoc.uri);
            break;
          }

          const fileName = activeDoc.uri.split('/').pop() || activeDoc.uri;
          logger.info('Debug', `Compiling for debug: ${fileName}`);

          const electronAPI: any = (window as any).electronAPI;
          if (!electronAPI?.runCompile) {
            logger.error('Debug', 'electronAPI.runCompile not available');
            break;
          }

          const filePath = activeDoc.diskPath || activeDoc.uri;
          
          if (filePath.startsWith('file://')) {
            logger.error('Debug', 'Document has no diskPath, cannot compile:', activeDoc.uri);
            break;
          }

          const args: any = {
            path: filePath,
            autoStart: false  // No auto-run, queremos control manual
          };

          // Si el documento está sucio, enviarlo para que se guarde antes de compilar
          if (activeDoc.dirty) {
            args.saveIfDirty = {
              content: activeDoc.content,
              expectedMTime: activeDoc.mtime
            };
          }

          // 2. Compilar
          const result = await electronAPI.runCompile(args);
          
          if (result.error) {
            logger.error('Debug', 'Compilation failed:', result.error, result.detail || '');
            break;
          }

          if (result.conflict) {
            logger.error('Debug', 'File conflict detected, cannot start debug session');
            break;
          }

          logger.info('Debug', 'Compilation successful:', result.binPath);

          // 3. Si el archivo fue guardado, actualizar estado
          if (activeDoc.dirty && result.savedMTime) {
            useEditorStore.getState().markSaved(activeDoc.uri, result.savedMTime);
          }

          // 4. Verificar que tenemos .pdb data
          if (!result.pdbData) {
            logger.warn('Debug', 'No debug symbols (.pdb) available, debugging will be limited');
          } else {
            logger.info('Debug', '✓ Debug symbols loaded');
          }

          // 5. El .pdb ya fue cargado automáticamente en EmulatorPanel via onCompiledBin
          // Pero el binario también se cargó y ejecutó. Para debugging necesitamos control.
          
          // 6. Entrar en modo debug (pausado en entry point)
          useDebugStore.getState().setState('paused');
          
          logger.info('Debug', '✓ Debug session started - paused at entry point');
          logger.info('Debug', 'Use F5 to continue, F10 to step over, F11 to step into');
          
        } catch (error) {
          logger.error('Debug', 'Failed to start debug session:', error);
        }
        break;
      }
      case 'debug.stop': {
        logger.info('Debug', 'Stopping debug session...');
        
        try {
          // Cambiar a estado stopped
          useDebugStore.getState().setState('stopped');
          
          // Limpiar datos de debug
          useDebugStore.getState().setCurrentVpyLine(null);
          useDebugStore.getState().setCurrentAsmAddress(null);
          useDebugStore.getState().updateCallStack([]);
          
          logger.info('Debug', '✓ Debug session stopped');
        } catch (error) {
          logger.error('Debug', 'Failed to stop debug session:', error);
        }
        break;
      }
      case 'debug.continue': {
        logger.info('Debug', 'Continuing execution...');
        try {
          useDebugStore.getState().run();
          logger.info('Debug', '✓ Execution resumed');
        } catch (error) {
          logger.error('Debug', 'Failed to continue execution:', error);
        }
        break;
      }
      case 'debug.pause': {
        logger.info('Debug', 'Pausing execution...');
        try {
          useDebugStore.getState().pause();
          logger.info('Debug', '✓ Execution paused');
        } catch (error) {
          logger.error('Debug', 'Failed to pause execution:', error);
        }
        break;
      }
      case 'debug.stepOver':
  logger.debug('App', 'step over (pending implementation)');
        break;
      case 'debug.stepInto':
  logger.debug('App', 'step into (pending implementation)');
        break;
      case 'debug.stepOut':
  logger.debug('App', 'step out (pending implementation)');
        break;
      case 'debug.toggleBreakpoint':
  logger.debug('App', 'toggle breakpoint (pending implementation)');
        break;
      default:
        logger.warn('App', 'unknown command:', id);
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
      else if (e.key === 'F5' && !ctrl) { 
        e.preventDefault(); 
        // Smart F5: If in debug session, continue. Otherwise, build and run.
        const debugState = useDebugStore.getState().state;
        if (debugState !== 'stopped') {
          commandExec('debug.continue');
        } else {
          commandExec('build.run');
        }
      }
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
      } catch (e) { logger.error('LSP', 'init failed:', e); }
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
                      if (['files','emulator','debug','errors','memory','trace','bioscalls','ai-assistant'].includes(c)) activeComp = c;
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
                      if (['files','emulator','debug','errors','memory','trace','bioscalls','ai-assistant'].includes(c)) activeComp = c;
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
try { restoreEditorState(); } catch (e) { logger.warn('App', 'restore failed:', e); }
// Start persistence subscription
try { ensureEditorPersistence(); } catch (e) { logger.warn('App', 'persist init failed:', e); }

const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(<App />);
}
