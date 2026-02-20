Launch the VPy IDE.

Run `./run-ide.sh` from the project root. The IDE is an Electron app with a React+Monaco frontend.

If the script is not found or fails, check:
- `ide/electron/` for the Electron entry point
- `ide/frontend/` for the React UI (run with `npm run dev` in that directory)
- Node.js 22.x is required

The IDE starts an IPC server on port 9123 used by the MCP server.
