import { app, BrowserWindow, ipcMain, Menu, session, dialog } from 'electron';
import { spawn } from 'child_process';
// Legacy TypeScript emulator removed: all references to './emu6809' have been deleted.
// NOTE: Remaining emulator-related IPC endpoints that depended on globalCpu have been pruned.
// Future work: if Electron main needs limited emulator introspection, expose it explicitly
// via the existing WASM front-end (renderer) bridge or add a new secure preload API.
import { createInterface } from 'readline';
import { join, basename } from 'path';
import { existsSync } from 'fs';
import * as fs from 'fs/promises';
import { watch } from 'fs';
import * as crypto from 'crypto';

let mainWindow: BrowserWindow | null = null;

// --- Emulator load helpers (shared by emu:load and run:compile) -----------------
// Removed cpuColdReset/loadBinaryBase64IntoEmu: renderer now responsible for loading programs
// through WASM interface. If a future headless compile+run flow is required from main process,
// implement a minimal message pass to renderer to request load.

// Attempt BIOS load early; search multiple locations and emit rich diagnostics.
// Search order (first existing directory wins candidate ordering, but we aggregate unique files):
//   1. core/bios/
//   2. bios/ (at repo root)
//   3. repo root (process.cwd())
// Preferred filenames: bios.bin, vectrex.bin (each directory), then any other *.bin
// BIOS auto-loading removed from main process (legacy TS emulator). Responsibility can move to renderer
// using WASM memory inspection. Placeholder retained for minimal compatibility if IPC callers exist.
async function tryLoadBiosOnce(){
  mainWindow?.webContents.send('emu://status', 'BIOS auto-load (legacy) skipped: TS emulator removed.');
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
// Removed ipcMain.handle('emu:load') legacy handler.

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
// Parse compiler diagnostics from output, including semantic errors without location info
function parseCompilerDiagnostics(output: string, sourceFile: string): Array<{ file: string; line: number; col: number; message: string }> {
  const diags: Array<{ file: string; line: number; col: number; message: string }> = [];
  
  for (const line of output.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    
    // Standard format: filename:line:col: message
    const standardMatch = /(.*?):(\d+):(\d+):\s*(.*)/.exec(trimmed);
    if (standardMatch) {
      diags.push({ 
        file: standardMatch[1], 
        line: parseInt(standardMatch[2], 10) - 1, 
        col: parseInt(standardMatch[3], 10) - 1, 
        message: standardMatch[4] 
      });
      continue;
    }
    
    // Semantic errors: [error] SemanticsErrorArity: llamada a 'PRINT_TEXT' con 4 argumentos; se esperaban 3.
    const semanticMatch = /\[error\]\s*(\w+):\s*(.*)/.exec(trimmed);
    if (semanticMatch) {
      diags.push({
        file: sourceFile, // Use the source file being compiled
        line: 0, // Default to line 0 since location is not provided
        col: 0,
        message: `${semanticMatch[1]}: ${semanticMatch[2]}`
      });
      continue;
    }
    
    // Codegen errors: [codegen] Code generation failed due to 1 error(s)
    const codegenMatch = /\[codegen\]\s*(.*)/.exec(trimmed);
    if (codegenMatch && codegenMatch[1].includes('failed')) {
      diags.push({
        file: sourceFile,
        line: 0,
        col: 0,
        message: `Codegen: ${codegenMatch[1]}`
      });
      continue;
    }
    
    // General error patterns
    if (/error|Error|ERROR/.test(trimmed) && !trimmed.includes('ERROR:') && !trimmed.includes('checking')) {
      diags.push({
        file: sourceFile,
        line: 0,
        col: 0,
        message: trimmed
      });
    }
  }
  
  return diags;
}

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
  let savedMTime: number | undefined;
  if (saveIfDirty && typeof saveIfDirty.content === 'string') {
    try {
      const statBefore = await fs.stat(fsPath).catch(()=>null);
      if (saveIfDirty.expectedMTime && statBefore && statBefore.mtimeMs !== saveIfDirty.expectedMTime) {
        return { conflict: true, currentMTime: statBefore.mtimeMs };
      }
      await fs.writeFile(fsPath, saveIfDirty.content, 'utf8');
      // Capture the new modification time after saving
      const statAfter = await fs.stat(fsPath);
      savedMTime = statAfter.mtimeMs;
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
    // Set working directory to project root (three levels up from ide/electron/dist/)
    const projectRoot = join(__dirname, '..', '..', '..');
    const child = spawn(compiler, argsv, { stdio: ['ignore','pipe','pipe'], cwd: projectRoot });
    let stdoutBuf = '';
    let stderrBuf = '';
    child.stdout.on('data', (c: Buffer) => { const txt = c.toString('utf8'); stdoutBuf += txt; mainWindow?.webContents.send('run://stdout', txt); });
    child.stderr.on('data', (c: Buffer) => { const txt = c.toString('utf8'); stderrBuf += txt; mainWindow?.webContents.send('run://stderr', txt); });
    child.on('error', (err) => { resolvePromise({ error: 'spawn_failed', detail: err.message }); });
    child.on('exit', async (code) => {
      if (code !== 0) {
        mainWindow?.webContents.send('run://status', `Compilation FAILED (exit ${code})`);
        // Parse diagnostics from compiler output using improved parser
        const allOutput = stdoutBuf + '\n' + stderrBuf;
        const diags = parseCompilerDiagnostics(allOutput, fsPath);
        if (diags.length) {
          mainWindow?.webContents.send('run://diagnostics', diags);
        }
        return resolvePromise({ error: 'compile_failed', code, stdout: stdoutBuf, stderr: stderrBuf });
      }
      // Check compilation phases: ASM generation + binary assembly
      const binPath = outAsm.replace(/\.asm$/, '.bin');
      
      // Phase 1: Check if ASM was generated
      mainWindow?.webContents.send('run://status', `✓ Compilation Phase 1: Checking ASM generation...`);
      try {
        const asmExists = await fs.access(outAsm).then(() => true).catch(() => false);
        if (!asmExists) {
          mainWindow?.webContents.send('run://stderr', `ERROR: ASM file not generated: ${outAsm}`);
          mainWindow?.webContents.send('run://status', `❌ Phase 1 FAILED: ASM generation failed`);
          
          // Parse semantic errors from stdout/stderr even when ASM is not generated
          const allOutput = stdoutBuf + '\n' + stderrBuf;
          const diags = parseCompilerDiagnostics(allOutput, fsPath);
          if (diags.length) {
            mainWindow?.webContents.send('run://diagnostics', diags);
          }
          
          return resolvePromise({ error: 'asm_not_generated', detail: `Expected ASM file: ${outAsm}` });
        }
        
        const asmStats = await fs.stat(outAsm);
        if (asmStats.size === 0) {
          mainWindow?.webContents.send('run://stderr', `ERROR: ASM file is empty: ${outAsm}`);
          mainWindow?.webContents.send('run://status', `❌ Phase 1 FAILED: Empty ASM file generated`);
          
          // Parse semantic errors from stdout/stderr even when compilation "succeeds" but generates empty ASM
          const allOutput = stdoutBuf + '\n' + stderrBuf;
          const diags = parseCompilerDiagnostics(allOutput, fsPath);
          if (diags.length) {
            mainWindow?.webContents.send('run://diagnostics', diags);
          }
          
          return resolvePromise({ error: 'empty_asm_file', detail: `ASM file exists but is empty: ${outAsm}` });
        }
        
        mainWindow?.webContents.send('run://status', `✓ Phase 1 SUCCESS: ASM generated (${asmStats.size} bytes)`);
      } catch (e: any) {
        mainWindow?.webContents.send('run://stderr', `ERROR checking ASM file: ${e.message}`);
        mainWindow?.webContents.send('run://status', `❌ Phase 1 FAILED: Error checking ASM file`);
        return resolvePromise({ error: 'asm_check_failed', detail: e.message });
      }
      
      // Phase 2: Check if binary was assembled
      mainWindow?.webContents.send('run://status', `✓ Compilation Phase 2: Checking binary assembly...`);
      
      // Add delay to ensure lwasm has completed
      await new Promise(resolve => setTimeout(resolve, 200));
      
      try {
        const binExists = await fs.access(binPath).then(() => true).catch(() => false);
        
        if (!binExists) {
          // Binary not found - check for lwasm errors in stderr
          const lwasmeErrors = stderrBuf.split('\n').filter(line => 
            line.includes('lwasm') || 
            line.includes('fallo') || 
            line.includes('failed') ||
            line.includes('error') ||
            line.includes('ERROR')
          );
          
          if (lwasmeErrors.length > 0) {
            mainWindow?.webContents.send('run://stderr', `❌ LWASM ASSEMBLY FAILED:`);
            lwasmeErrors.forEach(err => mainWindow?.webContents.send('run://stderr', `   ${err}`));
          } else {
            mainWindow?.webContents.send('run://stderr', `❌ BINARY NOT GENERATED: ${binPath}`);
            mainWindow?.webContents.send('run://stderr', `   This usually means lwasm (6809 assembler) is not installed or failed silently.`);
            mainWindow?.webContents.send('run://stderr', `   Install lwasm or check if the generated ASM has syntax errors.`);
          }
          
          // List available files for debugging
          const dir = require('path').dirname(binPath);
          const files = await fs.readdir(dir).catch(() => []);
          const relevantFiles = files.filter(f => f.includes(require('path').basename(binPath, '.bin')));
          mainWindow?.webContents.send('run://stderr', `   Files in directory: ${relevantFiles.join(', ') || 'none'}`);
          
          mainWindow?.webContents.send('run://status', `❌ Phase 2 FAILED: Binary assembly failed`);
          return resolvePromise({ error: 'binary_not_generated', detail: `Binary file not created: ${binPath}` });
        }
        
        // Binary exists - check if it's valid
        const buf = await fs.readFile(binPath);
        
        if (buf.length === 0) {
          mainWindow?.webContents.send('run://stderr', `❌ EMPTY BINARY: ${binPath} (0 bytes)`);
          mainWindow?.webContents.send('run://stderr', `   This indicates lwasm completed but produced no output.`);
          mainWindow?.webContents.send('run://stderr', `   Check the generated ASM file for missing ORG directive or syntax errors.`);
          mainWindow?.webContents.send('run://status', `❌ Phase 2 FAILED: Empty binary generated`);
          return resolvePromise({ error: 'empty_binary', detail: `Binary file is empty: ${binPath}` });
        }
        
        // Success!
        const base64 = Buffer.from(buf).toString('base64');
        mainWindow?.webContents.send('run://status', `✅ COMPILATION SUCCESS: ${binPath} (${buf.length} bytes)`);
        mainWindow?.webContents.send('run://stdout', `✅ Generated binary: ${buf.length} bytes`);
        
        // Notify renderer to load binary
        mainWindow?.webContents.send('emu://compiledBin', { base64, size: buf.length, binPath });
        resolvePromise({ 
          ok: true, 
          binPath, 
          size: buf.length, 
          stdout: stdoutBuf, 
          stderr: stderrBuf,
          savedMTime: savedMTime // Include the mtime if file was saved during compilation
        });
        
      } catch (e: any) {
        mainWindow?.webContents.send('run://stderr', `❌ ERROR reading binary: ${e.message}`);
        mainWindow?.webContents.send('run://status', `❌ Phase 2 FAILED: Error reading binary file`);
        resolvePromise({ error: 'bin_read_failed', detail: e.message });
      }
    });
  });
});

