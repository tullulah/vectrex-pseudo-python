import { BaseAiProvider } from './BaseAiProvider';
import { AiProviderConfig, AiRequest, AiResponse } from '../../types/aiProvider';

export class GroqProvider extends BaseAiProvider {
  public readonly name = 'Groq (Free)';
  private baseUrl: string;

  constructor(config?: AiProviderConfig) {
    super(config);
    this.baseUrl = 'https://api.groq.com/openai/v1';
  }

  public isConfigured(): boolean {
    return !!(this.config.apiKey && this.config.apiKey.length > 0);
  }

  async testConnection(): Promise<boolean> {
    console.log('🔍 Testing Groq connection...');
    console.log('Raw config received:', this.config);
    console.log('API Key validation:', {
      hasKey: !!this.config.apiKey,
      keyStart: this.config.apiKey?.substring(0, 15) + '...',
      keyLength: this.config.apiKey?.length,
      model: this.config.model || 'llama-3.1-8b-instant',
      baseUrl: this.baseUrl
    });

    if (!this.isConfigured()) {
      console.error('Groq not configured - missing API key');
      return false;
    }

    try {
      const testUrl = `${this.baseUrl}/chat/completions`;
      console.log('Testing URL:', testUrl);

      const response = await fetch(testUrl, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${this.config.apiKey}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          model: this.config.model || 'llama-3.1-8b-instant',
          messages: [{ role: 'user', content: 'Hello' }],
          max_tokens: 10
        })
      });

      console.log('Groq response status:', response.status);
      
      if (!response.ok) {
        const errorText = await response.text();
        console.error('Groq API error:', {
          status: response.status,
          statusText: response.statusText,
          error: errorText
        });
        return false;
      }

      console.log('✅ Groq connection successful');
      return true;
    } catch (error) {
      console.error('Groq connection test failed:', error);
      console.error('Error details:', {
        message: error instanceof Error ? error.message : 'Unknown error',
        stack: error instanceof Error ? error.stack : 'No stack trace',
        type: typeof error
      });
      return false;
    }
  }

  public async sendRequest(request: AiRequest): Promise<AiResponse> {
    if (!this.isConfigured()) {
      throw new Error('Groq provider not configured');
    }

    const messages = [
      {
        role: 'system',
        content: this.buildSystemPrompt()
      },
      {
        role: 'user',
        content: this.buildUserPrompt(request)
      }
    ];

    try {
      const response = await fetch(`${this.baseUrl}/chat/completions`, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${this.config.apiKey}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          model: this.config.model || 'llama-3.1-8b-instant',
          messages,
          max_tokens: 1000,
          temperature: 0.7
        })
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Groq API error: ${response.status} ${errorText}`);
      }

      const data = await response.json();
      return {
        content: data.choices[0].message.content
      };
    } catch (error) {
      throw new Error(`Failed to get response from Groq: ${error}`);
    }
  }

  public async getAvailableModels(): Promise<string[]> {
    if (!this.isConfigured()) {
      console.warn('Groq not configured for model discovery');
      return [
        'llama-3.1-70b-versatile',
        'llama-3.1-8b-instant', 
        'mixtral-8x7b-32768',
        'gemma2-9b-it'
      ];
    }

    try {
      console.log('🔍 Fetching Groq models from API...');
      const response = await fetch(`${this.baseUrl}/models`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${this.config.apiKey}`,
          'Content-Type': 'application/json',
        }
      });

      if (!response.ok) {
        console.warn(`Groq models API error: ${response.status}, using fallback models`);
        return [
          'llama-3.1-70b-versatile',
          'llama-3.1-8b-instant', 
          'mixtral-8x7b-32768',
          'gemma2-9b-it'
        ];
      }

      const data = await response.json();
      const models = data.data?.map((model: any) => model.id) || [];
      console.log('✅ Groq models fetched:', models);
      
      return models.length > 0 ? models : [
        'llama-3.1-70b-versatile',
        'llama-3.1-8b-instant', 
        'mixtral-8x7b-32768',
        'gemma2-9b-it'
      ];
    } catch (error) {
      console.error('Failed to fetch Groq models:', error);
      return [
        'llama-3.1-70b-versatile',
        'llama-3.1-8b-instant', 
        'mixtral-8x7b-32768',
        'gemma2-9b-it'
      ];
    }
  }
}