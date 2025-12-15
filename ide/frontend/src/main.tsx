import React, { useEffect, useRef, useState, useCallback } from 'react';
import { createRoot } from 'react-dom/client';
import './i18n.js';
import './global.css';
import { useTranslation } from 'react-i18next';
// (import eliminado duplicado) 
import { initLsp, lspClient } from './lspClient.js';
import { DockWorkspace } from './components/DockWorkspace.js';
import { restoreEditorState, ensureEditorPersistence } from './state/editorPersistence.js';
import { deriveBinaryName } from './utils/index.js';
import { toggleComponent, resetLayout, ensureComponent } from './state/dockBus.js';
import { useEditorStore } from './state/editorStore.js';
import { useProjectStore, setEditorStoreRef } from './state/projectStore.js';
import { useDebugStore } from './state/debugStore.js';
import { MenuRoot, MenuItem, MenuSeparator, MenuCheckItem, SubMenu } from './components/MenuComponents.js';
import { NewProjectDialog } from './components/dialogs/NewProjectDialog.js';
import { InputDialog } from './components/dialogs/InputDialog.js';
import { initLoggerWithRefreshDetection, logger, detectRefresh } from './utils/logger.js';

// Initialize store reference for cross-store access
setEditorStoreRef(useEditorStore);

// Expose stores globally for MCP server access (Electron main process)
if (typeof window !== 'undefined') {
  (window as any).__editorStore__ = useEditorStore;
  (window as any).__projectStore__ = useProjectStore;
  (window as any).__debugStore__ = useDebugStore;
  (window as any).dockBus = { emit: ensureComponent };
}

