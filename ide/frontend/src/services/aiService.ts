import type { AiProvider, AiRequest, AiResponse, VectrexCommandInfo } from '../types/ai';
import { logger } from '../utils/logger';

// Base de conocimiento de comandos Vectrex
const VECTREX_COMMANDS: VectrexCommandInfo[] = [
  {
    name: 'MOVE',
    syntax: 'MOVE(x, y)',
    description: 'Mueve el haz electrónico a coordenadas absolutas',
    example: 'MOVE(0, 0)  # Mueve al centro de la pantalla',
    category: 'movement'
  },
  {
    name: 'DRAW_LINE',
    syntax: 'DRAW_LINE(dx, dy)',
    description: 'Dibuja una línea desde la posición actual',
    example: 'DRAW_LINE(50, 50)  # Línea diagonal',
    category: 'drawing'
  },
  {
    name: 'INTENSITY',
    syntax: 'INTENSITY(value)',
    description: 'Establece la intensidad del haz (0-255)',
    example: 'INTENSITY(255)  # Máxima intensidad',
    category: 'intensity'
  },
  {
    name: 'PRINT_TEXT',
    syntax: 'PRINT_TEXT(x, y, text)',
    description: 'Muestra texto en pantalla usando la fuente del Vectrex',
    example: 'PRINT_TEXT(-50, 60, "HELLO WORLD")',
    category: 'text'
  },
  {
    name: 'DRAW_CIRCLE',
    syntax: 'DRAW_CIRCLE(radius)',
    description: 'Dibuja un círculo centrado en la posición actual',
    example: 'DRAW_CIRCLE(30)  # Círculo de radio 30',
    category: 'drawing'
  },
  {
    name: 'RECT',
    syntax: 'RECT(x, y, width, height)',
    description: 'Dibuja un rectángulo',
    example: 'RECT(-25, -25, 50, 50)  # Cuadrado centrado',
    category: 'drawing'
  },
  {
    name: 'POLYGON',
    syntax: 'POLYGON(count, x1, y1, x2, y2, ...)',
    description: 'Dibuja un polígono con los puntos especificados',
    example: 'POLYGON(3, 0, 50, -50, -50, 50, -50)  # Triángulo',
    category: 'drawing'
  },
  {
    name: 'ORIGIN',
    syntax: 'ORIGIN()',
    description: 'Resetea la posición de referencia al centro (0,0)',
    example: 'ORIGIN()  # Reset al centro',
    category: 'control'
  }
];

class AiService {
  private provider: AiProvider = {
    name: 'Mock',
    enabled: false
  };

  setProvider(provider: AiProvider) {
    this.provider = provider;
    logger.info('AI', 'Provider updated:', provider.name);
  }

  async sendRequest(request: AiRequest): Promise<AiResponse> {
    logger.debug('AI', 'Sending request:', { command: request.command, messageLength: request.message.length });

    // Si no hay provider configurado, usar mock
    if (!this.provider.enabled || this.provider.name === 'Mock') {
      return this.getMockResponse(request);
    }

    try {
      switch (this.provider.name) {
        case 'OpenAI':
          return await this.sendToOpenAI(request);
        case 'Anthropic':
          return await this.sendToAnthropic(request);
        case 'Local':
          return await this.sendToLocal(request);
        default:
          return this.getMockResponse(request);
      }
    } catch (error) {
      logger.error('AI', 'AI request failed:', error);
      return {
        content: `❌ Error comunicándose con ${this.provider.name}: ${error instanceof Error ? error.message : 'Unknown error'}`,
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  private async sendToOpenAI(request: AiRequest): Promise<AiResponse> {
    if (!this.provider.apiKey) {
      throw new Error('API Key de OpenAI no configurada');
    }

    const systemPrompt = this.buildSystemPrompt();
    const userPrompt = this.buildUserPrompt(request);

    const response = await fetch('https://api.openai.com/v1/chat/completions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${this.provider.apiKey}`
      },
      body: JSON.stringify({
        model: this.provider.model || 'gpt-4',
        messages: [
          { role: 'system', content: systemPrompt },
          { role: 'user', content: userPrompt }
        ],
        temperature: 0.7,
        max_tokens: 2000
      })
    });

    if (!response.ok) {
      throw new Error(`OpenAI API error: ${response.status} ${response.statusText}`);
    }

    const data = await response.json();
    const content = data.choices?.[0]?.message?.content || 'No response from OpenAI';

    return {
      content,
      suggestions: this.extractSuggestions(content)
    };
  }

  private async sendToAnthropic(request: AiRequest): Promise<AiResponse> {
    if (!this.provider.apiKey) {
      throw new Error('API Key de Anthropic no configurada');
    }

    const systemPrompt = this.buildSystemPrompt();
    const userPrompt = this.buildUserPrompt(request);

    const response = await fetch('https://api.anthropic.com/v1/messages', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': this.provider.apiKey,
        'anthropic-version': '2023-06-01'
      },
      body: JSON.stringify({
        model: this.provider.model || 'claude-3-sonnet-20240229',
        max_tokens: 2000,
        system: systemPrompt,
        messages: [
          { role: 'user', content: userPrompt }
        ]
      })
    });

