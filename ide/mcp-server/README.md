# VPy IDE MCP Server

Model Context Protocol (MCP) server for VPy IDE. Exposes IDE state and operations to AI agents.

## ✅ STATUS: FULLY OPERATIONAL

The MCP server is working and verified. Copilot can now see IDE state in real-time!

**Verified Working:**
- ✅ Open documents (4 documents detected: main.vpy, pijij.vmus, world1.vec, aaa3D.vec)
- ✅ Emulator state (PC, registers, cycles, FPS)
- ✅ Complete project structure (assets/, src/, build/ folders)
- ⏳ Compilation diagnostics (next)
- ⏳ Debugger operations (next)

## Architecture

```
┌──────────────┐         ┌─────────────┐         ┌──────────────┐
│ AI Agent     │◄─stdio─►│ MCP Server  │◄─TCP────►│ Electron IDE │
│ (Copilot)    │         │ (Node.js)   │  :9123   │              │
└──────────────┘         └─────────────┘         └──────────────┘
```

- **AI Agent**: Copilot, Claude Desktop, Cline, etc.
- **MCP Server**: Standalone Node.js process (this directory)
- **Electron IDE**: Running VPy IDE with state access

## Protocol

- **Transport**: stdin/stdout with Content-Length headers
- **Format**: JSON-RPC 2.0
- **Specification**: MCP 2024-11-05

## Available Tools

### Editor
- `editor_list_documents` - List open documents
- `editor_read_document` - Read document content
- `editor_get_diagnostics` - Get compilation errors

### Emulator
- `emulator_get_state` - Get CPU state (PC, registers, cycles)

### Project
- `project_get_structure` - Get project file structure

## Usage

### With Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "vpy-ide": {
      "command": "/Users/daniel/projects/vectrex-pseudo-python/ide/mcp-server/mcp-server.js"
    }
  }
}
```

### With VS Code Copilot ✅ CONFIGURED

Add to `~/Library/Application Support/Code/User/mcp.json`:

```json
{
  "servers": {
    "vpy-ide": {
      "command": "/Users/daniel/projects/vectrex-pseudo-python/ide/mcp-server/mcp-server.js",
      "args": [],
      "type": "stdio",
      "env": {
        "MCP_VERBOSE": "1"
      }
    }
  }
}
```

**✅ Already configured on this system!** Restart VS Code to activate.

### Manual Testing ✅ VERIFIED

```bash
# Start VPy IDE first
./run-ide.sh

# Run comprehensive test client
cd ide/mcp-server
node test-client.js

# Output shows:
# ✅ Connection to IDE on port 9123
# ✅ Initialize protocol
# ✅ List 5 available tools
# ✅ editor_list_documents - Returns 4 open documents
# ✅ emulator_get_state - Returns emulator state
# ✅ project_get_structure - Returns complete project tree
```

## Requirements

- Node.js 14+
- VPy IDE running (launches IPC server on port 9123)

## Development

Enable verbose logging:

```bash
export MCP_VERBOSE=1
node ide/mcp-server/server.js
```

## Troubleshooting

### "Failed to connect to IDE"
- Make sure VPy IDE is running
- Check that port 9123 is not in use by another process
- Try restarting the IDE with verbose logging: `VPY_IDE_VERBOSE_LSP=1 ./run-ide.sh`

### "Method not found"
- Check that you're using the correct tool names (use underscores, not slashes)
- Run `tools/list` to see available tools

### "IPC request timeout"
- IDE may be busy or frozen
- Try stopping and restarting the IDE
