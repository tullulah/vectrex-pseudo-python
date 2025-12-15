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
  readDirectory: (path: string) => ipcRenderer.invoke('file:readDirectory', path) as Promise<{ files: Array<{ name: string; path: string; isDir: boolean; children?: any[] }> } | { error: string }>,
  readFile: (path: string) => ipcRenderer.invoke('file:read', path) as Promise<{ path: string; content: string; mtime: number; size: number; name: string } | { error: string }>,
  readFileBin: (path: string) => ipcRenderer.invoke('file:readBin', path) as Promise<{ path: string; base64: string; size: number; name: string } | { error: string }>,
  openBin: () => ipcRenderer.invoke('bin:open') as Promise<{ path: string; base64: string; size: number } | { error: string } | null>,
  deleteFile: (path: string) => ipcRenderer.invoke('file:delete', path) as Promise<{ success: boolean; path: string } | { error: string }>,
  moveFile: (args: { sourcePath: string; targetDir: string }) => ipcRenderer.invoke('file:move', args) as Promise<{ success: boolean; sourcePath: string; targetPath: string } | { error: string; targetPath?: string }>,
  watchDirectory: (path: string) => ipcRenderer.invoke('file:watchDirectory', path) as Promise<{ ok: boolean; error?: string }>,
  unwatchDirectory: (path: string) => ipcRenderer.invoke('file:unwatchDirectory', path) as Promise<{ ok: boolean }>,
  onFileChanged: (cb: (event: { type: 'added' | 'removed' | 'changed'; path: string; isDir: boolean }) => void) => {
    const handler = (_e: IpcRendererEvent, data: any) => cb(data);
    ipcRenderer.on('file://changed', handler);
    // Return cleanup function to remove listener
    return () => ipcRenderer.removeListener('file://changed', handler);
  },
});

contextBridge.exposeInMainWorld('recents', {
  load: () => ipcRenderer.invoke('recents:load') as Promise<Array<{ path: string; lastOpened: number; kind: 'file' | 'folder' }>>,
  write: (list: Array<{ path: string; lastOpened: number; kind: 'file' | 'folder' }>) => ipcRenderer.invoke('recents:write', list) as Promise<{ ok: boolean }>,
});

// Project management API
contextBridge.exposeInMainWorld('project', {
  // Open project file dialog
  open: () => ipcRenderer.invoke('project:open') as Promise<{
    path: string;
    config: any;
    rootDir: string;
  } | { error: string } | null>,
  
  // Read project file directly
  read: (path: string) => ipcRenderer.invoke('project:read', path) as Promise<{
    path: string;
    config: any;
    rootDir: string;
  } | { error: string }>,
  
  // Create new project
  create: (args: { name: string; location?: string }) => ipcRenderer.invoke('project:create', args) as Promise<{
    ok: boolean;
    projectFile: string;
    projectDir: string;
    name: string;
  } | { canceled: true } | { error: string }>,
  
  // Find project file in directory or parents
  find: (startDir: string) => ipcRenderer.invoke('project:find', startDir) as Promise<{ path: string | null }>,
});

