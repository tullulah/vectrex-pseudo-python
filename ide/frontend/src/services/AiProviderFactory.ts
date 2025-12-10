import type { IAiProvider, AiProviderConfig, AiProviderType } from '../types/aiProvider.js';
import { MockProvider } from './providers/MockProvider.js';
import { DeepSeekProvider } from './providers/DeepSeekProvider.js';
import { OpenAiProvider } from './providers/OpenAiProvider.js';
import { AnthropicProvider } from './providers/AnthropicProvider.js';
import { GitHubModelsProvider } from './providers/GitHubModelsProvider.js';
import { GroqProvider } from './providers/GroqProvider.js';
import { OllamaProvider } from './providers/OllamaProvider.js';

export class AiProviderFactory {
  private static instances: Map<AiProviderType, IAiProvider> = new Map();

  /**
   * Crea o reutiliza una instancia del proveedor especificado
   */
  public static getProvider(type: AiProviderType, config?: AiProviderConfig): IAiProvider {
    // Para Mock provider, no necesitamos configuración
    if (type === 'mock') {
      if (!this.instances.has('mock')) {
        this.instances.set('mock', new MockProvider());
      }
      return this.instances.get('mock')!;
    }

    // Para otros proveedores, necesitamos configuración
    if (!config) {
      throw new Error(`Configuration required for provider: ${type}`);
    }

    // Crear nueva instancia siempre que cambie la config (por si cambia API key)
    const provider = this.createProvider(type, config);
    this.instances.set(type, provider);
    return provider;
  }

  /**
   * Crea una nueva instancia del proveedor
   */
  private static createProvider(type: AiProviderType, config: AiProviderConfig): IAiProvider {
    switch (type) {
      case 'mock':
        return new MockProvider();
      
      case 'deepseek':
        if (!config.apiKey) {
          throw new Error('DeepSeek API key is required');
        }
        return new DeepSeekProvider(config);
      
      case 'openai':
        if (!config.apiKey) {
          throw new Error('OpenAI API key is required');
        }
        return new OpenAiProvider(config);
      
      case 'anthropic':
        if (!config.apiKey) {
          throw new Error('Anthropic API key is required');
        }
        return new AnthropicProvider(config);
      
      case 'github':
        if (!config.apiKey) {
          throw new Error('GitHub Models API key is required');
        }
        return new GitHubModelsProvider(config);
      
      case 'groq':
        if (!config.apiKey) {
          throw new Error('Groq API key is required');
        }
        return new GroqProvider(config);
      
      case 'ollama':
        // Ollama doesn't require API key, just needs to be running locally
        return new OllamaProvider(config || {});
      
      default:
        throw new Error(`Unknown provider type: ${type}`);
    }
  }

  /**
   * Obtiene todos los tipos de proveedores disponibles
   */
  public static getAvailableProviders(): Array<{
    type: AiProviderType;
    name: string;
    description: string;
    requiresApiKey: boolean;
    isConfigured: boolean;
  }> {
    const mockProvider = this.getProvider('mock');
    
    return [
      {
        type: 'mock',
        name: 'Mock Provider',
        description: 'Simulador local para testing - no requiere API key',
        requiresApiKey: false,
        isConfigured: mockProvider.isConfigured()
      },
      {
        type: 'deepseek',
        name: 'DeepSeek',
        description: 'API gratuita de DeepSeek - Rápida y eficiente',
        requiresApiKey: true,
        isConfigured: false // Will be checked when config is provided
      },
      {
        type: 'openai',
        name: 'OpenAI',
        description: 'GPT-4o y GPT-4o-mini - Máxima calidad',
        requiresApiKey: true,
        isConfigured: false
      },
      {
        type: 'anthropic',
        name: 'Anthropic Claude',
        description: 'Claude 3.5 Sonnet/Haiku - Razonamiento avanzado',
        requiresApiKey: true,
        isConfigured: false
      },
      {
        type: 'github',
        name: 'GitHub Models',
        description: 'Modelos via GitHub Copilot/Models - GPT-4o, Claude, Llama',
        requiresApiKey: true,
        isConfigured: false
      },
      {
        type: 'groq',
        name: 'Groq',
        description: 'API gratuita con Llama 3 - Ultrarrápida inferencia',
        requiresApiKey: true,
        isConfigured: false
      },
      {
        type: 'ollama',
        name: 'Ollama (Local)',
        description: 'Modelos locales en tu Mac - 100% privado, sin API key',
        requiresApiKey: false,
        isConfigured: true // Always configured, just needs Ollama running
      }
    ];
  }

  /**
   * Verifica si un proveedor está correctamente configurado
   */
  public static async isProviderConfigured(type: AiProviderType, config?: AiProviderConfig): Promise<boolean> {
    try {
      const provider = this.getProvider(type, config);
      return provider.isConfigured();
    } catch {
      return false;
    }
  }

  /**
   * Prueba la conexión con un proveedor
   */
  public static async testProviderConnection(type: AiProviderType, config?: AiProviderConfig): Promise<boolean> {
    try {
      const provider = this.getProvider(type, config);
      if (!provider.isConfigured() || !provider.testConnection) {
        return false;
      }
      return await provider.testConnection();
    } catch {
      return false;
    }
  }

  /**
   * Limpia la caché de instancias (útil cuando cambian configs)
   */
  public static clearCache(): void {
    this.instances.clear();
  }

  /**
   * Obtiene información sobre modelos disponibles para un proveedor
   */
  public static async getProviderModels(type: AiProviderType, config?: AiProviderConfig): Promise<string[]> {
    try {
      console.log('🔄 AiProviderFactory.getProviderModels called for:', type, 'with config:', {
        hasApiKey: !!config?.apiKey,
        apiKeyLength: config?.apiKey?.length
      });
      
      const provider = this.getProvider(type, config);
      console.log('✅ Provider instance created:', provider.name);
      
      if (!provider.getModels) {
        console.warn('⚠️ Provider does not have getModels method');
        return [];
      }
      
      console.log('🚀 Calling provider.getModels()...');
      const models = await provider.getModels();
      console.log('✅ Models returned from provider:', models);
      
      return models;
    } catch (error) {
      console.error('❌ Error in getProviderModels:', error);
      return [];
    }
  }
}