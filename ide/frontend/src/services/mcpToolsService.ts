/**
 * MCP Tools Service for PyPilot
 * 
 * Provides PyPilot with access to MCP server tools to control the IDE
 */

import type { MCPRequest, MCPResponse } from '../../../electron/src/mcp/types';

export interface MCPTool {
  name: string;
  description: string;
  inputSchema: {
    type: string;
    properties: Record<string, any>;
    required?: string[];
  };
}

export class MCPToolsService {
  private static instance: MCPToolsService;
  private tools: MCPTool[] = [];
  private isInitialized = false;

  private constructor() {}

  static getInstance(): MCPToolsService {
    if (!MCPToolsService.instance) {
      MCPToolsService.instance = new MCPToolsService();
    }
    return MCPToolsService.instance;
  }

  /**
   * Initialize MCP connection and fetch available tools
   */
  async initialize(): Promise<void> {
    if (this.isInitialized) return;

    try {
      // Check if MCP is available
      if (!(window as any).mcp) {
        console.warn('[MCP Tools] window.mcp not available');
        return;
      }

      // Fetch available tools from MCP server
      const response: MCPResponse = await (window as any).mcp.request({
        jsonrpc: '2.0',
        id: 1,
        method: 'tools/list',
        params: {}
      });

      if (response.result && response.result.tools) {
        this.tools = response.result.tools;
        this.isInitialized = true;
        console.log(`[MCP Tools] Initialized with ${this.tools.length} tools`);
      }
    } catch (error) {
      console.error('[MCP Tools] Failed to initialize:', error);
    }
  }

  /**
   * Get list of available MCP tools
   */
  getAvailableTools(): MCPTool[] {
    return this.tools;
  }

  /**
   * Check if MCP is available and initialized
   */
  isAvailable(): boolean {
    return this.isInitialized && this.tools.length > 0;
  }

  /**
   * Call an MCP tool
   */
  async callTool(name: string, args: Record<string, any> = {}): Promise<any> {
    if (!(window as any).mcp) {
      throw new Error('MCP not available');
    }

    try {
      const response: MCPResponse = await (window as any).mcp.request({
        jsonrpc: '2.0',
        id: Date.now(),
        method: 'tools/call',
        params: {
          name,
          arguments: args
        }
      });

      if (response.error) {
        throw new Error(response.error.message);
      }

      return response.result;
    } catch (error) {
      console.error(`[MCP Tools] Failed to call ${name}:`, error);
      throw error;
    }
  }

  /**
   * Get formatted tool descriptions for LLM context
   */
  getToolsContext(): string {
    if (!this.isAvailable()) {
      return '';
    }

    return `
## Available IDE Tools (via MCP):

You have access to the following tools to control the VPy IDE:

${this.tools.map(tool => `
### ${tool.name}
${tool.description}

**Parameters:**
${JSON.stringify(tool.inputSchema.properties, null, 2)}
${tool.inputSchema.required ? `**Required:** ${tool.inputSchema.required.join(', ')}` : ''}
`).join('\n')}

## How to Use Tools:

To use a tool, respond with a JSON function call in a code block:

\`\`\`json
{
  "tool": "editor_list_documents",
  "arguments": {}
}
\`\`\`

### File Editing Tools:

**To replace text in a file:**
\`\`\`json
{
  "tool": "editor_replace_range",
  "arguments": {
    "uri": "file:///path/to/file.vpy",
    "startLine": 10,
    "startColumn": 1,
    "endLine": 15,
    "endColumn": 20,
    "newText": "def new_function():\\n    pass"
  }
}
\`\`\`

**To insert text:**
\`\`\`json
{
  "tool": "editor_insert_at",
  "arguments": {
    "uri": "file:///path/to/file.vpy",
    "line": 5,
    "column": 1,
    "text": "# New comment\\n"
  }
}
\`\`\`

**To delete text:**
\`\`\`json
{
  "tool": "editor_delete_range",
  "arguments": {
    "uri": "file:///path/to/file.vpy",
    "startLine": 10,
    "startColumn": 1,
    "endLine": 10,
    "endColumn": 50
  }
}
\`\`\`

**IMPORTANT:** Line and column numbers are 1-indexed (first line is 1, not 0).
`;
  }

