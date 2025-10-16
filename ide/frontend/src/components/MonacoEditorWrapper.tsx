import React, { useCallback, useEffect, useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { useEditorStore } from '../state/editorStore';
import Editor, { OnChange, Monaco, BeforeMount } from '@monaco-editor/react';
// Import full ESM Monaco API and expose it globally so the react wrapper skips AMD loader.js
import * as monacoApi from 'monaco-editor/esm/vs/editor/editor.api';
// Include all standard editor contributions (hover, markers, etc.) so that
// diagnostic underlines show native tooltips and language features work.
// Without this, only the bare API loads and marker hovers won't appear.
import 'monaco-editor/esm/vs/editor/editor.all';
import { logger } from '../utils/logger';
// Bundle the core editor worker via Vite (?worker) so we don't craft blob strings manually.
// If later we add languages needing their own workers we can import them similarly.
// import TsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker'; (example)
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';

// Ensure global monaco is present before @monaco-editor/react evaluates its loader path.
// This prevents the fallback that attempts to inject https://cdn.jsdelivr.../loader.js
if (!(window as any).monaco) {
  (window as any).monaco = monacoApi;
}
import { dockBus } from '../state/dockBus';
import { lspClient } from '../lspClient';
// TODO(i18n): Adapt Monaco UI strings (context menu, messages) when supporting dynamic locale changes.

// Simple language placeholder registration for 'vpy'
function ensureLanguage(monaco: Monaco) {
  const already = (monaco.languages.getLanguages() || []).some(l => l.id === 'vpy');
  if (!already) {
    monaco.languages.register({ id: 'vpy' });
    monaco.languages.setMonarchTokensProvider('vpy', {
      // Improved tokenizer to highlight declarations and common constructs
      tokenizer: {
        root: [
          // Comments
          [/#[^$]*/, 'comment'],
          // Function declaration: def <name>
          [/(def)(\s+)([A-Za-z_][A-Za-z0-9_]*)/, ['keyword','white','function.declaration']],
          // Const declaration: const <NAME>
          [/(const)(\s+)([A-Za-z_][A-Za-z0-9_]*)/, ['keyword','white','constant']],
          // Variable declaration: var <name>
          [/(var)(\s+)([A-Za-z_][A-Za-z0-9_]*)/, ['keyword','white','variable']],
          // Python keywords (lowercase)
          [/\b(if|else|elif|while|for|return|break|continue|pass|try|except|finally|with|as|import|from|global|nonlocal|class|lambda|yield|assert|del|in|is|not|and|or)\b/, 'keyword'],
          // VPy keywords (uppercase)
          [/\b(META|RETURN|IF|ELSE|WHILE|FOR)\b/i, 'keyword'],
          // Built-in drawing / std library like calls
          [/\b(DRAW_(POLYGON|CIRCLE_SEG|CIRCLE|ARC|SPIRAL)|PRINT_TEXT)\b/, 'function'],
          // Intensity / constant style (I_FOO) & ALL_CAPS identifiers
          [/\bI_[A-Z0-9_]+\b/, 'constant'],
          [/\b[A-Z_]{2,}\b/, 'constant'],
          // Hex & decimal numbers
          [/0x[0-9A-Fa-f]+\b/, 'number'],
          [/[0-9]+/, 'number'],
          // Strings
          [/".*?"|'.*?'/, 'string'],
          // Operators
          [/[-+/*=<>!]+/, 'operator'],
          // Identifiers (fallback) – 'main' will be colored by declaration rule if defined
          [/\b[A-Za-z_][A-Za-z0-9_]*\b/, 'identifier']
        ]
      }
    });
    monaco.languages.setLanguageConfiguration('vpy', {
      comments: { lineComment: '#' },
      autoClosingPairs: [
        { open: '"', close: '"' },
        { open: "'", close: "'" },
        { open: '(', close: ')' },
        { open: '[', close: ']' }
      ],
      brackets: [ ['{','}'], ['[',']'], ['(',')'] ]
    });
  }
  // Always (re)define theme so we can tweak later without reload
  monaco.editor.defineTheme('vpy-dark', {
    base: 'vs-dark',
    inherit: true,
    // NOTE: rules cover both classic Monarch tokens and semantic token types.
    // When semantic highlighting is enabled, semantic token ranges override lexical ones,
    // so we explicitly style enumMember + modifiers to color I_* constants.
    rules: [
      // Lexical/Monarch tokens
      { token: 'comment', foreground: '6A9955' },
      { token: 'keyword', foreground: 'C586C0' },
      { token: 'function', foreground: 'DCDCAA' },
      { token: 'constant', foreground: '4FC1FF' },
      { token: 'number', foreground: 'B5CEA8' },
      { token: 'string', foreground: 'CE9178' },
      { token: 'operator', foreground: 'D4D4D4' },
      { token: 'identifier', foreground: 'D4D4D4' },
      // Semantic token specific (enumMember is what server emits for I_* constants)
      { token: 'enumMember', foreground: '4FC1FF' },
      { token: 'enumMember.readonly', foreground: '4FC1FF' },
      // Optional semantic refinements (keep same base color for now)
      { token: 'function.declaration', foreground: 'DCDCAA' },
      { token: 'function.defaultLibrary', foreground: 'DCDCAA' }
    ],
    colors: {
      'editor.background': '#1E1E1E'
    }
  });
}

// Configure self-hosted Monaco paths (no CDN) to satisfy strict CSP 'script-src self'
// We rely on Vite serving node_modules/monaco-editor/min under /monaco via alias injection in dev server config (future improvement: plugin).
// For now we point to default 'vs' expected inside bundle; @monaco-editor/react will inline ESM without loader.js when possible.
export const MonacoEditorWrapper: React.FC<{ uri?: string }> = ({ uri }) => {
  const { t } = useTranslation(['editor','common']);
  // Use individual selectors to avoid creating a fresh object each render (React 19 getSnapshot loop guard)
  const documents = useEditorStore(s => s.documents);
  const active = useEditorStore(s => s.active);
  const setActive = useEditorStore(s => s.setActive);
  const updateContent = useEditorStore(s => s.updateContent);
  const setScrollPosition = useEditorStore(s => s.setScrollPosition);
  const scrollPositions = useEditorStore(s => s.scrollPositions);
  const setHadFocus = useEditorStore(s => s.setHadFocus);
  const hadFocus = useEditorStore(s => s.hadFocus);
  const breakpoints = useEditorStore(s => s.breakpoints);
  const toggleBreakpoint = useEditorStore(s => s.toggleBreakpoint);
  const clearAllBreakpoints = useEditorStore(s => s.clearAllBreakpoints);

  const targetUri = uri || active;
  const doc = documents.find(d => d.uri === targetUri);
  const lastModelRef = useRef<string | undefined>(undefined);
  // Keep track of hover provider so we can dispose on remount (avoid duplicate hover tooltips)
  const hoverDisposableRef = useRef<any>(null);

  const editorRef = useRef<any>(null);
  const monacoRef = useRef<Monaco | null>(null);
  const breakpointDecorationsRef = useRef<string[]>([]); // Track breakpoint decoration IDs
  const beforeMount: BeforeMount = (_monaco) => {
    (window as any).MonacoEnvironment = {
      getWorker: function (_moduleId: string, _label: string) {
        // Return a new bundled worker instance; Vite inlines the URL with proper CSP compliance
        return new (EditorWorker as any)();
      },
      baseUrl: '/' // not strictly needed with ESM import but kept for completeness
    };
  };

  // Track which URIs already sent didOpen to LSP
  const openedRef = useRef<Set<string>>(new Set());

  const handleMount = useCallback((editor: any, monaco: Monaco) => {
    logger.debug('App', 'Monaco Editor mounted');
    ensureLanguage(monaco);
    editorRef.current = editor;
    monacoRef.current = monaco;
    monaco.editor.setTheme('vpy-dark');
    
    // Debug: Check if editor is read-only
    logger.debug('App', 'Monaco Editor readOnly setting:', editor.getOption(monaco.editor.EditorOption.readOnly));
    logger.debug('App', 'Monaco Editor configuration check:', {
      readOnly: editor.getOption(monaco.editor.EditorOption.readOnly),
      domReadOnly: editor.getOption(monaco.editor.EditorOption.domReadOnly)
    });
    // Semantic tokens provider bridging LSP tokens (on-demand full refresh)
    monaco.languages.registerDocumentSemanticTokensProvider('vpy', {
      getLegend: () => ({ tokenTypes: ['keyword','function','variable','parameter','number','string','operator','enumMember'], tokenModifiers: ['readonly','declaration','defaultLibrary'] }),
      provideDocumentSemanticTokens: async (model) => {
        try {
          const uri = model.uri.toString();
          const params = { textDocument: { uri } };
          const res = await (lspClient as any).request('textDocument/semanticTokens/full', params);
          if (!res || !res.data) return { data: new Uint32Array() } as any;
          return { data: new Uint32Array(res.data) } as any;
        } catch (e) {
          // Evitar inundar logs: primer fallo puede ocurrir antes de didOpen sincronizado.
          if (!(window as any)._vpySemanticWarned) {
            logger.warn('LSP', 'semantic tokens error (silenced after first):', e);
            (window as any)._vpySemanticWarned = true;
          }
          return { data: new Uint32Array() } as any;
        }
      },
      releaseDocumentSemanticTokens: () => {}
    });
    // LSP-backed completion provider
    monaco.languages.registerCompletionItemProvider('vpy', {
      triggerCharacters: ['_', '(', ',', ' ', '.', ':'],
      provideCompletionItems: async (model, position) => {
        try {
          const uri = model.uri.toString();
          const text = model.getValue();
          // Send didChange before requesting completion to keep server in sync
          lspClient.didChange(uri, text);
          const params = {
            textDocument: { uri },
            position: { line: position.lineNumber - 1, character: position.column - 1 },
            context: { triggerKind: 1 }
          };
          const res = await (lspClient as any).request('textDocument/completion', params);
          if (!res) {
            // fallback: simple word scan
            const words = Array.from(new Set(text.match(/[A-Za-z_][A-Za-z0-9_]{2,}/g) || []));
            const suggestions = words.slice(0,100).map(w => ({ label:w, kind: monaco.languages.CompletionItemKind.Text, insertText:w }));
            return { suggestions };
          }
          const items = Array.isArray(res.items) ? res.items : (Array.isArray(res) ? res : []);
          const suggestions = items.map((it: any) => ({
            label: it.label,
            kind: monaco.languages.CompletionItemKind.Function,
            insertText: it.insertText || it.label,
            range: undefined
          }));
          // console.debug('[LSP] completion items', suggestions.length);
          return { suggestions };
        } catch (e) {
          logger.warn('LSP', 'completion error:', e);
          return { suggestions: [] };
        }
      }
    });
    // Rename provider
    monaco.languages.registerRenameProvider('vpy', {
      provideRenameEdits: async (model, position, newName) => {
        try {
          const uri = model.uri.toString();
          const res = await (lspClient as any).rename(uri, position.lineNumber - 1, position.column - 1, newName);
          if (!res) return { edits: [] } as any;
          const edits: any[] = [];
          if (res.changes) {
            Object.keys(res.changes).forEach(docUri => {
              const targetModel = monaco.editor.getModel(monaco.Uri.parse(docUri));
              if (!targetModel) return;
              res.changes[docUri].forEach((e:any) => {
                edits.push({
                  resource: targetModel.uri,
                  edit: {
                    range: new monaco.Range(
                      e.range.start.line + 1,
                      e.range.start.character + 1,
                      e.range.end.line + 1,
                      e.range.end.character + 1
                    ),
                    text: e.new_text
                  }
                });
              });
            });
          }
          return { edits } as any;
        } catch (e) { logger.warn('LSP', 'rename error:', e); return { edits: [] } as any; }
      },
      resolveRenameLocation: async (model, position) => {
        // Optimistic: allow rename of any identifier; server will veto if not a symbol
        const word = model.getWordAtPosition(position);
        if (!word) return { range: new monaco.Range(position.lineNumber, position.column, position.lineNumber, position.column), text: '' } as any;
        const range = new monaco.Range(position.lineNumber, word.startColumn, position.lineNumber, word.endColumn);
        return { range, text: model.getValueInRange(range) } as any;
      }
    });
    // Signature help provider
    monaco.languages.registerSignatureHelpProvider('vpy', {
      signatureHelpTriggerCharacters: ['(', ','],
      provideSignatureHelp: async (model, position) => {
        try {
          const uri = model.uri.toString();
          const res = await (lspClient as any).signatureHelp(uri, position.lineNumber - 1, position.column - 1);
          if (!res) return { value: { signatures: [], activeParameter: 0, activeSignature: 0 }, dispose: () => {} };
          return { value: res, dispose: () => {} } as any;
        } catch (e) { logger.warn('LSP', 'signatureHelp error:', e); return { value: { signatures: [], activeParameter: 0, activeSignature: 0 }, dispose: () => {} }; }
      }
    });
    // Hover provider (dispose any previous instance first to avoid stacking)
    if (hoverDisposableRef.current) {
      try { hoverDisposableRef.current.dispose(); } catch {}
      hoverDisposableRef.current = null;
    }
    hoverDisposableRef.current = monaco.languages.registerHoverProvider('vpy', {
      provideHover: async (model, position) => {
        logger.verbose('LSP', 'hover trigger:', model.uri.toString(), position.lineNumber, position.column);
        try {
          const uri = model.uri.toString();
          const params = { textDocument: { uri }, position: { line: position.lineNumber - 1, character: position.column - 1 } };
          const res = await (lspClient as any).request('textDocument/hover', params);
          if (res && res.contents) {
            const contents = typeof res.contents === 'string' ? res.contents : (res.contents.value || '');
            logger.verbose('LSP', 'hover response:', contents);
            return { contents: [{ value: contents }], range: undefined } as any;
          }
        } catch (e) { logger.warn('LSP', 'hover error:', e); }
        logger.verbose('LSP', 'hover empty');
        return null as any;
      }
    });
    logger.verbose('LSP', 'hover provider registered');
    // Extra instrumentation: log mouse move & model language to diagnose missing hover triggers
    try {
      const lang = editor.getModel()?.getLanguageId();
      logger.verbose('App', 'Monaco model languageId=', lang, 'uri=', editor.getModel()?.uri.toString());
      let lastLog = 0;
      editor.onMouseMove((e: any) => {
        const now = performance.now();
        if (now - lastLog < 250) return; // throttle
        lastLog = now;
        const pos = e.target?.position ? `${e.target.position.lineNumber}:${e.target.position.column}` : 'n/a';
        logger.verbose('App', 'Monaco mouseMove target=', e.target?.type, 'pos=', pos, 'detail=', e.target?.detail || '');
      });
      // Force re-apply hover enabled in case wrapper options lost
      editor.updateOptions({ hover: { enabled: true, delay: 150 } });
      // Fallback hover removed (native Monaco hover now active). If needed, reintroduce with a feature flag.
    } catch (e) { logger.warn('App', 'Monaco instrumentation error:', e); }
    // Definition provider
    monaco.languages.registerDefinitionProvider('vpy', {
      provideDefinition: async (model, position) => {
        try {
          const uri = model.uri.toString();
          const params = { textDocument: { uri }, position: { line: position.lineNumber - 1, character: position.column - 1 } };
          const res = await (lspClient as any).request('textDocument/definition', params);
          if (!res) return [];
          const locs = Array.isArray(res) ? res : (res ? [res] : []);
          return locs.map((loc: any) => ({
            uri: monaco.Uri.parse(loc.uri || (loc.targetUri && loc.targetUri.uri) || uri),
            range: new monaco.Range(
              loc.range.start.line + 1,
              loc.range.start.character + 1,
              loc.range.end.line + 1,
              loc.range.end.character + 1
            )
          }));
        } catch (e) { logger.warn('LSP', 'definition error:', e); }
        return [];
      }
    });
    if (doc) {
      const mUri = monaco.Uri.parse(doc.uri);
      let model = monaco.editor.getModel(mUri);
      if (!model) {
        model = monaco.editor.createModel(doc.content, 'vpy', mUri);
      }
      editor.setModel(model);
      lastModelRef.current = doc.uri;
      // Restore previous scroll position if known
      try {
        const top = scrollPositions[doc.uri];
        if (typeof top === 'number') {
          editor.setScrollTop(top);
        }
        // Restore focus only if previously focused & container visible
        if (hadFocus[doc.uri]) {
          requestAnimationFrame(() => editor.focus());
        }
      } catch {}
      // Trigger an initial didChange to encourage semanticTokens/full soon after mount
      if (!openedRef.current.has(doc.uri)) {
        try { lspClient.didOpen(doc.uri, 'vpy', model.getValue()); openedRef.current.add(doc.uri); } catch {}
      } else {
        lspClient.didChange(doc.uri, model.getValue());
      }
      // Listen for scroll to persist position (debounced lightly)
      try {
        let lastScrollEv = 0;
        editor.onDidScrollChange((e: any) => {
          const now = performance.now();
          if (now - lastScrollEv > 50) {
            lastScrollEv = now;
            try { setScrollPosition(doc.uri, editor.getScrollTop()); } catch {}
          }
        });
        editor.onDidBlurEditorWidget(() => { try { setHadFocus(doc.uri, false); } catch {} });
        editor.onDidFocusEditorWidget(() => { try { setHadFocus(doc.uri, true); } catch {} });
      } catch {}
    }
    return () => {
      if (hoverDisposableRef.current) {
        try { hoverDisposableRef.current.dispose(); } catch {}
        hoverDisposableRef.current = null;
      }
    };
  }, [doc, scrollPositions, hadFocus, setScrollPosition, setHadFocus]);

  const handleChange: OnChange = useCallback((value) => {
    logger.debug('App', 'Monaco onChange called, value length:', value?.length, 'doc:', doc?.uri);
    if (doc && typeof value === 'string') {
      logger.debug('App', 'Monaco Updating content for:', doc.uri);
      // Check if editor has focus before update
      const hadFocusBeforeUpdate = editorRef.current?.hasTextFocus();
      logger.debug('App', 'Monaco hadFocusBeforeUpdate:', hadFocusBeforeUpdate);
      
      updateContent(doc.uri, value);
      // Don't duplicate lspClient.didChange - it's already called in updateContent
      
      // Restore focus if it was lost during update
      if (hadFocusBeforeUpdate) {
        requestAnimationFrame(() => {
          const stillHasFocus = editorRef.current?.hasTextFocus();
          logger.debug('App', 'Monaco stillHasFocus after update:', stillHasFocus);
          if (!stillHasFocus && editorRef.current) {
            logger.debug('App', 'Monaco restoring focus after content update');
            editorRef.current.focus();
          }
        });
      }
    }
  }, [doc, updateContent]);

  // When switching documents, ensure model with correct URI is bound
  useEffect(() => {
    if (monacoRef.current && editorRef.current && doc) {
      const monaco = monacoRef.current;
      const mUri = monaco.Uri.parse(doc.uri);
      let model = monaco.editor.getModel(mUri);
      if (!model) {
        model = monaco.editor.createModel(doc.content, 'vpy', mUri);
      } else if (model.getValue() !== doc.content) {
        model.setValue(doc.content);
      }
      if (lastModelRef.current !== doc.uri) {
        editorRef.current.setModel(model);
        lastModelRef.current = doc.uri;
        // Restore scroll & focus after switching
        try {
          const top = scrollPositions[doc.uri];
          if (typeof top === 'number') {
            requestAnimationFrame(() => editorRef.current?.setScrollTop(top));
          }
          if (hadFocus[doc.uri]) {
            requestAnimationFrame(() => editorRef.current?.focus());
          }
        } catch {}
      }
    }
  }, [doc?.uri, doc?.content, scrollPositions, hadFocus]);

  useEffect(() => {
    const unsub = dockBus.on(ev => {
      if (ev.type === 'changed') {
        if (editorRef.current) {
          // slight delay to allow layout container to settle
          requestAnimationFrame(() => editorRef.current.layout());
        }
      }
    });
    return () => { unsub(); };
  }, []);

  // Subscribe to publishDiagnostics once
  useEffect(() => {
    const handler = (method: string, params: any) => {
      if (method === 'textDocument/publishDiagnostics' && monacoRef.current) {
        const { uri, diagnostics } = params || {};
        const rawUri: string = uri || '';
        const cleaned = rawUri.replace(/\\/g,'/');
        // Normalize: ensure triple slash after scheme for Windows paths, lowercase drive letter for matching
        let norm = cleaned;
        // file:///C:/ -> drive letter uppercase in Monaco normally; we will compare case-insensitive
        if (/^file:\/\/[A-Za-z]:\//.test(norm)) {
          // Add extra slash to make file:/// if only two
          norm = norm.replace(/^file:\/\//,'file:///');
        } else if (/^file:\/\/[A-Za-z]:\//.test(norm) === false && /^file:\/\/[A-Za-z]:/.test(norm) === false) {
          // leave others
        }
        const lcNorm = norm.toLowerCase();
        const models = monacoRef.current.editor.getModels();
        let model = models.find(m => m.uri.toString().toLowerCase() === lcNorm);
        if (!model) {
          // Fallback: try collapsing multiple slashes or adding one
            const variants = [
              lcNorm.replace('file:////','file:///'),
              lcNorm.replace('file:///','file://'),
              lcNorm.replace(/^file:\/\//,'file:///')
            ];
            model = models.find(m => variants.includes(m.uri.toString().toLowerCase()));
        }
          if (!model) {
            // Path-based loose match: compare file path tail ignoring case
            try {
              const rawPath = lcNorm.replace('file:///','').replace('file://','');
              const tail = rawPath.split('/') .slice(-3).join('/'); // last 3 segments heuristic
              model = models.find(m => m.uri.toString().toLowerCase().endsWith(tail));
            } catch {}
          }
          if (!model) {
            // As last resort, if this doc is currently active in store with same rawUri (ignoring case), recreate model so we can show markers.
            try {
              const st: any = (useEditorStore as any).getState();
              const docMatch = st.documents.find((d: any) => d.uri.toLowerCase() === lcNorm);
              if (docMatch && monacoRef.current) {
                const mUri = monacoRef.current.Uri.parse(docMatch.uri);
                model = monacoRef.current.editor.createModel(docMatch.content, 'vpy', mUri);
                logger.verbose('LSP', 'recreated missing model for:', docMatch.uri);
              }
            } catch {}
          }
        logger.verbose('LSP', 'diagnostics received uri=', rawUri, 'norm=', lcNorm, 'count=', (diagnostics||[]).length, 'matchedModel=', !!model, 'models=', models.map(m=>m.uri.toString()));
        // Note: Store update is handled by global handler in main.tsx to avoid duplication
        // This handler only applies visual markers to Monaco editor
        if (!model) return; // markers only for open model
        const markers = (diagnostics || []).map((d: any) => ({
          severity: severityToMonaco(d.severity, monacoRef.current!),
            message: d.message,
            startLineNumber: d.range.start.line + 1,
            startColumn: d.range.start.character + 1,
            endLineNumber: d.range.end.line + 1,
            endColumn: d.range.end.character + 1,
            source: d.source || 'vpy'
        }));
        monacoRef.current.editor.setModelMarkers(model, 'vpy', markers);
        logger.debug('LSP', 'applied markers:', markers.length);
      }
    };
    lspClient.onNotification(handler);
  }, []);

  // Listen for goto events to move cursor
  useEffect(() => {
    const listener = (e: any) => {
      if (!editorRef.current || !monacoRef.current) return;
      const { uri, line, column } = e.detail || {};
      if (!uri || line===undefined || column===undefined) return;
      const monaco = monacoRef.current;
      const model = monaco.editor.getModel(monaco.Uri.parse(uri));
      if (model) {
        editorRef.current.setModel(model);
        lastModelRef.current = uri;
        const position = { lineNumber: line + 1, column: column + 1 };
        editorRef.current.revealLineInCenter(position.lineNumber);
        editorRef.current.setPosition(position);
        editorRef.current.focus();
      }
    };
    window.addEventListener('vpy.goto', listener as any);
    return () => window.removeEventListener('vpy.goto', listener as any);
  }, []);

  // Update breakpoint decorations when breakpoints change
  useEffect(() => {
    if (!editorRef.current || !monacoRef.current || !doc) return;
    
    const bps = breakpoints[doc.uri] || new Set<number>();
    const decorations = Array.from(bps).map(lineNumber => ({
      range: new monacoRef.current!.Range(lineNumber, 1, lineNumber, 1),
      options: {
        isWholeLine: false,
        glyphMarginClassName: 'breakpoint-glyph',
        glyphMarginHoverMessage: { value: 'Breakpoint' }
      }
    }));
    
    breakpointDecorationsRef.current = editorRef.current.deltaDecorations(
      breakpointDecorationsRef.current,
      decorations
    );
  }, [breakpoints, doc?.uri]);

  // Keyboard shortcuts for breakpoints
  useEffect(() => {
    if (!editorRef.current || !monacoRef.current || !doc) return;
    
    const editor = editorRef.current;
    const monaco = monacoRef.current;
    
    // Store the current doc URI in closure to avoid stale references
    const currentUri = doc.uri;
    
    // F9: Toggle breakpoint
    const f9Disposable = editor.addCommand(monaco.KeyCode.F9, () => {
      const position = editor.getPosition();
      if (position) {
        toggleBreakpoint(currentUri, position.lineNumber);
        logger.debug('App', `F9 pressed - toggled breakpoint at line ${position.lineNumber}`);
      }
    });
    
    // Ctrl+Shift+F9: Clear all breakpoints
    const clearAllDisposable = editor.addCommand(
      monaco.KeyMod.CtrlCmd | monaco.KeyMod.Shift | monaco.KeyCode.F9,
      () => {
        const bps = breakpoints[currentUri] || new Set<number>();
        const count = bps.size;
        
        if (count === 0) {
          logger.debug('App', 'Ctrl+Shift+F9 pressed - no breakpoints to clear');
          return;
        } else if (count === 1) {
          clearAllBreakpoints(currentUri);
          logger.debug('App', 'Ctrl+Shift+F9 pressed - cleared 1 breakpoint');
        } else {
          const confirmed = confirm(`Delete all ${count} breakpoints in this file?`);
          if (confirmed) {
            clearAllBreakpoints(currentUri);
            logger.debug('App', `Ctrl+Shift+F9 pressed - cleared ${count} breakpoints`);
          }
        }
      }
    );
    
    // Cleanup: Monaco addCommand doesn't return disposable, but we log registration
    logger.debug('App', `F9 shortcuts registered for ${currentUri}`);
    
    return () => {
      // No cleanup needed - Monaco commands are editor-scoped
      logger.debug('App', `F9 shortcuts cleanup for ${currentUri}`);
    };
  }, [doc?.uri, toggleBreakpoint, clearAllBreakpoints, breakpoints]);

  // Gutter (margin) click handler for breakpoints
  useEffect(() => {
    if (!editorRef.current || !doc) return;
    
    const editor = editorRef.current;
    const disposable = editor.onMouseDown((e: any) => {
      // Check if click is in the glyph margin (where breakpoints appear)
      if (e.target?.type === 2) { // GUTTER_GLYPH_MARGIN = 2
        const lineNumber = e.target.position?.lineNumber;
        if (lineNumber) {
          toggleBreakpoint(doc.uri, lineNumber);
        }
      }
    });
    
    return () => disposable?.dispose();
  }, [doc?.uri, toggleBreakpoint]);

  useEffect(() => {
    const w: any = window as any;
    if (!w.electronAPI || !editorRef.current || !monacoRef.current) return;
    const handler = (cmd: string, payload?: any) => {
      const editor = editorRef.current;
      const monaco = monacoRef.current!;
      switch (cmd) {
        case 'indent': editor.trigger('menu','editor.action.indentLines',null); break;
        case 'outdent': editor.trigger('menu','editor.action.outdentLines',null); break;
        case 'toggle-line-comment': editor.trigger('menu','editor.action.commentLine',null); break;
        case 'toggle-block-comment': editor.trigger('menu','editor.action.blockComment',null); break;
        case 'new-file': {
          // Basic new unsaved buffer
          const uri = monaco.Uri.parse(`inmemory://untitled-${Date.now()}.vpy`);
          const model = monaco.editor.createModel('# New File\n', 'vpy', uri);
          editor.setModel(model); break;
        }
        default: break;
      }
    };
    w.electronAPI.onCommand(handler);
  }, []);

  if (!doc) {
    return <div style={{ padding:16, color:'#666' }}>{t('editor:noFile')}</div>;
  }

  // When this wrapper is tied to a specific uri prop and the tab gains focus, ensure active doc updates
  useEffect(() => {
    if (uri && doc && active !== doc.uri) {
      // Passive: do not forcibly override global active unless this tab's model is shown (heuristic: small timeout after mount)
      const t = setTimeout(() => {
        try { setActive(doc.uri); } catch {}
      }, 0);
      return () => clearTimeout(t);
    }
  }, [uri, doc?.uri]);

  // Lazy load content if this is a restored placeholder (empty content, has diskPath)
  useEffect(() => {
    if (doc && doc.diskPath && doc.content === '' && doc.lastSavedContent === '') {
      const api: any = (window as any).files;
      if (!api || !api.readFile) return;
      api.readFile(doc.diskPath).then((res: any) => {
        if (!res || res.error) return;
        const text = res.content || '';
        // Update content in store without marking dirty
        try {
          const st = (useEditorStore as any).getState();
          st.updateContent(doc.uri, text);
          st.markSaved(doc.uri, res.mtime);
        } catch {}
      });
    }
  }, [doc?.diskPath]);

  return (
    <Editor
      height="100%"
      defaultLanguage="vpy"
      language="vpy"
  theme="vpy-dark"
  // Model is managed manually; prevent internal re-create by not binding value each render
  value={undefined}
      onChange={handleChange}
      onMount={handleMount}
      beforeMount={beforeMount}
      options={{
        automaticLayout: true,
        minimap: { enabled: false },
        fontSize: 14,
        scrollBeyondLastLine: false,
        wordWrap: 'on',
        wordBasedSuggestions: 'off',
        hover: { enabled: true, delay: 150 },
        'semanticHighlighting.enabled': true,
        quickSuggestions: { other: true, strings: false, comments: false },
        suggestOnTriggerCharacters: true,
        renderValidationDecorations: 'on',
        glyphMargin: true, // Enable glyph margin for breakpoints
        lineNumbersMinChars: 3 // Make room for line numbers + glyph margin
      }}
    />
  );
};

function severityToMonaco(lspSeverity: number | undefined, monaco: Monaco) {
  const S = monaco.MarkerSeverity;
  switch (lspSeverity) {
    case 1: return S.Error;
    case 2: return S.Warning;
    case 3: return S.Info;
    case 4: return S.Hint;
    default: return S.Info;
  }
}

function lspSeverityToText(sev: number | undefined): 'error' | 'warning' | 'info' {
  switch (sev) {
    case 1: return 'error';
    case 2: return 'warning';
    default: return 'info';
  }
}
