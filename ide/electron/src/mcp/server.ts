// MCP Server - Model Context Protocol server implementation
// Exposes IDE state and operations to AI agents (Copilot, PyPilot, etc.)

import { BrowserWindow } from 'electron';
import {
  MCPRequest,
  MCPResponse,
  MCPError,
  MCPNotification,
  MCPTool,
  ErrorCodes,
} from './types.js';

export class MCPServer {
  private tools: Map<string, MCPToolHandler> = new Map();
  private toolSchemas: Map<string, MCPTool> = new Map();
  private mainWindow: BrowserWindow | null = null;

  constructor() {
    this.registerAllTools();
  }

  setMainWindow(window: BrowserWindow) {
    this.mainWindow = window;
  }

  // Register all available tools
  private registerAllTools() {
    // Editor tools
    this.registerTool('editor/list_documents', this.listDocuments.bind(this), {
      name: 'editor/list_documents',
      description: 'List all open documents in the editor',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('editor/read_document', this.readDocument.bind(this), {
      name: 'editor/read_document',
      description: 'Read content of a document ALREADY OPEN in the editor. For NEW files use editor/write_document or project/create_music/create_vector instead.',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI (file must be currently open in editor)' },
        },
        required: ['uri'],
      },
    });

    this.registerTool('editor/write_document', this.writeDocument.bind(this), {
      name: 'editor/write_document',
      description: 'Create OR update a document (automatically opens in editor if new). Use for text files. For .vec and .vmus, prefer project/create_vector or project/create_music which validate JSON format. Auto-detects language.',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI (file://, relative path, or filename like "game.vpy")' },
          content: { type: 'string', description: 'Complete file content' },
        },
        required: ['uri', 'content'],
      },
    });

