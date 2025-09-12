import { create } from 'zustand';
import type { DocumentModel, DiagnosticModel } from '../types/models';

interface EditorState {
  documents: DocumentModel[];
  active?: string; // uri
  openDocument: (doc: DocumentModel) => void;
  setActive: (uri: string) => void;
  updateContent: (uri: string, content: string) => void;
  setDiagnostics: (uri: string, diags: DiagnosticModel[]) => void;
  closeDocument: (uri: string) => void;
}

export const useEditorStore = create<EditorState>((set, get) => ({
  documents: [],
  active: undefined,
  openDocument: (doc) => set((s) => ({ documents: [...s.documents, doc], active: doc.uri })),
  setActive: (uri) => set({ active: uri }),
  updateContent: (uri, content) => set((s) => ({
    documents: s.documents.map(d => d.uri === uri ? { ...d, content, dirty: true } : d)
  })),
  setDiagnostics: (uri, diags) => set((s) => ({
    documents: s.documents.map(d => d.uri === uri ? { ...d, diagnostics: diags } : d)
  })),
  closeDocument: (uri) => set((s) => ({
    documents: s.documents.filter(d => d.uri !== uri),
    active: s.active === uri ? undefined : s.active
  })),
}));
