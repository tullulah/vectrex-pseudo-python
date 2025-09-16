import { app, BrowserWindow, ipcMain, Menu, session, dialog } from 'electron';
import { spawn } from 'child_process';
import { globalCpu, getStats, resetStats } from './emu6809';
import { createInterface } from 'readline';
import { join, basename } from 'path';
import { existsSync } from 'fs';
import * as fs from 'fs/promises';

let mainWindow: BrowserWindow | null = null;

// --- Emulator load helpers (shared by emu:load and run:compile) -----------------
function cpuColdReset(){
  globalCpu.a=0; globalCpu.b=0; globalCpu.dp=0xD0; globalCpu.x=0; globalCpu.y=0; globalCpu.u=0; globalCpu.s=0xC000; globalCpu.pc=0; // PC starts at $0000 (fixed cartridge ORG)
  globalCpu.callStack=[]; globalCpu.lastIntensity=0x5F; globalCpu.frameSegments=[]; globalCpu.frameReady=false;
  // Re-apply BIOS contents (if any) after bulk clear performed by caller.
  if (globalCpu.biosPresent && typeof globalCpu.reapplyBios === 'function') {
    globalCpu.reapplyBios();
    // If BIOS present, honor its reset vector at 0xFFFE/0xFFFF
    const rvHi = globalCpu.mem[0xFFFE];
    const rvLo = globalCpu.mem[0xFFFF];
    globalCpu.pc = ((rvHi<<8)|rvLo) & 0xFFFF;
  }
  // clear opcode stats so each run starts fresh
  resetStats();
}
function loadBinaryBase64IntoEmu(base64: string){
  const bytes = Buffer.from(base64, 'base64');
  // Clear RAM only (do not wipe BIOS region if loaded). We'll zero everything first then reapply BIOS.
  globalCpu.mem.fill(0);
  if (globalCpu.biosPresent && typeof globalCpu.reapplyBios === 'function') globalCpu.reapplyBios();
  cpuColdReset();
  globalCpu.loadBin(new Uint8Array(bytes), 0x0000);
  // Notify renderer explicitly so panels can react (e.g., auto-play)
  mainWindow?.webContents.send('emu://loaded', { size: bytes.length, bios: globalCpu.biosPresent });
  return { ok: true } as const;
}

// Attempt BIOS load early; search multiple locations and emit rich diagnostics.
// Search order (first existing directory wins candidate ordering, but we aggregate unique files):
//   1. core/bios/
//   2. bios/ (at repo root)
//   3. repo root (process.cwd())
// Preferred filenames: bios.bin, vectrex.bin (each directory), then any other *.bin
async function tryLoadBiosOnce(){
  const trace = (note: string) => { try { if ((globalCpu as any).traceEnabled) (globalCpu as any).debugTraces.push({ type:'info', pc:0xF000, note }); } catch {} };
  const cwd = process.cwd();
  // Allow explicit override (can be a file or directory). If file, we try it directly; if directory, we search inside.
  const envOverride = process.env.VPY_BIOS_PATH;
  if (envOverride) trace(`bios-env:${envOverride}`);
  // Heuristic repo root: walk up from cwd until we find Cargo.toml containing a [workspace] or a core/ directory.
  function detectRepoRoot(start: string): string {
    let dir = start;
    for (let i=0;i<6;i++) { // limit upward traversal
      try {
        const cargo = join(dir, 'Cargo.toml');
        if (existsSync(cargo)) return dir;
      } catch {}
      const parent = join(dir, '..');
      if (parent === dir) break;
      dir = parent;
    }
    return start;
  }
  const repoRoot = detectRepoRoot(cwd);
  if (repoRoot !== cwd) trace(`bios-root:${repoRoot}`);
  // Candidate directories (in priority order):
  //  - env override if it is a directory
  //  - <repoRoot>/core/src/bios
  //  - <repoRoot>/core/bios
  //  - <repoRoot>/bios
  //  - <repoRoot>
  //  - cwd variants (if different) to maintain previous behavior
  const dirCandidates: string[] = [];
  if (envOverride && !/\.bin$/i.test(envOverride)) dirCandidates.push(envOverride);
  dirCandidates.push(
    join(repoRoot, 'core', 'src', 'bios'),
    join(repoRoot, 'core', 'bios'),
    join(repoRoot, 'bios'),
    repoRoot
  );
  if (cwd !== repoRoot) {
    dirCandidates.push(join(cwd, 'core', 'bios'), join(cwd, 'bios'), cwd);
  }
  // If env override looks like a .bin file, treat it as a single candidate path later.
  const candidates: string[] = [];
  const seen = new Set<string>();
  for (const dir of dirCandidates){
    try {
      const entries = await fs.readdir(dir).catch(()=>null);
      if (!entries) { trace(`bios-dir-missing:${dir}`); continue; }
      if (!entries.length) { trace(`bios-dir-empty:${dir}`); continue; }
      const lower = entries.map(e => e.toLowerCase());
      const preferNames = ['bios.bin','vectrex.bin'];
      const ordered: string[] = [];
      for (const name of preferNames){
        const idx = lower.indexOf(name);
        if (idx !== -1) ordered.push(entries[idx]);
      }
      const rest = entries.filter(e => /\.bin$/i.test(e) && !ordered.includes(e));
      const dirCandidates = [...ordered, ...rest].map(e => join(dir, e));
      let added = 0;
      for (const p of dirCandidates){ if (!seen.has(p)) { candidates.push(p); seen.add(p); added++; } }
      trace(`bios-dir:${dir}:files=${entries.length}:candidatesAdded=${added}`);
    } catch (e:any) {
      trace(`bios-dir-error:${dir}`);
    }
  }
  trace(`bios-candidates:${candidates.length}`);
  // If env override is a file path, attempt it first (if not already in candidates list)
  if (envOverride && /\.bin$/i.test(envOverride)) {
    if (!seen.has(envOverride)) { trace(`bios-env-file:${envOverride}`); candidates.unshift(envOverride); seen.add(envOverride); }
  }
  for (const p of candidates){
    // Per-candidate attempt
    trace(`bios-try:${p}`);
    try {
      if (!existsSync(p)) { trace(`bios-missing-file:${p}`); continue; }
      const buf = await fs.readFile(p);
      const ok = (globalCpu as any).loadBios?.(new Uint8Array(buf));
      if (ok) {
        mainWindow?.webContents.send('emu://status', `Loaded Vectrex BIOS (${buf.length} bytes) from ${p}`);
        trace(`bios-success:${p}`);
        return true;
      } else {
        trace(`bios-load-failed:${p}`);
      }
    } catch (e:any) {
      trace(`bios-error:${p}`);
    }
  }
  mainWindow?.webContents.send('emu://status', 'Vectrex BIOS not found (looked for bios.bin / vectrex.bin in core/bios, bios, root). Execution will proceed without BIOS features.');
  trace('bios-missing');
  return false;
}

