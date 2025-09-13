import React, { useCallback, useEffect, useRef } from 'react';
import { useEditorStore } from '../state/editorStore';
import Editor, { OnChange, Monaco, BeforeMount } from '@monaco-editor/react';
// Import full ESM Monaco API and expose it globally so the react wrapper skips AMD loader.js
import * as monacoApi from 'monaco-editor/esm/vs/editor/editor.api';
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
export const MonacoEditorWrapper: React.FC = () => {
  // Use individual selectors to avoid creating a fresh object each render (React 19 getSnapshot loop guard)
  const documents = useEditorStore(s => s.documents);
  const active = useEditorStore(s => s.active);
  const updateContent = useEditorStore(s => s.updateContent);
  const setDiagnostics = useEditorStore(s => s.setDiagnostics);

  const doc = documents.find(d => d.uri === active);
  const lastModelRef = useRef<string | undefined>(undefined);

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
        } catch (e) { console.warn('[LSP] semantic tokens error', e); return { data: new Uint32Array() } as any; }
      },
      releaseDocumentSemanticTokens: () => {}
    });
    // LSP-backed completion provider
    monaco.languages.registerCompletionItemProvider('vpy', {
      triggerCharacters: ['_', '(', ',', ' '],
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
          if (!res) return { suggestions: [] };
          const items = Array.isArray(res.items) ? res.items : (Array.isArray(res) ? res : []);
          const suggestions = items.map((it: any) => ({
            label: it.label,
            kind: monaco.languages.CompletionItemKind.Function,
            insertText: it.insertText || it.label,
            range: undefined
          }));
          return { suggestions };
        } catch (e) {
          console.warn('[LSP] completion error', e);
          return { suggestions: [] };
        }
      }
    });
    // Hover provider
    monaco.languages.registerHoverProvider('vpy', {
      provideHover: async (model, position) => {
        try {
          const uri = model.uri.toString();
          const params = { textDocument: { uri }, position: { line: position.lineNumber - 1, character: position.column - 1 } };
          const res = await (lspClient as any).request('textDocument/hover', params);
          if (res && res.contents) {
            const contents = typeof res.contents === 'string' ? res.contents : (res.contents.value || '');
            return { contents: [{ value: contents }], range: undefined } as any;
          }
        } catch (e) { console.warn('[LSP] hover error', e); }
        return null as any;
      }
    });
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
      const uri = monaco.Uri.parse(doc.uri);
      let model = monaco.editor.getModel(uri);
      if (!model) {
        model = monaco.editor.createModel(doc.content, 'vpy', uri);
      }
      editor.setModel(model);
      lastModelRef.current = doc.uri;
      // Trigger an initial didChange to encourage semanticTokens/full soon after mount
      lspClient.didChange(doc.uri, model.getValue());
    }
  }, [doc]);

  const handleChange: OnChange = useCallback((value) => {
    if (doc && typeof value === 'string') {
      updateContent(doc.uri, value);
    }
  }, [doc, updateContent]);

  // When switching documents, ensure model with correct URI is bound
  useEffect(() => {
    if (monacoRef.current && editorRef.current && doc) {
      const monaco = monacoRef.current;
      const uri = monaco.Uri.parse(doc.uri);
      let model = monaco.editor.getModel(uri);
      if (!model) {
        model = monaco.editor.createModel(doc.content, 'vpy', uri);
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
        console.debug('[LSP] diagnostics received', params);
        const { uri, diagnostics } = params;
        const model = monacoRef.current.editor.getModels().find(m => m.uri.toString() === uri);
        // Always update store even if model not currently open (will aggregate in Errors panel)
        const diagsForStore = (diagnostics || []).map((d: any) => ({
          message: d.message,
          severity: lspSeverityToText(d.severity),
          line: d.range.start.line,
          column: d.range.start.character
        }));
        try { setDiagnostics(uri, diagsForStore as any); } catch (_) {}
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
    return <div style={{padding:16, color:'#666'}}>No document open</div>;
  }

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
        'semanticHighlighting.enabled': true,
        quickSuggestions: { other: true, strings: false, comments: false }
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
