#!/usr/bin/env node
/**
 * VPy IDE MCP Server - Model Context Protocol stdio server
 * 
 * This server implements the MCP protocol via stdin/stdout to expose
 * IDE state and operations to AI agents like Copilot, Claude, etc.
 * 
 * Protocol: JSON-RPC 2.0 over stdio with Content-Length headers
 * 
 * Usage:
 *   node server.js
 * 
 * The server communicates with the running Electron IDE via IPC
 * to access editor state, compiler, emulator, debugger, etc.
 */

const net = require('net');
const readline = require('readline');

// Configuration
const IPC_PORT = 9123; // Port where Electron main listens for MCP IPC
const VERBOSE = process.env.MCP_VERBOSE === '1';

function log(...args) {
  if (VERBOSE) {
    console.error('[MCP Server]', ...args);
  }
}

// Connection to Electron IDE
let ipcSocket = null;
let ipcConnected = false;
let ipcCallbacks = new Map();
let ipcCallId = 0;

// Connect to Electron IDE's IPC server
async function connectToIDE() {
  return new Promise((resolve, reject) => {
    log('Connecting to IDE IPC on port', IPC_PORT);
    
    const socket = net.createConnection({ port: IPC_PORT, host: 'localhost' });
    
    socket.on('connect', () => {
      log('Connected to IDE IPC');
      ipcSocket = socket;
      ipcConnected = true;
      resolve();
    });
    
    socket.on('error', (err) => {
      log('IPC connection error:', err.message);
      if (!ipcConnected) {
        reject(err);
      }
    });
    
    socket.on('close', () => {
      log('IPC connection closed');
      ipcSocket = null;
      ipcConnected = false;
    });
    
    // Read JSON-RPC responses from IPC
    let buffer = '';
    socket.on('data', (chunk) => {
      buffer += chunk.toString();
      
      // Split by newlines (simple protocol for now)
      const lines = buffer.split('\n');
      buffer = lines.pop() || '';
      
      for (const line of lines) {
        if (line.trim()) {
          try {
            const response = JSON.parse(line);
            const callback = ipcCallbacks.get(response.id);
            if (callback) {
              ipcCallbacks.delete(response.id);
              callback(response);
            }
          } catch (e) {
            log('Failed to parse IPC response:', e.message);
          }
        }
      }
    });
  });
}

// Send request to IDE via IPC
async function sendToIDE(request) {
  if (!ipcConnected || !ipcSocket) {
    throw new Error('Not connected to IDE');
  }
  
  return new Promise((resolve, reject) => {
    const id = ipcCallId++;
    const wrappedRequest = { ...request, id };
    
    ipcCallbacks.set(id, (response) => {
      if (response.error) {
        reject(new Error(response.error.message));
      } else {
        resolve(response.result);
      }
    });
    
    // Send with newline delimiter
    ipcSocket.write(JSON.stringify(wrappedRequest) + '\n');
    
    // Timeout after 30s
    setTimeout(() => {
      if (ipcCallbacks.has(id)) {
        ipcCallbacks.delete(id);
        reject(new Error('IPC request timeout'));
      }
    }, 30000);
  });
}

// MCP Protocol - stdio transport
class StdioTransport {
  constructor() {
    this.buffer = '';
    this.expectedLength = null;
    this.setupStdio();
  }
  
  setupStdio() {
    // Read from stdin
    process.stdin.on('data', (chunk) => {
      this.buffer += chunk.toString();
      this.processBuffer();
    });
    
    process.stdin.on('end', () => {
      log('stdin closed');
      process.exit(0);
    });
  }
  
