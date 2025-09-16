import { contextBridge, ipcRenderer, IpcRendererEvent } from 'electron';

contextBridge.exposeInMainWorld('electronAPI', {
  lspStart: () => ipcRenderer.invoke('lsp_start'),
  lspSend: (payload: string) => ipcRenderer.invoke('lsp_send', payload),
  onLspMessage: (cb: (json: string) => void) => ipcRenderer.on('lsp://message', (_e: IpcRendererEvent, data: string) => cb(data)),
  onLspStdout: (cb: (line: string) => void) => ipcRenderer.on('lsp://stdout', (_e: IpcRendererEvent, data: string) => cb(data)),
  onLspStderr: (cb: (line: string) => void) => ipcRenderer.on('lsp://stderr', (_e: IpcRendererEvent, data: string) => cb(data)),
  onCommand: (cb: (cmd: string, payload?: any) => void) => ipcRenderer.on('command', (_e, cmd, payload) => cb(cmd, payload)),
  emuLoad: (base64: string) => ipcRenderer.invoke('emu:load', { base64 }) as Promise<{ ok?: boolean; error?: string }>,
  emuRunFrame: () => ipcRenderer.invoke('emu:runFrame') as Promise<{ frameReady?: boolean; segments?: any[]; viaEvents?: Array<{pc:number;reg:number;val:number}>; debugTraces?: Array<any>; error?: string }>,
  emuGetPC: () => ipcRenderer.invoke('emu:getPC') as Promise<{ pc:number }>,
  emuSetPC: (pc:number) => ipcRenderer.invoke('emu:setPC', pc) as Promise<{ ok?:boolean; pc?:number }>,
  emuPeek: (addr:number, len?:number) => ipcRenderer.invoke('emu:peek', addr, len) as Promise<{ base:number; bytes:number[] }>,
  emuToggleTrace: (enabled?: boolean) => ipcRenderer.invoke('emu:toggleTrace', enabled) as Promise<{ traceEnabled:boolean }>,
    emuToggleAutoStart: (enabled?: boolean) => ipcRenderer.invoke('emu:autoStart', enabled) as Promise<{ autoStartEnabled:boolean }>,
    emuAutoStartInfo: () => ipcRenderer.invoke('emu:autoStartInfo') as Promise<{attempted:boolean; info?: {performed:boolean; reason:string} | null}>,
    emuToggleOpcodeTrace: (enabled?: boolean) => ipcRenderer.invoke('emu:toggleOpcodeTrace', enabled) as Promise<{opcodeTraceEnabled:boolean}>,
    emuRegs: () => ipcRenderer.invoke('emu:regs') as Promise<{a:number;b:number;x:number;y:number;u:number;s:number;pc:number;dp:number;}>,
    emuForceStart: () => ipcRenderer.invoke('emu:forceStart') as Promise<{pc:number}>,
    emuStatus: () => ipcRenderer.invoke('emu:status') as Promise<{biosPresent:boolean;vectorMode:string;autoStartUser:boolean;opcodeTraceEnabled:boolean;traceEnabled:boolean;}>,
    emuBiosStatus: () => ipcRenderer.invoke('emu:biosStatus') as Promise<{biosPresent:boolean}>,
    emuBiosReload: () => ipcRenderer.invoke('emu:biosReload') as Promise<{biosPresent:boolean;reloaded:boolean}>,
  emuStats: () => ipcRenderer.invoke('emu:stats') as Promise<any>,
  emuStatsReset: () => ipcRenderer.invoke('emu:statsReset') as Promise<{ok:boolean}>,
  emuDiagnoseIntercept: (frames:number=8) => ipcRenderer.invoke('emu:diagnoseIntercept', frames) as Promise<any>,
  emuAssemble: (args: { asmPath: string; outPath?: string; extra?: string[] }) => ipcRenderer.invoke('emu:assemble', args) as Promise<{ ok?: boolean; error?: string; binPath?: string; size?: number; base64?: string; stdout?: string; stderr?: string }>,
  runCompile: (args: { path: string; saveIfDirty?: { content: string; expectedMTime?: number }; autoStart?: boolean }) => ipcRenderer.invoke('run:compile', args) as Promise<{ ok?: boolean; error?: string; binPath?: string; size?: number; stdout?: string; stderr?: string; conflict?: boolean; currentMTime?: number }>,
  onRunStdout: (cb: (chunk: string) => void) => ipcRenderer.on('run://stdout', (_e: IpcRendererEvent, data: string) => cb(data)),
  onRunStderr: (cb: (chunk: string) => void) => ipcRenderer.on('run://stderr', (_e: IpcRendererEvent, data: string) => cb(data)),
  onRunDiagnostics: (cb: (diags: Array<{ file: string; line: number; col: number; message: string }>) => void) => ipcRenderer.on('run://diagnostics', (_e: IpcRendererEvent, diags) => cb(diags)),
  onRunStatus: (cb: (line: string) => void) => ipcRenderer.on('run://status', (_e: IpcRendererEvent, data: string) => cb(data)),
  onEmuLoaded: (cb: (info: { size: number }) => void) => ipcRenderer.on('emu://loaded', (_e: IpcRendererEvent, data) => cb(data)),
  setVectorMode: (mode: 'intercept'|'via') => ipcRenderer.invoke('emu:setVectorMode', mode) as Promise<{ ok?:boolean; error?:string; mode?:string }>,
  listSources: (args?: { limit?: number }) => ipcRenderer.invoke('list:sources', args) as Promise<{ ok?:boolean; sources?: Array<{ path:string; kind:'vpy'|'asm'; size:number; mtime:number }> }> ,
});

contextBridge.exposeInMainWorld('files', {
  openFile: () => ipcRenderer.invoke('file:open') as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string } | null>,
  openFilePath: (path: string) => ipcRenderer.invoke('file:openPath', path) as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string } | null>,
  saveFile: (args: { path: string; content: string; expectedMTime?: number }) => ipcRenderer.invoke('file:save', args) as Promise<{ path: string; mtime: number; size: number } | { conflict: true; currentMTime: number } | { error: string }>,

  saveFileAs: (args: { suggestedName?: string; content: string }) => ipcRenderer.invoke('file:saveAs', args) as Promise<{ path: string; mtime: number; size: number; name: string } | { canceled: true } | { error: string }>,

  openFolder: () => ipcRenderer.invoke('file:openFolder') as Promise<{ path: string } | null>,
  readFile: (path: string) => ipcRenderer.invoke('file:read', path) as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string }>,
  openBin: () => ipcRenderer.invoke('bin:open') as Promise<{ path: string; base64: string; size: number } | { error: string } | null>,
});

contextBridge.exposeInMainWorld('recents', {
  load: () => ipcRenderer.invoke('recents:load') as Promise<Array<{ path: string; lastOpened: number; kind: 'file' | 'folder' }>>,
  write: (list: Array<{ path: string; lastOpened: number; kind: 'file' | 'folder' }>) => ipcRenderer.invoke('recents:write', list) as Promise<{ ok: boolean }>,
});

export {};