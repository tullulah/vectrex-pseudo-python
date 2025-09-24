import { contextBridge, ipcRenderer, IpcRendererEvent } from 'electron';

contextBridge.exposeInMainWorld('electronAPI', {
  lspStart: () => ipcRenderer.invoke('lsp_start'),
  lspSend: (payload: string) => ipcRenderer.invoke('lsp_send', payload),
  onLspMessage: (cb: (json: string) => void) => ipcRenderer.on('lsp://message', (_e: IpcRendererEvent, data: string) => cb(data)),
  onLspStdout: (cb: (line: string) => void) => ipcRenderer.on('lsp://stdout', (_e: IpcRendererEvent, data: string) => cb(data)),
  onLspStderr: (cb: (line: string) => void) => ipcRenderer.on('lsp://stderr', (_e: IpcRendererEvent, data: string) => cb(data)),
  onCommand: (cb: (cmd: string, payload?: any) => void) => ipcRenderer.on('command', (_e, cmd, payload) => cb(cmd, payload)),
  // Legacy emulator IPC removed. All runtime control now via WASM service in renderer.
  emuAssemble: (args: { asmPath: string; outPath?: string; extra?: string[] }) => ipcRenderer.invoke('emu:assemble', args) as Promise<{ ok?: boolean; error?: string; binPath?: string; size?: number; base64?: string; stdout?: string; stderr?: string }>,
  runCompile: (args: { path: string; saveIfDirty?: { content: string; expectedMTime?: number }; autoStart?: boolean }) => ipcRenderer.invoke('run:compile', args) as Promise<{ ok?: boolean; error?: string; binPath?: string; size?: number; stdout?: string; stderr?: string; conflict?: boolean; currentMTime?: number }>,
  onRunStdout: (cb: (chunk: string) => void) => ipcRenderer.on('run://stdout', (_e: IpcRendererEvent, data: string) => cb(data)),
  onRunStderr: (cb: (chunk: string) => void) => ipcRenderer.on('run://stderr', (_e: IpcRendererEvent, data: string) => cb(data)),
  onRunDiagnostics: (cb: (diags: Array<{ file: string; line: number; col: number; message: string }>) => void) => ipcRenderer.on('run://diagnostics', (_e: IpcRendererEvent, diags) => cb(diags)),
  onRunStatus: (cb: (line: string) => void) => ipcRenderer.on('run://status', (_e: IpcRendererEvent, data: string) => cb(data)),
  onEmuLoaded: (cb: (info: { size: number }) => void) => ipcRenderer.on('emu://loaded', (_e: IpcRendererEvent, data) => cb(data)), // kept for backward compatibility (may be unused)
  onCompiledBin: (cb: (payload: { base64: string; size: number; binPath: string }) => void) => ipcRenderer.on('emu://compiledBin', (_e: IpcRendererEvent, data) => cb(data)),
  // setVectorMode legacy removed
  listSources: (args?: { limit?: number }) => ipcRenderer.invoke('list:sources', args) as Promise<{ ok?:boolean; sources?: Array<{ path:string; kind:'vpy'|'asm'; size:number; mtime:number }> }> ,
});

contextBridge.exposeInMainWorld('files', {
  openFile: () => ipcRenderer.invoke('file:open') as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string } | null>,
  openFilePath: (path: string) => ipcRenderer.invoke('file:openPath', path) as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string } | null>,
  saveFile: (args: { path: string; content: string; expectedMTime?: number }) => ipcRenderer.invoke('file:save', args) as Promise<{ path: string; mtime: number; size: number } | { conflict: true; currentMTime: number } | { error: string }>,

  saveFileAs: (args: { suggestedName?: string; content: string }) => ipcRenderer.invoke('file:saveAs', args) as Promise<{ path: string; mtime: number; size: number; name: string } | { canceled: true } | { error: string }>,

  openFolder: () => ipcRenderer.invoke('file:openFolder') as Promise<{ path: string } | null>,
  readFile: (path: string) => ipcRenderer.invoke('file:read', path) as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string }>,
  readFileBin: (path: string) => ipcRenderer.invoke('file:readBin', path) as Promise<{ path: string; base64: string; size: number; name: string } | { error: string }>,
  openBin: () => ipcRenderer.invoke('bin:open') as Promise<{ path: string; base64: string; size: number } | { error: string } | null>,
});

contextBridge.exposeInMainWorld('recents', {
  load: () => ipcRenderer.invoke('recents:load') as Promise<Array<{ path: string; lastOpened: number; kind: 'file' | 'folder' }>>,
  write: (list: Array<{ path: string; lastOpened: number; kind: 'file' | 'folder' }>) => ipcRenderer.invoke('recents:write', list) as Promise<{ ok: boolean }>,
});

export {};