// Git operations API
contextBridge.exposeInMainWorld('git', {
  // Get git status (staged/unstaged changes)
  status: (projectDir: string) => ipcRenderer.invoke('git:status', projectDir) as Promise<{
    ok: boolean;
    files?: Array<{ path: string; status: 'M' | 'A' | 'D' | '?'; staged: boolean }>;
    error?: string;
  }>,
  
  // Stage a file
  stage: (args: { projectDir: string; filePath: string }) => ipcRenderer.invoke('git:stage', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
  
  // Unstage a file
  unstage: (args: { projectDir: string; filePath: string }) => ipcRenderer.invoke('git:unstage', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
  
  // Create a commit
  commit: (args: { projectDir: string; message: string }) => ipcRenderer.invoke('git:commit', args) as Promise<{
    ok: boolean;
    commit?: any;
    error?: string;
  }>,
  
  // Get file diff
  diff: (args: { projectDir: string; filePath?: string; staged?: boolean }) => ipcRenderer.invoke('git:diff', args) as Promise<{
    ok: boolean;
    diff?: string;
    error?: string;
  }>,
  
  // List branches
  branches: (projectDir: string) => ipcRenderer.invoke('git:branches', projectDir) as Promise<{
    ok: boolean;
    current?: string;
    branches?: Array<{ name: string; current: boolean; isRemote: boolean }>;
    error?: string;
  }>,
  
  // Checkout branch
  checkout: (args: { projectDir: string; branch: string }) => ipcRenderer.invoke('git:checkout', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
  
  // Discard changes in a file
  discard: (args: { projectDir: string; filePath: string }) => ipcRenderer.invoke('git:discard', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
  
  // Get commit log
  log: (args: { projectDir: string; limit?: number }) => ipcRenderer.invoke('git:log', args) as Promise<{
    ok: boolean;
    commits?: Array<{
      hash: string;
      fullHash: string;
      message: string;
      author: string;
      email: string;
      date: string;
      body: string;
    }>;
    error?: string;
  }>,
  
  // Push changes to remote
  push: (args: { projectDir: string; remote?: string; branch?: string }) => ipcRenderer.invoke('git:push', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
  
  // Pull changes from remote
  pull: (args: { projectDir: string; remote?: string; branch?: string }) => ipcRenderer.invoke('git:pull', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
  
  // Create new branch
  createBranch: (args: { projectDir: string; branch: string; fromBranch?: string }) => ipcRenderer.invoke('git:createBranch', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
  
  // Stash changes
  stash: (args: { projectDir: string; message?: string }) => ipcRenderer.invoke('git:stash', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
  
  // List stashes
  stashList: (projectDir: string) => ipcRenderer.invoke('git:stashList', projectDir) as Promise<{
    ok: boolean;
    stashes?: Array<{
      index: number;
      hash: string;
      fullHash: string;
      message: string;
      date: string;
    }>;
    error?: string;
  }>,
  
  // Pop stash
  stashPop: (args: { projectDir: string; index?: number }) => ipcRenderer.invoke('git:stashPop', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
  
  // Revert commit
  revert: (args: { projectDir: string; commitHash: string }) => ipcRenderer.invoke('git:revert', args) as Promise<{
    ok: boolean;
    error?: string;
  }>,
});

// MCP Server API for AI agents
contextBridge.exposeInMainWorld('mcp', {
  // Send JSON-RPC request to MCP server
  request: (request: any) => ipcRenderer.invoke('mcp:request', request),
});

// Shell command execution (for Ollama installation)
contextBridge.exposeInMainWorld('electron', {
  runCommand: (command: string) => ipcRenderer.invoke('shell:runCommand', command) as Promise<{
    success: boolean;
    output: string;
    exitCode: number;
  }>,
});

// AI Provider Proxy (for CORS-blocked APIs like Anthropic, DeepSeek)
contextBridge.exposeInMainWorld('aiProxy', {
  request: (request: {
    provider: 'anthropic' | 'deepseek';
    apiKey: string;
    endpoint: string;
    method: string;
    body: any;
    headers?: Record<string, string>;
  }) => ipcRenderer.invoke('ai-proxy-request', request) as Promise<{
    success: boolean;
    data?: any;
    error?: string;
    status?: number;
  }>,
});

// Persistent Storage API (replaces localStorage)
contextBridge.exposeInMainWorld('storage', {
  get: (key: string) => ipcRenderer.invoke('storage:get', key),
  set: (key: string, value: any) => ipcRenderer.invoke('storage:set', key, value),
  delete: (key: string) => ipcRenderer.invoke('storage:delete', key),
  keys: () => ipcRenderer.invoke('storage:keys') as Promise<string[]>,
  clear: () => ipcRenderer.invoke('storage:clear') as Promise<boolean>,
  getPath: () => ipcRenderer.invoke('storage:path') as Promise<string>,
  getKeys: () => ipcRenderer.invoke('storage:getKeys') as Promise<Record<string, string>>,
});

export {};