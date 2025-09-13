# Electron variant for VPY IDE

This directory contains the Electron shell reusing the existing React + Vite frontend in `../frontend`.

## Structure
```
ide/
  frontend/        <- existing React/Vite app (renderer)
  electron/
    package.json   <- electron app package (scripts for dev/build)
    tsconfig.json
    src/
   main.ts      <- Electron main process (spawns LSP child)
      preload.ts   <- Exposes limited API to renderer (lspStart, lspSend, event hooks)
```

## Dev workflow
1. Install deps (renderer + electron):
   - `cd ide/frontend && npm install`
   - `cd ../electron && npm install`
2. Start dev (two processes coordinated):
   - `npm run dev` inside `ide/electron` will start Vite (renderer) and then Electron once port 5173 is up.
3. The renderer detects Electron via `window.electronAPI` and routes LSP calls through IPC.

## Build
```
cd ide/electron
npm run build   # tsc compile main + preload
# For packaging (installer / distributable):
npm run package
```
Outputs will appear per electron-builder defaults (configure as needed in future).

## LSP integration
`main.ts` spawns `vpy_lsp` (debug path heuristic: `target/debug/vpy_lsp(.exe)`). It replicates the Tauri Rust logic:
- Parses `Content-Length` framed messages.
- Emits events: `lsp://message`, `lsp://stdout`, `lsp://stderr`.
- Preload exposes subscription helpers consumed by `lspClient.ts`.

## Frontend adaptation
`lspClient.ts`:
- Detects Electron (`window.electronAPI`).
- No-op in plain web build (can add WebSocket fallback later).

## Next enhancements (optional)
- Add file open/save dialogs via `ipcMain.handle('dialog_open')` etc.
- Implement binary compilation commands bridging to the Rust compiler.
- Provide auto-download / build check for `vpy_lsp` missing binary.
- Harden process lifecycle (restart on crash, kill on quit).

## Security notes
- `contextIsolation: true` & only whitelisted API via preload.
- No `nodeIntegration` in renderer.
- Add CSP meta tag in production build if needed.

## Packaging strategy (future)
Add electron-builder config to produce platform installers and bundle the `vpy_lsp` binary.
