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
      description: 'Read content of a specific document',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI' },
        },
        required: ['uri'],
      },
    });

    this.registerTool('editor/write_document', this.writeDocument.bind(this), {
      name: 'editor/write_document',
      description: 'Write content to a document',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI' },
          content: { type: 'string', description: 'Document content' },
        },
        required: ['uri', 'content'],
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

    // Compiler tools
    this.registerTool('compiler/build', this.compilerBuild.bind(this), {
      name: 'compiler/build',
      description: 'Compile a VPy program',
      inputSchema: {
        type: 'object',
        properties: {
          entryFile: { type: 'string', description: 'Entry VPy file path' },
        },
        required: ['entryFile'],
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

    this.registerTool('debugger/get_callstack', this.getCallstack.bind(this), {
      name: 'debugger/get_callstack',
      description: 'Get current call stack',
      inputSchema: {
        type: 'object',
        properties: {},
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
  }

  private registerTool(name: string, handler: MCPToolHandler, schema: MCPTool) {
    this.tools.set(name, handler);
  }

  // Handle incoming JSON-RPC request
  async handleRequest(request: MCPRequest): Promise<MCPResponse> {
    try {
      // Handle special methods
      if (request.method === 'tools/list') {
        return this.createResponse(request.id, await this.listTools());
      }

      // Find and execute tool
      const handler = this.tools.get(request.method);
      if (!handler) {
        return this.createError(request.id, ErrorCodes.MethodNotFound, `Method not found: ${request.method}`);
      }

      const result = await handler(request.params || {});
      return this.createResponse(request.id, result);
    } catch (error: any) {
      console.error('[MCP] Error handling request:', error);
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

  // Tool implementations
  private async listTools(): Promise<{ tools: MCPTool[] }> {
    const tools: MCPTool[] = [
      // Will be populated from registered tools
      // For now, return basic list
    ];
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
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const doc = state.documents.find(d => d.uri === '${uri}');
        if (!doc) throw new Error('Document not found: ${uri}');
        return {
          uri: doc.uri,
          content: doc.content,
          language: doc.language,
          dirty: doc.dirty,
        };
      })()
    `);

    return result;
  }

  private async writeDocument(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { uri, content } = params;
    await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        store.getState().updateContent('${uri}', ${JSON.stringify(content)});
        return { success: true };
      })()
    `);

    return { success: true };
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
    // Will trigger build through IPC
    // For now, placeholder
    return { success: false, errors: [{ file: params.entryFile, line: 1, column: 1, message: 'Not implemented yet' }] };
  }

  private async getCompilerErrors(params: any): Promise<any> {
    return { errors: [] };
  }

  private async emulatorRun(params: any): Promise<any> {
    // Will send IPC to start emulator
    return { success: false, state: 'stopped' };
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
    return { success: false };
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
    // Will use fs to read file
    return { path: params.path, content: '' };
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
