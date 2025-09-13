import { contextBridge, ipcRenderer, IpcRendererEvent } from 'electron';

contextBridge.exposeInMainWorld('electronAPI', {
  lspStart: () => ipcRenderer.invoke('lsp_start'),
  lspSend: (payload: string) => ipcRenderer.invoke('lsp_send', payload),
  onLspMessage: (cb: (json: string) => void) => ipcRenderer.on('lsp://message', (_e: IpcRendererEvent, data: string) => cb(data)),
  onLspStdout: (cb: (line: string) => void) => ipcRenderer.on('lsp://stdout', (_e: IpcRendererEvent, data: string) => cb(data)),
  onLspStderr: (cb: (line: string) => void) => ipcRenderer.on('lsp://stderr', (_e: IpcRendererEvent, data: string) => cb(data)),
});

export {};