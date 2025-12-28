// Persistence layer for editor documents (open tabs, order, pinned, hidden, active)
// Uses localStorage for now. Could be swapped to filesystem (Electron userData) later.

import { useEditorStore } from './editorStore.js';
import type { DocumentModel } from '../types/models.js';

const LS_KEY = 'vpy.editor.state.v1';

interface PersistShape {
  active?: string;
  docs: Array<{
    uri: string;
    content: string; // only for unsaved in-memory or dirty files (optional for clean disk-backed)
    diskPath?: string;
    mtime?: number;
    dirty?: boolean;
    lastSavedContent?: string;
  }>;
}

let restoring = false;
export function isRestoring() { return restoring; }

export function restoreEditorState() {
  try {
    const raw = localStorage.getItem(LS_KEY);
    if (!raw) return;
    const parsed: PersistShape = JSON.parse(raw);
    restoring = true;
    const openDocument = useEditorStore.getState().openDocument;
    const setActive = useEditorStore.getState().setActive;

    for (const d of parsed.docs) {
      // Strategy: we always restore a content snapshot, even for disk files (could verify freshness later)
      openDocument({
        uri: d.uri,
        language: 'vpy',
        content: d.content || d.lastSavedContent || '',
        diagnostics: [],
        dirty: !!d.dirty,
        diskPath: d.diskPath,
        mtime: d.mtime,
        lastSavedContent: d.lastSavedContent,
      } as DocumentModel);
      // If dirty flag set but lastSavedContent present, ensure store's internal dirty calc matches
      if (d.dirty) {
        // Force dirty if mismatch cleared by openDocument logic
        useEditorStore.setState(s => ({
          documents: s.documents.map(doc => doc.uri === d.uri ? { ...doc, dirty: true } : doc)
        }));
      }
    }
    if (parsed.active) setActive(parsed.active);
    
    // After UI is stable, check for externally modified files asynchronously
    setTimeout(() => checkRestoredFilesForChanges(parsed.docs), 100);
  } catch (e) {
    console.warn('[editorPersistence] restore failed', e);
  } finally {
    restoring = false;
  }
}

// Check if restored files were modified on disk and reload them
function checkRestoredFilesForChanges(docs: Array<any>) {
  const w = typeof window !== 'undefined' ? window : undefined;
  const files = (w as any)?.files;
  if (!files?.getFileInfo || !files?.readFile) return;

  const updateContent = useEditorStore.getState().updateContent;

  docs.forEach((d) => {
    if (!d.diskPath) return; // Skip in-memory files

    files.getFileInfo(d.diskPath).then((infoResult: any) => {
      if (infoResult.ok && infoResult.mtime !== d.mtime) {
        // File changed on disk - reload fresh content
        console.log('[editorPersistence] File changed on disk, reloading:', d.diskPath);
        files.readFile(d.diskPath).then((readResult: any) => {
          if (!readResult.error && readResult.content !== undefined) {
            // Update content and mark as not dirty
            updateContent(d.uri, readResult.content);
            useEditorStore.setState(s => ({
              documents: s.documents.map(doc => 
                doc.uri === d.uri 
                  ? { ...doc, dirty: false, mtime: infoResult.mtime, lastSavedContent: readResult.content }
                  : doc
              )
            }));
          }
        }).catch(() => {
          console.warn('[editorPersistence] Failed to reload changed file:', d.diskPath);
        });
      }
    }).catch(() => {
      // Silently ignore files that no longer exist
    });
  });
}

export function persistEditorState() {
  if (restoring) return; // skip during restore
  try {
    const { documents, active } = useEditorStore.getState();
    const shape: PersistShape = {
      active,
      docs: documents.map(d => ({
        uri: d.uri,
        // Persist content if in-memory or dirty; otherwise optional (still keep to simplify for now)
        content: d.content,
        diskPath: d.diskPath,
        mtime: d.mtime,
        dirty: d.dirty,
        lastSavedContent: d.lastSavedContent
      }))
    };
    localStorage.setItem(LS_KEY, JSON.stringify(shape));
  } catch (e) {
    console.warn('[editorPersistence] persist failed', e);
  }
}

// Helper to initialize subscription once (call in main.tsx)
let subscribed = false;
export function ensureEditorPersistence() {
  if (subscribed) return;
  subscribed = true;
  // Subscribe to changes of documents or active
  const selector = (s: ReturnType<typeof useEditorStore.getState>) => ({ docs: s.documents, active: s.active });
  let prev = selector(useEditorStore.getState());
  const unsub = useEditorStore.subscribe((state, prevState) => {
    const next = selector(state as any);
    if (next.docs !== prev.docs || next.active !== prev.active) {
      persistEditorState();
    }
    prev = next;
  });
  // Optionally expose unsubscribe
  (window as any).__editorPersistUnsub = unsub;
}
