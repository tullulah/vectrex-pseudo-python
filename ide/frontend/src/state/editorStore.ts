import { create } from 'zustand';
import type { DocumentModel, DiagnosticModel } from '../types/models';
import { lspClient } from '../lspClient';

interface EditorState {
  documents: DocumentModel[];
  active?: string; // uri
  openDocument: (doc: DocumentModel) => void;
  setActive: (uri: string) => void;
  updateContent: (uri: string, content: string) => void;
  setDiagnostics: (uri: string, diags: DiagnosticModel[]) => void;
  closeDocument: (uri: string) => void;
  // Flattened diagnostics across all docs (derived via get())
  getAllDiagnostics: () => Array<{
    uri: string; file: string; line: number; column: number; severity: DiagnosticModel['severity']; message: string;
  }>;
  // Navigate to specific location (open if not already)
  gotoLocation: (uri: string, line: number, column: number) => void;
}

export const useEditorStore = create<EditorState>((set, get) => ({
  documents: [],
  active: undefined,
  openDocument: (doc) => set((s) => ({ documents: [...s.documents, doc], active: doc.uri })),
  setActive: (uri) => set({ active: uri }),
  updateContent: (uri, content) => {
    set((s) => ({
      documents: s.documents.map(d => d.uri === uri ? { ...d, content, dirty: true } : d)
    }));
    // Fire didChange (best effort; if not started yet, no-op inside client)
    try { lspClient.didChange(uri, content); } catch (_) {}
  },
  setDiagnostics: (uri, diags) => set((s) => ({
    documents: s.documents.map(d => d.uri === uri ? { ...d, diagnostics: diags } : d)
  })),
  closeDocument: (uri) => set((s) => ({
    documents: s.documents.filter(d => d.uri !== uri),
    active: s.active === uri ? undefined : s.active
  })),
  getAllDiagnostics: () => {
    const s = get();
    const rows: Array<{uri:string; file:string; line:number; column:number; severity: DiagnosticModel['severity']; message:string}> = [];
    for (const d of s.documents) {
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
    // Sort: errors first, then warnings, then info; then file; then line
    const sevOrder: Record<string, number> = { error: 0, warning: 1, info: 2 } as any;
    rows.sort((a,b) => {
      const so = (sevOrder[a.severity]??9) - (sevOrder[b.severity]??9); if (so!==0) return so;
      const f = a.file.localeCompare(b.file); if (f!==0) return f;
      return a.line - b.line || a.column - b.column;
    });
    return rows;
  },
  gotoLocation: (uri, line, column) => {
    const s = get();
    // If document not open, cannot load content from server yet; create a placeholder (could request via lsp in future)
    if (!s.documents.some(d => d.uri === uri)) {
      set({ documents: [...s.documents, { uri, language: 'vpy', content: '', dirty: false, diagnostics: [] }], active: uri });
    } else {
      set({ active: uri });
    }
    // Fire a custom event so Monaco wrapper can reveal position (avoid direct coupling)
    const ev = new CustomEvent('vpy.goto', { detail: { uri, line, column } });
    window.dispatchEvent(ev);
  }
}));
