import { contextBridge, ipcRenderer, IpcRendererEvent } from 'electron';

contextBridge.exposeInMainWorld('electronAPI', {
  lspStart: () => ipcRenderer.invoke('lsp_start'),
  lspSend: (payload: string) => ipcRenderer.invoke('lsp_send', payload),
  onLspMessage: (cb: (json: string) => void) => ipcRenderer.on('lsp://message', (_e: IpcRendererEvent, data: string) => cb(data)),
  onLspStdout: (cb: (line: string) => void) => ipcRenderer.on('lsp://stdout', (_e: IpcRendererEvent, data: string) => cb(data)),
  onLspStderr: (cb: (line: string) => void) => ipcRenderer.on('lsp://stderr', (_e: IpcRendererEvent, data: string) => cb(data)),
  onCommand: (cb: (cmd: string, payload?: any) => void) => ipcRenderer.on('command', (_e, cmd, payload) => cb(cmd, payload)),
});

contextBridge.exposeInMainWorld('files', {
  openFile: () => ipcRenderer.invoke('file:open') as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string } | null>,
  openFilePath: (path: string) => ipcRenderer.invoke('file:openPath', path) as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string } | null>,
  saveFile: (args: { path: string; content: string; expectedMTime?: number }) => ipcRenderer.invoke('file:save', args) as Promise<{ path: string; mtime: number; size: number } | { conflict: true; currentMTime: number } | { error: string }>,

  saveFileAs: (args: { suggestedName?: string; content: string }) => ipcRenderer.invoke('file:saveAs', args) as Promise<{ path: string; mtime: number; size: number; name: string } | { canceled: true } | { error: string }>,

  openFolder: () => ipcRenderer.invoke('file:openFolder') as Promise<{ path: string } | null>,
  readFile: (path: string) => ipcRenderer.invoke('file:read', path) as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string }>,
});

contextBridge.exposeInMainWorld('recents', {
  load: () => ipcRenderer.invoke('recents:load') as Promise<Array<{ path: string; lastOpened: number; kind: 'file' | 'folder' }>>,
  write: (list: Array<{ path: string; lastOpened: number; kind: 'file' | 'folder' }>) => ipcRenderer.invoke('recents:write', list) as Promise<{ ok: boolean }>,
});

export {};