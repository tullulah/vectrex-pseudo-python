# VPy IDE MCP Server

Model Context Protocol (MCP) server for VPy IDE. Exposes IDE state and operations to AI agents.

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

### With VS Code Copilot

Add to VS Code settings (when MCP support is available):

```json
{
  "github.copilot.advanced": {
    "mcp": {
      "servers": {
        "vpy-ide": {
          "command": "/Users/daniel/projects/vectrex-pseudo-python/ide/mcp-server/mcp-server.js"
        }
      }
    }
  }
}
```

### Manual Testing

```bash
# Start VPy IDE first
./run-ide.sh

# In another terminal, test MCP server manually
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | node ide/mcp-server/server.js

# Or with verbose logging
MCP_VERBOSE=1 node ide/mcp-server/server.js
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
