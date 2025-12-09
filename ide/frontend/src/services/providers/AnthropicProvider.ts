import { BaseAiProvider } from './BaseAiProvider.js';
import type { AiRequest, AiResponse } from '../../types/aiProvider.js';

export class AnthropicProvider extends BaseAiProvider {
  public readonly name = 'Anthropic';
  private readonly defaultBaseUrl = 'https://api.anthropic.com/v1';

  public isConfigured(): boolean {
    return !!this.config.apiKey;
  }

  private get baseUrl(): string {
    return this.config.endpoint || this.defaultBaseUrl;
  }

  public async sendRequest(request: AiRequest): Promise<AiResponse> {
    if (!this.isConfigured()) {
      throw new Error('Anthropic API key no configurada');
    }

    try {
      const systemPrompt = this.buildSystemPrompt();
      const userPrompt = this.buildUserPrompt(request);

      const response = await fetch(`${this.baseUrl}/messages`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-api-key': this.config.apiKey!,
          'anthropic-version': '2023-06-01'
        },
        body: JSON.stringify({
          model: this.config.model || 'claude-3-haiku-20240307',
          max_tokens: this.config.maxTokens || 2000,
          system: systemPrompt,
          messages: [
            { role: 'user', content: userPrompt }
          ]
        })
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(`Anthropic API error: ${response.status} ${response.statusText} - ${errorData.error?.message || 'Unknown error'}`);
      }

      const data = await response.json();
      const content = data.content?.[0]?.text || 'No response from Anthropic';
      const usage = data.usage;

      return {
        content,
        suggestions: this.extractSuggestions(content),
        usage: usage ? {
          promptTokens: usage.input_tokens || 0,
          completionTokens: usage.output_tokens || 0,
          totalTokens: (usage.input_tokens || 0) + (usage.output_tokens || 0)
        } : undefined
      };
    } catch (error) {
      return this.handleError(error, 'Anthropic');
    }
  }

  public async getModels(): Promise<string[]> {
    // Anthropic Claude models
    return [
      'claude-3-5-sonnet-20241022',
      'claude-3-5-haiku-20241022',
      'claude-3-opus-20240229',
      'claude-3-sonnet-20240229',
      'claude-3-haiku-20240307'
    ];
  }

  public async testConnection(): Promise<boolean> {
    if (!this.isConfigured()) {
      return false;
    }

    try {
      // Anthropic doesn't have a models endpoint, so we'll try a simple completion
      const response = await fetch(`${this.baseUrl}/messages`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-api-key': this.config.apiKey!,
          'anthropic-version': '2023-06-01'
        },
        body: JSON.stringify({
          model: 'claude-3-haiku-20240307',
          max_tokens: 10,
          messages: [
            { role: 'user', content: 'test' }
          ]
        })
      });
      return response.ok;
    } catch {
      return false;
    }
  }
}