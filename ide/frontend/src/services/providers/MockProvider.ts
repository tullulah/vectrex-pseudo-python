import { BaseAiProvider } from './BaseAiProvider.js';
import type { AiRequest, AiResponse } from '../../types/aiProvider.js';

export class MockProvider extends BaseAiProvider {
  public readonly name = 'Mock';

  public isConfigured(): boolean {
    return true; // Mock provider is always "configured"
  }

  public async sendRequest(request: AiRequest): Promise<AiResponse> {
    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, 500 + Math.random() * 1000));

    return this.getMockResponse(request);
  }

  private getMockResponse(request: AiRequest): AiResponse {
    // Mock responses inteligentes basadas en el tipo de request
    if (request.command === '/explain') {
      const codeToExplain = request.context.selectedCode || request.context.documentContent;
      const isFullDocument = !request.context.selectedCode && request.context.documentContent;
      
      if (codeToExplain) {
        return {
          content: `📖 **Explicación del código VPy:**

\`\`\`vpy
${isFullDocument && codeToExplain.length > 500 ? 
  codeToExplain.substring(0, 500) + '\n...[código completo disponible]' : 
  codeToExplain}
\`\`\`

🔍 **Análisis (Mock Response):**

Este código utiliza la sintaxis VPy (Vectrex Python) que se compila a ensamblador 6809 para la consola Vectrex.

**Fuente:** ${isFullDocument ? 'Documento completo' : 'Código seleccionado'} (${codeToExplain.length} caracteres)

**Elementos identificados:**
• Comandos de dibujo vectorial típicos del Vectrex
• Coordenadas en el sistema Vectrex (-127 a +127 en ambos ejes)
• Posible uso de intensidad para controlar el brillo del haz

**Para análisis real:** Configura tu API key en Settings.

💡 **Sugerencia:** Los comandos Vectrex son optimizados para gráficos vectoriales - evita usar demasiados puntos en polígonos complejos.`,
          usage: {
            promptTokens: 150,
            completionTokens: 300,
            totalTokens: 450
          }
        };
      }
    }

    if (request.command === '/generate') {
      const description = request.message.replace('/generate ', '');
      return {
        content: `🔧 **Código VPy generado para:** "${description}"

\`\`\`vpy
# Generado por PyPilot Mock para: ${description}
def main():
    # Configuración inicial
    INTENSITY(255)
    ORIGIN()
    
    # Código específico para: ${description}
    # TODO: Reemplazar con lógica real de IA
    
    # Ejemplo básico
    MOVE(-50, 50)
    PRINT_TEXT(0, 0, "${description.toUpperCase()}")
    
    return 0
\`\`\`

💡 **Mock Response:** Para generación real de código, configura una API de IA en Settings.

**Próximos pasos:**
• Ajusta las coordenadas según tu diseño
• Añade lógica de game loop si es necesario
• Usa los comandos específicos de Vectrex según el objetivo`,
        suggestions: [
          {
            type: 'code',
            title: 'Código base generado',
            code: `def main():\n    INTENSITY(255)\n    PRINT_TEXT(0, 0, "${description.toUpperCase()}")\n    return 0`,
            description: 'Estructura básica para el proyecto'
          }
        ],
        usage: {
          promptTokens: 100,
          completionTokens: 250,
          totalTokens: 350
        }
      };
    }

    // Respuesta genérica mock
    return {
      content: `🤖 **PyPilot (Mock Mode)**

Has enviado: "${request.message}"

**Contexto detectado:**
• Archivo: ${request.context.fileName || 'ninguno'}
• Código seleccionado: ${request.context.selectedCode ? 'Sí (' + request.context.selectedCode.length + ' chars)' : 'No'}
• Documento completo: ${request.context.documentContent ? 'Sí (' + (request.context.documentLength || 0) + ' chars)' : 'No'}
• Contexto manual: ${request.context.manualContext ? 'Sí (' + request.context.manualContext.length + ' chars)' : 'No'}
• Errores: ${request.context.errors?.length || 0}

**Esta es una respuesta simulada.** Para obtener asistencia real de IA:

1. Ve a ⚙️ **Settings**
2. Selecciona un proveedor (DeepSeek, OpenAI, Anthropic)
3. Configura tu API Key
4. ¡Disfruta de asistencia IA real!

**💡 Contexto mejorado:**
• ✅ Auto-contexto incluye el archivo completo activo
• ✅ Puedes adjuntar contexto manual adicional
• ✅ Código seleccionado tiene prioridad sobre documento completo

**Comandos disponibles:**
• \`/help\` - Ver todos los comandos
• \`/generate [descripción]\` - Generar código VPy
• \`/explain\` - Explicar código seleccionado
• \`/fix\` - Sugerir correcciones
• \`/vectrex [comando]\` - Info sobre comandos Vectrex`,
      suggestions: [
        {
          type: 'code',
          title: 'Ejemplo básico VPy',
          code: 'def main():\n    INTENSITY(255)\n    PRINT_TEXT(0, 0, "Hello Vectrex!")\n    return 0',
          description: 'Estructura básica de un programa VPy'
        }
      ],
      usage: {
        promptTokens: 120,
        completionTokens: 280,
        totalTokens: 400
      }
    };
  }

  public async getModels(): Promise<string[]> {
    return ['mock-model-v1', 'mock-advanced-v2'];
  }

  public async testConnection(): Promise<boolean> {
    // Mock always works
    return true;
  }
}