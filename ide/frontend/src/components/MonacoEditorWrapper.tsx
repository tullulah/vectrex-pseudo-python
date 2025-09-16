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
          // Keywords (control/meta)
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
  const setDiagnostics = useEditorStore(s => s.setDiagnostics);

  const targetUri = uri || active;
  const doc = documents.find(d => d.uri === targetUri);
  const lastModelRef = useRef<string | undefined>(undefined);
  // Keep track of hover provider so we can dispose on remount (avoid duplicate hover tooltips)
  const hoverDisposableRef = useRef<any>(null);

  const editorRef = useRef<any>(null);
  const monacoRef = useRef<Monaco | null>(null);
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
    ensureLanguage(monaco);
    editorRef.current = editor;
    monacoRef.current = monaco;
  monaco.editor.setTheme('vpy-dark');
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
            console.warn('[LSP] semantic tokens error (silenced after first)', e);
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
          console.warn('[LSP] completion error', e);
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
        } catch (e) { console.warn('[LSP] rename error', e); return { edits: [] } as any; }
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
        } catch (e) { console.warn('[LSP] signatureHelp error', e); return { value: { signatures: [], activeParameter: 0, activeSignature: 0 }, dispose: () => {} }; }
      }
    });
    // Hover provider (dispose any previous instance first to avoid stacking)
    if (hoverDisposableRef.current) {
      try { hoverDisposableRef.current.dispose(); } catch {}
      hoverDisposableRef.current = null;
    }
    hoverDisposableRef.current = monaco.languages.registerHoverProvider('vpy', {
      provideHover: async (model, position) => {
        console.debug('[LSP][hover] trigger', model.uri.toString(), position.lineNumber, position.column);
        try {
          const uri = model.uri.toString();
          const params = { textDocument: { uri }, position: { line: position.lineNumber - 1, character: position.column - 1 } };
          const res = await (lspClient as any).request('textDocument/hover', params);
          if (res && res.contents) {
            const contents = typeof res.contents === 'string' ? res.contents : (res.contents.value || '');
            console.debug('[LSP][hover] response', contents);
            return { contents: [{ value: contents }], range: undefined } as any;
          }
        } catch (e) { console.warn('[LSP] hover error', e); }
        console.debug('[LSP][hover] empty');
        return null as any;
      }
    });
    console.debug('[LSP][hover] provider registered');
    // Extra instrumentation: log mouse move & model language to diagnose missing hover triggers
    try {
      const lang = editor.getModel()?.getLanguageId();
      console.debug('[hover-debug] current model languageId=', lang, 'uri=', editor.getModel()?.uri.toString());
      let lastLog = 0;
      editor.onMouseMove((e: any) => {
        const now = performance.now();
        if (now - lastLog < 250) return; // throttle
        lastLog = now;
        const pos = e.target?.position ? `${e.target.position.lineNumber}:${e.target.position.column}` : 'n/a';
        console.debug('[hover-debug] mouseMove target=', e.target?.type, 'pos=', pos, 'detail=', e.target?.detail || '');
      });
      // Force re-apply hover enabled in case wrapper options lost
      editor.updateOptions({ hover: { enabled: true, delay: 150 } });
      // Fallback hover removed (native Monaco hover now active). If needed, reintroduce with a feature flag.
    } catch (e) { console.warn('[hover-debug] instrumentation error', e); }
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
        } catch (e) { console.warn('[LSP] definition error', e); }
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
      // Trigger an initial didChange to encourage semanticTokens/full soon after mount
      if (!openedRef.current.has(doc.uri)) {
        try { lspClient.didOpen(doc.uri, 'vpy', model.getValue()); openedRef.current.add(doc.uri); } catch {}
      } else {
        lspClient.didChange(doc.uri, model.getValue());
      }
    }
    return () => {
      if (hoverDisposableRef.current) {
        try { hoverDisposableRef.current.dispose(); } catch {}
        hoverDisposableRef.current = null;
      }
    };
  }, [doc]);

  const handleChange: OnChange = useCallback((value) => {
    if (doc && typeof value === 'string') {
      updateContent(doc.uri, value);
      try { lspClient.didChange(doc.uri, value); } catch {}
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
      }
    }
  }, [doc?.uri, doc?.content]);

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
                console.debug('[LSP][diag] recreated missing model for', docMatch.uri);
              }
            } catch {}
          }
        console.debug('[LSP] diagnostics received uri=', rawUri, 'norm=', lcNorm, 'count=', (diagnostics||[]).length, 'matchedModel=', !!model, 'models=', models.map(m=>m.uri.toString()));
        // Always update store even if model not currently open (will aggregate in Errors panel)
        const diagsForStore = (diagnostics || []).map((d: any) => ({
          message: d.message,
          severity: lspSeverityToText(d.severity),
          line: d.range.start.line,
          column: d.range.start.character
        }));
        try {
          const storeUri = model ? model.uri.toString() : uri;
          if (model && storeUri !== uri) {
            console.debug('[LSP][diag] storeUri differs from raw uri', storeUri, uri);
          }
          setDiagnostics(storeUri, diagsForStore as any);
        } catch (_) {}
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
        console.debug('[LSP] applied markers', markers.length);
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
        renderValidationDecorations: 'on'
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