interface LspChild {
  proc: ReturnType<typeof spawn>;
  stdin: NodeJS.WritableStream;
}
let lsp: LspChild | null = null;

async function createWindow() {
  const verbose = process.env.VPY_IDE_VERBOSE_LSP === '1';
  if (verbose) console.log('[IDE] createWindow() start');
  // Asegurar que no exista menú antes de crear la ventana
  try { Menu.setApplicationMenu(null); } catch {}
  const isDev = !!process.env.VITE_DEV_SERVER_URL;
  const sandboxEnabled = process.env.VPY_IDE_SANDBOX !== '0'; // Permitir desactivar sólo si hay problema específico con preload
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      // Preload aislado: exporta API mínima vía contextBridge
      preload: join(__dirname, 'preload.js'),
      contextIsolation: true,
      // Sin integración Node directa en el renderer
      nodeIntegration: false,
      // Sandbox Chromium (reduce superficie de ataque). Puede desactivarse con VPY_IDE_SANDBOX=0 para depurar si algo rompe.
      sandbox: sandboxEnabled,
      // No permitir contenido inseguro mixto
      allowRunningInsecureContent: false,
      // Bloquear navegación arbitraria (seguimos validando manualmente de todos modos)
      webSecurity: true,
      // DevTools sólo si se habilita variable explícita; en prod quedan bloqueadas incluso si usuario presiona F12
      devTools: process.env.VPY_IDE_DEVTOOLS === '1',
      // Desactivar spellcheck (no lo necesitamos y reduce código cargado)
      spellcheck: false,
    },
    autoHideMenuBar: true,
    frame: true,
  });
  if (verbose) console.log('[IDE] sandbox=', sandboxEnabled, 'dev=', isDev);
  // Refuerzo: eliminar menú y ocultar barra (Windows a veces muestra placeholder en primer frame)
  try {
    mainWindow.setMenu(null);
    mainWindow.setMenuBarVisibility(false);
  } catch {}

  const devUrl = process.env.VITE_DEV_SERVER_URL;
  if (devUrl) {
    if (verbose) console.log('[IDE] loading dev URL', devUrl);
    await mainWindow.loadURL(devUrl);
  } else {
    if (verbose) console.log('[IDE] loading file index.html');
    await mainWindow.loadFile(join(__dirname, '../../frontend/dist/index.html'));
  }
  // Bloquear apertura automática salvo flag explícita
  if (process.env.VPY_IDE_DEVTOOLS === '1') {
    if (verbose) console.log('[IDE] opening devtools (flag set)');
    mainWindow.webContents.openDevTools({ mode: 'detach' });
  } else {
    // Cerrar si ya se abrió por alguna razón
    if (mainWindow.webContents.isDevToolsOpened()) {
      try { mainWindow.webContents.closeDevTools(); } catch {}
    }
    // Listener para cerrar si un atajo externo la abre
    mainWindow.webContents.on('devtools-opened', () => {
      if (process.env.VPY_IDE_DEVTOOLS !== '1') {
        try { mainWindow?.webContents.closeDevTools(); } catch {}
        if (verbose) console.log('[IDE] devtools closed (not allowed)');
      }
    });
  }
  if (verbose) console.log('[IDE] createWindow() end');
}

