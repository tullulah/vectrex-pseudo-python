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

To use a tool, respond with a JSON function call like:
\`\`\`json
{
  "tool": "editor_list_documents",
  "arguments": {}
}
\`\`\`

Or for tools with parameters:
\`\`\`json
{
  "tool": "editor_read_document",
  "arguments": {
    "uri": "file:///path/to/file.vpy"
  }
}
\`\`\`
`;
  }

  /**
   * Parse tool calls from LLM response
   */
  parseToolCalls(response: string): Array<{ tool: string; arguments: Record<string, any> }> {
    const calls: Array<{ tool: string; arguments: Record<string, any> }> = [];
    
    // Look for JSON code blocks
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
        console.warn('[MCP Tools] Failed to parse tool call:', e);
      }
    }

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

        results.push(`✅ **${call.tool}**:\n${resultText}`);
      } catch (error) {
        results.push(`❌ **${call.tool}**: ${error instanceof Error ? error.message : 'Failed'}`);
      }
    }

    return results.join('\n\n');
  }
}

export const mcpTools = MCPToolsService.getInstance();