// Emulator: run until next frame (or max steps)
// Removed ipcMain.handle('emu:runFrame') legacy handler.
// Removed all emulator-specific debug IPC handlers (legacy TS). Renderer-side WASM now owns emulator control.

// File helpers (restored after emulator legacy removal)
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

ipcMain.handle('file:readDirectory', async (_e, dirPath: string) => {
  try {
    const fs = require('fs').promises;
    const path = require('path');
    
    async function readDirRecursive(currentPath: string, relativePath: string = ''): Promise<any[]> {
      const entries = await fs.readdir(currentPath, { withFileTypes: true });
      const result = [];
      
      for (const entry of entries) {
        const fullPath = path.join(currentPath, entry.name);
        const relPath = relativePath ? path.join(relativePath, entry.name) : entry.name;
        
        if (entry.isDirectory()) {
          const children = await readDirRecursive(fullPath, relPath);
          result.push({
            name: entry.name,
            path: relPath,
            isDir: true,
            children: children
          });
        } else {
          result.push({
            name: entry.name,
            path: relPath,
            isDir: false
          });
        }
      }
      
      // Sort: directories first, then files, alphabetically
      return result.sort((a, b) => {
        if (a.isDir && !b.isDir) return -1;
        if (!a.isDir && b.isDir) return 1;
        return a.name.localeCompare(b.name);
      });
    }
    
    const files = await readDirRecursive(dirPath);
    return { files };
  } catch (error) {
    return { error: `Failed to read directory: ${error}` };
  }
});