  /**
   * Parse tool calls from LLM response
   */
  parseToolCalls(response: string): Array<{ tool: string; arguments: Record<string, any> }> {
    const calls: Array<{ tool: string; arguments: Record<string, any> }> = [];
    
    // Method 1: Look for JSON code blocks with backticks
    const jsonBlockRegex = /```json\s*\n([\s\S]*?)\n```/g;
    let match;

    while ((match = jsonBlockRegex.exec(response)) !== null) {
      try {
        const parsed = JSON.parse(match[1]);
        if (parsed.tool) {
          calls.push({
            tool: parsed.tool,
            arguments: parsed.arguments || {}
          });
        }
      } catch (e) {
        console.warn('[MCP Tools] Failed to parse tool call from code block:', e);
      }
    }

    // Method 2: Look for JSON objects without code blocks (plain text)
    // Pattern: "json\n{...}\n" or just "{...}" with "tool" property
    const plainJsonRegex = /(?:json\s*\n)?(\{[^}]*"tool"\s*:\s*"[^"]+"[^}]*\})/g;
    
    while ((match = plainJsonRegex.exec(response)) !== null) {
      try {
        const parsed = JSON.parse(match[1]);
        if (parsed.tool) {
          // Check if not already added (from code block)
          const exists = calls.some(c => c.tool === parsed.tool && 
            JSON.stringify(c.arguments) === JSON.stringify(parsed.arguments));
          if (!exists) {
            calls.push({
              tool: parsed.tool,
              arguments: parsed.arguments || {}
            });
          }
        }
      } catch (e) {
        console.warn('[MCP Tools] Failed to parse plain JSON tool call:', e);
      }
    }

    console.log('[MCP Tools] Parsed', calls.length, 'tool calls from response');
    return calls;
  }

  /**
   * Execute tool calls and format results
   */
  async executeToolCalls(calls: Array<{ tool: string; arguments: Record<string, any> }>): Promise<string> {
    const results: string[] = [];

    for (const call of calls) {
      try {
        console.log(`[MCP Tools] Executing ${call.tool}`, call.arguments);
        
        // Check if this is an editing operation
        const isEdit = call.tool.includes('replace') || call.tool.includes('insert') || call.tool.includes('delete');
        
        if (isEdit) {
          // Show what will be changed
          results.push(`🔧 **${call.tool}**\nAplicando cambios...`);
        }
        
        const result = await this.callTool(call.tool, call.arguments);
        
        // Extract text content from MCP response
        let resultText = '';
        if (result.content && Array.isArray(result.content)) {
          resultText = result.content
            .filter((item: any) => item.type === 'text')
            .map((item: any) => item.text)
            .join('\n');
        } else {
          resultText = JSON.stringify(result, null, 2);
        }

        if (isEdit) {
          results.push(`✅ Cambios aplicados correctamente\n${resultText}`);
        } else {
          results.push(`✅ **${call.tool}**:\n${resultText}`);
        }
      } catch (error) {
        results.push(`❌ **${call.tool}**: ${error instanceof Error ? error.message : 'Failed'}`);
      }
    }

    return results.join('\n\n');
  }

  /**
   * Preview what an edit operation will do
   */
  async previewEdit(tool: string, args: Record<string, any>): Promise<string> {
    const { uri, startLine, endLine, startColumn, endColumn, newText, text } = args;
    
    let preview = `📝 **Preview de cambio:**\n\n`;
    preview += `**Archivo:** ${uri.split('/').pop()}\n`;
    
    if (tool.includes('replace')) {
      preview += `**Líneas:** ${startLine}:${startColumn} → ${endLine}:${endColumn}\n`;
      preview += `**Nuevo texto:**\n\`\`\`\n${newText}\n\`\`\``;
    } else if (tool.includes('insert')) {
      preview += `**Insertar en:** Línea ${args.line}, Columna ${args.column}\n`;
      preview += `**Texto:**\n\`\`\`\n${text}\n\`\`\``;
    } else if (tool.includes('delete')) {
      preview += `**Eliminar:** Líneas ${startLine}:${startColumn} → ${endLine}:${endColumn}`;
    }
    
    return preview;
  }
}

export const mcpTools = MCPToolsService.getInstance();