    if (!response.ok) {
      throw new Error(`Anthropic API error: ${response.status} ${response.statusText}`);
    }

    const data = await response.json();
    const content = data.content?.[0]?.text || 'No response from Anthropic';

    return {
      content,
      suggestions: this.extractSuggestions(content)
    };
  }

  private async sendToLocal(request: AiRequest): Promise<AiResponse> {
    const endpoint = this.provider.endpoint || 'http://localhost:11434/api/generate';
    
    const systemPrompt = this.buildSystemPrompt();
    const userPrompt = this.buildUserPrompt(request);

    const response = await fetch(endpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        model: this.provider.model || 'llama2',
        prompt: systemPrompt + '\n\n' + userPrompt,
        stream: false
      })
    });

    if (!response.ok) {
      throw new Error(`Local LLM error: ${response.status} ${response.statusText}`);
    }

    const data = await response.json();
    const content = data.response || 'No response from local LLM';

    return {
      content,
      suggestions: this.extractSuggestions(content)
    };
  }

  private getMockResponse(request: AiRequest): AiResponse {
    // Mock responses inteligentes basadas en el tipo de request
    if (request.command === '/explain' && request.context.selectedCode) {
      return {
        content: `📖 **Explicación del código VPy:**

\`\`\`vpy
${request.context.selectedCode}
\`\`\`

🔍 **Análisis (Mock Response):**

Este código utiliza la sintaxis VPy (Vectrex Python) que se compila a ensamblador 6809 para la consola Vectrex.

**Elementos identificados:**
• Comandos de dibujo vectorial típicos del Vectrex
• Coordenadas en el sistema Vectrex (-127 a +127 en ambos ejes)
• Posible uso de intensidad para controlar el brillo del haz

**Para análisis real:** Configura tu API key en Settings.

💡 **Sugerencia:** Los comandos Vectrex son optimizados para gráficos vectoriales - evita usar demasiados puntos en polígonos complejos.`
      };
    }

    if (request.command === '/generate') {
      const description = request.message.replace('/generate ', '');
      return {
        content: `🔧 **Código VPy generado para:** "${description}"

\`\`\`vpy
# Generado por IA Mock para: ${description}
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
• Usa los comandos específicos de Vectrex según el objetivo`
      };
    }

    if (request.command === '/fix' && request.context.errors?.length) {
      return {
        content: `🔧 **Análisis de errores (Mock):**

**Errores detectados:**
${request.context.errors.map(e => `• Línea ${e.line}: ${e.message}`).join('\n')}

**Sugerencias de corrección:**
• Verifica la sintaxis VPy
• Revisa que los paréntesis estén balanceados
• Comprueba la aridad de las funciones Vectrex
• Asegúrate de usar la indentación correcta

**Para análisis real:** Configura IA en Settings para obtener sugerencias específicas.`,
        suggestions: [
          {
            type: 'fix',
            title: 'Revisar sintaxis',
            description: 'Verificar paréntesis y comas en llamadas a funciones'
          }
        ]
      };
    }

    if (request.command?.startsWith('/vectrex')) {
      const cmdName = request.command.split(' ')[1]?.toUpperCase();
      const cmd = VECTREX_COMMANDS.find(c => c.name === cmdName);
      
      if (cmd) {
        return {
          content: `📚 **Comando Vectrex: ${cmd.name}**

**Sintaxis:** \`${cmd.syntax}\`

**Descripción:** ${cmd.description}

**Ejemplo:**
\`\`\`vpy
${cmd.example}
\`\`\`

**Categoría:** ${cmd.category}

💡 **Tip:** Los comandos de dibujo del Vectrex usan coordenadas relativas al centro de la pantalla (0,0).`
        };
      }
    }

    // Respuesta genérica mock
    return {
      content: `🤖 **VPy AI Assistant (Mock Mode)**

Has enviado: "${request.message}"

**Contexto detectado:**
• Archivo: ${request.context.fileName || 'ninguno'}
• Código seleccionado: ${request.context.selectedCode ? 'Sí (' + request.context.selectedCode.length + ' chars)' : 'No'}
• Errores: ${request.context.errors?.length || 0}

**Esta es una respuesta simulada.** Para obtener asistencia real de IA:

1. Ve a ⚙️ **Settings**
2. Selecciona un proveedor (OpenAI, Anthropic, Local)
3. Configura tu API Key
4. ¡Disfruta de asistencia IA real!

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
          code: 'def main():\n    INTENSITY(255)\n    PRINT_TEXT(0, 0, "Hello Vectrex!")',
          description: 'Estructura básica de un programa VPy'
        }
      ]
    };
  }

  private buildSystemPrompt(): string {
    return `Eres un asistente especializado en VPy (Vectrex Python), un lenguaje que compila a ensamblador 6809 para la consola retro Vectrex.

CONOCIMIENTO BASE:
• Vectrex usa gráficos vectoriales con coordenadas (-127, +127)
• Comandos principales: MOVE(x,y), DRAW_LINE(dx,dy), INTENSITY(0-255), PRINT_TEXT(x,y,text)
• Sintaxis VPy: Python-like con funciones específicas de Vectrex
• El sistema de coordenadas tiene (0,0) en el centro de la pantalla
• Intensidad controla el brillo del haz electrónico

COMANDOS DISPONIBLES:
${VECTREX_COMMANDS.map(cmd => `• ${cmd.syntax} - ${cmd.description}`).join('\n')}

RESPUESTA ESPERADA:
• Usa markdown para formatear código (\`\`\`vpy)
• Incluye explicaciones técnicas específicas de Vectrex
• Proporciona ejemplos prácticos y funcionales
• Menciona consideraciones de performance cuando sea relevante
• Usa emojis para hacer las respuestas más amigables

Responde siempre en español y enfócate en ayudar con desarrollo para Vectrex.`;
  }

  private buildUserPrompt(request: AiRequest): string {
    let prompt = `CONSULTA: ${request.message}\n\n`;

    if (request.context.fileName) {
      prompt += `ARCHIVO ACTUAL: ${request.context.fileName}\n`;
    }

    if (request.context.selectedCode) {
      prompt += `CÓDIGO SELECCIONADO:\n\`\`\`vpy\n${request.context.selectedCode}\n\`\`\`\n`;
    }

    if (request.context.errors?.length) {
      prompt += `ERRORES DETECTADOS:\n${request.context.errors.map(e => `Línea ${e.line}: ${e.message}`).join('\n')}\n`;
    }

    if (request.command) {
      prompt += `COMANDO: ${request.command}\n`;
    }

    return prompt;
  }

  private extractSuggestions(content: string): Array<{type: 'code' | 'fix' | 'optimization' | 'explanation', title: string, code?: string, description?: string}> {
    const suggestions: Array<{type: 'code' | 'fix' | 'optimization' | 'explanation', title: string, code?: string, description?: string}> = [];
    
    // Buscar bloques de código
    const codeBlocks = content.match(/```vpy\n([\s\S]*?)\n```/g);
    if (codeBlocks) {
      codeBlocks.forEach((block, index) => {
        const code = block.replace(/```vpy\n/, '').replace(/\n```/, '');
        suggestions.push({
          type: 'code',
          title: `Código generado ${index + 1}`,
          code,
          description: 'Código VPy generado por IA'
        });
      });
    }

    return suggestions;
  }

  // Obtener información de comandos Vectrex
  getVectrexCommands(): VectrexCommandInfo[] {
    return VECTREX_COMMANDS;
  }

  getVectrexCommand(name: string): VectrexCommandInfo | undefined {
    return VECTREX_COMMANDS.find(cmd => 
      cmd.name.toLowerCase() === name.toLowerCase()
    );
  }
}

export const aiService = new AiService();
export default aiService;