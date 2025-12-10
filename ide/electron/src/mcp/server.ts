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
        return this.createError(request.id, ErrorCodes.MethodNotFound, `Method not found: ${request.method} (tried: ${triedMethods.join(', ')})`);
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
        const store = window.__editorStore__;
        if (!store) throw new Error('Editor store not available');
        const state = store.getState();
        const doc = state.documents.find(d => d.uri === '${uri}' || d.uri.endsWith('/${uri}'));
        if (!doc) {
          const openDocs = state.documents.map(d => d.uri).join(', ');
          throw new Error('Document not found: "${uri}". Document must be OPEN in editor first. Open documents: [' + openDocs + ']. Use editor/write_document to CREATE new files, or project/create_music for .vmus files.');
        }
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
        const doc = state.documents.find(d => d.uri === '${uri.replace(/\\/g, '\\\\')}');
        
        if (doc) {
          // Document is open, update content
          store.getState().updateContent('${uri.replace(/\\/g, '\\\\')}', ${JSON.stringify(content)});
        } else {
          // Document not open, open it
          const language = '${uri.endsWith('.vpy') ? 'vpy' : uri.endsWith('.json') || uri.endsWith('.vec') || uri.endsWith('.vmus') ? 'json' : 'plaintext'}';
          store.getState().openDocument({
            uri: '${uri.replace(/\\/g, '\\\\')}',
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
          const doc = state.documents.find(d => d.uri.endsWith('/${uri}') || d.uri === '${uri}');
          if (!doc) throw new Error('Document not found: ${uri}');
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
        const doc = state.documents.find(d => d.uri === '${uri.replace(/\\/g, '\\\\')}');
        if (!doc) throw new Error('Document not found: ${uri}');
        
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
        store.getState().updateContent('${uri.replace(/\\/g, '\\\\')}', newContent);
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

    let { uri, startLine, startColumn, endLine, endColumn } = params;
    
    // Normalize URI
    uri = await this.normalizeUri(uri);
    
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
