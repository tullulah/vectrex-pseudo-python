#!/usr/bin/env node
/**
 * VPy IDE MCP Server Launcher
 * 
 * This script starts the MCP stdio server that connects to the running VPy IDE.
 * It can be used by AI agents (Copilot, Claude Desktop, etc.) to interact with the IDE.
 * 
 * Prerequisites:
 * - VPy IDE must be running
 * - IDE must have MCP IPC server enabled (automatic)
 * 
 * Usage from MCP client config:
 * {
 *   "vpy-ide": {
 *     "command": "/path/to/vectrex-pseudo-python/ide/mcp-server/mcp-server.js"
 *   }
 * }
 */

const path = require('path');
const { spawn } = require('child_process');

const serverPath = path.join(__dirname, 'server.js');

// Launch the server
const child = spawn('node', [serverPath], {
  stdio: 'inherit',
  env: {
    ...process.env,
    MCP_VERBOSE: process.env.MCP_VERBOSE || '0'
  }
});

child.on('error', (err) => {
  console.error('Failed to start MCP server:', err);
  process.exit(1);
});

child.on('exit', (code) => {
  process.exit(code || 0);
});

// Handle termination
process.on('SIGINT', () => {
  child.kill('SIGINT');
});

process.on('SIGTERM', () => {
  child.kill('SIGTERM');
});