let lspPathWarned = false;
function resolveLspPath(): string | null {
  const exeName = process.platform === 'win32' ? 'vpy_lsp.exe' : 'vpy_lsp';
  const cwd = process.cwd();
  // Posibles ubicaciones (orden de prioridad):
  const candidates = [
    // Ejecución desde root (run-ide.ps1 hace Set-Location root antes de lanzar)
    join(cwd, 'target', 'debug', exeName),
    join(cwd, 'target', 'release', exeName),
    // Bin copiado manualmente
    join(cwd, exeName),
    // Layout monorepo: bin dentro de crate core
    join(cwd, 'core', 'target', 'debug', exeName),
    join(cwd, 'core', 'target', 'release', exeName),
    // Si por alguna razón el cwd termina en ide/electron
    join(cwd, '..', '..', 'target', 'debug', exeName),
    join(cwd, '..', '..', 'target', 'release', exeName),
    join(cwd, '..', '..', 'core', 'target', 'debug', exeName),
    join(cwd, '..', '..', 'core', 'target', 'release', exeName),
  ];
  for (const p of candidates) {
    try { if (existsSync(p)) return p; } catch {}
  }
  if (!lspPathWarned) {
    lspPathWarned = true;
    mainWindow?.webContents.send('lsp://stderr', `[LSP] CWD=${cwd}`);
    mainWindow?.webContents.send('lsp://stderr', `LSP binary not found. Tried paths:\n${candidates.join('\n')}\nCompile with: cargo build -p vectrex_lang --bin vpy_lsp`);
  }
  return null;
}

// Enumerate .vpy and .asm under examples/ and working directory (non-recursive + shallow recursive examples)
ipcMain.handle('list:sources', async (_e, args: { limit?: number } = {}) => {
  const limit = args.limit ?? 200;
  const cwd = process.cwd();
  const exDir = join(cwd, 'examples');
  const results: Array<{ path:string; kind:'vpy'|'asm'; size:number; mtime:number }> = [];
  async function scanDir(dir:string, depth:number){
    if (results.length >= limit) return;
    try {
      const entries = await fs.readdir(dir, { withFileTypes: true });
      for (const ent of entries){
        if (results.length >= limit) break;
        const full = join(dir, ent.name);
        if (ent.isDirectory()) { if (depth<1) await scanDir(full, depth+1); continue; }
        if (/\.(vpy|asm)$/i.test(ent.name)) {
          try {
            const st = await fs.stat(full);
            results.push({ path: full, kind: /\.vpy$/i.test(ent.name)?'vpy':'asm', size: st.size, mtime: st.mtimeMs });
          } catch {}
        }
      }
    } catch {}
  }
  await scanDir(cwd, 0);
  await scanDir(exDir, 0);
  // De-dupe by path
  const seen = new Set<string>();
  const uniq = results.filter(r => { if (seen.has(r.path)) return false; seen.add(r.path); return true; });
  uniq.sort((a,b)=> a.path.localeCompare(b.path));
  return { ok:true, sources: uniq.slice(0, limit) };
});

ipcMain.handle('lsp_start', async () => {
  const verbose = process.env.VPY_IDE_VERBOSE_LSP === '1';
  if (lsp) return;
  if (verbose) console.log('[LSP] start request');
  const path = resolveLspPath();
  if (!path) return; // mensaje detallado ya emitido en resolveLspPath (una sola vez)
  if (verbose) console.log('[LSP] spawning', path);
  const child = spawn(path, [], { stdio: ['pipe', 'pipe', 'pipe'] });
  lsp = { proc: child, stdin: child.stdin! };

  let buffer = '';
  let expected: number | null = null;
  child.stdout.on('data', (chunk: Buffer) => {
    buffer += chunk.toString('utf8');
    while (true) {
      if (expected === null) {
        const headerEnd = buffer.indexOf('\r\n\r\n');
        if (headerEnd === -1) break;
        const header = buffer.slice(0, headerEnd);
        const match = /Content-Length: *([0-9]+)/i.exec(header);
        if (!match) {
          buffer = buffer.slice(headerEnd + 4);
          continue;
        }
        expected = parseInt(match[1], 10);
        buffer = buffer.slice(headerEnd + 4);
      }
      if (expected !== null && buffer.length >= expected) {
        const body = buffer.slice(0, expected);
        buffer = buffer.slice(expected);
        expected = null;
        mainWindow?.webContents.send('lsp://message', body);
        mainWindow?.webContents.send('lsp://stdout', body);
        if (verbose) console.log('[LSP<-] message len', body.length);
        continue;
      }
      break;
    }
  });

  const rlErr = createInterface({ input: child.stderr });
  rlErr.on('line', line => mainWindow?.webContents.send('lsp://stderr', line));
  child.on('exit', code => {
    mainWindow?.webContents.send('lsp://stderr', `[LSP exited ${code}]`);
    if (verbose) console.log('[LSP] exited', code);
    lsp = null;
  });
});

ipcMain.handle('lsp_send', async (_e, payload: string) => {
  if (!lsp) return;
  const bytes = Buffer.from(payload, 'utf8');
  const header = `Content-Length: ${bytes.length}\r\n\r\n`;
  lsp.stdin.write(header);
  lsp.stdin.write(bytes);
});

