# VPy IDE Architecture

## Stack
- Tauri (Rust backend shell)
- React + TypeScript (UI)
- Monaco Editor (code editing)
- Zustand (state stores as ViewModels)
- i18next (internationalization)
- JSON-RPC LSP client (spawn existing `vpy_lsp`)

## Layout
Flex layout split:
- Left: FileTreePanel
- Center: EditorTabs + EditorView
- Right: EmulatorPane (placeholder)
- Bottom: DebugPanel (Registers / Variables / Constants / Logs tabs)

## State (Models)
Project, FileNode, Document, DebugState, EmulatorState.

## ViewModels (Stores)
- projectStore: project root + file tree + selection
- editorStore: open documents, active tab tracking, content & diagnostics
- lspStore: connection, requests (initialize, didOpen, didChange, semantic tokens)
- debugStore: emulator registers/variables (mock until runtime exists)
- emulatorStore: execution state (running/stopped)

## i18n
Directory: `frontend/src/locales/{en,es}` with namespaces: common, editor, fileTree, debug, emulator, diagnostics.
Fallback: en.

## LSP Localization
`initialize` will accept `initializationOptions.locale`. Diagnostics routed through translation helper `tr(locale, key)` with fallback.

## File Tree Acquisition
Tauri command `list_dir_recursive` returns JSON tree. Future: file system watch.

## Semantic Tokens Flow
1. editorStore opens doc
2. lspStore sends didOpen
3. request semantic tokens -> map to Monaco tokens provider.

## Future Extensions
- Run/Preview pipeline: AST -> vector segments -> canvas render.
- Debug protocol (custom or DAP bridge) for step/inspect.
- Multi-file symbol index.

## Folder Structure (Proposed)
```
ide/
  ARCHITECTURE.md
  frontend/
    package.json (later)
    src/
      main.tsx
      app/
        components/
          FileTreePanel.tsx
          EditorTabs.tsx
          EditorView.tsx
          EmulatorPane.tsx
          DebugPanel.tsx
      state/
        projectStore.ts
        editorStore.ts
        lspStore.ts
        debugStore.ts
        emulatorStore.ts
      services/
        lspService.ts
        fsService.ts
        debugService.ts
        emulatorService.ts
      types/
        models.ts
      locales/
        en/*.json
        es/*.json
```

## Open Questions
- Packaging strategy per OS (code signing later)
- Whether to embed LSP directly vs spawning process
- Persistence of user settings (JSON vs SQLite)