    this.registerTool('editor/save_document', this.saveDocument.bind(this), {
      name: 'editor/save_document',
      description: 'Save an open document to disk and mark as clean (not dirty). CRITICAL: Use this after editor/write_document before compilation to ensure compiler reads latest content.',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI (file must be open in editor)' },
        },
        required: ['uri'],
      },
    });

    this.registerTool('editor/get_diagnostics', this.getDiagnostics.bind(this), {
      name: 'editor/get_diagnostics',
      description: 'Get compilation/lint diagnostics',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI (optional, all if omitted)' },
        },
      },
    });

    this.registerTool('editor/replace_range', this.replaceRange.bind(this), {
      name: 'editor/replace_range',
      description: 'Replace specific LINES in an open document (NOT character offsets). Document must be open. For replacing entire file use editor/write_document.',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI (must be open in editor)' },
          startLine: { type: 'number', description: 'Start line number (1-indexed, REQUIRED)' },
          startColumn: { type: 'number', description: 'Start column (1-indexed)' },
          endLine: { type: 'number', description: 'End line number (1-indexed, REQUIRED)' },
          endColumn: { type: 'number', description: 'End column (1-indexed)' },
          newText: { type: 'string', description: 'New text to insert' },
        },
        required: ['uri', 'startLine', 'startColumn', 'endLine', 'endColumn', 'newText'],
      },
    });

    this.registerTool('editor/insert_at', this.insertAt.bind(this), {
      name: 'editor/insert_at',
      description: 'Insert text at a specific position',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI' },
          line: { type: 'number', description: 'Line number (1-indexed)' },
          column: { type: 'number', description: 'Column number (1-indexed)' },
          text: { type: 'string', description: 'Text to insert' },
        },
        required: ['uri', 'line', 'column', 'text'],
      },
    });

    this.registerTool('editor/delete_range', this.deleteRange.bind(this), {
      name: 'editor/delete_range',
      description: 'Delete text in a specific range',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI' },
          startLine: { type: 'number', description: 'Start line (1-indexed)' },
          startColumn: { type: 'number', description: 'Start column (1-indexed)' },
          endLine: { type: 'number', description: 'End line (1-indexed)' },
          endColumn: { type: 'number', description: 'End column (1-indexed)' },
        },
        required: ['uri', 'startLine', 'startColumn', 'endLine', 'endColumn'],
      },
    });

    // Compiler tools
    this.registerTool('compiler/build', this.compilerBuild.bind(this), {
      name: 'compiler/build',
      description: 'Build the current project (same as F7). Compiles the project entry file. No parameters needed - uses current project configuration.',
      inputSchema: {
        type: 'object',
        properties: {},
        required: [],
      },
    });

    this.registerTool('compiler/get_errors', this.getCompilerErrors.bind(this), {
      name: 'compiler/get_errors',
      description: 'Get latest compilation errors',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('compiler/build_and_run', this.buildAndRun.bind(this), {
      name: 'compiler/build_and_run',
      description: 'Build current project and run it in emulator (combines compiler/build + emulator/run). Use this for quick testing. Returns compilation errors if build fails.',
      inputSchema: {
        type: 'object',
        properties: {
          breakOnEntry: { type: 'boolean', description: 'Pause at entry point (optional)' },
        },
        required: [],
      },
    });

    // Emulator tools
    this.registerTool('emulator/run', this.emulatorRun.bind(this), {
      name: 'emulator/run',
      description: 'Run a compiled ROM in the emulator',
      inputSchema: {
        type: 'object',
        properties: {
          romPath: { type: 'string', description: 'Path to .bin ROM file' },
          breakOnEntry: { type: 'boolean', description: 'Pause at entry point' },
        },
        required: ['romPath'],
      },
    });

    this.registerTool('emulator/get_state', this.getEmulatorState.bind(this), {
      name: 'emulator/get_state',
      description: 'Get current emulator state (PC, registers, cycles)',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('emulator/stop', this.emulatorStop.bind(this), {
      name: 'emulator/stop',
      description: 'Stop emulator execution',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    // Memory tools
    this.registerTool('memory/dump', this.memoryDump.bind(this), {
      name: 'memory/dump',
      description: 'Get memory snapshot from emulator RAM. Returns hex dump of specified region.',
      inputSchema: {
        type: 'object',
        properties: {
          start: { type: 'number', description: 'Start address (hex or decimal, default: 0xC800 = RAM start)' },
          end: { type: 'number', description: 'End address (hex or decimal, default: 0xCFFF = RAM end)' },
          format: { type: 'string', description: 'Output format: "hex" (default) or "decimal"', enum: ['hex', 'decimal'] },
        },
      },
    });

    this.registerTool('memory/list_variables', this.listVariables.bind(this), {
      name: 'memory/list_variables',
      description: 'Get all variables from PDB with addresses, sizes, and types. Useful for identifying which variables consume most RAM.',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('memory/read_variable', this.readVariable.bind(this), {
      name: 'memory/read_variable',
      description: 'Read current value of a specific variable from emulator RAM',
      inputSchema: {
        type: 'object',
        properties: {
          name: { type: 'string', description: 'Variable name (without VAR_ prefix, e.g., "player_x")' },
        },
        required: ['name'],
      },
    });

    this.registerTool('memory/write', this.memoryWrite.bind(this), {
      name: 'memory/write',
      description: 'Write value to memory address (for testing/debugging)',
      inputSchema: {
        type: 'object',
        properties: {
          address: { type: 'number', description: 'Memory address (hex or decimal)' },
          value: { type: 'number', description: 'Value to write (0-255 for 8-bit)' },
          size: { type: 'number', description: 'Size in bytes (1 or 2, default: 1)' },
        },
        required: ['address', 'value'],
      },
    });

    // Debugger tools
    this.registerTool('debugger/add_breakpoint', this.addBreakpoint.bind(this), {
      name: 'debugger/add_breakpoint',
      description: 'Add a breakpoint at a specific line',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI' },
          line: { type: 'number', description: 'Line number (1-indexed)' },
        },
        required: ['uri', 'line'],
      },
    });

    this.registerTool('debugger/remove_breakpoint', this.removeBreakpoint.bind(this), {
      name: 'debugger/remove_breakpoint',
      description: 'Remove a specific breakpoint',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI' },
          line: { type: 'number', description: 'Line number (1-indexed)' },
        },
        required: ['uri', 'line'],
      },
    });

    this.registerTool('debugger/list_breakpoints', this.listBreakpoints.bind(this), {
      name: 'debugger/list_breakpoints',
      description: 'List all active breakpoints',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('debugger/clear_breakpoints', this.clearBreakpoints.bind(this), {
      name: 'debugger/clear_breakpoints',
      description: 'Remove all breakpoints',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('debugger/step_into', this.stepInto.bind(this), {
      name: 'debugger/step_into',
      description: 'Step into next instruction (F11). If in VPy code, switches to ASM view without executing.',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('debugger/step_over', this.stepOver.bind(this), {
      name: 'debugger/step_over',
      description: 'Step over next instruction (F10)',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('debugger/step_out', this.stepOut.bind(this), {
      name: 'debugger/step_out',
      description: 'Step out of current function (Shift+F11)',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('debugger/continue', this.debugContinue.bind(this), {
      name: 'debugger/continue',
      description: 'Continue execution until next breakpoint (F5)',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('debugger/pause', this.debugPause.bind(this), {
      name: 'debugger/pause',
      description: 'Pause execution',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('debugger/get_callstack', this.getCallstack.bind(this), {
      name: 'debugger/get_callstack',
      description: 'Get current call stack',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('debugger/start', this.debugStart.bind(this), {
      name: 'debugger/start',
      description: 'Start debugging session (Ctrl+F5) - compiles and loads with breakpoints',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    // Observability tools
    this.registerTool('debugger/get_registers', this.getRegisters.bind(this), {
      name: 'debugger/get_registers',
      description: 'Get current CPU register values (A, B, X, Y, U, S, PC, DP, CC)',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('memory/dump', this.memoryDump.bind(this), {
      name: 'memory/dump',
      description: 'Get memory snapshot (hex dump of RAM region)',
      inputSchema: {
        type: 'object',
        properties: {
          address: { type: 'number', description: 'Start address (decimal or 0xHEX)' },
          size: { type: 'number', description: 'Number of bytes to read (default: 256, max: 4096)' },
        },
        required: ['address'],
      },
    });

    this.registerTool('memory/list_variables', this.listVariables.bind(this), {
      name: 'memory/list_variables',
      description: 'Get all variables from PDB with sizes and types (sorted by size, largest first)',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('memory/read_variable', this.readVariable.bind(this), {
      name: 'memory/read_variable',
      description: 'Read current value of specific variable from emulator',
      inputSchema: {
        type: 'object',
        properties: {
          name: { type: 'string', description: 'Variable name (e.g., "LEVEL_GP_COUNT", "player_x")' },
        },
        required: ['name'],
      },
    });

    // Project tools
    this.registerTool('project/get_structure', this.getProjectStructure.bind(this), {
      name: 'project/get_structure',
      description: 'Get complete project structure',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('project/read_file', this.readProjectFile.bind(this), {
      name: 'project/read_file',
      description: 'Read any file from the project',
      inputSchema: {
        type: 'object',
        properties: {
          path: { type: 'string', description: 'File path relative to project root' },
        },
        required: ['path'],
      },
    });

    this.registerTool('project/write_file', this.writeProjectFile.bind(this), {
      name: 'project/write_file',
      description: 'Write or update any file in the project (VPy code, vectors JSON, music JSON, config files, etc.). Automatically opens the file in the editor after writing.',
      inputSchema: {
        type: 'object',
        properties: {
          path: { type: 'string', description: 'File path relative to project root (e.g., "src/main.vpy", "assets/vectors/ship.vec", "assets/music/theme.vmus")' },
          content: { type: 'string', description: 'Complete file content (for JSON files like .vec and .vmus, provide valid JSON string)' },
        },
        required: ['path', 'content'],
      },
    });

    this.registerTool('project/close', this.closeProject.bind(this), {
      name: 'project/close',
      description: 'Close the currently open project',
      inputSchema: {
        type: 'object',
        properties: {},
      },
    });

    this.registerTool('project/open', this.openProject.bind(this), {
      name: 'project/open',
      description: 'Open a project by path',
      inputSchema: {
        type: 'object',
        properties: {
          path: { type: 'string', description: 'Full path to .vpyproj file' },
        },
        required: ['path'],
      },
    });

    this.registerTool('project/create', this.createProject.bind(this), {
      name: 'project/create',
      description: 'Create a new VPy project. If path is not provided, a folder selection dialog will be shown.',
      inputSchema: {
        type: 'object',
        properties: {
          name: { type: 'string', description: 'Project name' },
          path: { type: 'string', description: 'Directory path for new project (optional, will prompt if not provided)' },
        },
        required: ['name'],
      },
    });

    this.registerTool('project/create_vector', this.createVector.bind(this), {
      name: 'project/create_vector',
      description: 'Create .vec vector graphics file (JSON format ONLY). Structure: {"version":"1.0","name":"shape","canvas":{"width":256,"height":256,"origin":"center"},"layers":[{"name":"default","visible":true,"paths":[{"name":"line1","intensity":127,"closed":false,"points":[{"x":0,"y":0},{"x":10,"y":10}]}]}]}. Each path has points array with x,y coordinates. Triangle example: points:[{"x":0,"y":20},{"x":-15,"y":-10},{"x":15,"y":-10}], closed:true. NO text format - JSON only.',
      inputSchema: {
        type: 'object',
        properties: {
          name: { type: 'string', description: 'Vector file name (without .vec extension)' },
          content: { type: 'string', description: 'Valid JSON string matching exact format: {"version":"1.0","name":"...","canvas":{...},"layers":[{"paths":[{"points":[...]}]}]}. Leave empty for template.' },
        },
        required: ['name'],
      },
    });

    this.registerTool('project/create_music', this.createMusic.bind(this), {
      name: 'project/create_music',
      description: 'Create .vmus music file (JSON format ONLY). Structure: {"version":"1.0","tempo":120,"notes":[{"pitch":440,"duration":500,"start":0}],"noise":[{"frequency":1000,"duration":100,"start":1000}]}. Each note: pitch (Hz), duration (ms), start (ms). Each noise: frequency, duration, start. Example melody: [{"pitch":523,"duration":250,"start":0},{"pitch":587,"duration":250,"start":250}]. NO text format - JSON only.',
      inputSchema: {
        type: 'object',
        properties: {
          name: { type: 'string', description: 'Music file name (without .vmus extension)' },
          content: { type: 'string', description: 'Valid JSON string matching exact format: {"version":"1.0","tempo":120,"notes":[...],"noise":[...]}. Leave empty for template.' },
        },
        required: ['name'],
      },
    });

    this.registerTool('project/create_sfx', this.createSfx.bind(this), {
      name: 'project/create_sfx',
      description: 'Create .vsfx sound effect file (SFXR-style parametric format). Structure: {"version":"1.0","name":"laser","oscillator":{...},"envelope":{...},"pitchEnvelope":{...},"noise":{...}}. Presets available: laser, explosion, powerup, hit, jump, blip, coin. Leave content empty for default laser preset.',
      inputSchema: {
        type: 'object',
        properties: {
          name: { type: 'string', description: 'SFX file name (without .vsfx extension)' },
          content: { type: 'string', description: 'Valid JSON string for parametric SFX. Leave empty for laser preset template.' },
        },
        required: ['name'],
      },
    });
  }

  private registerTool(name: string, handler: MCPToolHandler, schema: MCPTool) {
    this.tools.set(name, handler);
    this.toolSchemas.set(name, schema);
  }

  // Handle incoming JSON-RPC request
  async handleRequest(request: MCPRequest): Promise<MCPResponse> {
    console.log('[MCP handleRequest] ▶ START:', request.method, 'id:', request.id);
    try {
      // Handle special methods
      if (request.method === 'tools/list') {
        console.log('[MCP handleRequest] ✓ Listing tools...');
        const result = this.createResponse(request.id, await this.listTools());
        console.log('[MCP handleRequest] ✓ END (tools/list)');
        return result;
      }

      // Normalize method name to handle different formats
      // editor/replace/range -> editor/replace_range
      // editor_replace_range -> editor/replace_range  
      // project_create_music -> project/create_music
      
      // Try original method first
      let handler = this.tools.get(request.method);
      let triedMethods = [request.method];
      
      // If not found, try normalization strategies
      if (!handler) {
        // Strategy 1: If has underscores, replace FIRST underscore with slash
        // project_create_music -> project/create_music
        if (request.method.includes('_')) {
          const withSlash = request.method.replace('_', '/');
          triedMethods.push(withSlash);
          handler = this.tools.get(withSlash);
        }
        
        // Strategy 2: If has multiple slashes, convert all but first to underscores
        // editor/replace/range -> editor/replace_range
        if (!handler && request.method.split('/').length > 2) {
          const parts = request.method.split('/');
          const normalized = parts[0] + '/' + parts.slice(1).join('_');
          triedMethods.push(normalized);
          handler = this.tools.get(normalized);
        }
      }
      
      if (!handler) {
        console.log('[MCP handleRequest] ✗ Method not found:', request.method);
        return this.createError(request.id, ErrorCodes.MethodNotFound, `Method not found: ${request.method} (tried: ${triedMethods.join(', ')})`);
      }

      console.log('[MCP handleRequest] ▶ Calling handler for:', request.method);
      const result = await handler(request.params || {});
      console.log('[MCP handleRequest] ✓ Handler returned:', typeof result, result);
      const response = this.createResponse(request.id, result);
      console.log('[MCP handleRequest] ✓ END (success)');
      return response;
    } catch (error: any) {
      console.error('[MCP handleRequest] ✗ Error handling request:', error);
      return this.createError(request.id, ErrorCodes.InternalError, error.message, error);
    }
  }

  private createResponse(id: string | number, result: any): MCPResponse {
    return {
      jsonrpc: '2.0',
      id,
      result,
    };
  }

  private createError(id: string | number, code: number, message: string, data?: any): MCPResponse {
    return {
      jsonrpc: '2.0',
      id,
      error: { code, message, data },
    };
  }

  // Helper to normalize URI from various formats
  private async normalizeUri(uri: string | undefined): Promise<string> {
    if (!this.mainWindow || !uri) {
      throw new Error(`Invalid URI: ${uri}`);
    }

    // Already a full URI
    if (uri.startsWith('file://') || uri.startsWith('untitled:')) {
      return uri;
    }

    // Try to resolve relative path or filename
    const path = await import('path');
    
    // Get project root
    const projectRoot = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__projectStore__;
        if (store) {
          const state = store.getState();
          return state.project?.rootPath || state.vpyProject?.rootDir;
        }
        return null;
      })()
    `);

    if (projectRoot) {
      // Just a filename - search in open documents
      if (!uri.includes('/') && !uri.includes('\\')) {
        const foundUri = await this.mainWindow.webContents.executeJavaScript(`
          (function() {
            const store = window.__editorStore__;
            if (!store) return null;
            const state = store.getState();
            const doc = state.documents.find(d => d.uri.endsWith('/${uri}'));
            return doc ? doc.uri : null;
          })()
        `);
        
        if (foundUri) {
          return foundUri;
        }
        
        // Default to src/ directory
        const fullPath = path.join(projectRoot, 'src', uri);
        return `file://${fullPath}`;
      }
      
      // Relative path
      const fullPath = path.join(projectRoot, uri);
      return `file://${fullPath}`;
    }

    return uri;
  }

  // Tool implementations
  private async listTools(): Promise<{ tools: MCPTool[] }> {
    console.log('[MCP Server] listTools() called - registered tools:', this.toolSchemas.size);
    const tools = Array.from(this.toolSchemas.values());
    console.log('[MCP Server] Returning', tools.length, 'tools:', tools.map(t => t.name));
    return { tools };
  }

  private async listDocuments(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    // Request editor state from renderer
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) return { documents: [] };
        const state = store.getState();
        return {
          documents: state.documents.map(doc => ({
            uri: doc.uri,
            language: doc.language,
            dirty: doc.dirty,
          })),
        };
      })()
    `);

    return result;
  }

  private async readDocument(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { uri } = params;
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const store = window.__editorStore__;
          if (!store) throw new Error('Editor store not available');
          const state = store.getState();
          const targetUri = ${JSON.stringify(uri)};
          const doc = state.documents.find(d => d.uri === targetUri || d.uri.endsWith('/' + targetUri));
          if (!doc) {
            const openDocs = state.documents.map(d => d.uri);
            const errorMsg = 'Document not found: ' + targetUri + 
                           '. Document must be OPEN in editor first. ' +
                           'Open documents: ' + JSON.stringify(openDocs) + 
                           '. Use editor/write_document to CREATE new files.';
            throw new Error(errorMsg);
          }
          return {
            uri: doc.uri,
            content: doc.content,
            language: doc.language,
            dirty: doc.dirty,
          };
        } catch (e) {
          // Ensure error is properly serialized
          return { __error: true, message: e.message || String(e), stack: e.stack };
        }
      })()
    `);
    
    // Check if result contains an error
    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async writeDocument(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    // Accept both 'uri' and 'path' for compatibility
    let uri = params.uri || params.path;
    const { content } = params;
    
    if (!uri) {
      throw new Error('Missing uri or path parameter');
    }

    // Normalize URI
    uri = await this.normalizeUri(uri);

    // Write to filesystem if it's a file:// URI
    if (uri.startsWith('file://')) {
      const fs = await import('fs/promises');
      const path = await import('path');
      const filePath = uri.replace('file://', '');
      
      try {
        // Ensure directory exists
        const dir = path.dirname(filePath);
        await fs.mkdir(dir, { recursive: true });
        
        // Write file
        await fs.writeFile(filePath, content, 'utf-8');
      } catch (error: any) {
        console.error('[MCP] Failed to write file:', error);
        throw new Error(`Failed to write file: ${error.message}`);
      }
    }

    // Update editor content (or open document if not open)
    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const targetUri = ${JSON.stringify(uri)};
        const doc = state.documents.find(d => d.uri === targetUri);
        
        if (doc) {
          // Document is open, update content
          store.getState().updateContent(targetUri, ${JSON.stringify(content)});
        } else {
          // Document not open, open it
          const language = targetUri.endsWith('.vpy') ? 'vpy' : (targetUri.endsWith('.json') || targetUri.endsWith('.vec') || targetUri.endsWith('.vmus')) ? 'json' : 'plaintext';
          store.getState().openDocument({
            uri: targetUri,
            language: language,
            content: ${JSON.stringify(content)},
            dirty: false,
            diagnostics: [],
            lastSavedContent: ${JSON.stringify(content)}
          });
        }
        return { success: true };
      })()
    `);

    return { success: true, uri };
  }

  private async saveDocument(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { uri } = params;

    // Get document from renderer
    const docData = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const store = window.__editorStore__;
          if (!store) throw new Error('Editor store not available');
          const state = store.getState();
          const targetUri = ${JSON.stringify(uri)};
          const doc = state.documents.find(d => d.uri === targetUri || d.uri.endsWith('/' + targetUri));
          if (!doc) {
            throw new Error('Document not found: ' + targetUri);
          }
          return {
            uri: doc.uri,
            content: doc.content,
            diskPath: doc.diskPath
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (docData && (docData as any).__error) {
      throw new Error((docData as any).message);
    }

    const diskPath = (docData as any).diskPath || (docData as any).uri;
    
    // Remove file:// prefix if present
    const fsPath = diskPath.startsWith('file://') ? diskPath.substring(7) : diskPath;

    try {
      const fs = await import('fs/promises');
      const path = await import('path');
      
      // Auto-create parent directory if needed
      const parentDir = path.dirname(fsPath);
      await fs.mkdir(parentDir, { recursive: true }).catch(() => {});
      
      // Write to disk
      await fs.writeFile(fsPath, (docData as any).content, 'utf-8');
      
      // Get new mtime
      const stat = await fs.stat(fsPath);
      
      // Mark as saved in editor store
      await this.mainWindow.webContents.executeJavaScript(`
        (function() {
          const store = window.__editorStore__;
          if (store) {
            const state = store.getState();
            state.markSaved(${JSON.stringify((docData as any).uri)}, ${stat.mtimeMs});
          }
          return { success: true };
        })()
      `);

      return {
        success: true,
        uri: (docData as any).uri,
        path: fsPath,
        mtime: stat.mtimeMs,
        size: stat.size
      };
    } catch (error: any) {
      return {
        success: false,
        message: `Failed to save: ${error.message}`
      };
    }
  }

  private async replaceRange(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    console.log('[MCP] replaceRange params:', JSON.stringify(params, null, 2));
    
    // Accept multiple naming conventions
    let uri = params.uri || params.path;
    let startLine = params.startLine || params.start_line;
    let startColumn = params.startColumn || params.start_column || 1; // Default to start of line
    let endLine = params.endLine || params.end_line;
    let endColumn = params.endColumn || params.end_column;
    let newText = params.newText || params.new_text || '';
    
    if (!uri) {
      throw new Error(`replaceRange: Missing uri/path parameter. Received params: ${JSON.stringify(params)}`);
    }
    
    if (!startLine || !endLine) {
      throw new Error(`replaceRange: Missing line parameters (startLine/endLine required, NOT character offsets). Use editor/write_document to replace ENTIRE file content instead. Received params: ${JSON.stringify(params)}`);
    }
    
    // If endColumn not specified, replace entire line(s)
    if (!endColumn) {
      const result = await this.mainWindow.webContents.executeJavaScript(`
        (function() {
          const store = window.__editorStore__;
          if (!store) throw new Error('Editor store not available');
          const state = store.getState();
          const targetUri = ${JSON.stringify(uri)};
          const doc = state.documents.find(d => d.uri.endsWith('/' + targetUri) || d.uri === targetUri);
          if (!doc) throw new Error('Document not found: ' + targetUri);
          const lines = doc.content.split('\\n');
          return lines[${endLine - 1}]?.length || 0;
        })()
      `);
      endColumn = result + 1;
    }
    
    // Normalize URI (same logic as writeDocument)
    uri = await this.normalizeUri(uri);
    
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const targetUri = ${JSON.stringify(uri)};
        const doc = state.documents.find(d => d.uri === targetUri);
        if (!doc) throw new Error('Document not found: ' + targetUri);
        
        const lines = doc.content.split('\\n');
        const startIdx = ${startLine - 1};
        const endIdx = ${endLine - 1};
        
        // Validate indices
        if (startIdx < 0 || startIdx >= lines.length) {
          throw new Error('Start line out of range: ${startLine}');
        }
        if (endIdx < 0 || endIdx >= lines.length) {
          throw new Error('End line out of range: ${endLine}');
        }
        
        if (startIdx === endIdx) {
          // Same line replacement
          const line = lines[startIdx];
          const colStart = Math.min(${startColumn - 1}, line.length);
          const colEnd = Math.min(${endColumn - 1}, line.length);
          lines[startIdx] = line.substring(0, colStart) + 
                           ${JSON.stringify(newText)} + 
                           line.substring(colEnd);
        } else {
          // Multi-line replacement
          const firstLineStart = lines[startIdx].substring(0, Math.min(${startColumn - 1}, lines[startIdx].length));
          const lastLineEnd = lines[endIdx].substring(Math.min(${endColumn - 1}, lines[endIdx].length));
          const newLine = firstLineStart + ${JSON.stringify(newText)} + lastLineEnd;
          lines.splice(startIdx, endIdx - startIdx + 1, newLine);
        }
        
        const newContent = lines.join('\\n');
        store.getState().updateContent(targetUri, newContent);
        return { 
          success: true, 
          linesChanged: endIdx - startIdx + 1,
          newLineCount: newContent.split('\\n').length
        };
      })()
    `);

    return result;
  }

  private async insertAt(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    let { uri, line, column, text } = params;
    
    // Normalize URI
    uri = await this.normalizeUri(uri);
    
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const targetUri = ${JSON.stringify(uri)};
        const doc = state.documents.find(d => d.uri === targetUri);
        if (!doc) throw new Error('Document not found: ' + targetUri);
        
        const lines = doc.content.split('\\n');
        const lineIdx = ${line - 1};
        const colIdx = ${column - 1};
        
        if (lineIdx < 0 || lineIdx >= lines.length) {
          throw new Error('Line out of range: ' + ${line});
        }
        
        const currentLine = lines[lineIdx];
        const insertText = ${JSON.stringify(text)};
        lines[lineIdx] = currentLine.substring(0, colIdx) + 
                        insertText + 
                        currentLine.substring(colIdx);
        
        const newContent = lines.join('\\n');
        store.getState().updateContent(targetUri, newContent);
        return { success: true, insertedLength: insertText.length };
      })()
    `);

    return result;
  }

  private async deleteRange(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    let { uri, startLine, startColumn, endLine, endColumn } = params;
    
    // Normalize URI
    uri = await this.normalizeUri(uri);
    
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const targetUri = ${JSON.stringify(uri)};
        const doc = state.documents.find(d => d.uri === targetUri);
        if (!doc) throw new Error('Document not found: ' + targetUri);
        
        const lines = doc.content.split('\\n');
        const startIdx = ${startLine - 1};
        const endIdx = ${endLine - 1};
        
        if (startIdx === endIdx) {
          // Same line deletion
          const line = lines[startIdx];
          lines[startIdx] = line.substring(0, ${startColumn - 1}) + line.substring(${endColumn - 1});
        } else {
          // Multi-line deletion
          const firstPart = lines[startIdx].substring(0, ${startColumn - 1});
          const lastPart = lines[endIdx].substring(${endColumn - 1});
          lines.splice(startIdx, endIdx - startIdx + 1, firstPart + lastPart);
        }
        
        const newContent = lines.join('\\n');
        store.getState().updateContent(targetUri, newContent);
        return { success: true, linesDeleted: endIdx - startIdx + 1 };
      })()
    `);

    return result;
  }

  private async getDiagnostics(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) return { diagnostics: [] };
        const state = store.getState();
        const uri = ${params.uri ? `'${params.uri}'` : 'null'};
        const allDiags = state.allDiagnostics || [];
        const filtered = uri ? allDiags.filter(d => d.uri === uri) : allDiags;
        return { diagnostics: filtered };
      })()
    `);

    return result;
  }

  private async compilerBuild(params: any): Promise<any> {
    if (!this.mainWindow) {
      return { 
        success: false, 
        errors: [{ line: 1, column: 1, message: 'IDE window not available' }] 
      };
    }

    try {
      // Check if there are active breakpoints - if so, enable debug mode
      const hasBreakpoints = await this.mainWindow.webContents.executeJavaScript(`
        (function() {
          const store = window.__editorStore__;
          if (!store) return false;
          const state = store.getState();
          const breakpoints = state.breakpoints || {};
          // Check if ANY file has breakpoints
          for (const uri in breakpoints) {
            if (breakpoints[uri] && breakpoints[uri].length > 0) {
              return true;
            }
          }
          return false;
        })()
      `);

      // If there are breakpoints, set loadingForDebug flag
      if (hasBreakpoints) {
        console.log('[MCP Server] Breakpoints detected - enabling debug mode');
        await this.mainWindow.webContents.executeJavaScript(`
          (function() {
            const debugStore = window.__debugStore__;
            if (debugStore) {
              debugStore.getState().setLoadingForDebug(true);
              console.log('[MCP Server] ✓ loadingForDebug set to true');
            }
          })()
        `);
      }

      // Get current project and call executeCompilation directly
      const { getCurrentProject, executeCompilation } = await import('../main.js');
      const project = getCurrentProject();
      
      if (!project?.entryFile) {
        return {
          success: false,
          errors: [{ 
            line: 1, 
            column: 1, 
            message: 'No project loaded. Open a project first (File → Open Project).' 
          }]
        };
      }

      // Call compilation directly and wait for result
      // When project is open, pass the .vpyproj file path for proper project compilation
      const projectName = require('path').basename(project.rootDir);
      const vpyprojPath = require('path').join(project.rootDir, `${projectName}.vpyproj`);
      
      const result: any = await executeCompilation({
        path: vpyprojPath,
        autoStart: false
      });

      // Check if compilation was successful
      if (result.error) {
        // Parse error details
        const errors = [];
        
        if (result.error === 'compile_failed' && result.stderr) {
          // Extract error messages from stderr
          const lines = result.stderr.split('\n');
          for (const line of lines) {
            // Match error patterns like "error 24:5 - SemanticsError: ..."
            const match = line.match(/error (\d+):(\d+) - (.+)/);
            if (match) {
              errors.push({
                line: parseInt(match[1]),
                column: parseInt(match[2]),
                message: match[3]
              });
            }
          }
        }
        
        // If no specific errors found, return generic error
        if (errors.length === 0) {
          errors.push({
            line: 1,
            column: 1,
            message: result.detail || result.error || 'Compilation failed'
          });
        }

        return {
          success: false,
          errors
        };
      }

      // Success
      return {
        success: true,
        binPath: result.binPath,
        asmPath: result.asmPath,
        pdbPath: result.pdbPath
      };
    } catch (error: any) {
      return {
        success: false,
        errors: [{
          line: 1,
          column: 1,
          message: `Internal error: ${error.message}`
        }]
      };
    }
  }

  private async getCompilerErrors(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const store = window.__editorStore__;
          if (!store) return { errors: [] };
          const state = store.getState();
          const diagnostics = state.allDiagnostics || [];
          
          // Filter only errors (not warnings or info)
          const errors = diagnostics
            .filter(d => d.severity === 'error')
            .map(d => ({
              file: d.file,
              uri: d.uri,
              line: d.line,
              column: d.column,
              message: d.message,
              source: d.source
            }));
          
          return { errors, totalCount: errors.length };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async buildAndRun(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    // First, build the project
    const buildResult = await this.compilerBuild({});
    
    if (!buildResult.success) {
      return {
        success: false,
        phase: 'compilation',
        errors: buildResult.errors || [],
        message: 'Compilation failed. Fix errors and try again.',
      };
    }

    // If build succeeded, get the ROM path from build result
    const romPath = buildResult.binPath;
    
    if (!romPath) {
      return {
        success: false,
        phase: 'compilation',
        message: 'Compilation succeeded but no ROM path returned',
      };
    }

    // Now run in emulator
    const runResult = await this.emulatorRun({ romPath, breakOnEntry: params.breakOnEntry || false });
    
    return {
      success: runResult.success,
      phase: 'execution',
      romPath,
      state: runResult.state,
      message: runResult.success ? 'Program running in emulator' : 'Failed to start emulator',
    };
  }

  private async emulatorRun(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { romPath, breakOnEntry } = params;
    
    if (!romPath) {
      return {
        success: false,
        message: 'romPath parameter is required'
      };
    }

    try {
      // Read the binary file
      const fs = await import('fs/promises');
      const path = await import('path');
      
      const binaryData = await fs.readFile(romPath);
      const base64 = binaryData.toString('base64');
      
      // Try to read associated .pdb file
      const pdbPath = romPath.replace(/\.bin$/, '.pdb');
      let pdbData = null;
      
      try {
        const pdbContent = await fs.readFile(pdbPath, 'utf-8');
        pdbData = JSON.parse(pdbContent);
      } catch {
        // .pdb not found or invalid - continue without debug symbols
      }

      // Send to renderer via window.electronAPI.onCompiledBin
      this.mainWindow.webContents.send('compiled-binary', {
        base64,
        size: binaryData.length,
        binPath: romPath,
        pdbData
      });

      return {
        success: true,
        state: 'running',
        romPath,
        size: binaryData.length,
        hasDebugSymbols: !!pdbData,
        message: `ROM loaded successfully: ${path.basename(romPath)}`
      };
    } catch (error: any) {
      return {
        success: false,
        message: `Failed to load ROM: ${error.message}`
      };
    }
  }

  private async getEmulatorState(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__debugStore__;
        if (!store) return { state: 'stopped', pc: 0, cycles: 0, registers: {} };
        const state = store.getState();
        return {
          state: state.state,
          pc: state.pc,
          cycles: state.cycles,
          fps: state.currentFps,
          registers: state.registers,
        };
      })()
    `);

    return result;
  }

  private async emulatorStop(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const debugStore = window.__debugStore__;
          if (!debugStore) throw new Error('Debug store not available');
          
          const state = debugStore.getState();
          if (state.stopEmulation) {
            state.stopEmulation();
            return { 
              success: true, 
              state: 'stopped',
              message: 'Emulator stopped successfully'
            };
          }
          
          return { 
            success: false,
            message: 'stopEmulation function not available'
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async memoryDump(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const start = params.start || 0xC800; // Default: RAM start
    const end = params.end || 0xCFFF;     // Default: RAM end
    const format = params.format || 'hex';

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const vecx = window.vecx;
          if (!vecx) throw new Error('Emulator not available');
          
          const start = ${start};
          const end = ${end};
          const format = '${format}';
          
          // Read memory using direct RAM access (fast)
          const bytes = [];
          if (vecx.ram && Array.isArray(vecx.ram)) {
            // RAM is at 0xC800-0xCBFF (1024 bytes)
            for (let addr = start; addr <= end && addr < 0xD000; addr++) {
              const ramOffset = addr - 0xC800;
              if (ramOffset >= 0 && ramOffset < vecx.ram.length) {
                bytes.push({ addr, value: vecx.ram[ramOffset] & 0xFF });
              } else {
                bytes.push({ addr, value: 0 });
              }
            }
          }
          
          // Format output
          let output = '';
          for (let i = 0; i < bytes.length; i += 16) {
            const row = bytes.slice(i, i + 16);
            const addrStr = row[0].addr.toString(16).toUpperCase().padStart(4, '0');
            const hexBytes = row.map(b => b.value.toString(16).toUpperCase().padStart(2, '0')).join(' ');
            const asciiBytes = row.map(b => {
              const c = b.value;
              return (c >= 32 && c <= 126) ? String.fromCharCode(c) : '.';
            }).join('');
            
            output += addrStr + ': ' + hexBytes.padEnd(48, ' ') + ' | ' + asciiBytes + '\\n';
          }
          
          return {
            success: true,
            start,
            end,
            bytes: bytes.length,
            format,
            dump: output
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async listVariables(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const debugStore = window.__debugStore__;
          if (!debugStore) throw new Error('Debug store not available');
          
          const state = debugStore.getState();
          const pdbData = state.pdbData;
          
          if (!pdbData || !pdbData.variables) {
            return {
              success: false,
              message: 'No PDB data available. Compile a program first.',
              variables: {}
            };
          }
          
          // Calculate memory usage
          const variables = pdbData.variables;
          const usedBytes = Object.values(variables).reduce((sum, v) => sum + (v.size || 0), 0);
          const totalBytes = 1024; // JSVecx RAM size
          const freeBytes = totalBytes - usedBytes;
          
          // Sort by size (largest first) for optimization recommendations
          const sortedVars = Object.entries(variables)
            .map(([name, info]) => ({ name, ...info }))
            .sort((a, b) => (b.size || 0) - (a.size || 0));
          
          return {
            success: true,
            count: Object.keys(variables).length,
            usedBytes,
            freeBytes,
            totalBytes,
            usagePercent: ((usedBytes / totalBytes) * 100).toFixed(1),
            variables: sortedVars
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async readVariable(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { name } = params;
    
    if (!name) {
      return {
        success: false,
        message: 'Variable name is required'
      };
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const debugStore = window.__debugStore__;
          const vecx = window.vecx;
          
          if (!debugStore) throw new Error('Debug store not available');
          if (!vecx) throw new Error('Emulator not available');
          
          const state = debugStore.getState();
          const pdbData = state.pdbData;
          
          if (!pdbData || !pdbData.variables) {
            throw new Error('No PDB data available. Compile a program first.');
          }
          
          const varName = '${name}';
          const varInfo = pdbData.variables[varName];
          
          if (!varInfo) {
            return {
              success: false,
              message: 'Variable "' + varName + '" not found in PDB',
              availableVariables: Object.keys(pdbData.variables).slice(0, 10)
            };
          }
          
          // Parse address
          const addr = parseInt(varInfo.address, 16);
          const size = varInfo.size || 2;
          
          // Read value from RAM
          let value;
          if (vecx.ram && Array.isArray(vecx.ram) && addr >= 0xC800 && addr < 0xD000) {
            const ramOffset = addr - 0xC800;
            
            if (size === 1) {
              // 8-bit value
              value = vecx.ram[ramOffset] & 0xFF;
            } else if (size === 2) {
              // 16-bit value (big-endian)
              const high = vecx.ram[ramOffset] & 0xFF;
              const low = vecx.ram[ramOffset + 1] & 0xFF;
              value = (high << 8) | low;
            } else {
              // Array: read first few elements
              const elements = [];
              const numElements = Math.min(size / 2, 8); // Max 8 elements to display
              for (let i = 0; i < numElements; i++) {
                const elemHigh = vecx.ram[ramOffset + i * 2] & 0xFF;
                const elemLow = vecx.ram[ramOffset + i * 2 + 1] & 0xFF;
                elements.push((elemHigh << 8) | elemLow);
              }
              value = elements;
            }
          } else {
            value = null;
          }
          
          return {
            success: true,
            name: varName,
            address: varInfo.address,
            size: varInfo.size,
            type: varInfo.type,
            value: value,
            valueHex: Array.isArray(value) 
              ? value.map(v => '0x' + v.toString(16).toUpperCase().padStart(4, '0')).join(', ')
              : '0x' + (value || 0).toString(16).toUpperCase().padStart(size === 1 ? 2 : 4, '0'),
            valueDec: value
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async memoryWrite(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { address, value, size = 1 } = params;

    // Validate parameters
    if (typeof address !== 'number') {
      throw new Error('Address must be a number (hex or decimal)');
    }
    if (typeof value !== 'number') {
      throw new Error('Value must be a number');
    }
    if (size !== 1 && size !== 2) {
      throw new Error('Size must be 1 (8-bit) or 2 (16-bit)');
    }

    // Validate value range
    if (size === 1 && (value < 0 || value > 255)) {
      throw new Error('Value must be 0-255 for 8-bit write');
    }
    if (size === 2 && (value < 0 || value > 65535)) {
      throw new Error('Value must be 0-65535 for 16-bit write');
    }

    // Write to memory
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const vecx = window.vecx;
        if (!vecx) {
          return { __error: true, message: 'Emulator not available' };
        }

        const address = ${address};
        const value = ${value};
        const size = ${size};

        try {
          // Validate RAM range (0xC800-0xCFFF is RAM)
          if (address < 0xC800 || address >= 0xD000) {
            throw new Error('Address 0x' + address.toString(16).toUpperCase() + ' outside RAM range (0xC800-0xCFFF)');
          }

          // Write to memory
          if (size === 1) {
            // 8-bit write
            vecx.write8(address, value);
          } else {
            // 16-bit write (big-endian)
            const high = (value >> 8) & 0xFF;
            const low = value & 0xFF;
            vecx.write8(address, high);
            vecx.write8(address + 1, low);
          }

          // Read back for confirmation
          let readBack;
          if (size === 1) {
            readBack = vecx.read8(address) & 0xFF;
          } else {
            const high = vecx.read8(address) & 0xFF;
            const low = vecx.read8(address + 1) & 0xFF;
            readBack = (high << 8) | low;
          }

          return {
            success: true,
            address: '0x' + address.toString(16).toUpperCase().padStart(4, '0'),
            value: readBack,
            valueHex: '0x' + readBack.toString(16).toUpperCase().padStart(size === 1 ? 2 : 4, '0'),
            valueDec: readBack,
            size: size
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async debugStart(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    // Send debug.start command via window.postMessage
    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        window.postMessage({ type: 'command', command: 'debug.start' }, '*');
        return { success: true };
      })()
    `);

    return { success: true, message: 'Debug session started' };
  }

  private async getRegisters(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const vecx = window.vecx;
          if (!vecx || !vecx.cpu) {
            return { __error: true, message: 'Emulator not running or CPU not available' };
          }

          const cpu = vecx.cpu;
          
          // Read all registers
          const a = cpu.a & 0xFF;
          const b = cpu.b & 0xFF;
          const d = (a << 8) | b;
          const x = cpu.x & 0xFFFF;
          const y = cpu.y & 0xFFFF;
          const u = cpu.u & 0xFFFF;
          const s = cpu.s & 0xFFFF;
          const pc = cpu.pc & 0xFFFF;
          const dp = cpu.dp & 0xFF;
          const cc = cpu.cc & 0xFF;
          
          return {
            A: { value: a, hex: '0x' + a.toString(16).toUpperCase().padStart(2, '0'), decimal: a },
            B: { value: b, hex: '0x' + b.toString(16).toUpperCase().padStart(2, '0'), decimal: b },
            D: { value: d, hex: '0x' + d.toString(16).toUpperCase().padStart(4, '0'), decimal: d },
            X: { value: x, hex: '0x' + x.toString(16).toUpperCase().padStart(4, '0'), decimal: x },
            Y: { value: y, hex: '0x' + y.toString(16).toUpperCase().padStart(4, '0'), decimal: y },
            U: { value: u, hex: '0x' + u.toString(16).toUpperCase().padStart(4, '0'), decimal: u },
            S: { value: s, hex: '0x' + s.toString(16).toUpperCase().padStart(4, '0'), decimal: s },
            PC: { value: pc, hex: '0x' + pc.toString(16).toUpperCase().padStart(4, '0'), decimal: pc },
            DP: { value: dp, hex: '0x' + dp.toString(16).toUpperCase().padStart(2, '0'), decimal: dp },
            CC: { 
              value: cc, 
              hex: '0x' + cc.toString(16).toUpperCase().padStart(2, '0'), 
              decimal: cc,
              flags: {
                C: (cc & 0x01) ? 1 : 0, // Carry
                V: (cc & 0x02) ? 1 : 0, // Overflow
                Z: (cc & 0x04) ? 1 : 0, // Zero
                N: (cc & 0x08) ? 1 : 0, // Negative
                I: (cc & 0x10) ? 1 : 0, // IRQ mask
                H: (cc & 0x20) ? 1 : 0, // Half-carry
                F: (cc & 0x40) ? 1 : 0, // FIRQ mask
                E: (cc & 0x80) ? 1 : 0  // Entire flag
              }
            }
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async memoryDump(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { address, size = 256 } = params;
    const maxSize = 4096;
    const actualSize = Math.min(size, maxSize);

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const vecx = window.vecx;
          if (!vecx) {
            return { __error: true, message: 'Emulator not running' };
          }

          const startAddr = ${address};
          const numBytes = ${actualSize};
          const bytes = [];
          
          for (let i = 0; i < numBytes; i++) {
            const addr = (startAddr + i) & 0xFFFF;
            const byte = vecx.read8(addr) & 0xFF;
            bytes.push(byte);
          }

          // Format as hex dump (16 bytes per line)
          const lines = [];
          for (let i = 0; i < bytes.length; i += 16) {
            const addr = (startAddr + i) & 0xFFFF;
            const addrHex = '0x' + addr.toString(16).toUpperCase().padStart(4, '0');
            const chunk = bytes.slice(i, i + 16);
            const hexBytes = chunk.map(b => b.toString(16).toUpperCase().padStart(2, '0')).join(' ');
            const ascii = chunk.map(b => (b >= 32 && b <= 126) ? String.fromCharCode(b) : '.').join('');
            lines.push(addrHex + ': ' + hexBytes.padEnd(48, ' ') + ' | ' + ascii);
          }

          return {
            address: startAddr,
            size: numBytes,
            bytes: bytes,
            dump: lines.join('\\n')
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async listVariables(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const compilerState = window.__compilerState__;
          if (!compilerState || !compilerState.pdbData) {
            return { __error: true, message: 'No PDB data available (compile first)' };
          }

          const pdb = compilerState.pdbData;
          const variables = [];

          // Extract variables from PDB symbols
          if (pdb.symbols) {
            for (const [name, info] of Object.entries(pdb.symbols)) {
              const addr = typeof info === 'number' ? info : (info.address || 0);
              const size = (info.size || 2); // Default 2 bytes (16-bit)
              const type = info.type || 'unknown';
              
              variables.push({
                name: name,
                address: addr,
                addressHex: '0x' + addr.toString(16).toUpperCase().padStart(4, '0'),
                size: size,
                type: type
              });
            }
          }

          // Sort by size (largest first)
          variables.sort((a, b) => b.size - a.size);

          return {
            count: variables.length,
            variables: variables
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async readVariable(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { name } = params;

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        try {
          const vecx = window.vecx;
          const compilerState = window.__compilerState__;
          
          if (!vecx) {
            return { __error: true, message: 'Emulator not running' };
          }
          
          if (!compilerState || !compilerState.pdbData) {
            return { __error: true, message: 'No PDB data available (compile first)' };
          }

          const pdb = compilerState.pdbData;
          const varName = '${name}';
          
          // Find variable in PDB
          if (!pdb.symbols || !pdb.symbols[varName]) {
            return { __error: true, message: 'Variable "' + varName + '" not found in PDB' };
          }

          const info = pdb.symbols[varName];
          const addr = typeof info === 'number' ? info : (info.address || 0);
          const size = (info.size || 2); // Default 2 bytes
          
          // Read value from memory
          let value;
          if (size === 1) {
            value = vecx.read8(addr) & 0xFF;
          } else {
            // 16-bit big-endian
            const high = vecx.read8(addr) & 0xFF;
            const low = vecx.read8(addr + 1) & 0xFF;
            value = (high << 8) | low;
          }

          return {
            name: varName,
            address: addr,
            addressHex: '0x' + addr.toString(16).toUpperCase().padStart(4, '0'),
            size: size,
            value: value,
            valueHex: '0x' + value.toString(16).toUpperCase().padStart(size * 2, '0'),
            valueDec: value,
            valueBin: '0b' + value.toString(2).padStart(size * 8, '0')
          };
        } catch (e) {
          return { __error: true, message: e.message || String(e) };
        }
      })()
    `);

    if (result && (result as any).__error) {
      throw new Error((result as any).message);
    }

    return result;
  }

  private async addBreakpoint(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { uri, line } = params;
    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        store.getState().toggleBreakpoint('${uri}', ${line});
        return { success: true };
      })()
    `);

    return { success: true };
  }

  private async getCallstack(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__debugStore__;
        if (!store) return { frames: [] };
        const state = store.getState();
        return { frames: state.callStack || [] };
      })()
    `);

    return result;
  }

  private async removeBreakpoint(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { uri, line } = params;
    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const doc = state.documents.find(d => d.uri === '${uri}');
        if (doc && doc.breakpoints?.includes(${line})) {
          store.getState().toggleBreakpoint('${uri}', ${line});
        }
        return { success: true };
      })()
    `);

    return { success: true };
  }

  private async listBreakpoints(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) return { breakpoints: [] };
        const state = store.getState();
        const breakpoints = [];
        for (const doc of state.documents) {
          if (doc.breakpoints && doc.breakpoints.length > 0) {
            for (const line of doc.breakpoints) {
              breakpoints.push({ uri: doc.uri, line });
            }
          }
        }
        return { breakpoints };
      })()
    `);

    return result;
  }

  private async clearBreakpoints(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        // Remove all breakpoints from all documents
        for (const doc of state.documents) {
          if (doc.breakpoints && doc.breakpoints.length > 0) {
            const lines = [...doc.breakpoints];
            for (const line of lines) {
              store.getState().toggleBreakpoint(doc.uri, line);
            }
          }
        }
        return { success: true };
      })()
    `);

    return { success: true };
  }

  private async stepInto(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        window.postMessage({ type: 'debug-step-into' }, '*');
        return { success: true };
      })()
    `);

    return { success: true };
  }

  private async stepOver(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        window.postMessage({ type: 'debug-step-over' }, '*');
        return { success: true };
      })()
    `);

    return { success: true };
  }

  private async stepOut(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        window.postMessage({ type: 'debug-step-out' }, '*');
        return { success: true };
      })()
    `);

    return { success: true };
  }

  private async debugContinue(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__debugStore__;
        if (!store) throw new Error('Debug store not available');
        store.getState().setState('running');
        
        // Resume emulator
        const vecx = window.vecx;
        if (vecx && !vecx.running) {
          vecx.run();
        }
        return { success: true };
      })()
    `);

    return { success: true };
  }

  private async debugPause(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__debugStore__;
        if (!store) throw new Error('Debug store not available');
        store.getState().setState('paused');
        
        // Pause emulator
        const vecx = window.vecx;
        if (vecx && vecx.running) {
          vecx.stop();
        }
        return { success: true };
      })()
    `);

    return { success: true };
  }

  private async getProjectStructure(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__projectStore__;
        if (!store) return { root: '', name: '', files: [] };
        const state = store.getState();
        return {
          root: state.project || '',
          name: state.workspaceName || '',
          files: state.selected ? [{ path: state.selected, type: 'file' }] : [],
        };
      })()
    `);

    return result;
  }

  private async readProjectFile(params: any): Promise<any> {
    const fs = await import('fs/promises');
    const path = await import('path');
    
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    // Get project root from store
    const projectRoot = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__projectStore__;
        if (!store) throw new Error('Project store not available');
        const state = store.getState();
        return state.project?.rootPath || state.vpyProject?.rootDir;
      })()
    `);

    if (!projectRoot) {
      throw new Error('No project open');
    }

    const { path: relativePath } = params;
    const fullPath = path.join(projectRoot, relativePath);
    
    try {
      const content = await fs.readFile(fullPath, 'utf-8');
      return { path: relativePath, fullPath, content };
    } catch (error: any) {
      throw new Error(`Failed to read file ${relativePath}: ${error.message}`);
    }
  }

  private async writeProjectFile(params: any): Promise<any> {
    const fs = await import('fs/promises');
    const path = await import('path');
    
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    // Get project root from store
    const projectRoot = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__projectStore__;
        if (!store) throw new Error('Project store not available');
        const state = store.getState();
        return state.project?.rootPath || state.vpyProject?.rootDir;
      })()
    `);

    if (!projectRoot) {
      throw new Error('No project open');
    }

    const { path: relativePath, content } = params;
    const fullPath = path.join(projectRoot, relativePath);
    
    try {
      // Create directory if it doesn't exist
      const dir = path.dirname(fullPath);
      await fs.mkdir(dir, { recursive: true });
      
      // Write file
      await fs.writeFile(fullPath, content, 'utf-8');
      
      // If file is open in editor, refresh it
      await this.mainWindow.webContents.executeJavaScript(`
        (function() {
          const editorStore = window.__editorStore__;
          if (!editorStore) return;
          const state = editorStore.getState();
          const uri = 'file://${fullPath.replace(/\\/g, '\\\\')}';
          const doc = state.documents.find(d => d.uri === uri);
          if (doc) {
            editorStore.getState().updateContent(uri, ${JSON.stringify(content)});
          }
        })()
      `);
      
      return { 
        success: true, 
        path: relativePath, 
        fullPath,
        message: `File ${relativePath} written successfully`
      };
    } catch (error: any) {
      throw new Error(`Failed to write file ${relativePath}: ${error.message}`);
    }
  }

  private async closeProject(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const projectStore = window.__projectStore__;
        const editorStore = window.__editorStore__;
        
        if (!projectStore) throw new Error('Project store not available');
        if (!editorStore) throw new Error('Editor store not available');
        
        const projectState = projectStore.getState();
        const editorState = editorStore.getState();
        
        const currentProject = projectState.workspaceName || projectState.vpyProject?.config.project.name || 'Unknown';
        
        // Close all open documents first
        const allDocs = [...editorState.documents];
        for (const doc of allDocs) {
          editorState.closeDocument(doc.uri);
        }
        
        // Then close both workspace and VPy project
        projectState.closeVpyProject();
        projectState.clearWorkspace();
        
        return { success: true, closedProject: currentProject, closedFiles: allDocs.length };
      })()
    `);

    return result;
  }

  private async openProject(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { path } = params;
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__projectStore__;
        if (!store) throw new Error('Project store not available');
        store.getState().openProject('${path}');
        return { success: true, openedProject: '${path}' };
      })()
    `);

    return result;
  }

  private async createProject(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const fs = await import('fs/promises');
    const path = await import('path');
    const os = await import('os');
    const { dialog } = await import('electron');
    const { name } = params;
    let projectPath = params.path;

    // If no path provided, show folder selection dialog
    if (!projectPath) {
      const result = await dialog.showOpenDialog(this.mainWindow, {
        title: `Select location for project '${name}'`,
        properties: ['openDirectory', 'createDirectory'],
        buttonLabel: 'Select Folder',
        message: 'Choose where to create the new project'
      });

      if (result.canceled || !result.filePaths || result.filePaths.length === 0) {
        // User cancelled, use default location
        const defaultProjectsDir = path.join(os.homedir(), 'VPyProjects');
        await fs.mkdir(defaultProjectsDir, { recursive: true });
        projectPath = path.join(defaultProjectsDir, name);
      } else {
        projectPath = path.join(result.filePaths[0], name);
      }
    }
    
    try {
      // Create project directory structure
      await fs.mkdir(projectPath, { recursive: true });
      await fs.mkdir(path.join(projectPath, 'src'), { recursive: true });
      await fs.mkdir(path.join(projectPath, 'assets'), { recursive: true });
      await fs.mkdir(path.join(projectPath, 'assets', 'vectors'), { recursive: true });
      await fs.mkdir(path.join(projectPath, 'assets', 'music'), { recursive: true });
      await fs.mkdir(path.join(projectPath, 'build'), { recursive: true });

      // Create .vpyproj file
      const projectFile = path.join(projectPath, `${name}.vpyproj`);
      const projectConfig = `[project]
