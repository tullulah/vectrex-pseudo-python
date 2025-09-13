import { app, BrowserWindow, ipcMain, Menu, session, dialog } from 'electron';
import { spawn } from 'child_process';
import { createInterface } from 'readline';
import { join, basename } from 'path';
import { existsSync } from 'fs';
import * as fs from 'fs/promises';

let mainWindow: BrowserWindow | null = null;

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
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      preload: join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
    },
    autoHideMenuBar: true,
    frame: true,
  });
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

// After window creation call buildMenus
app.whenReady().then(() => {
  const verbose = process.env.VPY_IDE_VERBOSE_LSP === '1';
  if (verbose) console.log('[IDE] app.whenReady');
  // Seguridad adicional: anular menú global
  try { Menu.setApplicationMenu(null); } catch {}
  // Inyectar Content-Security-Policy por cabecera (más fuerte que meta) en dev y prod
  const isDev = !!process.env.VITE_DEV_SERVER_URL;
  // CSP:
  // - Prod: máxima restricción (sin inline script/style, sin eval)
  // - Dev: Vite React Fast Refresh inserta un preamble inline + estilos inline para HMR overlay.
  //   Para que funcione el refresco sin errores, concedemos 'unsafe-inline' (script y style) y 'unsafe-eval'.
  //   Estos sólo se añaden cuando isDev es true (cuando cargamos desde Vite dev server).
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
