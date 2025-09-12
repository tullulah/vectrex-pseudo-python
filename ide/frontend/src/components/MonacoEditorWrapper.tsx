import React, { useCallback, useEffect, useRef } from 'react';
import { useEditorStore } from '../state/editorStore';
import Editor, { OnChange, Monaco } from '@monaco-editor/react';
import { dockBus } from '../state/dockBus';
import { lspClient } from '../lspClient';
// TODO(i18n): Adapt Monaco UI strings (context menu, messages) when supporting dynamic locale changes.

// Simple language placeholder registration for 'vpy'
function ensureLanguage(monaco: Monaco) {
  if ((monaco.languages.getLanguages() || []).some(l => l.id === 'vpy')) return;
  monaco.languages.register({ id: 'vpy' });
  monaco.languages.setMonarchTokensProvider('vpy', {
    // Extremely minimal placeholder tokenizer; refine later
    tokenizer: {
      root: [
        [/\b(DEF|RETURN|IF|ELSE|WHILE|FOR)\b/i, 'keyword'],
        [/\b(PLOT|LINE|CIRCLE|POLYGON|TEXT)\b/i, 'type.identifier'],
        [/#[^$]*/, 'comment'],
        [/".*?"|'.*?'/, 'string'],
        [/[0-9]+/, 'number'],
        [/[-+/*=<>!]+/, 'operator']
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

export const MonacoEditorWrapper: React.FC = () => {
  const { documents, active, updateContent } = useEditorStore(s => ({
    documents: s.documents,
    active: s.active,
    updateContent: s.updateContent
  }));

  const doc = documents.find(d => d.uri === active);
  const lastModelRef = useRef<string | undefined>();

  const editorRef = useRef<any>(null);
  const monacoRef = useRef<Monaco | null>(null);
  const handleMount = useCallback((editor: any, monaco: Monaco) => {
    ensureLanguage(monaco);
    editorRef.current = editor;
    monacoRef.current = monaco;
  }, []);

  const handleChange: OnChange = useCallback((value) => {
    if (doc && typeof value === 'string') {
      updateContent(doc.uri, value);
    }
  }, [doc, updateContent]);

  // When switching documents, ensure Monaco updates the model value
  useEffect(() => {
    // Nothing special here yet; Editor component will re-render with new value prop
  }, [doc?.uri]);

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
        const { uri, diagnostics } = params;
        const model = monacoRef.current.editor.getModels().find(m => m.uri.toString() === uri);
        if (!model) return;
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

  if (!doc) {
    return <div style={{padding:16, color:'#666'}}>No document open</div>;
  }

  return (
    <Editor
      height="100%"
      defaultLanguage="vpy"
      language="vpy"
      theme="vs-dark"
      value={doc.content}
      onChange={handleChange}
      onMount={handleMount}
      options={{
        automaticLayout: true,
        minimap: { enabled: false },
        fontSize: 14,
        scrollBeyondLastLine: false,
        wordWrap: 'on'
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
