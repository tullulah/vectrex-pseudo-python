LSP Framing Implementation
==========================

Overview
--------
The Language Server Protocol uses a simple HTTP-like header framing over stdio:

Content-Length: <bytes>\r\n
<optional-other-headers>\r\n
\r\n
<JSON payload bytes>

Changes Implemented (2025-09-13)
---------------------------------
1. Electron main process now parses headers and reads an exact byte length before emitting events.
2. Each complete JSON payload is emitted via the `lsp://message` IPC channel (string JSON).
3. Raw JSON also forwarded to `lsp://stdout` for debugging; stderr lines go to `lsp://stderr`.
4. Frontend `lspClient` subscribes only to `lsp://message` for protocol handling; no heuristic line splitting.

Flow
----
Frontend -> Server:
1. Serialize JSON-RPC object.
2. Call `window.electronAPI.lspSend(json)`.
3. Main writes `Content-Length` header + CRLF CRLF + body to LSP child stdin.

Server -> Frontend:
1. LSP child writes framed responses/notifications.
2. Main reader loop extracts bodies and emits `lsp://message` with UTF-8 JSON text.
3. Frontend parses and routes responses or notifications.

Error Handling & Edge Cases
---------------------------
- Non-UTF8 bodies: Logged to `lsp://stderr` and skipped.
- Partial reads / EOF: Terminates reader thread silently.
- Missing Content-Length header: Header block skipped and loop continues.

Future Enhancements
-------------------
- Add backpressure / queue metrics (e.g., number of pending requests).
- Optional binary logging toggle for debugging large payloads.
- Reconnect strategy if LSP process exits unexpectedly.
- Support cancellation tokens (send `$\/cancelRequest`).

Testing Tips
------------
- Introduce a syntax error; expect `textDocument/publishDiagnostics` to fire and Monaco markers to appear.
- Insert many rapid edits; ensure version increments without lost diagnostics.
- Use DevTools console with log filter `[LSP-RAW]` to inspect raw payloads.

Packaging Note
--------------
In a packaged build the `vpy_lsp` binary must be colocated with the main executable or on PATH. Current spawn relies on relative resolution; configure electron-builder (future) to copy it next to the app binary.
