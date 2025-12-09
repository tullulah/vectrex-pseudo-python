import { BaseAiProvider } from './BaseAiProvider.js';
import type { AiRequest, AiResponse } from '../../types/aiProvider.js';

export class DeepSeekProvider extends BaseAiProvider {
  public readonly name = 'DeepSeek';
  private readonly defaultBaseUrl = 'https://api.deepseek.com/v1';

  public isConfigured(): boolean {
    return !!this.config.apiKey;
  }

  private get baseUrl(): string {
    return this.config.endpoint || this.defaultBaseUrl;
  }

  public async sendRequest(request: AiRequest): Promise<AiResponse> {
    if (!this.isConfigured()) {
      throw new Error('DeepSeek API key no configurada');
    }

    try {
      const systemPrompt = this.buildSystemPrompt();
      const userPrompt = this.buildUserPrompt(request);

      const response = await fetch(`${this.baseUrl}/chat/completions`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.config.apiKey}`
        },
        body: JSON.stringify({
          model: this.config.model || 'deepseek-chat',
          messages: [
            { role: 'system', content: systemPrompt },
            { role: 'user', content: userPrompt }
          ],
          temperature: this.config.temperature || 0.7,
          max_tokens: this.config.maxTokens || 2000,
          stream: false
        })
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(`DeepSeek API error: ${response.status} ${response.statusText} - ${errorData.error?.message || 'Unknown error'}`);
      }

      const data = await response.json();
      const content = data.choices?.[0]?.message?.content || 'No response from DeepSeek';
      const usage = data.usage;

      return {
        content,
        suggestions: this.extractSuggestions(content),
        usage: usage ? {
          promptTokens: usage.prompt_tokens || 0,
          completionTokens: usage.completion_tokens || 0,
          totalTokens: usage.total_tokens || 0
        } : undefined
      };
    } catch (error) {
      return this.handleError(error, 'DeepSeek');
    }
  }

  public async getModels(): Promise<string[]> {
    // DeepSeek models available
    return [
      'deepseek-chat',
      'deepseek-coder',
      'deepseek-reasoner'
    ];
  }

  public async testConnection(): Promise<boolean> {
    if (!this.isConfigured()) {
      return false;
    }

    try {
      const response = await fetch(`${this.baseUrl}/models`, {
        headers: {
          'Authorization': `Bearer ${this.config.apiKey}`
        }
      });
      return response.ok;
    } catch {
      return false;
    }
  }
}