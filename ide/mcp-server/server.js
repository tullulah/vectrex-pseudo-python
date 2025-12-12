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
          name: 'editor_write_document',
          description: 'Create OR update a document (automatically opens in editor if new). Use this to create any text file. For .vec and .vmus files, prefer project_create_vector/project_create_music which validate JSON format. CRITICAL VPy RULES: 1) Variables in main() are NOT accessible in loop() - each function has separate scope. Declare variables inside loop() where they are used, NOT in main(). 2) ALWAYS call WAIT_RECAL() at the START of loop() function - this is MANDATORY for proper frame synchronization. 3) Each DRAW_LINE call repositions the beam (creates gaps) - for connected shapes use vector assets (project_create_vector) or coordinate multiple calls carefully. Example: def loop():\n    WAIT_RECAL()  # MANDATORY FIRST\n    # your drawing code',
          inputSchema: {
            type: 'object',
            properties: {
              uri: { type: 'string', description: 'Document URI or relative path (e.g., "src/game.vpy", "README.md")' },
              content: { type: 'string', description: 'Complete file content' }
            },
            required: ['uri', 'content']
          }
        },
        {
          name: 'editor_replace_range',
          description: 'Replace text in a specific range of a document',
          inputSchema: {
            type: 'object',
            properties: {
              uri: { type: 'string', description: 'Document URI' },
              startLine: { type: 'number', description: 'Start line (1-indexed)' },
              startColumn: { type: 'number', description: 'Start column (1-indexed)' },
              endLine: { type: 'number', description: 'End line (1-indexed)' },
              endColumn: { type: 'number', description: 'End column (1-indexed)' },
              newText: { type: 'string', description: 'New text to insert' }
            },
            required: ['uri', 'startLine', 'startColumn', 'endLine', 'endColumn', 'newText']
          }
        },
        {
          name: 'editor_insert_at',
          description: 'Insert text at a specific position',
          inputSchema: {
            type: 'object',
            properties: {
              uri: { type: 'string', description: 'Document URI' },
              line: { type: 'number', description: 'Line number (1-indexed)' },
              column: { type: 'number', description: 'Column number (1-indexed)' },
              text: { type: 'string', description: 'Text to insert' }
            },
            required: ['uri', 'line', 'column', 'text']
          }
        },
        {
          name: 'editor_delete_range',
          description: 'Delete text in a specific range',
          inputSchema: {
            type: 'object',
            properties: {
              uri: { type: 'string', description: 'Document URI' },
              startLine: { type: 'number', description: 'Start line (1-indexed)' },
              startColumn: { type: 'number', description: 'Start column (1-indexed)' },
              endLine: { type: 'number', description: 'End line (1-indexed)' },
              endColumn: { type: 'number', description: 'End column (1-indexed)' }
            },
            required: ['uri', 'startLine', 'startColumn', 'endLine', 'endColumn']
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
          name: 'compiler_build',
          description: 'Build the current project (equivalent to pressing F7). Compiles the project entry file automatically. No parameters needed.',
          inputSchema: {
            type: 'object',
            properties: {},
            required: []
          }
        },
        {
          name: 'compiler_get_errors',
          description: 'Get latest compilation errors',
          inputSchema: {
            type: 'object',
            properties: {}
          }
        },
        {
          name: 'emulator_run',
          description: 'Run a compiled ROM in the emulator',
          inputSchema: {
            type: 'object',
            properties: {
              romPath: { type: 'string', description: 'Path to .bin ROM file' },
              breakOnEntry: { type: 'boolean', description: 'Pause at entry point' }
            },
            required: ['romPath']
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
          name: 'emulator_stop',
          description: 'Stop emulator execution',
          inputSchema: {
            type: 'object',
            properties: {}
          }
        },
        {
          name: 'debugger_add_breakpoint',
          description: 'Add a breakpoint at a specific line',
          inputSchema: {
            type: 'object',
            properties: {
              uri: { type: 'string', description: 'Document URI' },
              line: { type: 'number', description: 'Line number (1-indexed)' }
            },
            required: ['uri', 'line']
          }
        },
        {
          name: 'debugger_get_callstack',
          description: 'Get current call stack',
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
        },
        {
          name: 'project_read_file',
          description: 'Read any file from the project',
          inputSchema: {
            type: 'object',
            properties: {
              path: { type: 'string', description: 'File path relative to project root' }
            },
            required: ['path']
          }
        },
        {
          name: 'project_write_file',
          description: 'Write any file in the project (VPy code, config, text files, etc.). For .vec or .vmus files, prefer project_create_vector/project_create_music which validate JSON format and provide templates. Automatically opens file in editor.',
          inputSchema: {
            type: 'object',
            properties: {
              path: { type: 'string', description: 'File path relative to project root (e.g., "src/main.vpy", "README.md", "config.json")' },
              content: { type: 'string', description: 'Complete file content' }
            },
            required: ['path', 'content']
          }
        },
        {
          name: 'project_create',
          description: 'Create a new VPy project. If path is not provided, a folder selection dialog will be shown to the user.',
          inputSchema: {
            type: 'object',
            properties: {
              name: { type: 'string', description: 'Project name' },
              path: { type: 'string', description: 'Project directory path (optional, will prompt user if not provided)' }
            },
            required: ['name']
          }
        },
        {
          name: 'project_close',
          description: 'Close the current project',
          inputSchema: {
            type: 'object',
            properties: {}
          }
        },
        {
          name: 'project_open',
          description: 'Open an existing VPy project',
          inputSchema: {
            type: 'object',
            properties: {
              path: { type: 'string', description: 'Project directory path' }
            },
            required: ['path']
          }
        },
        {
          name: 'project_create_vector',
          description: 'Create .vec vector graphics file for VPy games (JSON format ONLY). Assets are auto-discovered in assets/vectors/ and embedded in ROM at compile time. Use in code: DRAW_VECTOR("name"). Structure: {"version":"1.0","name":"shape","canvas":{"width":256,"height":256,"origin":"center"},"layers":[{"name":"default","visible":true,"paths":[{"name":"line1","intensity":127,"closed":false,"points":[{"x":0,"y":0},{"x":10,"y":10}]}]}]}. Each path has points array with x,y coordinates (-127 to 127). Triangle example: points:[{"x":0,"y":20},{"x":-15,"y":-10},{"x":15,"y":-10}], closed:true. NO text format - JSON only.',
          inputSchema: {
            type: 'object',
            properties: {
              name: { type: 'string', description: 'Vector file name (without .vec extension)' },
              content: { type: 'string', description: 'Valid JSON string matching exact format: {"version":"1.0","name":"...","canvas":{...},"layers":[{"paths":[{"points":[...]}]}]}. Leave empty for template.' }
            },
            required: ['name']
          }
        },
        {
          name: 'project_create_music',
          description: 'Create .vmus music file for VPy games (JSON format ONLY). Assets are auto-discovered in assets/music/ and embedded in ROM at compile time. Use in code: PLAY_MUSIC("name"). Structure: {"version":"1.0","name":"Song","author":"Composer","tempo":120,"ticksPerBeat":24,"totalTicks":384,"notes":[{"id":"note1","note":60,"start":0,"duration":48,"velocity":12,"channel":0}],"noise":[{"id":"noise1","start":0,"duration":24,"period":15,"channels":1}],"loopStart":0,"loopEnd":384}. note: MIDI number (60=C4), velocity: 0-15 (volume), channel: 0-2 (PSG A/B/C), period: 0-31 (noise pitch). NO text format - JSON only.',
          inputSchema: {
            type: 'object',
            properties: {
              name: { type: 'string', description: 'Music file name (without .vmus extension)' },
              content: { type: 'string', description: 'Valid JSON string matching exact format: {"version":"1.0","tempo":120,"notes":[...],"noise":[...]}. Leave empty for template.' }
            },
            required: ['name']
          }
        }
      ]
    };
  }
  
  async handleToolCall(params) {
    const { name, arguments: args } = params;
    log('Tool call:', name, 'Arguments:', JSON.stringify(args));
    
    // Convert tool name to method (editor_list_documents -> editor/list_documents)
    // Only replace first underscore with slash to match Electron server naming
    const method = name.replace('_', '/');
    
    // Ensure args is an object (MCP spec may send undefined for no arguments)
    const toolParams = args || {};
    log('Forwarding to IDE - Method:', method, 'Params:', JSON.stringify(toolParams));
    
    // Forward to IDE via IPC
    const result = await sendToIDE({
      jsonrpc: '2.0',
      method,
      params: toolParams
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