let recentsCache: Array<{ path: string; lastOpened: number; kind: 'file' | 'folder' }> | null = null;
function getRecentsPath() { return join(app.getPath('userData'), 'recent.json'); }
async function loadRecents(): Promise<typeof recentsCache> {
  if (recentsCache) return recentsCache;
  try {
    const txt = await fs.readFile(getRecentsPath(), 'utf8');
    recentsCache = JSON.parse(txt);
  } catch { recentsCache = []; }
  return recentsCache!;
}
async function persistRecents() {
  try { await fs.writeFile(getRecentsPath(), JSON.stringify(recentsCache||[], null, 2), 'utf8'); } catch {}
}
function touchRecent(path: string, kind: 'file'|'folder') {
  const now = Date.now();
  if (!recentsCache) recentsCache = [];
  recentsCache = recentsCache.filter(r => r.path !== path);
  recentsCache.unshift({ path, lastOpened: now, kind });
  if (recentsCache.length > 30) recentsCache.length = 30;
  persistRecents();
}

ipcMain.handle('file:open', async () => {
  const win = BrowserWindow.getFocusedWindow() || mainWindow;
  if (!win) return null;
  const { canceled, filePaths } = await dialog.showOpenDialog(win, { properties: ['openFile'], filters: [{ name: 'Source', extensions: ['vpy','pseudo','asm','txt'] }] });
  if (canceled || filePaths.length === 0) return null;
  const p = filePaths[0];
  try {
    const content = await fs.readFile(p, 'utf8');
    const stat = await fs.stat(p);
    await loadRecents();
    touchRecent(p, 'file');
    return { path: p, content, mtime: stat.mtimeMs, size: stat.size, name: basename(p) };
  } catch (e: any) {
    return { error: e?.message || 'read_failed' };
  }
});

// Binary open (returns base64)
ipcMain.handle('bin:open', async () => {
  const win = BrowserWindow.getFocusedWindow() || mainWindow;
  if (!win) return null;
  const { canceled, filePaths } = await dialog.showOpenDialog(win, { properties: ['openFile'], filters: [{ name: 'Binary', extensions: ['bin'] }] });
  if (canceled || filePaths.length === 0) return null;
  const p = filePaths[0];
  try {
    const buf = await fs.readFile(p);
    return { path: p, base64: buf.toString('base64'), size: buf.length };
  } catch (e:any) {
    return { error: e?.message || 'read_failed' };
  }
});

// Emulator: load BIN
ipcMain.handle('emu:load', async (_e, args: { base64: string }) => {
  try {
    return loadBinaryBase64IntoEmu(args.base64);
  } catch (e:any) { return { error: e?.message || 'emu_load_failed' }; }
});

// Resolve compiler binary path. Supports env override VPY_COMPILER_BIN.
function resolveCompilerPath(): string | null {
  const override = process.env.VPY_COMPILER_BIN;
  if (override) {
    try { if (existsSync(override)) return override; } catch {}
    mainWindow?.webContents.send('run://stderr', `VPY_COMPILER_BIN set but file not found: ${override}`);
  }
  const names = process.platform === 'win32' ? ['vectrexc.exe','vectrex_lang.exe'] : ['vectrexc','vectrex_lang'];
  const cwd = process.cwd();
  const candidates: string[] = [];
  for (const exe of names) {
    candidates.push(
      join(cwd, 'target', 'debug', exe),
      join(cwd, 'target', 'release', exe),
      join(cwd, exe),
      join(cwd, 'core', 'target', 'debug', exe),
      join(cwd, 'core', 'target', 'release', exe),
      join(cwd, '..', '..', 'target', 'debug', exe),
      join(cwd, '..', '..', 'target', 'release', exe),
      join(cwd, '..', '..', 'core', 'target', 'debug', exe),
      join(cwd, '..', '..', 'core', 'target', 'release', exe),
    );
  }
  for (const p of candidates) { try { if (existsSync(p)) return p; } catch {} }
  mainWindow?.webContents.send('run://stderr', `Compiler binary not found. Tried paths (names: ${names.join(', ')}):\n${candidates.join('\n')}\nBuild with one of:\n  cargo build -p vectrex_lang --bin vectrexc\n  cargo build -p vectrex_lang --bin vectrex_lang\nOr set VPY_COMPILER_BIN=full\\path\\to\\compiler.exe`);
  return null;
}

