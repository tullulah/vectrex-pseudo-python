#!/usr/bin/env node
/**
 * Test client for VPy IDE MCP Server
 * 
 * This client tests the MCP stdio server by sending requests
 * and displaying responses in a human-readable format.
 */

const { spawn } = require('child_process');
const readline = require('readline');

// ANSI colors
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  green: '\x1b[32m',
  blue: '\x1b[34m',
  yellow: '\x1b[33m',
  red: '\x1b[31m',
  cyan: '\x1b[36m'
};

function log(color, ...args) {
  console.log(color + args.join(' ') + colors.reset);
}

// Spawn MCP server
log(colors.cyan, '\n🚀 Starting MCP Server...\n');
const server = spawn('node', ['server.js'], {
  cwd: __dirname,
  stdio: ['pipe', 'pipe', 'pipe'],
  env: { ...process.env, MCP_VERBOSE: '1' }
});

let buffer = '';
let requestId = 1;

// Parse MCP messages (Content-Length protocol)
server.stdout.on('data', (data) => {
  buffer += data.toString();
  
  while (true) {
    const headerMatch = buffer.match(/^Content-Length: (\d+)\r?\n\r?\n/);
    if (!headerMatch) break;
    
    const contentLength = parseInt(headerMatch[1]);
    const headerLength = headerMatch[0].length;
    
    if (buffer.length < headerLength + contentLength) break;
    
    const content = buffer.substring(headerLength, headerLength + contentLength);
    buffer = buffer.substring(headerLength + contentLength);
    
    try {
      const message = JSON.parse(content);
      log(colors.green, '\n📥 Response:');
      console.log(JSON.stringify(message, null, 2));
    } catch (e) {
      log(colors.red, '❌ Failed to parse response:', e.message);
    }
  }
});

server.stderr.on('data', (data) => {
  log(colors.yellow, data.toString().trim());
});

server.on('exit', (code) => {
  log(colors.blue, `\n👋 Server exited with code ${code}`);
  process.exit(code);
});

// Send MCP request
function sendRequest(method, params = {}) {
  const request = {
    jsonrpc: '2.0',
    id: requestId++,
    method,
    params
  };
  
  const content = JSON.stringify(request);
  const header = `Content-Length: ${content.length}\r\n\r\n`;
  
  log(colors.blue, `\n📤 Sending: ${method}`);
  console.log(JSON.stringify(request, null, 2));
  
  server.stdin.write(header + content);
}

// Wait for server to initialize
setTimeout(() => {
  // Test sequence
  log(colors.cyan, '\n🧪 Starting test sequence...\n');
  
  // 1. Initialize
  sendRequest('initialize', {
    protocolVersion: '2024-11-05',
    capabilities: {},
    clientInfo: {
      name: 'test-client',
      version: '1.0.0'
    }
  });
  
  setTimeout(() => {
    // 2. List available tools
    sendRequest('tools/list', {});
  }, 1000);
  
  setTimeout(() => {
    // 3. List documents
    sendRequest('tools/call', {
      name: 'editor_list_documents',
      arguments: {}
    });
  }, 2000);
  
  setTimeout(() => {
    // 4. Get emulator state
    sendRequest('tools/call', {
      name: 'emulator_get_state',
      arguments: {}
    });
  }, 3000);
  
  setTimeout(() => {
    // 5. Get project structure
    sendRequest('tools/call', {
      name: 'project_get_structure',
      arguments: {}
    });
  }, 4000);
  
  setTimeout(() => {
    log(colors.cyan, '\n✅ Test sequence complete!\n');
    server.stdin.end();
  }, 5000);
  
}, 2000);

// Handle Ctrl+C
process.on('SIGINT', () => {
  log(colors.yellow, '\n\n⚠️  Interrupted by user');
  server.kill();
  process.exit(0);
});
