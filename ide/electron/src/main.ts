import { app, BrowserWindow, ipcMain, Menu, session } from 'electron';
import { spawn } from 'child_process';
import { createInterface } from 'readline';
import { join } from 'path';
import { existsSync } from 'fs';

let mainWindow: BrowserWindow | null = null;

interface LspChild {
  proc: ReturnType<typeof spawn>;
  stdin: NodeJS.WritableStream;
}
let lsp: LspChild | null = null;

async function createWindow() {
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
    await mainWindow.loadURL(devUrl);
  } else {
    await mainWindow.loadFile(join(__dirname, '../../frontend/dist/index.html'));
  }
  // Bloquear apertura automática salvo flag explícita
  if (process.env.VPY_IDE_DEVTOOLS === '1') {
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
      }
    });
  }
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
  if (lsp) return;
  const path = resolveLspPath();
  if (!path) return; // mensaje detallado ya emitido en resolveLspPath (una sola vez)
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
        continue;
      }
      break;
    }
  });

  const rlErr = createInterface({ input: child.stderr });
  rlErr.on('line', line => mainWindow?.webContents.send('lsp://stderr', line));
  child.on('exit', code => {
    mainWindow?.webContents.send('lsp://stderr', `[LSP exited ${code}]`);
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

// After window creation call buildMenus
app.whenReady().then(() => {
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
  createWindow();
});
app.on('window-all-closed', () => { if (process.platform !== 'darwin') app.quit(); });
app.on('activate', () => { if (BrowserWindow.getAllWindows().length === 0) createWindow(); });
