import { AiProviderFactory } from './AiProviderFactory';
import type { 
  IAiProvider, 
  AiProviderType, 
  AiProviderConfig, 
  AiRequest, 
  AiResponse 
} from '../types/aiProvider';
import { logger } from '../utils/logger';

export class AiService {
  private currentProvider: IAiProvider | null = null;
  private currentProviderType: AiProviderType = 'mock';
  private currentConfig: AiProviderConfig = {};

  constructor() {
    // Inicializar con Mock provider por defecto
    this.switchProvider('mock');
  }

  /**
   * Cambia el proveedor de IA activo
   */
  public switchProvider(type: AiProviderType, config?: AiProviderConfig): void {
    try {
      this.currentProviderType = type;
      this.currentConfig = config || {};
      this.currentProvider = AiProviderFactory.getProvider(type, config);
      
      logger.info('AI', `Provider switched to: ${this.currentProvider.name}`);
    } catch (error) {
      logger.error('AI', 'Failed to switch AI provider:', error);
      // Fallback a Mock provider
      this.currentProviderType = 'mock';
      this.currentConfig = {};
      this.currentProvider = AiProviderFactory.getProvider('mock');
    }
  }

  /**
   * Obtiene el proveedor actual
   */
  public getCurrentProvider(): IAiProvider | null {
    return this.currentProvider;
  }

  /**
   * Obtiene el tipo de proveedor actual
   */
  public getCurrentProviderType(): AiProviderType {
    return this.currentProviderType;
  }

  /**
   * Verifica si el proveedor actual está configurado
   */
  public isConfigured(): boolean {
    return this.currentProvider?.isConfigured() ?? false;
  }

  /**
   * Envía una solicitud al proveedor de IA actual
   */
  public async sendRequest(request: AiRequest): Promise<AiResponse> {
    if (!this.currentProvider) {
      throw new Error('No AI provider configured');
    }

    try {
      logger.debug('AI', 'Sending request:', {
        command: request.command,
        messageLength: request.message.length,
        hasContext: !!request.context.selectedCode || !!request.context.documentContent
      });

      const response = await this.currentProvider.sendRequest(request);
      
      logger.debug('AI', 'Response received:', {
        contentLength: response.content.length,
        hasSuggestions: !!response.suggestions?.length,
        hasUsage: !!response.usage
      });

      return response;
    } catch (error) {
      logger.error('AI', `AI request failed (${this.currentProvider.name}):`, error);
      
      // Retornar respuesta de error
      return {
        content: `❌ Error al procesar la solicitud con ${this.currentProvider.name}:\n\n${error instanceof Error ? error.message : String(error)}`,
        error: error instanceof Error ? error.message : String(error)
      };
    }
  }

  /**
   * Obtiene los proveedores disponibles
   */
  public getAvailableProviders() {
    return AiProviderFactory.getAvailableProviders();
  }

  /**
   * Prueba la conexión con un proveedor específico
   */
  public async testProviderConnection(type: AiProviderType, config?: AiProviderConfig): Promise<boolean> {
    try {
      return await AiProviderFactory.testProviderConnection(type, config);
    } catch (error) {
      logger.error('AI', `Connection test failed for ${type}:`, error);
      return false;
    }
  }

  /**
   * Obtiene los modelos disponibles para un proveedor
   */
  public async getProviderModels(type: AiProviderType, config?: AiProviderConfig): Promise<string[]> {
    try {
      return await AiProviderFactory.getProviderModels(type, config);
    } catch (error) {
      logger.error('AI', `Failed to get models for ${type}:`, error);
      return [];
    }
  }

  /**
   * Configura el proveedor actual
   */
  public configureCurrentProvider(config: AiProviderConfig): void {
    if (this.currentProvider) {
      this.currentProvider.configure(config);
      this.currentConfig = { ...this.currentConfig, ...config };
    }
  }

  /**
   * Obtiene la configuración actual del proveedor
   */
  public getCurrentConfig(): AiProviderConfig {
    return this.currentConfig;
  }

  /**
   * Limpia la caché de proveedores (útil cuando cambian configs)
   */
  public clearProvidersCache(): void {
    AiProviderFactory.clearCache();
    // Recrear provider actual
    this.switchProvider(this.currentProviderType, this.currentConfig);
  }
}

// Instancia singleton del servicio
export const aiService = new AiService();