  processBuffer() {
    while (true) {
      if (this.expectedLength === null) {
        // Look for Content-Length header
        const headerEnd = this.buffer.indexOf('\r\n\r\n');
        if (headerEnd === -1) break;
        
        const headers = this.buffer.slice(0, headerEnd);
        const match = /Content-Length: *(\d+)/i.exec(headers);
        
        if (!match) {
          this.buffer = this.buffer.slice(headerEnd + 4);
          continue;
        }
        
        this.expectedLength = parseInt(match[1], 10);
        this.buffer = this.buffer.slice(headerEnd + 4);
      }
      
      if (this.buffer.length >= this.expectedLength) {
        const message = this.buffer.slice(0, this.expectedLength);
        this.buffer = this.buffer.slice(this.expectedLength);
        this.expectedLength = null;
        
        try {
          const request = JSON.parse(message);
          this.handleRequest(request).catch(err => {
            log('Error handling request:', err);
          });
        } catch (e) {
          log('Failed to parse message:', e.message);
        }
        
        continue;
      }
      
      break;
    }
  }
  
  async handleRequest(request) {
    log('Received request:', request.method);
    
    try {
      let result;
      
      // Handle MCP protocol methods
      if (request.method === 'initialize') {
        result = await this.handleInitialize(request.params);
      } else if (request.method === 'tools/list') {
        result = await this.handleToolsList();
      } else if (request.method === 'tools/call') {
        result = await this.handleToolCall(request.params);
      } else {
        // Unknown method
        this.sendResponse({
          jsonrpc: '2.0',
          id: request.id,
          error: {
            code: -32601,
            message: `Method not found: ${request.method}`
          }
        });
        return;
      }
      
      this.sendResponse({
        jsonrpc: '2.0',
        id: request.id,
        result
      });
    } catch (error) {
      log('Error:', error);
      this.sendResponse({
        jsonrpc: '2.0',
        id: request.id,
        error: {
          code: -32603,
          message: error.message
        }
      });
    }
  }
  
  async handleInitialize(params) {
    log('Initialize with params:', params);
    return {
      protocolVersion: '2024-11-05',
      capabilities: {
        tools: {}
      },
      serverInfo: {
        name: 'vpy-ide-mcp',
        version: '0.1.0'
      }
    };
  }
  
  async handleToolsList() {
    return {
      tools: [
        {
          name: 'editor_list_documents',
          description: 'List all open documents in the IDE',
          inputSchema: {
            type: 'object',
            properties: {}
          }
        },
        {
          name: 'editor_read_document',
          description: 'Read content of a specific document',
          inputSchema: {
            type: 'object',
            properties: {
              uri: { type: 'string', description: 'Document URI' }
            },
            required: ['uri']
          }
        },
        {
          name: 'editor_get_diagnostics',
          description: 'Get compilation/lint diagnostics',
          inputSchema: {
            type: 'object',
            properties: {
              uri: { type: 'string', description: 'Document URI (optional)' }
            }
          }
        },
        {
          name: 'emulator_get_state',
          description: 'Get current emulator state (PC, registers, cycles)',
          inputSchema: {
            type: 'object',
            properties: {}
          }
        },
        {
          name: 'project_get_structure',
          description: 'Get project structure and files',
          inputSchema: {
            type: 'object',
            properties: {}
          }
        }
      ]
    };
  }
  
  async handleToolCall(params) {
    const { name, arguments: args } = params;
    log('Tool call:', name, args);
    
    // Convert tool name to method (editor_list_documents -> editor/list_documents)
    const method = name.replace(/_/g, '/');
    
    // Forward to IDE via IPC
    const result = await sendToIDE({
      jsonrpc: '2.0',
      method,
      params: args || {}
    });
    
    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(result, null, 2)
        }
      ]
    };
  }
  
  sendResponse(response) {
    const message = JSON.stringify(response);
    const header = `Content-Length: ${Buffer.byteLength(message)}\r\n\r\n`;
    process.stdout.write(header + message);
    log('Sent response:', response.id || 'notification');
  }
}

// Main
async function main() {
  log('VPy IDE MCP Server starting...');
  
  try {
    // Connect to IDE
    await connectToIDE();
    log('Connected to IDE');
    
    // Start stdio transport
    const transport = new StdioTransport();
    log('MCP server ready on stdio');
    
  } catch (error) {
    console.error('Failed to start MCP server:', error.message);
    console.error('Make sure the VPy IDE is running with MCP IPC enabled');
    process.exit(1);
  }
}

main();