// run:compile => compile a .vpy file, produce .asm + .bin, load into emulator
// args: { path: string; saveIfDirty?: { content: string; expectedMTime?: number } }
ipcMain.handle('run:compile', async (_e, args: { path: string; saveIfDirty?: { content: string; expectedMTime?: number }; autoStart?: boolean }) => {
  const { path, saveIfDirty, autoStart } = args || {} as any;
  if (!path) return { error: 'no_path' };
  // Normalize potential file:// URI to local filesystem path (especially on Windows)
  let fsPath = path;
  if (/^file:\/\//i.test(fsPath)) {
    try {
      // new URL handles decoding; strip leading slash for Windows drive letter patterns like /C:/
      const u = new URL(fsPath);
      fsPath = u.pathname;
      if (process.platform === 'win32' && /^\/[A-Za-z]:/.test(fsPath)) fsPath = fsPath.slice(1);
      fsPath = fsPath.replace(/\//g, require('path').sep);
    } catch {}
  }
  // Optionally save current buffer content before compiling
  const targetDisplay = fsPath !== path ? `${path} -> ${fsPath}` : fsPath;
  // Optionally save current buffer content before compiling
  if (saveIfDirty && typeof saveIfDirty.content === 'string') {
    try {
      const statBefore = await fs.stat(fsPath).catch(()=>null);
      if (saveIfDirty.expectedMTime && statBefore && statBefore.mtimeMs !== saveIfDirty.expectedMTime) {
        return { conflict: true, currentMTime: statBefore.mtimeMs };
      }
      await fs.writeFile(fsPath, saveIfDirty.content, 'utf8');
    } catch (e:any) {
      return { error: 'save_failed_before_compile', detail: e?.message };
    }
  }
  const compiler = resolveCompilerPath();
  if (!compiler) return { error: 'compiler_not_found' };
  const verbose = process.env.VPY_IDE_VERBOSE_RUN === '1';
  if (verbose) console.log('[RUN] spawning compiler', compiler, fsPath);
  mainWindow?.webContents.send('run://status', `Starting compilation: ${targetDisplay}`);
  return new Promise(async (resolvePromise) => {
    const outAsm = fsPath.replace(/\.[^.]+$/, '.asm');
    const argsv = ['build', fsPath, '--target', 'vectrex', '--title', basename(fsPath).replace(/\.[^.]+$/, '').toUpperCase(), '--bin'];
    const child = spawn(compiler, argsv, { stdio: ['ignore','pipe','pipe'] });
    let stdoutBuf = '';
    let stderrBuf = '';
    child.stdout.on('data', (c: Buffer) => { const txt = c.toString('utf8'); stdoutBuf += txt; mainWindow?.webContents.send('run://stdout', txt); });
    child.stderr.on('data', (c: Buffer) => { const txt = c.toString('utf8'); stderrBuf += txt; mainWindow?.webContents.send('run://stderr', txt); });
    child.on('error', (err) => { resolvePromise({ error: 'spawn_failed', detail: err.message }); });
    child.on('exit', async (code) => {
      if (code !== 0) {
        mainWindow?.webContents.send('run://status', `Compilation FAILED (exit ${code})`);
        // Attempt to parse basic diagnostics from stderr (pattern: filename:line:col: message)
        const diags: Array<{ file:string; line:number; col:number; message:string }> = [];
        const diagRegex = /(.*?):(\d+):(\d+):\s*(.*)/;
        for (const line of stderrBuf.split(/\r?\n/)) {
          const m = diagRegex.exec(line.trim());
          if (m) {
            diags.push({ file: m[1], line: parseInt(m[2],10)-1, col: parseInt(m[3],10)-1, message: m[4] });
          }
        }
        if (diags.length) mainWindow?.webContents.send('run://diagnostics', diags);
        return resolvePromise({ error: 'compile_failed', code, stdout: stdoutBuf, stderr: stderrBuf });
      }
      // On success, look for produced bin with same stem (.bin)
      const binPath = outAsm.replace(/\.asm$/, '.bin');
      try {
        const buf = await fs.readFile(binPath);
        const base64 = Buffer.from(buf).toString('base64');
        // Load into emulator
        // Directly load into emulator (previous attempt used ipcMain.invoke which is invalid here)
        try {
          const loadRes = loadBinaryBase64IntoEmu(base64);
          if (verbose) console.log('[RUN] loaded binary', binPath, loadRes);
          mainWindow?.webContents.send('run://status', `Compilation succeeded & loaded: ${binPath} (${buf.length} bytes)`);
        } catch (e:any) {
          mainWindow?.webContents.send('run://status', `Compilation succeeded but load FAILED: ${e?.message}`);
          return resolvePromise({ error: 'emu_load_failed', detail: e?.message });
        }
        // If there are unknown opcodes already registered (rare immediately), surface them
        try {
          const stats = getStats();
          const unknown = (stats as any).unknownOpcodes || {};
          const keys = Object.keys(unknown);
          if (keys.length) {
            mainWindow?.webContents.send('run://status', `Unknown opcode counts after load:`);
            keys.slice(0,20).forEach(k => {
              mainWindow?.webContents.send('run://status', `  ${k}: ${unknown[k]}`);
            });
            if (keys.length > 20) mainWindow?.webContents.send('run://status', `  ... (${keys.length-20} more)`);
          }
        } catch {}
        resolvePromise({ ok: true, binPath, size: buf.length, stdout: stdoutBuf, stderr: stderrBuf });
      } catch (e:any) {
        mainWindow?.webContents.send('run://status', `Compiled but failed to read bin: ${e?.message}`);
        resolvePromise({ error: 'bin_read_failed', detail: e?.message });
      }
    });
  });
});

// Emulator: run until next frame (or max steps)
ipcMain.handle('emu:runFrame', async () => {
  try {
    const cpu:any = globalCpu as any;
    const { frameReady, segments, viaEvents, debugTraces, opcodeTrace } = cpu.runUntilFrame();
    return {
      frameReady,
      segments,
      viaEvents,
      debugTraces,
      opcodeTrace: opcodeTrace || [],
      irqPending: !!cpu.irqPending,
      waiWaiting: !!cpu.waiWaiting,
      pc: cpu.pc,
    };
  } catch (e:any) { return { error: e?.message || 'emu_run_failed' }; }
});

// Debug helpers
ipcMain.handle('emu:getPC', () => ({ pc: globalCpu.pc }));
ipcMain.handle('emu:setPC', (_e, pc:number) => { globalCpu.pc = pc & 0xFFFF; return { ok:true, pc: globalCpu.pc }; });
ipcMain.handle('emu:peek', (_e, addr:number, len:number=32) => {
  addr &= 0xFFFF; len = Math.min(Math.max(len,1),256);
  const bytes:number[] = [];
  for (let i=0;i<len;i++){ bytes.push(globalCpu.mem[(addr+i)&0xFFFF]); }
  return { base: addr, bytes };
});
ipcMain.handle('emu:toggleTrace', (_e, enabled?: boolean) => {
  if (typeof enabled === 'boolean') globalCpu.traceEnabled = enabled;
  else globalCpu.traceEnabled = !globalCpu.traceEnabled;
  return { traceEnabled: globalCpu.traceEnabled };
});
// Auto-start heuristic toggle
ipcMain.handle('emu:autoStart', (_e, enabled?: boolean) => {
  if (typeof enabled === 'boolean') (globalCpu as any).autoStartUser = enabled;
  else (globalCpu as any).autoStartUser = !(globalCpu as any).autoStartUser;
  // Allow re-attempt if re-enabled before first frame
  (globalCpu as any).attemptedAutoStart = false;
  (globalCpu as any).autoStartInfo = null;
  return { autoStartUser: (globalCpu as any).autoStartUser };
});
ipcMain.handle('emu:autoStartInfo', () => {
  const cpu:any = globalCpu as any;
  return { attempted: cpu.attemptedAutoStart, info: cpu.autoStartInfo };
});
ipcMain.handle('emu:toggleOpcodeTrace', (_e, enabled?: boolean) => {
  const cpu:any = globalCpu as any;
  if (typeof enabled === 'boolean') cpu.opcodeTraceEnabled = enabled; else cpu.opcodeTraceEnabled = !cpu.opcodeTraceEnabled;
  cpu.opcodeTrace.length = 0; // clear when toggled
  return { opcodeTraceEnabled: cpu.opcodeTraceEnabled };
});
ipcMain.handle('emu:regs', () => {
  return { a:globalCpu.a, b:globalCpu.b, x:globalCpu.x, y:globalCpu.y, u:globalCpu.u, s:globalCpu.s, pc:globalCpu.pc, dp:globalCpu.dp };
});
ipcMain.handle('emu:forceStart', () => {
  const cpu:any = globalCpu as any;
  cpu.pc = 0x0000;
  cpu.attemptedAutoStart = true;
  cpu.autoStartInfo = { performed:true, reason:'forceStartIPC' };
  if (cpu.traceEnabled) cpu.debugTraces.push({ type:'info', pc:0xFFFF, note:'force-start-user->0000' });
  return { pc: cpu.pc };
});
ipcMain.handle('emu:status', () => {
  const cpu:any = globalCpu as any;
  return {
    biosPresent: cpu.biosPresent,
    vectorMode: cpu.vectorMode,
    autoStartUser: cpu.autoStartUser,
    opcodeTraceEnabled: cpu.opcodeTraceEnabled,
    traceEnabled: cpu.traceEnabled,
  };
});
ipcMain.handle('emu:biosStatus', () => {
  const cpu:any = globalCpu as any;
  return { biosPresent: cpu.biosPresent };
});
ipcMain.handle('emu:biosReload', async () => {
  const ok = await tryLoadBiosOnce();
  return { biosPresent: globalCpu.biosPresent, reloaded: ok };
});
ipcMain.handle('emu:stats', async () => {
  return getStats();
});
ipcMain.handle('emu:statsReset', async () => { resetStats(); return { ok:true }; });
// Switch vector rendering mode (intercept vs via)
ipcMain.handle('emu:setVectorMode', async (_e, mode: 'intercept' | 'via') => {
  if (mode === 'intercept' || mode === 'via') {
    try { (globalCpu as any).setVectorMode?.(mode); return { ok:true, mode }; } catch {}
  }
  return { error: 'invalid_mode' };
});

// Diagnostic: run N frames in intercept mode and summarize traces, opcode stats, U pointer data
ipcMain.handle('emu:diagnoseIntercept', async (_e, frames: number = 8) => {
  const cpu:any = globalCpu as any;
  const originalMode = cpu.vectorMode;
  try {
    cpu.setVectorMode?.('intercept');
    const out: any = { frames: [], regsStart: { a:cpu.a,b:cpu.b,x:cpu.x,y:cpu.y,u:cpu.u,pc:cpu.pc,dp:cpu.dp } };
    for (let i=0;i<frames;i++){
      const beforeU = cpu.u & 0xFFFF;
      const memSample: number[] = [];
      for (let j=0;j<16;j++) memSample.push(cpu.mem[(beforeU+j)&0xFFFF]);
      const { frameReady, segments, debugTraces } = cpu.runUntilFrame();
      const notes = (debugTraces||[]).map((t:any)=>t.note||t.type);
      out.frames.push({ i, frameReady, segs: segments.length, notes, u: beforeU.toString(16), uBytes: memSample });
    }
    out.regsEnd = { a:cpu.a,b:cpu.b,x:cpu.x,y:cpu.y,u:cpu.u,pc:cpu.pc,dp:cpu.dp };
    out.unknownOpcodes = { ...(cpu.unknownLog||{}) };
    return out;
  } catch (e:any){
    return { error: e?.message || 'diagnose_failed' };
  } finally {
    if (originalMode !== 'intercept') { try { cpu.setVectorMode?.(originalMode); } catch {} }
  }
});

ipcMain.handle('file:openPath', async (_e, p: string) => {
  if (!p) return { error: 'no_path' };
  try {
    const content = await fs.readFile(p, 'utf8');
    const stat = await fs.stat(p);
    await loadRecents();
    touchRecent(p, 'file');
    return { path: p, content, mtime: stat.mtimeMs, size: stat.size, name: basename(p) };
  } catch (e:any) {
    return { error: e?.message || 'read_failed' };
  }
});

ipcMain.handle('file:read', async (_e, path: string) => {
  if (!path) return { error: 'no_path' };
  try {
    const content = await fs.readFile(path, 'utf8');
    const stat = await fs.stat(path);
    return { path, content, mtime: stat.mtimeMs, size: stat.size, name: basename(path) };
  } catch (e:any) {
    return { error: e?.message || 'read_failed' };
  }
});

// Read arbitrary binary file and return base64 (for emulator program loading without file:// fetch)
ipcMain.handle('file:readBin', async (_e, path: string) => {
  if (!path) return { error: 'no_path' };
  try {
    const buf = await fs.readFile(path);
    return { path, base64: Buffer.from(buf).toString('base64'), size: buf.length, name: basename(path) };
  } catch (e:any) {
    return { error: e?.message || 'read_failed' };
  }
});

ipcMain.handle('file:save', async (_e, args: { path: string; content: string; expectedMTime?: number }) => {
  const { path, content, expectedMTime } = args || {} as any;
  if (!path) return { error: 'no_path' };
  try {
    const statBefore = await fs.stat(path).catch(()=>null);
    if (expectedMTime && statBefore && statBefore.mtimeMs !== expectedMTime) {
      return { conflict: true, currentMTime: statBefore.mtimeMs };
    }
    await fs.writeFile(path, content, 'utf8');
    const statAfter = await fs.stat(path);
    await loadRecents();
    touchRecent(path, 'file');
    return { path, mtime: statAfter.mtimeMs, size: statAfter.size };
  } catch (e: any) {
    return { error: e?.message || 'save_failed' };
  }
});

ipcMain.handle('file:saveAs', async (_e, args: { suggestedName?: string; content: string }) => {
  const win = BrowserWindow.getFocusedWindow() || mainWindow;
  if (!win) return { error: 'no_window' };
  const { suggestedName, content } = args || {} as any;
  const { canceled, filePath } = await dialog.showSaveDialog(win, {
    defaultPath: suggestedName || 'untitled.vpy',
    filters: [{ name: 'Source', extensions: ['vpy','pseudo','asm','txt'] }]
  });
  if (canceled || !filePath) return { canceled: true };
  try {
    await fs.writeFile(filePath, content, 'utf8');
    const stat = await fs.stat(filePath);
    await loadRecents();
    touchRecent(filePath, 'file');
    return { path: filePath, mtime: stat.mtimeMs, size: stat.size, name: basename(filePath) };
  } catch (e: any) {
    return { error: e?.message || 'save_failed' };
  }
});

ipcMain.handle('file:openFolder', async () => {
  const win = BrowserWindow.getFocusedWindow() || mainWindow;
  if (!win) return null;
  const { canceled, filePaths } = await dialog.showOpenDialog(win, { properties: ['openDirectory'] });
  if (canceled || filePaths.length === 0) return null;
  const p = filePaths[0];
  await loadRecents();
  touchRecent(p, 'folder');
  return { path: p };
});

ipcMain.handle('recents:load', async () => {
  const list = await loadRecents();
  return list;
});
ipcMain.handle('recents:write', async (_e, list: any[]) => {
  recentsCache = Array.isArray(list) ? list : [];
  await persistRecents();
  return { ok: true };
});

// Assemble a Vectrex 6809 raw binary from an .asm file via PowerShell lwasm wrapper
// args: { asmPath: string; outPath?: string; extra?: string[] }
ipcMain.handle('emu:assemble', async (_e, args: { asmPath: string; outPath?: string; extra?: string[] }) => {
  const { asmPath, outPath, extra } = args || {} as any;
  if (!asmPath) return { error: 'no_asm_path' };
  // Normalize possible file:/// URI
  let fsPath = asmPath;
  if (/^file:\/\//i.test(fsPath)) {
    try { const u = new URL(fsPath); fsPath = u.pathname; if (process.platform==='win32' && /^\/[A-Za-z]:/.test(fsPath)) fsPath = fsPath.slice(1); } catch {}
  }
  try { if (!existsSync(fsPath)) return { error: 'asm_not_found', path: fsPath }; } catch { return { error: 'asm_not_found', path: fsPath }; }
  const outBin = outPath || fsPath.replace(/\.[^.]+$/, '.bin');
  const script = join(process.cwd(), 'tools', 'lwasm.ps1');
  try { if (!existsSync(script)) return { error: 'script_not_found', script }; } catch { return { error: 'script_not_found', script }; }
  const baseArgs = ['-NoProfile','-ExecutionPolicy','Bypass','-File', script, '--6809','--format=raw', `--output=${outBin}`, fsPath];
  if (Array.isArray(extra) && extra.length) baseArgs.push(...extra);
  mainWindow?.webContents.send('run://status', `Assembling ${fsPath} -> ${outBin}`);
  return new Promise((resolve) => {
    const child = spawn('pwsh', baseArgs, { stdio:['ignore','pipe','pipe'] });
    let stdoutBuf=''; let stderrBuf='';
    child.stdout.on('data', (c:Buffer)=>{ const t=c.toString('utf8'); stdoutBuf+=t; mainWindow?.webContents.send('run://stdout', t); });
    child.stderr.on('data', (c:Buffer)=>{ const t=c.toString('utf8'); stderrBuf+=t; mainWindow?.webContents.send('run://stderr', t); });
    child.on('error', (err)=>{ mainWindow?.webContents.send('run://status', `Assembly spawn failed: ${err.message}`); resolve({ error:'spawn_failed', detail: err.message }); });
    child.on('exit', async (code)=>{
      if (code!==0){ mainWindow?.webContents.send('run://status', `Assembly FAILED (exit ${code})`); return resolve({ error:'assemble_failed', code, stdout:stdoutBuf, stderr:stderrBuf }); }
      try {
        const buf = await fs.readFile(outBin);
        const base64 = Buffer.from(buf).toString('base64');
        mainWindow?.webContents.send('run://status', `Assembly OK: ${outBin} (${buf.length} bytes)`);
        resolve({ ok:true, binPath: outBin, size: buf.length, base64, stdout:stdoutBuf, stderr:stderrBuf });
      } catch(e:any){ resolve({ error:'bin_read_failed', detail:e?.message }); }
    });
  });
});

// After window creation call buildMenus
app.whenReady().then(() => {
  const verbose = process.env.VPY_IDE_VERBOSE_LSP === '1';
  if (verbose) console.log('[IDE] app.whenReady');
  // Seguridad adicional: anular menú global
  try { Menu.setApplicationMenu(null); } catch {}
  // Inyectar Content-Security-Policy por cabecera (más fuerte que meta) en dev y prod
  const isDev = !!process.env.VITE_DEV_SERVER_URL;
  // CSP:
  // - Prod (sin VITE_DEV_SERVER_URL): se espera estricta salvo que el empaquetado habilite relajación explícita.
  // - Dev: ahora relajado por defecto (run-ide.ps1 exporta VPY_IDE_RELAX_CSP=1 a menos que se use -StrictCSP) para soportar React Fast Refresh.
  //   Use -StrictCSP en el script de arranque para probar política estricta durante desarrollo.
  const relax = process.env.VPY_IDE_RELAX_CSP === '1';
  // En dev podemos necesitar React Refresh (eval + inline). Lo hacemos opt-in con VPY_IDE_RELAX_CSP=1
  const scriptSrc = (isDev && relax) ? "script-src 'self' 'unsafe-inline' 'unsafe-eval'" : "script-src 'self'";
  const styleSrc = (isDev && relax) ? "style-src 'self' 'unsafe-inline'" : "style-src 'self'";
  const imgSrc = "img-src 'self' data:";
  const fontSrc = "font-src 'self' data:";
  const connectSrc = isDev ? "connect-src 'self' ws: http: https:" : "connect-src 'self'";
  const workerSrc = "worker-src 'self' blob:"; // para blob workers Monaco
  const csp = [
    "default-src 'self'",
    scriptSrc,
    styleSrc,
    imgSrc,
    fontSrc,
    connectSrc,
    workerSrc,
    "object-src 'none'",
    "base-uri 'self'",
    "frame-ancestors 'none'"
  ].join('; ');
  // Nota para futuros cambios:
  // Si se requiere ejecutar un script inline específico (no recomendado), usar nonce o hash en lugar de reintroducir 'unsafe-inline'.
  // Ejemplo nonce:
  //  1. Generar const nonce = crypto.randomBytes(16).toString('base64');
  //  2. Añadir a CSP: script-src 'self' 'nonce-${nonce}'
  //  3. Inyectar en la etiqueta: <script nonce="${nonce}">...</script>
  // Ejemplo hash (para contenido fijo): calcular SHA256 del contenido y añadir 'sha256-<base64digest>' a script-src.
  // Evitar ampliar connect-src u otras fuentes salvo necesidad clara.
  session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
    const headers = details.responseHeaders || {};
    headers['Content-Security-Policy'] = [csp];
    callback({ cancel: false, responseHeaders: headers });
  });
  if (verbose) console.log('[IDE] CSP applied');
  createWindow();
  // Defer BIOS load slightly until window exists to allow status message.
  setTimeout(()=>{ tryLoadBiosOnce(); }, 500);
});
app.on('render-process-gone', (_e, details) => {
  console.error('[IDE] render process gone', details);
});
process.on('uncaughtException', (err) => {
  console.error('[IDE] uncaughtException', err);
});
process.on('unhandledRejection', (reason) => {
  console.error('[IDE] unhandledRejection', reason);
});
app.on('window-all-closed', () => { if (process.platform !== 'darwin') app.quit(); });
app.on('activate', () => { if (BrowserWindow.getAllWindows().length === 0) createWindow(); });