// Delete file or directory
ipcMain.handle('file:delete', async (_e, filePath: string) => {
  if (!filePath) return { error: 'no_path' };
  
  try {
    const stat = await fs.stat(filePath);
    
    if (stat.isDirectory()) {
      // Delete directory recursively
      await fs.rm(filePath, { recursive: true, force: true });
    } else {
      // Delete single file
      await fs.unlink(filePath);
    }
    
    return { success: true, path: filePath };
  } catch (e: any) {
    return { error: e?.message || 'delete_failed' };
  }
});

// Move file or directory
ipcMain.handle('file:move', async (_e, args: { sourcePath: string; targetDir: string }) => {
  const { sourcePath, targetDir } = args;
  if (!sourcePath || !targetDir) return { error: 'missing_paths' };
  
  try {
    const sourceName = basename(sourcePath);
    const targetPath = join(targetDir, sourceName);
    
    // Check if target already exists
    try {
      await fs.stat(targetPath);
      return { error: 'target_exists', targetPath };
    } catch {
      // Target doesn't exist, good to proceed
    }
    
    // Move the file/directory
    await fs.rename(sourcePath, targetPath);
    
    return { success: true, sourcePath, targetPath };
  } catch (e: any) {
    return { error: e?.message || 'move_failed' };
  }
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

// File watcher system
const watchers = new Map<string, ReturnType<typeof watch>>();

ipcMain.handle('file:watchDirectory', async (_e, dirPath: string) => {
  try {
    if (watchers.has(dirPath)) {
      // Already watching this directory
      return { ok: true };
    }

    const watcher = watch(dirPath, { recursive: true }, (eventType, filename) => {
      if (!filename) return;
      
      const fullPath = join(dirPath, filename);
      
      // Skip temporary files and hidden files
      if (filename.startsWith('.') || filename.includes('~') || filename.endsWith('.tmp')) {
        return;
      }
      
      // Determine if it's a directory by trying to stat it
      let isDir = false;
      try {
        const stat = require('fs').statSync(fullPath);
        isDir = stat.isDirectory();
      } catch {
        // File might have been deleted, assume it's a file
        isDir = false;
      }
      
      let changeType: 'added' | 'removed' | 'changed' = 'changed';
      
      // Try to determine if file was added or removed
      try {
        require('fs').accessSync(fullPath);
        changeType = eventType === 'rename' ? 'added' : 'changed';
      } catch {
        changeType = 'removed';
      }
      
      console.log(`[FileWatcher] ${changeType}: ${filename} (dir: ${isDir})`);
      
      // Notify renderer
      mainWindow?.webContents.send('file://changed', {
        type: changeType,
        path: filename,
        isDir
      });
    });
    
    watchers.set(dirPath, watcher);
    console.log(`[FileWatcher] Started watching: ${dirPath}`);
    return { ok: true };
  } catch (error) {
    console.error(`[FileWatcher] Error watching directory ${dirPath}:`, error);
    return { ok: false, error: `Failed to watch directory: ${error}` };
  }
});

ipcMain.handle('file:unwatchDirectory', async (_e, dirPath: string) => {
  const watcher = watchers.get(dirPath);
  if (watcher) {
    watcher.close();
    watchers.delete(dirPath);
    console.log(`[FileWatcher] Stopped watching: ${dirPath}`);
  }
  return { ok: true };
});

// Clean up watchers when app closes
app.on('before-quit', () => {
  for (const watcher of watchers.values()) {
    watcher.close();
  }
  watchers.clear();
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
  // CSP simplificado: por ahora permitimos unsafe-inline para desarrollo y producción
  // En el futuro, cuando creemos un paquete de instalación, implementaremos CSP estricto
  const scriptSrc = "script-src 'self' 'unsafe-inline' 'unsafe-eval'";
  const styleSrc = "style-src 'self' 'unsafe-inline'";
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
app.on('child-process-gone', (_e, details) => {
  console.error('[IDE] child process gone', details);
});
process.on('uncaughtException', (err) => {
  console.error('[IDE] uncaughtException', err);
});
process.on('unhandledRejection', (reason) => {
  console.error('[IDE] unhandledRejection', reason);
});
app.on('window-all-closed', () => {
  console.warn('[IDE] all windows closed');
  if (process.platform !== 'darwin') app.quit();
});
app.on('browser-window-created', (_e, win) => {
  console.log('[IDE] browser-window-created id=', win.id);
  win.on('closed', () => console.log('[IDE] window closed id=', win.id));
  win.webContents.on('did-finish-load', () => console.log('[IDE] did-finish-load main window'));
  win.webContents.on('did-fail-load', (_e, errCode, errDesc) => console.error('[IDE] did-fail-load', errCode, errDesc));
  win.webContents.on('render-process-gone', (_e, details) => console.error('[IDE] wc render-process-gone', details));
  win.webContents.on('unresponsive', () => console.error('[IDE] window unresponsive'));
  win.webContents.on('responsive', () => console.log('[IDE] window responsive again'));
});
app.on('activate', () => { if (BrowserWindow.getAllWindows().length === 0) createWindow(); });
