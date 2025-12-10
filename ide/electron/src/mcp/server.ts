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

    this.registerTool('editor/replace_range', this.replaceRange.bind(this), {
      name: 'editor/replace_range',
      description: 'Replace text in a specific range of a document',
      inputSchema: {
        type: 'object',
        properties: {
          uri: { type: 'string', description: 'Document URI' },
          startLine: { type: 'number', description: 'Start line (1-indexed)' },
          startColumn: { type: 'number', description: 'Start column (1-indexed)' },
          endLine: { type: 'number', description: 'End line (1-indexed)' },
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
      description: 'Create a new VPy project',
      inputSchema: {
        type: 'object',
        properties: {
          path: { type: 'string', description: 'Directory path for new project' },
          name: { type: 'string', description: 'Project name' },
        },
        required: ['path', 'name'],
      },
    });
  }

  private registerTool(name: string, handler: MCPToolHandler, schema: MCPTool) {
    this.tools.set(name, handler);
    this.toolSchemas.set(name, schema);
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

  private async replaceRange(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { uri, startLine, startColumn, endLine, endColumn, newText } = params;
    
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const doc = state.documents.find(d => d.uri === '${uri}');
        if (!doc) throw new Error('Document not found: ${uri}');
        
        const lines = doc.content.split('\\n');
        const startIdx = ${startLine - 1};
        const endIdx = ${endLine - 1};
        
        if (startIdx === endIdx) {
          // Same line replacement
          const line = lines[startIdx];
          lines[startIdx] = line.substring(0, ${startColumn - 1}) + 
                           ${JSON.stringify(newText)} + 
                           line.substring(${endColumn - 1});
        } else {
          // Multi-line replacement
          const firstLine = lines[startIdx].substring(0, ${startColumn - 1}) + ${JSON.stringify(newText)};
          const lastLine = lines[endIdx].substring(${endColumn - 1});
          lines.splice(startIdx, endIdx - startIdx + 1, firstLine + lastLine);
        }
        
        const newContent = lines.join('\\n');
        store.getState().updateContent('${uri}', newContent);
        return { success: true, linesChanged: endIdx - startIdx + 1 };
      })()
    `);

    return result;
  }

  private async insertAt(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { uri, line, column, text } = params;
    
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const doc = state.documents.find(d => d.uri === '${uri}');
        if (!doc) throw new Error('Document not found: ${uri}');
        
        const lines = doc.content.split('\\n');
        const lineIdx = ${line - 1};
        const colIdx = ${column - 1};
        
        if (lineIdx < 0 || lineIdx >= lines.length) {
          throw new Error('Line out of range: ' + ${line});
        }
        
        const currentLine = lines[lineIdx];
        lines[lineIdx] = currentLine.substring(0, colIdx) + 
                        ${JSON.stringify(text)} + 
                        currentLine.substring(colIdx);
        
        const newContent = lines.join('\\n');
        store.getState().updateContent('${uri}', newContent);
        return { success: true, insertedLength: ${JSON.stringify(text)}.length };
      })()
    `);

    return result;
  }

  private async deleteRange(params: any): Promise<any> {
    if (!this.mainWindow) {
      throw new Error('No main window available');
    }

    const { uri, startLine, startColumn, endLine, endColumn } = params;
    
    const result = await this.mainWindow.webContents.executeJavaScript(`
      (function() {
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const doc = state.documents.find(d => d.uri === '${uri}');
        if (!doc) throw new Error('Document not found: ${uri}');
        
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
        store.getState().updateContent('${uri}', newContent);
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

    const { path, name } = params;
    
    // This would need IPC to main process to create file system structure
    // For now, return not implemented
    return { 
      success: false, 
      message: 'Project creation requires file system access - use project/open with existing path instead',
      suggestion: `Create project folder manually at ${path}, then use project/open`
    };
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
