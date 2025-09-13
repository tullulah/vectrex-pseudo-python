import { create } from 'zustand';
import type { DocumentModel, DiagnosticModel } from '../types/models';
import { lspClient } from '../lspClient';

interface FlatDiag { uri: string; file: string; line: number; column: number; severity: DiagnosticModel['severity']; message: string; }
interface EditorState {
  documents: DocumentModel[];
  active?: string; // uri
  allDiagnostics: FlatDiag[]; // kept sorted & stable reference unless content changes
  openDocument: (doc: DocumentModel) => void;
  setActive: (uri: string) => void;
  updateContent: (uri: string, content: string) => void;
  setDiagnostics: (uri: string, diags: DiagnosticModel[]) => void;
  closeDocument: (uri: string) => void;
  gotoLocation: (uri: string, line: number, column: number) => void;
}

function recomputeAllDiagnostics(documents: DocumentModel[]): FlatDiag[] {
  const rows: FlatDiag[] = [];
  for (const d of documents) {
    for (const diag of d.diagnostics || []) {
      rows.push({
        uri: d.uri,
        file: d.uri.split('/').pop() || d.uri,
        line: diag.line,
        column: diag.column,
        severity: diag.severity,
        message: diag.message
      });
    }
  }
  const sevOrder: Record<string, number> = { error: 0, warning: 1, info: 2 } as any;
  rows.sort((a,b) => {
    const so = (sevOrder[a.severity]??9) - (sevOrder[b.severity]??9); if (so!==0) return so;
    const f = a.file.localeCompare(b.file); if (f!==0) return f;
    return a.line - b.line || a.column - b.column;
  });
  return rows;
}

export const useEditorStore = create<EditorState>((set, get) => ({
  documents: [],
  active: undefined,
  allDiagnostics: [],
  openDocument: (doc) => set((s) => {
    const documents = [...s.documents, doc];
    return { documents, active: doc.uri, allDiagnostics: recomputeAllDiagnostics(documents) };
  }),
  setActive: (uri) => set({ active: uri }),
  updateContent: (uri, content) => {
    set((s) => {
      const documents = s.documents.map(d => d.uri === uri ? { ...d, content, dirty: true } : d);
      return { documents, allDiagnostics: recomputeAllDiagnostics(documents) };
    });
    try { lspClient.didChange(uri, content); } catch (_) {}
  },
  setDiagnostics: (uri, diags) => set((s) => {
    const documents = s.documents.map(d => d.uri === uri ? { ...d, diagnostics: diags } : d);
    return { documents, allDiagnostics: recomputeAllDiagnostics(documents) };
  }),
  closeDocument: (uri) => set((s) => {
    const documents = s.documents.filter(d => d.uri !== uri);
    return { documents, active: s.active === uri ? undefined : s.active, allDiagnostics: recomputeAllDiagnostics(documents) };
  }),
  gotoLocation: (uri, line, column) => {
    const s = get();
    if (!s.documents.some(d => d.uri === uri)) {
  const documents: DocumentModel[] = [...s.documents, { uri, language: 'vpy', content: '', dirty: false, diagnostics: [] } as DocumentModel];
      set({ documents, active: uri, allDiagnostics: recomputeAllDiagnostics(documents) });
    } else {
      set({ active: uri });
    }
    const ev = new CustomEvent('vpy.goto', { detail: { uri, line, column } });
    window.dispatchEvent(ev);
  }
}));