name = "${name}"
version = "0.1.0"
author = ""
description = ""
entry = "src/main.vpy"

[build]
output = "build/${name}.bin"
target = "vectrex"
optimization = 0
debug_symbols = true

[sources]
vpy = ["src/**/*.vpy"]

[resources]
vectors = ["assets/vectors/**/*.vec"]
music = ["assets/music/**/*.vmus"]
`;
      await fs.writeFile(projectFile, projectConfig, 'utf-8');

      // Create main.vpy
      const mainFile = path.join(projectPath, 'src', 'main.vpy');
      const mainContent = `# ${name} - VPy Project
# Created by PyPilot

def setup():
    # Initialization code here

def loop():
    # Main loop code here
`;
      await fs.writeFile(mainFile, mainContent, 'utf-8');

      // Create .gitignore
      const gitignore = path.join(projectPath, '.gitignore');
      const gitignoreContent = `build/
*.bin
*.lst
*.sym
.DS_Store
`;
      await fs.writeFile(gitignore, gitignoreContent, 'utf-8');

      // Open the created project
      await this.mainWindow.webContents.executeJavaScript(`
        (function() {
          const store = window.__projectStore__;
          if (!store) throw new Error('Project store not available');
          store.getState().openVpyProject('${projectFile.replace(/\\/g, '\\\\')}');
          return true;
        })()
      `);

      return { 
        success: true, 
        projectPath,
        projectFile,
        message: `Project '${name}' created successfully`,
        files: [
          projectFile,
          mainFile,
          gitignore
        ]
      };
    } catch (error: any) {
      throw new Error(`Failed to create project: ${error.message}`);
    }
  }

  private async createVector(params: any): Promise<any> {
    console.log('[MCP] createVector called with params:', JSON.stringify(params));
    const fs = await import('fs/promises');
    const path = await import('path');
    
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    // Get project root
    const projectRoot = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__projectStore__;
        if (!store) throw new Error('Project store not available');
        const state = store.getState();
        return state.project?.rootPath || state.vpyProject?.rootDir;
      })()
    `);

    if (!projectRoot) {
      throw new Error('No project open');
    }

    // Validate parameters
    if (!params || typeof params !== 'object') {
      throw new Error('Invalid parameters: expected object with "name" field.\n\nTool: project_create_vector\nRequired arguments:\n  - name (string): Vector file name without .vec extension\n  - content (string, optional): Valid JSON string\n\nExample call:\n{"name": "spaceship", "content": "{\\"version\\":\\"1.0\\",\\\"layers\\":[]}"}');
    }
    
    const { name, content } = params;
    
    if (!name || typeof name !== 'string') {
      throw new Error(`Invalid or missing "name" parameter. Must be a non-empty string.\n\nReceived params: ${JSON.stringify(params)}\n\nTool: project_create_vector\nRequired: name (string) - Vector file name\nOptional: content (string) - JSON data\n\nExample: {"name": "spaceship"}`);
    }
    
    const fileName = name.endsWith('.vec') ? name : `${name}.vec`;
    const vectorPath = path.join(projectRoot, 'assets', 'vectors', fileName);
    
    // Validate JSON format if content provided
    if (content) {
      try {
        const parsed = JSON.parse(content);
        if (!parsed.version || !parsed.layers || !Array.isArray(parsed.layers)) {
          throw new Error('Invalid vector JSON structure. Required fields: version, layers (array)');
        }
      } catch (error: any) {
        throw new Error(`Vector file MUST be valid JSON format. Error: ${error.message}\n\nExample format:\n{"version":"1.0","name":"shape","canvas":{"width":256,"height":256,"origin":"center"},"layers":[{"name":"default","visible":true,"paths":[{"name":"line","intensity":127,"closed":false,"points":[{"x":0,"y":0},{"x":10,"y":10}]}]}]}\n\nNO text format like VECTOR_START, MOVE, DRAW_TO - JSON ONLY.`);
      }
    }
    
    // Default vector template if no content provided
    const defaultContent = content || `{
  "version": "1.0",
  "name": "${name}",
  "canvas": {
    "width": 256,
    "height": 256,
    "origin": "center"
  },
  "layers": [
    {
      "name": "default",
      "visible": true,
      "paths": [
        {
          "name": "shape",
          "intensity": 127,
          "closed": false,
          "points": [
            { "x": 0, "y": 0 },
            { "x": 10, "y": 10 }
          ]
        }
      ]
    }
  ]
}
`;

    try {
      // Ensure vectors directory exists
      await fs.mkdir(path.dirname(vectorPath), { recursive: true });
      
      // Write vector file
      await fs.writeFile(vectorPath, defaultContent, 'utf-8');
      
      // Get file stats for metadata
      const stats = await fs.stat(vectorPath);
      
      // Open in editor with proper file metadata
      const fileUri = `file://${vectorPath}`;
      console.log('[MCP] createVector - Opening file with URI:', fileUri);
      console.log('[MCP] createVector - File stats:', { mtime: stats.mtimeMs, size: stats.size });
      
      await this.mainWindow.webContents.executeJavaScript(`
        (function() {
          const store = window.__editorStore__;
          if (store) {
            const uri = '${fileUri.replace(/\\/g, '\\\\')}';
            console.log('[Frontend] Opening vector file with URI:', uri);
            const doc = {
              uri: uri,
              language: 'json',
              content: ${JSON.stringify(defaultContent)},
              dirty: false,
              diagnostics: [],
              lastSavedContent: ${JSON.stringify(defaultContent)},
              mtime: ${stats.mtimeMs},
              size: ${stats.size}
            };
            console.log('[Frontend] Document to open:', doc);
            store.getState().openDocument(doc);
            console.log('[Frontend] Document opened, current documents:', store.getState().documents.map(d => ({ uri: d.uri, dirty: d.dirty })));
          }
        })()
      `);
      
      return {
        success: true,
        filePath: vectorPath,
        relativePath: `assets/vectors/${fileName}`,
        message: `Vector file '${fileName}' created and opened successfully`
      };
    } catch (error: any) {
      throw new Error(`Failed to create vector file: ${error.message}`);
    }
  }

  private async createMusic(params: any): Promise<any> {
    const fs = await import('fs/promises');
    const path = await import('path');
    
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    // Get project root
    const projectRoot = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__projectStore__;
        if (!store) throw new Error('Project store not available');
        const state = store.getState();
        return state.project?.rootPath || state.vpyProject?.rootDir;
      })()
    `);

    if (!projectRoot) {
      throw new Error('No project open');
    }

    // Validate parameters
    if (!params || typeof params !== 'object') {
      throw new Error('Invalid parameters: expected object with "name" field');
    }
    
    const { name, content } = params;
    
    if (!name || typeof name !== 'string') {
      throw new Error('Invalid or missing "name" parameter. Must be a non-empty string.');
    }
    
    const fileName = name.endsWith('.vmus') ? name : `${name}.vmus`;
    const musicPath = path.join(projectRoot, 'assets', 'music', fileName);
    
    // Validate JSON format if content provided
    if (content) {
      // Check size BEFORE parsing (relaxed limit with 8000 max_tokens)
      const contentSize = Buffer.byteLength(content, 'utf-8');
      if (contentSize > 16384) { // 16KB limit (was 4KB)
        const noteCount = (content.match(/"id":/g) || []).length;
        throw new Error(`Music file too large (${Math.round(contentSize/1024)}KB). Maximum 16KB allowed.\n\nFile has ~${noteCount} notes/events. Reduce to 80-100 notes maximum.\n\nSolution: Create shorter melody and use "loopStart"/"loopEnd" for repetition.\nExample: "loopStart": 0, "loopEnd": 384 will loop first 16 beats.`);
      }
      
      try {
        const parsed = JSON.parse(content);
        if (!parsed.version || !parsed.tempo || !Array.isArray(parsed.notes)) {
          throw new Error('Invalid music JSON structure. Required fields: version, tempo, notes (array)');
        }
        
        // Count total events
        const totalEvents = (parsed.notes?.length || 0) + (parsed.noise?.length || 0);
        if (totalEvents > 120) { // Increased from 50 to 120 with expanded token limit
          throw new Error(`Too many events (${totalEvents}). Maximum 120 total notes+noise recommended.\n\nReduce note count and use looping for longer songs.`);
        }
      } catch (error: any) {
        if (error.message.includes('Too many events') || error.message.includes('too large')) {
          throw error; // Re-throw our custom errors
        }
        throw new Error(`Music file MUST be valid JSON format. Error: ${error.message}\n\nExample format:\n{"version":"1.0","name":"Song","author":"","tempo":120,"ticksPerBeat":24,"totalTicks":384,"notes":[{"id":"n1","note":60,"start":0,"duration":48,"velocity":12,"channel":0}],"noise":[],"loopStart":0,"loopEnd":384}\n\nNOTE: Use "note" (MIDI 0-127), NOT "pitch" (Hz). Use "period" (0-31) for noise, NOT "frequency".`);
      }
    }
    
    // Default music template if no content provided
    const defaultContent = content || `{
  "version": "1.0",
  "name": "${name}",
  "author": "",
  "tempo": 120,
  "ticksPerBeat": 24,
  "totalTicks": 384,
  "notes": [],
  "noise": [],
  "loopStart": 0,
  "loopEnd": 384
}
`;

    try {
      // Ensure music directory exists
      await fs.mkdir(path.dirname(musicPath), { recursive: true });
      
      // Write music file
      await fs.writeFile(musicPath, defaultContent, 'utf-8');
      
      // Get file stats for metadata
      const stats = await fs.stat(musicPath);
      
      // Open in editor with proper file metadata
      const fileUri = `file://${musicPath}`;
      await this.mainWindow.webContents.executeJavaScript(`
        (function() {
          const store = window.__editorStore__;
          if (store) {
            const doc = {
              uri: '${fileUri.replace(/\\/g, '\\\\')}',
              language: 'json',
              content: ${JSON.stringify(defaultContent)},
              dirty: false,
              diagnostics: [],
              lastSavedContent: ${JSON.stringify(defaultContent)},
              mtime: ${stats.mtimeMs},
              size: ${stats.size}
            };
            store.getState().openDocument(doc);
          }
        })()
      `);
      
      return {
        success: true,
        filePath: musicPath,
        relativePath: `assets/music/${fileName}`,
        message: `Music file '${fileName}' created and opened successfully`
      };
    } catch (error: any) {
      throw new Error(`Failed to create music file: ${error.message}`);
    }
  }

  private async createSfx(params: any): Promise<any> {
    const fs = await import('fs/promises');
    const path = await import('path');
    
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    // Get project root
    const projectRoot = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__projectStore__;
        if (!store) throw new Error('Project store not available');
        const state = store.getState();
        return state.project?.rootPath || state.vpyProject?.rootDir;
      })()
    `);

    if (!projectRoot) {
      throw new Error('No project open');
    }

    // Validate parameters
    if (!params || typeof params !== 'object') {
      throw new Error('Invalid parameters: expected object with "name" field');
    }
    
    const { name, content } = params;
    
    if (!name || typeof name !== 'string') {
      throw new Error('Invalid or missing "name" parameter. Must be a non-empty string.');
    }
    
    const fileName = name.endsWith('.vsfx') ? name : `${name}.vsfx`;
    const sfxPath = path.join(projectRoot, 'assets', 'sfx', fileName);
    
    // Validate JSON format if content provided
    if (content) {
      try {
        const parsed = JSON.parse(content);
        if (!parsed.version || !parsed.oscillator || !parsed.envelope) {
          throw new Error('Invalid SFX JSON structure. Required fields: version, oscillator, envelope');
        }
      } catch (error: any) {
        if (error.message.includes('Invalid SFX')) {
          throw error;
        }
        throw new Error(`SFX file MUST be valid JSON format. Error: ${error.message}\n\nExample format:\n{"version":"1.0","name":"laser","oscillator":{"type":"saw","frequency":1200},"envelope":{"attack":5,"decay":50,"sustain":0,"release":100}}`);
      }
    }
    
    // Default SFX template (laser preset) if no content provided
    const defaultContent = content || JSON.stringify({
      version: "1.0",
      name: name,
      oscillator: {
        type: "saw",
        frequency: 1200,
        duty: 50
      },
      envelope: {
        attack: 5,
        decay: 50,
        sustain: 0,
        release: 100
      },
      pitchEnvelope: {
        attack: 0,
        sustain: 100,
        decay: 80,
        amount: -800
      },
      noise: {
        enabled: false,
        mix: 0,
        type: "white"
      },
      modulation: {
        type: "none",
        frequency: 0,
        amount: 0
      }
    }, null, 2);

    try {
      // Ensure sfx directory exists
      await fs.mkdir(path.dirname(sfxPath), { recursive: true });
      
      // Write sfx file
      await fs.writeFile(sfxPath, defaultContent, 'utf-8');
      
      // Get file stats for metadata
      const stats = await fs.stat(sfxPath);
      
      // Open in editor with proper file metadata
      const fileUri = `file://${sfxPath}`;
      await this.mainWindow.webContents.executeJavaScript(`
        (function() {
          const store = window.__editorStore__;
          if (store) {
            const doc = {
              uri: '${fileUri.replace(/\\/g, '\\\\')}',
              language: 'json',
              content: ${JSON.stringify(defaultContent)},
              dirty: false,
              diagnostics: [],
              lastSavedContent: ${JSON.stringify(defaultContent)},
              mtime: ${stats.mtimeMs},
              size: ${stats.size}
            };
            store.getState().openDocument(doc);
          }
        })()
      `);
      
      return {
        success: true,
        filePath: sfxPath,
        relativePath: `assets/sfx/${fileName}`,
        message: `Sound effect file '${fileName}' created and opened successfully`
      };
    } catch (error: any) {
      throw new Error(`Failed to create SFX file: ${error.message}`);
    }
  }
}

type MCPToolHandler = (params: any) => Promise<any>;

// Singleton instance
let mcpServer: MCPServer | null = null;

export function getMCPServer(): MCPServer {
  if (!mcpServer) {
    mcpServer = new MCPServer();
  }
  return mcpServer;
}
