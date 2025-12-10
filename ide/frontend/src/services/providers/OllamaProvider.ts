import { BaseAiProvider } from './BaseAiProvider.js';
import type { AiRequest, AiResponse } from '../../types/aiProvider.js';

export interface OllamaModel {
  name: string;
  size: string;
  digest: string;
  modified_at: string;
}

export interface OllamaModelInfo {
  name: string;
  displayName: string;
  description: string;
  size: string;
  parameters: string;
  recommended: boolean;
}

export class OllamaProvider extends BaseAiProvider {
  public readonly name = 'Ollama (Local)';
  private readonly defaultBaseUrl = 'http://localhost:11434';

  // Recommended models for tool calling and code generation
  private readonly recommendedModels: OllamaModelInfo[] = [
    {
      name: 'qwen2.5:7b',
      displayName: 'Qwen 2.5 7B',
      description: 'Excellent for tool calling and code generation (RECOMMENDED)',
      size: '4.7 GB',
      parameters: '7B',
      recommended: true
    },
    {
      name: 'llama3.2:3b',
      displayName: 'Llama 3.2 3B',
      description: 'Fast and lightweight for quick responses',
      size: '2.0 GB',
      parameters: '3B',
      recommended: true
    },
    {
      name: 'qwen2.5:14b',
      displayName: 'Qwen 2.5 14B',
      description: 'Higher quality, requires more RAM (16GB+)',
      size: '9.0 GB',
      parameters: '14B',
      recommended: false
    },
    {
      name: 'codellama:7b',
      displayName: 'Code Llama 7B',
      description: 'Specialized for code generation',
      size: '3.8 GB',
      parameters: '7B',
      recommended: false
    },
    {
      name: 'deepseek-coder:6.7b',
      displayName: 'DeepSeek Coder 6.7B',
      description: 'Excellent for code understanding',
      size: '3.8 GB',
      parameters: '6.7B',
      recommended: false
    }
  ];

  public isConfigured(): boolean {
    // Ollama doesn't need API key, just needs to be running
    return true;
  }

  private get baseUrl(): string {
    return this.config.endpoint || this.defaultBaseUrl;
  }

  public async sendRequest(request: AiRequest): Promise<AiResponse> {
    try {
      // Check if Ollama is running
      const isRunning = await this.checkOllamaRunning();
      if (!isRunning) {
        throw new Error('Ollama is not running. Please start Ollama: brew services start ollama');
      }

      // Build system prompt with tool definitions
      const baseSystemPrompt = this.buildSystemPrompt();
      
      // Get tool list from request if available
      const toolsList = request.availableTools?.map(t => 
        `- ${t.name}: ${t.description}`
      ).join('\n') || 'Loading tools...';
      
      const systemPrompt = `${baseSystemPrompt}

# Tool Calling Mode

When the user wants to perform an action, respond with a JSON tool call in ONE of these formats:

**Format 1 (Preferred):**
\`\`\`json
{"tool": "namespace/action", "arguments": {}}
\`\`\`

**Format 2 (Also accepted):**
\`\`\`json
{"tool_calls": [{"name": "namespace/action", "params": {}}]}
\`\`\`

## Available Tools:
${toolsList}

## Examples:

User: "close the current project"
\`\`\`json
{"tool": "project/close", "arguments": {}}
\`\`\`

User: "list all open files"
\`\`\`json
{"tool": "editor/list_documents", "arguments": {}}
\`\`\`

User: "show project structure"
\`\`\`json
{"tool": "project/get_structure", "arguments": {}}
\`\`\`

IMPORTANT: 
- Understand the user's intent naturally
- Tool names use forward slash: "namespace/action"
- Use {} for empty arguments
- Respond with JSON only when the user wants an action`;

      const response = await fetch(`${this.baseUrl}/v1/chat/completions`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          model: this.config.model || 'qwen2.5:7b',
          messages: [
            { role: 'system', content: systemPrompt },
            { role: 'user', content: request.message }
          ],
          temperature: this.config.temperature || 0.1, // Lower temperature for more deterministic tool calling
          max_tokens: this.config.maxTokens || 2000,
          stream: false,
          // Additional parameters for better tool calling
          top_p: 0.9,
          frequency_penalty: 0.0,
          presence_penalty: 0.0
        })
      });

      if (!response.ok) {
        const errorText = await response.text().catch(() => '');
        throw new Error(`Ollama API error: ${response.status} ${response.statusText}\n${errorText}`);
      }

      const data = await response.json();
      const content = data.choices?.[0]?.message?.content || 'No response from Ollama';

      return {
        content,
        suggestions: this.extractSuggestions(content),
        usage: data.usage ? {
          promptTokens: data.usage.prompt_tokens || 0,
          completionTokens: data.usage.completion_tokens || 0,
          totalTokens: data.usage.total_tokens || 0
        } : undefined
      };
    } catch (error) {
      return this.handleError(error, 'Ollama');
    }
  }

  public async getModels(): Promise<string[]> {
    try {
      // Get installed models from Ollama
      const response = await fetch(`${this.baseUrl}/api/tags`);
      if (!response.ok) {
        console.warn('Failed to fetch Ollama models, returning recommended list');
        return this.recommendedModels.map(m => m.name);
      }

      const data = await response.json();
      const installedModels = data.models?.map((m: OllamaModel) => m.name) || [];
      
      // Combine installed + recommended (deduplicated)
      const allModels = new Set([...installedModels, ...this.recommendedModels.map(m => m.name)]);
      return Array.from(allModels);
    } catch (error) {
      console.warn('Error fetching Ollama models:', error);
      return this.recommendedModels.map(m => m.name);
    }
  }

  public getRecommendedModels(): OllamaModelInfo[] {
    return this.recommendedModels;
  }

  public async checkOllamaRunning(): Promise<boolean> {
    try {
      const response = await fetch(`${this.baseUrl}/api/version`, {
        method: 'GET',
        signal: AbortSignal.timeout(2000) // 2 second timeout
      });
      return response.ok;
    } catch (error) {
      return false;
    }
  }

  public async checkModelInstalled(modelName: string): Promise<boolean> {
    try {
      const response = await fetch(`${this.baseUrl}/api/tags`);
      if (!response.ok) return false;
      
      const data = await response.json();
      const models = data.models || [];
      return models.some((m: OllamaModel) => m.name === modelName);
    } catch (error) {
      return false;
    }
  }

  public async pullModel(modelName: string, onProgress?: (progress: string) => void): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/api/pull`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ name: modelName })
      });

      if (!response.ok) {
        throw new Error(`Failed to pull model: ${response.statusText}`);
      }

      // Stream progress updates
      const reader = response.body?.getReader();
      if (!reader) throw new Error('No response body');

      const decoder = new TextDecoder();
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        const text = decoder.decode(value);
        const lines = text.split('\n').filter(line => line.trim());
        
        for (const line of lines) {
          try {
            const progress = JSON.parse(line);
            if (onProgress && progress.status) {
              const percentage = progress.completed && progress.total 
                ? `${Math.round((progress.completed / progress.total) * 100)}%`
                : '';
              onProgress(`${progress.status}${percentage ? ' - ' + percentage : ''}`);
            }
          } catch (e) {
            // Ignore JSON parse errors
          }
        }
      }
    } catch (error: any) {
      throw new Error(`Failed to download model: ${error.message}`);
    }
  }

  public async testConnection(): Promise<boolean> {
    return await this.checkOllamaRunning();
  }
}
