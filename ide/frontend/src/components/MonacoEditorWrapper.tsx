import React, { useCallback, useEffect, useRef } from 'react';
import { useEditorStore } from '../state/editorStore';
import Editor, { OnChange, Monaco } from '@monaco-editor/react';
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

  const handleMount = useCallback((editor: any, monaco: Monaco) => {
    ensureLanguage(monaco);
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