function App() {
  const { t, i18n } = useTranslation(['common']);
  // IMPORTANT: Avoid grouping multiple fields into a new object each render (React 19 strict external store snapshot loop)
  const documents = useEditorStore(s => s.documents);
  const openDocument = useEditorStore(s => s.openDocument);
  const allDiagnostics = useEditorStore(s => s.allDiagnostics);
  const setDiagnosticsBySource = useEditorStore(s => s.setDiagnosticsBySource);

  const initializedRef = useRef(false);

  // Listen for commands from Electron main process
  useEffect(() => {
    const electronAPI = (window as any).electronAPI;
    if (!electronAPI?.onCommand) return;
    
    const handler = (id: string, _payload?: any) => {
      if (id === 'app.hardRefreshBlocked') {
        logger.error('App', '🚨 Hard refresh blocked! This would clear ALL settings and API keys.');
        logger.info('App', '💡 To reload the IDE, close and reopen the window instead.');
        alert('⚠️ Hard Refresh Blocked!\n\nCmd+Shift+R would delete all your settings, API keys, and chat history.\n\nTo reload the IDE properly, close and reopen the window instead.');
      }
    };
    
    electronAPI.onCommand(handler);
  }, []);

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
  const vpyProject = useProjectStore(s => s.vpyProject);
  const recentWorkspaces = useProjectStore(s => s.recentWorkspaces);
  const openVpyProject = useProjectStore(s => s.openVpyProject);
  const closeVpyProject = useProjectStore(s => s.closeVpyProject);
  const saveProjectState = useProjectStore(s => s.saveProjectState);
  const getProjectState = useProjectStore(s => s.getProjectState);
  
  useEffect(() => {
    if (!initializedRef.current && !hasWorkspace()) {
      logger.debug('App', 'Auto-restoring last workspace on startup');
      restoreLastWorkspace();
      initializedRef.current = true;
    }
  }, [restoreLastWorkspace, hasWorkspace]);

  // Save project state on window unload
  useEffect(() => {
    const handleBeforeUnload = () => {
      const projectState = useProjectStore.getState();
      const editorState = useEditorStore.getState();
      
      if (projectState.vpyProject) {
        const openFiles = editorState.documents
          .filter(d => d.diskPath)
          .map(d => d.uri);
        saveProjectState(
          projectState.vpyProject.projectFile, 
          openFiles, 
          editorState.active
        );
        logger.debug('App', 'Saved project state before unload');
      }
    };
    
    window.addEventListener('beforeunload', handleBeforeUnload);
    return () => window.removeEventListener('beforeunload', handleBeforeUnload);
  }, [saveProjectState]);

  // Track which menu is open
  const [openMenu, setOpenMenu] = useState<string | null>(null);
  // New Project dialog state
  const [showNewProjectDialog, setShowNewProjectDialog] = useState(false);
  const [defaultProjectLocation, setDefaultProjectLocation] = useState('');
  // New File dialog state (for .vec files that need a name)
  const [showNewFileDialog, setShowNewFileDialog] = useState(false);
  const [newFileType, setNewFileType] = useState<'vec' | 'c' | 'vpy' | 'vmus'>('vec');
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
    { key: 'build-output', label: 'Build Output', component: 'build-output' },
    { key: 'compiler-output', label: 'Compiler Output', component: 'compiler-output' },
    { key: 'memory', label: t('panel.memory','Memory'), component: 'memory' },
    { key: 'trace', label: t('panel.trace','Trace'), component: 'trace' },
    { key: 'psglog', label: 'PSG Log', component: 'psglog' },
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
    const projectState = useProjectStore.getState();
    
    // If we have a project, use project entry point
    let activeDoc;
    let buildFromProject = false;
    
    if (projectState.vpyProject) {
      // Build from project - use entry file
      const entryPath = projectState.getEntryPath();
      if (entryPath) {
        // Find if entry file is already open
        activeDoc = documents.find(d => d.diskPath === entryPath);
        if (!activeDoc) {
          // Entry file not open - build directly from path
          buildFromProject = true;
          activeDoc = { 
            uri: entryPath, 
            diskPath: entryPath, 
            dirty: false,
            content: '' 
          };
        }
        logger.info('Build', `Building project: ${projectState.vpyProject.config.project.name}`);
      }
    } else {
      // No project - use active document
      activeDoc = documents.find(d => d.uri === editorState.active);
    }
    
    if (!activeDoc) {
      logger.warn('Build', 'No active document to build');
      return;
    }

    const filePath = activeDoc.diskPath || activeDoc.uri;
    
    if (!filePath.endsWith('.vpy') && !buildFromProject) {
      logger.warn('Build', 'Active document is not a .vpy file:', filePath);
      return;
    }

    const fileName = filePath.split('/').pop() || filePath;
    logger.info('Build', `${autoRun ? 'Build & Run' : 'Build'} starting: ${fileName}`);

    try {
      // Preparar argumentos para runCompile
      // filePath is already set above from project or active doc
      
      if (filePath.startsWith('file://')) {
        logger.error('Build', 'Document has no diskPath, cannot compile:', activeDoc.uri);
        return;
      }
      
      logger.debug('Build', 'Using file path:', filePath);
      
      const args: any = {
        path: filePath,
        autoStart: autoRun
      };
      
      // If building from project, include output path
      if (projectState.vpyProject) {
        const outputPath = projectState.getOutputPath();
        if (outputPath) {
          args.outputPath = outputPath;
          logger.debug('Build', 'Project output path:', outputPath);
        }
      }

      // Si el documento está sucio, enviarlo para que se guarde antes de compilar
      // Solo aplica cuando el documento está realmente abierto (no buildFromProject sin abrir)
      if (activeDoc.dirty && !buildFromProject) {
        args.saveIfDirty = {
          content: activeDoc.content,
          expectedMTime: activeDoc.mtime
        };
        logger.debug('Build', 'Document is dirty - will save before compiling');
      }

      // Construct expected binary path and delete it before compiling
      if (activeDoc.diskPath) {
        const expectedBinPath = activeDoc.diskPath.replace(/\.(vpy|vpyproj)$/, '.bin');
        if (electronAPI.deleteFile) {
          try {
            await electronAPI.deleteFile(expectedBinPath);
            logger.debug('Build', 'Deleted existing binary before compilation:', expectedBinPath);
          } catch (deleteError) {
            // Ignore error if file doesn't exist
            logger.debug('Build', 'Could not delete existing binary (may not exist)');
          }
        }
      }

      // Ejecutar compilación
      const result = await electronAPI.runCompile(args);
      
      if (result.error) {
        logger.error('Build', 'Compilation failed:', result.error, result.detail || '');
        // Don't proceed to emulator - binary deleted and compilation failed
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
      
      // Load debug symbols (.pdb file) if available
      if (result.binPath) {
        try {
          const pdbPath = result.binPath.replace(/\.bin$/, '.pdb');
          const pdbRes = await electronAPI.readFile(pdbPath);
          if ('error' in pdbRes) {
            logger.warn('Build', 'No .pdb file found:', pdbPath);
          } else {
            const pdbData = JSON.parse(pdbRes.content);
            useDebugStore.getState().loadPdbData(pdbData);
            logger.info('Build', 'Loaded debug symbols from:', pdbPath);
          }
        } catch (pdbError) {
          logger.warn('Build', 'Failed to load .pdb:', pdbError);
        }
      }
      
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
      case 'file.new': 
      case 'file.new.vpy': {
        const idx = documents.filter(d => d.uri.startsWith('inmemory://untitled') && d.uri.endsWith('.vpy')).length + 1;
        const uri = `inmemory://untitled${idx}.vpy`;
        const content = `# New VPy file

def main():
    # Initialization
    Set_Intensity(127)

def loop():
    # Game loop
    Wait_Recal()
`;
        openDocument({ uri, language: 'vpy', content, dirty: true, diagnostics: [] });
        try {
          if ((window as any)._lspInit) {
            lspClient.didOpen(uri, 'vpy', content);
          }
        } catch {}
        break; }
      case 'file.new.c': {
        const idx = documents.filter(d => d.uri.match(/untitled.*\.(c|cpp|h)$/)).length + 1;
        const uri = `inmemory://untitled${idx}.c`;
        const content = `/* ${uri.split('/').pop()} - C source file */

#include <vectrex.h>

// Your C code here
`;
        openDocument({ uri, language: 'c', content, dirty: true, diagnostics: [] });
        break; }
      case 'file.new.vec': {
        // Open dialog to ask for filename
        setNewFileType('vec');
        setShowNewFileDialog(true);
        break; }
      case 'file.new.vec.create': {
        // This is called after the dialog confirms with a name
        // The actual creation is handled in handleNewFileConfirm
        break; }
      case 'file.new.vmus': {
        // Open dialog to ask for filename
        setNewFileType('vmus');
        setShowNewFileDialog(true);
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
        // Detect file extension from URI
        const ext = active.uri.match(/\.(vpy|vec|vmus|c|cpp|h)$/)?.[1] || 'vpy';
        const defaultName = active.diskPath ? undefined : `untitled.${ext}`;
        apiFiles.saveFileAs({ suggestedName: defaultName, content: active.content }).then((res: any) => {
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
          // 0. Activar flag ANTES de compilar para que EmulatorPanel maneje el auto-start
          useDebugStore.getState().setLoadingForDebug(true);
          // NO setear estado aquí - lo hará EmulatorPanel después de cargar el binary
          
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
          // El binario también se cargó y EmulatorPanel ya seteó el estado a 'running'
          
          // 6. Debug session ya está en modo 'running' (seteado por EmulatorPanel)
          // NO sobrescribir el estado aquí - dejar que corra hasta breakpoint
          
          logger.info('Debug', '✓ Debug session started - running until breakpoint');
          logger.info('Debug', 'Use F9 to toggle breakpoints, F5 to continue when paused');
          
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
        logger.debug('App', 'step over');
        window.postMessage({ type: 'debug-step-over' }, '*');
        break;
      case 'debug.stepInto':
        logger.debug('App', 'step into');
        window.postMessage({ type: 'debug-step-into' }, '*');
        break;
      case 'debug.stepOut':
        logger.debug('App', 'step out');
        window.postMessage({ type: 'debug-step-out' }, '*');
        break;
      case 'debug.toggleBreakpoint':
  logger.debug('App', 'toggle breakpoint (pending implementation)');
        break;
      
      // Project commands
      case 'project.new': {
        // Set default location - try from current project, recent projects, or empty
        const projectState = useProjectStore.getState();
        let defaultLocation = '';
        
        // First try: current open project's parent directory
        if (projectState.vpyProject?.rootDir) {
          const parts = projectState.vpyProject.rootDir.replace(/\\/g, '/').split('/');
          parts.pop(); // Get parent directory
          defaultLocation = parts.join('/');
        }
        // Second try: parent directory of most recent project
        else if (projectState.recentWorkspaces.length > 0) {
          const lastProject = projectState.recentWorkspaces.find(w => w.isProject);
          if (lastProject) {
            const parts = lastProject.path.replace(/\\/g, '/').split('/');
            parts.pop(); // Remove project file name
            parts.pop(); // Get parent of project directory
            defaultLocation = parts.join('/');
          }
        }
        
        setDefaultProjectLocation(defaultLocation);
        // Show the new project dialog
        setShowNewProjectDialog(true);
        break;
      }
      case 'project.open': {
        const projectAPI = (window as any).project;
        if (!projectAPI) {
          logger.error('Project', 'Project API not available');
          break;
        }
        const result = await projectAPI.open();
        if (result && !('error' in result) && result !== null) {
          // Use store to handle project state
          const success = await openVpyProject(result.path);
          if (success) {
            const apiFiles: any = (window as any).files;
            
            // Try to restore previously open files
            const savedState = useProjectStore.getState().getProjectState(result.path);
            if (savedState && savedState.openFiles.length > 0) {
              logger.info('Project', `Restoring ${savedState.openFiles.length} open files`);
              
              for (const uri of savedState.openFiles) {
                // Extract disk path from URI
                let diskPath = uri.replace('file:///', '').replace('file://', '');
                if (diskPath.match(/^[A-Za-z]:\//)) {
                  // Windows path - keep as is
                } else if (!diskPath.startsWith('/')) {
                  diskPath = '/' + diskPath;
                }
                
                if (apiFiles?.readFile) {
                  try {
                    const fileResult = await apiFiles.readFile(diskPath);
                    if (fileResult && !fileResult.error) {
                      openDocument({
                        uri,
                        language: diskPath.endsWith('.vpy') ? 'vpy' : 'plaintext',
                        content: fileResult.content,
                        dirty: false,
                        diagnostics: [],
                        diskPath,
                        mtime: fileResult.mtime,
                        lastSavedContent: fileResult.content
                      });
                    }
                  } catch (e) {
                    logger.warn('Project', `Could not restore file: ${diskPath}`);
                  }
                }
              }
              
              // Set the last active file
              if (savedState.activeFile) {
                useEditorStore.getState().setActive(savedState.activeFile);
              }
            } else {
              // No saved state - just open entry file
              const entryPath = useProjectStore.getState().getEntryPath();
              if (entryPath && apiFiles?.readFile) {
                const fileResult = await apiFiles.readFile(entryPath);
                if (fileResult && !fileResult.error) {
                  const normPath = entryPath.replace(/\\/g, '/');
                  const uri = normPath.match(/^[A-Za-z]:\//) ? `file:///${normPath}` : `file://${normPath}`;
                  openDocument({
                    uri,
                    language: 'vpy',
                    content: fileResult.content,
                    dirty: false,
                    diagnostics: [],
                    diskPath: entryPath,
                    mtime: fileResult.mtime,
                    lastSavedContent: fileResult.content
                  });
                }
              }
            }
          }
        } else if (result && 'error' in result) {
          logger.error('Project', result.error);
        }
        break;
      }
      case 'project.close': {
        // Check for unsaved files
        const unsavedDocs = documents.filter(d => d.dirty);
        if (unsavedDocs.length > 0) {
          const names = unsavedDocs.map(d => d.uri.split('/').pop()).join(', ');
          if (!window.confirm(`You have ${unsavedDocs.length} unsaved file(s): ${names}\n\nClose project anyway?`)) {
            break;
          }
        }
        
        // Close the project (saves state)
        closeVpyProject();
        
        // Close all open documents to show welcome screen
        const editorState = useEditorStore.getState();
        const allDocs = [...editorState.documents];
        for (const doc of allDocs) {
          editorState.closeDocument(doc.uri);
        }
        
        logger.info('Project', 'Project closed, all files closed');
        break;
      }
      
      default:
        logger.warn('App', 'unknown command:', id);
    }
  }, [documents, openDocument, activeBinName, openVpyProject, closeVpyProject]);

  // Keyboard shortcuts mapping (similar to VS conventions)
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const ctrl = e.ctrlKey || e.metaKey; // meta for mac future
      // File
      if (ctrl && e.key.toLowerCase() === 's' && !e.shiftKey) { e.preventDefault(); commandExec('file.save'); }
      else if (ctrl && e.key.toLowerCase() === 's' && e.shiftKey) { e.preventDefault(); commandExec('file.saveAs'); }
      else if (ctrl && e.key.toLowerCase() === 'o' && !e.shiftKey) { e.preventDefault(); commandExec('file.open'); }
      else if (ctrl && e.key.toLowerCase() === 'o' && e.shiftKey) { e.preventDefault(); commandExec('project.open'); }
      else if (ctrl && e.key.toLowerCase() === 'n') { e.preventDefault(); commandExec('file.new.vpy'); }
      // Build / Run
      else if (e.key === 'F7') { e.preventDefault(); commandExec('build.build'); }
      else if (e.key === 'F5' && !ctrl && !e.shiftKey) { 
        e.preventDefault(); 
        // Smart F5: If in debug session, continue. Otherwise, build and run.
        const debugState = useDebugStore.getState().state;
        if (debugState !== 'stopped') {
          commandExec('debug.continue');
        } else {
          commandExec('build.run');
        }
      }
      // Debug - Ctrl+F5 on Windows, Cmd+D on macOS (Cmd+F5 triggers VoiceOver)
      else if (ctrl && e.key === 'F5') { e.preventDefault(); commandExec('debug.start'); }
      else if (ctrl && e.key.toLowerCase() === 'd' && !e.shiftKey) { e.preventDefault(); commandExec('debug.start'); }
      else if (e.key === 'F9') { e.preventDefault(); commandExec('debug.toggleBreakpoint'); }
      else if (e.key === 'F10') { e.preventDefault(); commandExec('debug.stepOver'); }
      else if (e.key === 'F11' && !e.shiftKey) { e.preventDefault(); commandExec('debug.stepInto'); }
      else if (e.key === 'F11' && e.shiftKey) { e.preventDefault(); commandExec('debug.stepOut'); }
      else if (e.key === 'F5' && e.shiftKey) { e.preventDefault(); commandExec('debug.stop'); }
    };
    window.addEventListener('keydown', handler, { capture: true });
    return () => window.removeEventListener('keydown', handler, { capture: true } as any);
  }, [commandExec]);

  // Listen for vpy-command events from WelcomeView and other components
  useEffect(() => {
    const handler = (e: CustomEvent) => {
      const cmd = e.detail?.command;
      if (cmd) {
        commandExec(cmd);
      }
    };
    window.addEventListener('vpy-command', handler as EventListener);
    return () => window.removeEventListener('vpy-command', handler as EventListener);
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
          {/* File menu (merged with Project) */}
          <MenuRoot label={t('menu.file')} open={openMenu==='file'} setOpen={()=>setOpenMenu(openMenu==='file'?null:'file')}>
            <SubMenu label={t('file.new', 'New')}>
              <MenuItem label={`${t('project.new', 'Project...')}`} onClick={()=>{ commandExec('project.new'); setOpenMenu(null); }} />
              <MenuSeparator />
              <MenuItem label={`${t('file.new.vpy', 'VPy File')}	Ctrl+N`} onClick={()=>{ commandExec('file.new.vpy'); setOpenMenu(null); }} />
              <MenuItem label={`${t('file.new.c', 'C/C++ File')}`} onClick={()=>{ commandExec('file.new.c'); setOpenMenu(null); }} />
              <MenuItem label={`${t('file.new.vec', 'Vector List (.vec)')}`} onClick={()=>{ commandExec('file.new.vec'); setOpenMenu(null); }} />
              <MenuItem label={`${t('file.new.vmus', 'Music File (.vmus)')}`} onClick={()=>{ commandExec('file.new.vmus'); setOpenMenu(null); }} />
            </SubMenu>
            <SubMenu label={t('file.open', 'Open')}>
              <MenuItem label={`${t('project.open', 'Project...')}	Ctrl+Shift+O`} onClick={()=>{ commandExec('project.open'); setOpenMenu(null); }} />
              <MenuItem label={`${t('file.openFile', 'File...')}	Ctrl+O`} onClick={()=>{ commandExec('file.open'); setOpenMenu(null); }} />
            </SubMenu>
            <MenuSeparator />
            <MenuItem label={activeDoc?.dirty? `${t('file.save', 'Save')} *	Ctrl+S` : `${t('file.save', 'Save')}	Ctrl+S`} disabled={!activeDoc} onClick={()=>{ commandExec('file.save'); setOpenMenu(null); }} />
            <MenuItem label={`${t('file.saveAs', 'Save As...')}	Ctrl+Shift+S`} disabled={!activeDoc} onClick={()=>{ commandExec('file.saveAs'); setOpenMenu(null); }} />
            <MenuSeparator />
            <MenuItem label={t('file.close', 'Close File')} disabled={!activeDoc} onClick={()=>{ commandExec('file.close'); setOpenMenu(null); }} />
            <MenuItem label={t('project.close', 'Close Project')} disabled={!vpyProject} onClick={()=>{ commandExec('project.close'); setOpenMenu(null); }} />
            <MenuSeparator />
            {/* Recent projects */}
            <SubMenu label={t('file.recentProjects', 'Recent Projects')} disabled={recentWorkspaces.filter(w => w.isProject).length === 0}>
              {recentWorkspaces.filter(w => w.isProject).slice(0, 5).map((w, i) => (
                <MenuItem key={i} label={w.name} onClick={async () => {
                  await openVpyProject(w.path);
                  setOpenMenu(null);
                }} />
              ))}
            </SubMenu>
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
                      if (['files','emulator','debug','errors','memory','trace','bioscalls','ai-assistant','build-output','compiler-output'].includes(c)) activeComp = c;
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
                      if (['files','emulator','debug','errors','memory','trace','bioscalls','ai-assistant','build-output','compiler-output'].includes(c)) activeComp = c;
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
      
      {/* New Project Dialog */}
      <NewProjectDialog
        isOpen={showNewProjectDialog}
        onClose={() => setShowNewProjectDialog(false)}
        defaultLocation={defaultProjectLocation}
        onCreate={async (options) => {
          logger.info('Project', `Creating ${options.type}: ${options.name} at ${options.location}`);
          const projectAPI = (window as any).project;
          if (!projectAPI) {
            logger.error('Project', 'Project API not available');
            return;
          }
          try {
            const result = await projectAPI.create({
              name: options.name,
              location: options.location,
              type: options.type,
              template: options.template,
            });
            if (result && 'ok' in result && result.ok) {
              logger.info('Project', `Created project: ${result.name}`);
              // Open the created project
              const success = await openVpyProject(result.projectFile);
              if (success) {
                const entryPath = useProjectStore.getState().getEntryPath();
                if (entryPath) {
                  const apiFiles: any = (window as any).files;
                  if (apiFiles?.readFile) {
                    const fileResult = await apiFiles.readFile(entryPath);
                    if (fileResult && !fileResult.error) {
                      const normPath = entryPath.replace(/\\/g, '/');
                      const uri = normPath.match(/^[A-Za-z]:\//) ? `file:///${normPath}` : `file://${normPath}`;
                      openDocument({
                        uri,
                        language: 'vpy',
                        content: fileResult.content,
                        dirty: false,
                        diagnostics: [],
                        diskPath: entryPath,
                        mtime: fileResult.mtime,
                        lastSavedContent: fileResult.content
                      });
                    }
                  }
                }
              }
            } else if (result && 'error' in result) {
              logger.error('Project', result.error);
            }
          } catch (error) {
            logger.error('Project', 'Failed to create project:', error);
          }
        }}
      />
      
      {/* New File Dialog (for .vec and .vmus files) */}
      <InputDialog
        isOpen={showNewFileDialog}
        title={newFileType === 'vec' ? 'New Vector List' : newFileType === 'vmus' ? 'New Music File' : 'New File'}
        message={newFileType === 'vec' ? 'Enter a name for the vector list (without extension):' : newFileType === 'vmus' ? 'Enter a name for the music file (without extension):' : 'Enter filename:'}
        placeholder={newFileType === 'vec' ? 'my_sprite' : newFileType === 'vmus' ? 'my_music' : 'filename'}
        defaultValue=""
        validateFn={(value) => {
          if (!value.trim()) return 'Name is required';
          if (!/^[a-zA-Z][a-zA-Z0-9_-]*$/.test(value)) {
            return 'Name must start with a letter and contain only letters, numbers, hyphens, and underscores';
          }
          return null;
        }}
        onCancel={() => setShowNewFileDialog(false)}
        onConfirm={async (name) => {
          setShowNewFileDialog(false);
          
          const vpyProject = useProjectStore.getState().vpyProject;
          const apiFiles = (window as any).files;
          
          if (newFileType === 'vec') {
            const content = JSON.stringify({
              version: "1.0",
              name: name,
              canvas: { width: 256, height: 256, origin: "center" },
              layers: [{
                name: "default",
                visible: true,
                paths: [{
                  name: "shape",
                  intensity: 127,
                  closed: true,
                  points: [
                    { x: 0, y: 20 },
                    { x: -15, y: -10 },
                    { x: 15, y: -10 }
                  ]
                }]
              }]
            }, null, 2);
            
            // If we have a project, save to assets/vectors/
            if (vpyProject?.rootDir && apiFiles?.saveFile) {
              const filePath = `${vpyProject.rootDir}/assets/vectors/${name}.vec`.replace(/\\/g, '/');
              try {
                const result = await apiFiles.saveFile({ path: filePath, content });
                if (result && !result.error) {
                  const normPath = filePath.replace(/\\/g, '/');
                  const uri = normPath.match(/^[A-Za-z]:\//) ? `file:///${normPath}` : `file://${normPath}`;
                  openDocument({
                    uri,
                    language: 'json',
                    content,
                    dirty: false,
                    diagnostics: [],
                    diskPath: filePath,
                    mtime: result.mtime,
                    lastSavedContent: content
                  });
                  // Refresh workspace to show new file
                  useProjectStore.getState().refreshWorkspace();
                  logger.info('File', `Created ${filePath}`);
                  return;
                }
              } catch (e) {
                logger.warn('File', 'Failed to save to project folder, creating in-memory');
              }
            }
            
            // Fallback: create in-memory
            const uri = `inmemory://${name}.vec`;
            openDocument({ uri, language: 'json', content, dirty: true, diagnostics: [] });
          } else if (newFileType === 'vmus') {
            const content = JSON.stringify({
              version: "1.0",
              name: name,
              author: "",
              tempo: 120,
              ticksPerBeat: 24,
              totalTicks: 384,
              notes: [],
              noise: [],
              loopStart: 0,
              loopEnd: 384
            }, null, 2);
            
            // If we have a project, save to assets/music/
            if (vpyProject?.rootDir && apiFiles?.saveFile) {
              const filePath = `${vpyProject.rootDir}/assets/music/${name}.vmus`.replace(/\\/g, '/');
              try {
                const result = await apiFiles.saveFile({ path: filePath, content });
                if (result && !result.error) {
                  const normPath = filePath.replace(/\\/g, '/');
                  const uri = normPath.match(/^[A-Za-z]:\//) ? `file:///${normPath}` : `file://${normPath}`;
                  openDocument({
                    uri,
                    language: 'json',
                    content,
                    dirty: false,
                    diagnostics: [],
                    diskPath: filePath,
                    mtime: result.mtime,
                    lastSavedContent: content
                  });
                  // Refresh workspace to show new file
                  useProjectStore.getState().refreshWorkspace();
                  logger.info('File', `Created ${filePath}`);
                  return;
                }
              } catch (e) {
                logger.warn('File', 'Failed to save to project folder, creating in-memory');
              }
            }
            
            // Fallback: create in-memory
            const uri = `inmemory://${name}.vmus`;
            openDocument({ uri, language: 'json', content, dirty: true, diagnostics: [] });
          }
        }}
      />
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
