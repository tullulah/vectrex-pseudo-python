import React, { useState, useRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useEditorStore } from '../../state/editorStore';
import type { AiMessage } from '../../types/ai';
import type { AiProviderType, AiProviderConfig } from '../../types/aiProvider';
import { aiService } from '../../services/aiService';
import { logger } from '../../utils/logger';
import { mcpTools } from '../../services/mcpToolsService';
import { OllamaManagerDialog } from '../dialogs/OllamaManagerDialog';

// Base de conocimiento de comandos Vectrex
const VECTREX_COMMANDS = [
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
    name: 'ORIGIN',
    syntax: 'ORIGIN()',
    description: 'Resetea la posición de referencia al centro (0,0)',
    example: 'ORIGIN()  # Reset al centro',
    category: 'control'
  }
];

export const AiAssistantPanel: React.FC = () => {
  const { t } = useTranslation();
  const [messages, setMessages] = useState<AiMessage[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  
  // Load persisted settings
  const [currentProviderType, setCurrentProviderType] = useState<AiProviderType>(() => {
    const saved = localStorage.getItem('pypilot_provider');
    return (saved as AiProviderType) || 'mock';
  });
  
  const [providerConfig, setProviderConfig] = useState<AiProviderConfig>(() => {
    const saved = localStorage.getItem(`pypilot_config_${currentProviderType}`);
    return saved ? JSON.parse(saved) : {};
  });
  
  const [showSettings, setShowSettings] = useState(false);
  const [manualContext, setManualContext] = useState<string>('');
  const [availableModels, setAvailableModels] = useState<string[]>([]);
  const [isLoadingModels, setIsLoadingModels] = useState(false);
  const [mcpEnabled, setMcpEnabled] = useState(false);
  const [showOllamaManager, setShowOllamaManager] = useState(false);
  
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const activeDocument = useEditorStore(s => s.active);
  const documents = useEditorStore(s => s.documents);

  // Auto-scroll to bottom when new messages arrive
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Persist settings
  useEffect(() => {
    localStorage.setItem('pypilot_provider', currentProviderType);
  }, [currentProviderType]);

  // Load provider-specific config when provider changes
  useEffect(() => {
    const saved = localStorage.getItem(`pypilot_config_${currentProviderType}`);
    const loadedConfig = saved ? JSON.parse(saved) : {};
    console.log('📂 Loading config for provider:', currentProviderType, loadedConfig);
    setProviderConfig(loadedConfig);
  }, [currentProviderType]);

  // Persist provider-specific config
  useEffect(() => {
    localStorage.setItem(`pypilot_config_${currentProviderType}`, JSON.stringify(providerConfig));
  }, [providerConfig, currentProviderType]);

  // Sync provider with AI service
  useEffect(() => {
    console.log('Syncing provider:', currentProviderType, 'with config:', providerConfig);
    aiService.switchProvider(currentProviderType, providerConfig);
  }, [currentProviderType, providerConfig]);

  // Load available models when provider or config changes
  useEffect(() => {
    const loadModels = async () => {
      console.log('🔄 Loading models for provider:', currentProviderType, 'with config:', {
        hasApiKey: !!providerConfig.apiKey,
        apiKeyLength: providerConfig.apiKey?.length
      });
      
      if (currentProviderType === 'mock') {
        setAvailableModels([]);
        return;
      }

      setIsLoadingModels(true);
      try {
        const models = await aiService.getProviderModels(currentProviderType, providerConfig);
        console.log('✅ Models loaded:', models);
        setAvailableModels(models);
        
        // Set default model if none selected
        if (!providerConfig.model && models.length > 0) {
          const defaultModel = getDefaultModelForProvider(currentProviderType, models);
          console.log('🎯 Setting default model:', defaultModel);
          setProviderConfig(prev => ({ ...prev, model: defaultModel }));
        }
      } catch (error) {
        console.error('❌ Failed to load models:', error);
        logger.error('AI', 'Failed to load models:', error);
        setAvailableModels([]);
      } finally {
        setIsLoadingModels(false);
      }
    };

    loadModels();
  }, [currentProviderType, providerConfig.apiKey]);

  // Helper function to get default model for each provider
  const getDefaultModelForProvider = (type: AiProviderType, models: string[]): string => {
    const defaults: Record<AiProviderType, string[]> = {
      'mock': [],
      'github': ['gpt-4o', 'claude-3-5-sonnet', 'gpt-4o-mini'],
      'openai': ['gpt-4o', 'gpt-4o-mini'],
      'anthropic': ['claude-3-5-sonnet', 'claude-3-haiku'],
      'deepseek': ['deepseek-chat', 'deepseek-coder'],
      'groq': ['llama-3.1-70b-versatile', 'llama-3.1-8b-instant', 'mixtral-8x7b-32768']
    };

    const preferred = defaults[type] || [];
    for (const pref of preferred) {
      if (models.includes(pref)) return pref;
    }
    
    return models[0] || '';
  };

  // Get current editor context
  const getCurrentContext = () => {
    const context: any = { language: 'vpy' };
    
    if (!activeDocument) return context;
    
    const doc = documents.find(d => d.uri === activeDocument);
    if (!doc) return context;

    // Extract filename from URI or diskPath
    const fileName = doc.diskPath ? 
      doc.diskPath.split(/[/\\]/).pop() || doc.uri :
      doc.uri;
    
    context.fileName = fileName;
    
    // Get selected text if any (mock for now)
    // TODO: Integrate with Monaco editor to get real selection
    const selectedCode = ''; // Will be empty until Monaco integration
    if (selectedCode) {
      context.selectedCode = selectedCode;
    }
    
    // Auto-attach document content as context (always enabled)
    if (doc.content) {
      context.documentContent = doc.content;
      context.documentLength = doc.content.length;
    }
    
    // Add manual context if provided
    if (manualContext.trim()) {
      context.manualContext = manualContext.trim();
    }
    
    return context;
  };

  // Get context preview for display
  const getContextPreview = () => {
    const context = getCurrentContext();
    const items = [];
    
    if (context.fileName) {
      items.push(`📄 ${context.fileName}`);
    }
    
    if (context.selectedCode) {
      items.push(`✂️ Selección (${context.selectedCode.length} chars)`);
    }
    
    if (context.manualContext) {
      items.push(`📎 Contexto manual (${context.manualContext.length} chars)`);
    }
    
    return items.length > 0 ? items.join(' • ') : 'Sin contexto';
  };

  // Initialize MCP tools on mount
  useEffect(() => {
    console.log('[PyPilot] Initializing MCP tools...');
    mcpTools.initialize()
      .then(() => {
        console.log('[PyPilot] MCP initialize completed');
        const available = mcpTools.isAvailable();
        console.log('[PyPilot] MCP isAvailable:', available);
        if (available) {
          const tools = mcpTools.getAvailableTools();
          console.log('[PyPilot] Available tools:', tools.length, tools);
          setMcpEnabled(true);
          console.log('[PyPilot] ✅ MCP ENABLED - mcpEnabled state set to true');
        } else {
          console.warn('[PyPilot] ⚠️ MCP not available - mcpEnabled remains false');
        }
      })
      .catch(err => {
        console.error('[PyPilot] ❌ MCP initialization failed:', err);
      });
  }, []);

  // Add system message on first load
  useEffect(() => {
    if (messages.length === 0) {
      const mcpStatus = mcpEnabled ? 
        '\n\n🔧 **MCP Tools Enabled** - Puedo controlar el IDE directamente' : '';
      
      addMessage('system', `🤖 **PyPilot** activado

Soy tu asistente especializado en **Vectrex VPy development**. Puedo ayudarte con:

• 🔧 **Generar código VPy** - Describe lo que quieres crear
• 🐛 **Analizar errores** - Explica problemas en tu código  
• 📚 **Explicar sintaxis** - Aprende comandos VPy/Vectrex
• ⚡ **Optimizar código** - Mejora performance y legibilidad
• 🎮 **Ideas de juegos** - Sugiere mecánicas para Vectrex${mcpEnabled ? '\n• 🎛️ **Controlar IDE** - Abrir/cerrar proyectos, crear archivos, etc.' : ''}

**Comandos rápidos:**
\`/explain\` - Explica el código seleccionado
\`/fix\` - Sugiere fixes para errores
\`/generate\` - Genera código desde descripción
\`/optimize\` - Optimiza código seleccionado
\`/help\` - Ver todos los comandos${mcpStatus}

¿En qué puedo ayudarte hoy?`);
    }
  }, [mcpEnabled]);

  const addMessage = (role: AiMessage['role'], content: string, context?: AiMessage['context']) => {
    const newMessage: AiMessage = {
      id: Date.now().toString(),
      role,
      content,
      timestamp: new Date(),
      context
    };
    
    setMessages(prev => [...prev, newMessage]);
    logger.debug('AI', 'Message added:', { role, contentLength: content.length });
  };

  // Function to parse markdown and render code blocks properly
  const renderMarkdown = (content: string) => {
    const parts = content.split(/(```[\s\S]*?```)/);
    
    return parts.map((part, index) => {
      if (part.startsWith('```') && part.endsWith('```')) {
        // This is a code block
        const codeContent = part.slice(3, -3).trim();
        const lines = codeContent.split('\n');
        const language = lines[0].trim();
        const code = language && !language.includes(' ') && lines.length > 1 
          ? lines.slice(1).join('\n') 
          : codeContent;
        
        return (
          <div key={index} style={{
            margin: '8px 0',
            border: '1px solid #3c4043',
            borderRadius: '6px',
            overflow: 'hidden'
          }}>
            {language && !language.includes(' ') && (
              <div style={{
                backgroundColor: '#2d2d30',
                padding: '4px 8px',
                fontSize: '11px',
                color: '#969696',
                borderBottom: '1px solid #3c4043'
              }}>
                {language}
              </div>
            )}
            <pre style={{
              margin: 0,
              padding: '12px',
              backgroundColor: '#1e1e1e',
              color: '#d4d4d4',
              fontSize: '13px',
              lineHeight: '1.4',
              fontFamily: 'Consolas, Monaco, "Courier New", monospace',
              overflow: 'auto'
            }}>
              <code>{code}</code>
            </pre>
          </div>
        );
      } else {
        // Regular text - process inline code with single backticks
        const textParts = part.split(/(`[^`]+`)/);
        return (
          <span key={index}>
            {textParts.map((textPart, textIndex) => {
              if (textPart.startsWith('`') && textPart.endsWith('`')) {
                return (
                  <code key={textIndex} style={{
                    backgroundColor: '#2d2d30',
                    color: '#e6db74',
                    padding: '2px 4px',
                    borderRadius: '3px',
                    fontSize: '12px',
                    fontFamily: 'Consolas, Monaco, monospace'
                  }}>
                    {textPart.slice(1, -1)}
                  </code>
                );
              }
              return textPart;
            })}
          </span>
        );
      }
    });
  };

  const handleSendMessage = async () => {
    if (!inputValue.trim() || isLoading) return;
    
    const userMessage = inputValue.trim();
    const context = getCurrentContext();
    
    // Add user message
    addMessage('user', userMessage, context);
    setInputValue('');
    setIsLoading(true);

    try {
      // Check if it's a command
      if (userMessage.startsWith('/')) {
        await handleCommand(userMessage, context);
      } else {
        await sendToAI(userMessage, context);
      }
    } catch (error) {
      logger.error('AI', 'Failed to process message:', error);
      addMessage('assistant', `❌ Error: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCommand = async (command: string, context: any) => {
    const [cmd, ...args] = command.split(' ');
    
    switch (cmd) {
      case '/help':
        addMessage('assistant', `📋 **Comandos disponibles:**

• \`/explain\` - Explica código seleccionado
• \`/fix\` - Sugiere solución para errores
• \`/generate [descripción]\` - Genera código VPy
• \`/optimize\` - Optimiza código seleccionado  
• \`/vectrex [comando]\` - Info sobre comandos Vectrex
• \`/assets\` - Guía de uso de .vec y .vmus assets
• \`/examples\` - Ver ejemplos de código
• \`/clear\` - Limpiar conversación
• \`/settings\` - Configurar IA

**Ejemplo de uso:**
\`/generate una pelota que rebote en los bordes\`
\`/explain\` (con código seleccionado)
\`/fix\` (cuando hay errores en el panel)
\`/assets\` (para aprender sobre vectores y música)`);
        break;
        
      case '/clear':
        setMessages([]);
        setTimeout(() => {
          addMessage('system', '🗑️ Conversación limpiada. ¿En qué puedo ayudarte?');
        }, 100);
        break;
        
      case '/settings':
        setShowSettings(true);
        addMessage('assistant', '⚙️ Abriendo configuración de IA...');
        break;
        
      case '/generate':
        const description = args.join(' ');
        if (!description) {
          addMessage('assistant', '❌ Uso: `/generate [descripción]`\n\nEjemplo: `/generate una pelota rebotando`');
          return;
        }
        await generateCode(description, context);
        break;
        
      case '/explain':
        if (!context.selectedCode) {
          addMessage('assistant', '⚠️ Selecciona código en el editor primero, luego usa `/explain`');
          return;
        }
        await explainCode(context.selectedCode, context);
        break;
        
      case '/fix':
        await suggestFix(context);
        break;
        
      case '/optimize':
        if (!context.selectedCode) {
          addMessage('assistant', '⚠️ Selecciona código en el editor primero, luego usa `/optimize`');
          return;
        }
        await optimizeCode(context.selectedCode, context);
        break;
        
      case '/vectrex':
        const vectrexCmd = args.join(' ');
        await getVectrexHelp(vectrexCmd);
        break;
        
      case '/examples':
        showCodeExamples();
        break;
        
      case '/assets':
        showAssetsHelp();
        break;
        
      default:
        addMessage('assistant', `❌ Comando desconocido: \`${cmd}\`\n\nUsa \`/help\` para ver comandos disponibles.`);
    }
  };

  const sendToAI = async (message: string, context: any) => {
    try {
      console.log('[PyPilot] sendToAI called - mcpEnabled:', mcpEnabled);
      
      // Add MCP tools to context if available
      const enhancedContext = mcpEnabled ? {
        ...context,
        errors: [], // TODO: Get real errors from editor
        mcpTools: mcpTools.getToolsContext()
      } : {
        ...context,
        errors: []
      };
      
      console.log('[PyPilot] Enhanced context has mcpTools:', !!enhancedContext.mcpTools);

      const response = await aiService.sendRequest({
        message,
        context: enhancedContext
      });

      addMessage('assistant', response.content);
      
      // Check if response contains MCP tool calls
      if (mcpEnabled) {
        console.log('[PyPilot] Parsing response for MCP tool calls...');
        const toolCalls = mcpTools.parseToolCalls(response.content);
        console.log('[PyPilot] Found', toolCalls.length, 'tool calls:', toolCalls);
        
        if (toolCalls.length > 0) {
          addMessage('system', `⚙️ Ejecutando ${toolCalls.length} herramienta(s) MCP...`);
          
          try {
            const results = await mcpTools.executeToolCalls(toolCalls);
            addMessage('system', results);
          } catch (error) {
            logger.error('AI', 'MCP tool execution error:', error);
            addMessage('system', `❌ Error ejecutando herramientas: ${error instanceof Error ? error.message : 'Unknown error'}`);
          }
        } else {
          console.log('[PyPilot] No tool calls detected in response');
        }
      }
      
      // Handle suggestions if any
      if (response.suggestions?.length) {
        logger.info('AI', 'Received suggestions:', response.suggestions.length);
      }
    } catch (error) {
      logger.error('AI', 'AI service error:', error);
      addMessage('assistant', `❌ Error al comunicar con IA: ${error instanceof Error ? error.message : 'Error desconocido'}`);
    }
  };

  const generateCode = async (description: string, context: any) => {
    try {
      const response = await aiService.sendRequest({
        message: `/generate ${description}`,
        context: {
          ...context,
          errors: []
        },
        command: '/generate'
      });

      addMessage('assistant', response.content);
    } catch (error) {
      logger.error('AI', 'Generate code error:', error);
      addMessage('assistant', `❌ Error generando código: ${error instanceof Error ? error.message : 'Error desconocido'}`);
    }
  };

  const explainCode = async (code: string, context: any) => {
    try {
      const response = await aiService.sendRequest({
        message: '/explain',
        context: {
          ...context,
          selectedCode: code,
          errors: []
        },
        command: '/explain'
      });

      addMessage('assistant', response.content);
    } catch (error) {
      logger.error('AI', 'Explain code error:', error);
      addMessage('assistant', `❌ Error explicando código: ${error instanceof Error ? error.message : 'Error desconocido'}`);
    }
  };

  const suggestFix = async (context: any) => {
    try {
      const response = await aiService.sendRequest({
        message: '/fix',
        context: {
          ...context,
          errors: [] // TODO: Get real errors from editor
        },
        command: '/fix'
      });

      addMessage('assistant', response.content);
    } catch (error) {
      logger.error('AI', 'Suggest fix error:', error);
      addMessage('assistant', `❌ Error sugiriendo correcciones: ${error instanceof Error ? error.message : 'Error desconocido'}`);
    }
  };

  const optimizeCode = async (code: string, context: any) => {
    try {
      const response = await aiService.sendRequest({
        message: '/optimize',
        context: {
          ...context,
          selectedCode: code,
          errors: []
        },
        command: '/optimize'
      });

      addMessage('assistant', response.content);
    } catch (error) {
      logger.error('AI', 'Optimize code error:', error);
      addMessage('assistant', `❌ Error optimizando código: ${error instanceof Error ? error.message : 'Error desconocido'}`);
    }
  };

  const getVectrexHelp = async (command: string) => {
    if (!command) {
      // Show all available commands
      const commands = VECTREX_COMMANDS;
      const commandsList = commands.map((cmd: any) => `• **${cmd.name}** - ${cmd.description}`).join('\n');
      
      addMessage('assistant', `📚 **Comandos Vectrex disponibles:**

${commandsList}

**Uso:** \`/vectrex [comando]\`
**Ejemplo:** \`/vectrex MOVE\`

💡 Los comandos Vectrex usan coordenadas de -127 a +127 con (0,0) en el centro.`);
      return;
    }

    const cmdInfo = VECTREX_COMMANDS.find(cmd => cmd.name.toUpperCase() === command.toUpperCase());
    
    if (cmdInfo) {
      addMessage('assistant', `📚 **Comando Vectrex: ${cmdInfo.name}**

**Sintaxis:** \`${cmdInfo.syntax}\`

**Descripción:** ${cmdInfo.description}

**Ejemplo:**
\`\`\`vpy
${cmdInfo.example}
\`\`\`

**Categoría:** ${cmdInfo.category}

💡 **Tip:** Los comandos de dibujo del Vectrex usan coordenadas relativas al centro de la pantalla (0,0).`);
    } else {
      const commands = VECTREX_COMMANDS;
      const commandNames = commands.map((cmd: any) => cmd.name).join(', ');
      
      addMessage('assistant', `❓ Comando "${command.toUpperCase()}" no encontrado.

**Comandos disponibles:**
${commandNames}

**Uso:** \`/vectrex [comando]\`
**Ejemplo:** \`/vectrex MOVE\``);
    }
  };

  const showCodeExamples = () => {
    addMessage('assistant', `📝 **Ejemplos de código VPy:**

**1. Hola Mundo básico:**
\`\`\`vpy
def main():
    INTENSITY(255)
    PRINT_TEXT(-50, 0, "Hello Vectrex!")
\`\`\`

**2. Formas básicas:**
\`\`\`vpy
def main():
    INTENSITY(200)
    MOVE(-50, -50)
    RECT(0, 0, 100, 100)
    DRAW_CIRCLE(30)
\`\`\`

**3. Animación simple:**
\`\`\`vpy
var x = 0

def main():
    x = x + 1
    if x > 100:
        x = -100
    
    INTENSITY(255)
    MOVE(x, 0)
    DRAW_CIRCLE(10)
\`\`\`

¿Quieres ver ejemplos de algo específico?`);
  };

  const showAssetsHelp = () => {
    addMessage('assistant', `🎨 **Usando Assets en VPy**

## 📁 Estructura de Proyecto

\`\`\`
proyecto/
├── src/
│   └── main.vpy          # Tu código
├── assets/
│   ├── vectors/          # Gráficos 3D (.vec)
│   ├── music/            # Música (.vmus)
│   ├── sfx/              # Efectos de sonido (.vmus)
│   ├── voices/           # Samples de voz
│   └── animations/       # Animaciones
└── build/                # ROMs compiladas
\`\`\`

## 🎨 Editor de Vectores 3D (.vec)

**Crear gráficos:**
1. Abre/crea archivo .vec en el IDE
2. Usa el ViewCube para navegar en 3D (estilo Fusion 360)
3. Dibuja vectores en el espacio 3D (-127 a +127)
4. Guarda y referencia en VPy

**Usar en código:**
\`\`\`vpy
def setup():
    # Cargar un gráfico vectorial
    nave = load_vec("assets/vectors/spaceship.vec")

def loop():
    INTENSITY(255)
    # Dibujar el gráfico en posición x=0, y=0
    draw_vec(nave, 0, 0, scale=1.0)
\`\`\`

## 🎵 Editor de Música (.vmus)

**Componer música:**
1. Crea archivo .vmus en el IDE
2. Piano roll: 3 canales cuadrados + 1 ruido
3. Coloca notas con el ratón
4. Preview en tiempo real con el PSG emulador
5. Exporta para tu juego

**Usar en código:**
\`\`\`vpy
def setup():
    # Cargar y reproducir música de fondo
    tema = load_music("assets/music/theme.vmus")
    play_music(tema, loop=True)
    
    # Cargar efectos de sonido
    explosion = load_sfx("assets/sfx/boom.vmus")

def on_collision():
    # Reproducir efecto
    play_sfx(explosion)
\`\`\`

## 🎮 Ejemplo Completo

\`\`\`vpy
# Cargar assets en setup
def setup():
    nave = load_vec("assets/vectors/player.vec")
    enemigo = load_vec("assets/vectors/enemy.vec")
    musica = load_music("assets/music/game.vmus")
    disparo = load_sfx("assets/sfx/shoot.vmus")
    
    play_music(musica, loop=True)

def loop():
    INTENSITY(255)
    
    # Dibujar nave del jugador
    draw_vec(nave, player_x, player_y, scale=1.0)
    
    # Dibujar enemigos
    draw_vec(enemigo, enemy_x, enemy_y, scale=0.8)
    
    if button_pressed(1):
        play_sfx(disparo)
\`\`\`

**Comandos relacionados:**
• \`/examples\` - Ver más ejemplos de código
• \`/vectrex\` - Info sobre hardware Vectrex
• \`/generate\` - Generar código con assets

💡 **Tip:** El editor 3D tiene ViewCube para rotar la cámara - click en caras/bordes para vistas preestablecidas!`);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  return (
    <div style={{ 
      height: '100%', 
      display: 'flex', 
      flexDirection: 'column',
      background: '#1e1e1e',
      color: '#cccccc'
    }}>
      {/* Header */}
      <div style={{ 
        padding: '12px 16px', 
        borderBottom: '1px solid #3c3c3c',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between'
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <span style={{ fontSize: '16px' }}>🤖</span>
          <span style={{ fontWeight: '600' }}>PyPilot</span>
          <span style={{ 
            fontSize: '11px', 
            background: aiService.isConfigured() ? '#10b981' : '#6b7280',
            color: 'white',
            padding: '2px 6px',
            borderRadius: '10px'
          }}>
            {aiService.isConfigured() ? aiService.getCurrentProvider()?.name : 'Mock'}
          </span>
        </div>
        
        <button
          onClick={() => setShowSettings(!showSettings)}
          style={{
            background: 'transparent',
            border: '1px solid #3c3c3c',
            color: '#cccccc',
            padding: '4px 8px',
            borderRadius: '4px',
            cursor: 'pointer',
            fontSize: '12px'
          }}
        >
          ⚙️ Config
        </button>
      </div>

      {/* Settings Panel */}
      {showSettings && (
        <div style={{ 
          padding: '12px', 
          borderBottom: '1px solid #3c3c3c',
          background: '#252526',
          maxWidth: '100%',
          boxSizing: 'border-box',
          overflow: 'hidden'
        }}>
          <div style={{ marginBottom: '12px', fontWeight: '600' }}>⚙️ Configuración IA</div>
          
          <div style={{ 
            display: 'flex', 
            flexDirection: 'column', 
            gap: '8px',
            maxWidth: '100%',
            overflow: 'hidden'
          }}>
            <div>
              <label style={{ display: 'block', fontSize: '12px', marginBottom: '4px' }}>
              Proveedor:
            </label>
            <select 
              value={currentProviderType}
              onChange={(e) => {
                const newProvider = e.target.value as AiProviderType;
                console.log('Provider changed from', currentProviderType, 'to', newProvider);
                console.log('Current config before change:', providerConfig);
                setCurrentProviderType(newProvider);
              }}
              style={{
                background: '#1e1e1e',
                border: '1px solid #3c3c3c',
                color: '#cccccc',
                padding: '4px 8px',
                borderRadius: '4px',
                width: '100%',
                maxWidth: '100%',
                boxSizing: 'border-box',
                fontSize: '12px'
              }}
            >
              <option value="ollama">🏠 Ollama (Local - Privado)</option>
              <option value="mock">Mock (Testing)</option>
              <option value="deepseek">DeepSeek (Free)</option>
              <option value="groq">Groq (Free & Fast)</option>
              <option value="github">GitHub Models (Copilot)</option>
              <option value="openai">OpenAI GPT</option>
              <option value="anthropic">Anthropic Claude</option>
            </select>
          </div>

          {currentProviderType === 'ollama' && (
            <div style={{ 
              marginBottom: '12px',
              padding: '8px',
              background: '#2d2d30',
              border: '1px solid #3c3c3c',
              borderRadius: '4px',
              fontSize: '11px',
              lineHeight: '1.5'
            }}>
              <div style={{ marginBottom: '4px', fontWeight: 'bold', color: '#4ec9b0' }}>
                🏠 Modelo Local (Ollama)
              </div>
              <div style={{ color: '#cccccc' }}>
                • No requiere API key<br/>
                • 100% privado (corre en tu Mac)<br/>
                • Sin límites de uso<br/>
                • Requiere Ollama instalado y corriendo
              </div>
              <div style={{ marginTop: '6px', fontSize: '10px', color: '#858585' }}>
                Para instalar: <code style={{ background: '#1e1e1e', padding: '2px 4px' }}>brew install ollama</code>
              </div>
            </div>
          )}
          
          {currentProviderType !== 'mock' && currentProviderType !== 'ollama' && (
            <>
              <div style={{ marginBottom: '8px' }}>
                <label style={{ display: 'block', fontSize: '12px', marginBottom: '4px' }}>
                  API Key:
                </label>
                <input
                  type="password"
                  value={providerConfig.apiKey || ''}
                  onChange={(e) => {
                    const newApiKey = e.target.value;
                    console.log('🔑 API Key input changed:', {
                      newValue: newApiKey.substring(0, 10) + '...',
                      length: newApiKey.length,
                      currentProvider: currentProviderType
                    });
                    
                    setProviderConfig(prev => {
                      const newConfig = { ...prev, apiKey: newApiKey };
                      console.log('🔄 Setting new config:', {
                        ...newConfig,
                        apiKey: newConfig.apiKey?.substring(0, 10) + '...'
                      });
                      return newConfig;
                    });
                  }}
                  placeholder={currentProviderType === 'groq' ? 'gsk_...' : currentProviderType === 'github' ? 'github_pat_...' : 'sk-...'}
                  style={{
                    background: '#1e1e1e',
                    border: '1px solid #3c3c3c',
                    color: '#cccccc',
                    padding: '6px 8px',
                    borderRadius: '4px',
                    width: '100%',
                    maxWidth: '100%',
                    fontSize: '12px',
                    fontFamily: 'monospace',
                    boxSizing: 'border-box',
                    overflow: 'hidden',
                    textOverflow: 'ellipsis'
                  }}
                  onFocus={(e) => {
                    console.log('🔑 API Key field focused');
                    e.stopPropagation();
                  }}
                  onClick={(e) => {
                    console.log('🔑 API Key field clicked');
                    e.stopPropagation();
                    e.currentTarget.focus();
                  }}
                />
              </div>
              
              {availableModels.length > 0 && (
                <div style={{ marginBottom: '8px' }}>
                  <label style={{ display: 'block', fontSize: '12px', marginBottom: '4px' }}>
                    Modelo:
                    {isLoadingModels && <span style={{ color: '#6b7280', marginLeft: '8px' }}>Cargando...</span>}
                  </label>
                  <div style={{ display: 'flex', gap: '8px', alignItems: 'stretch' }}>
                    <select
                      value={providerConfig.model || ''}
                      onChange={(e) => setProviderConfig(prev => ({ ...prev, model: e.target.value }))}
                      disabled={isLoadingModels}
                      style={{
                        background: '#1e1e1e',
                        border: '1px solid #3c3c3c',
                        color: '#cccccc',
                        padding: '4px 8px',
                        borderRadius: '4px',
                        flex: 1,
                        opacity: isLoadingModels ? 0.6 : 1
                      }}
                    >
                      <option value="">Seleccionar modelo...</option>
                      {availableModels.map(model => (
                        <option key={model} value={model}>
                          {model}
                          {model.includes('gpt-5') && ' ⭐ (Nuevo)'}
                          {model.includes('claude-4') && ' ⭐ (Nuevo)'}
                          {model.includes('gpt-4o') && !model.includes('mini') && ' 🚀 (Recomendado)'}
                          {model.includes('mini') && ' ⚡ (Rápido)'}
                          {model.includes('free') && ' 🆓 (Gratis)'}
                        </option>
                      ))}
                    </select>
                    
                    {currentProviderType === 'ollama' && (
                      <button
                        onClick={() => setShowOllamaManager(true)}
                        style={{
                          background: '#374151',
                          border: '1px solid #4b5563',
                          color: '#e5e7eb',
                          padding: '4px 12px',
                          borderRadius: '4px',
                          cursor: 'pointer',
                          fontSize: '12px',
                          whiteSpace: 'nowrap'
                        }}
                        title="Manage Ollama models"
                      >
                        🏠 Manage
                      </button>
                    )}
                  </div>
                  
                  {providerConfig.model && (
                    <div style={{ 
                      fontSize: '10px', 
                      color: '#6b7280', 
                      marginTop: '4px',
                      fontStyle: 'italic'
                    }}>
                      Modelo seleccionado: {providerConfig.model}
                    </div>
                  )}
                  
                  <button
                    onClick={async () => {
                      setIsLoadingModels(true);
                      try {
                        const models = await aiService.getProviderModels(currentProviderType, providerConfig);
                        setAvailableModels(models);
                      } catch (error) {
                        logger.error('AI', 'Failed to reload models:', error);
                      } finally {
                        setIsLoadingModels(false);
                      }
                    }}
                    disabled={isLoadingModels || !providerConfig.apiKey}
                    style={{
                      background: 'transparent',
                      border: '1px solid #3c3c3c',
                      color: '#cccccc',
                      padding: '2px 8px',
                      borderRadius: '3px',
                      cursor: 'pointer',
                      fontSize: '10px',
                      marginTop: '4px',
                      opacity: isLoadingModels || !providerConfig.apiKey ? 0.5 : 1
                    }}
                  >
                    🔄 {isLoadingModels ? 'Cargando...' : 'Recargar modelos'}
                  </button>
                </div>
              )}
            </>
          )}
          
          <div style={{ display: 'flex', gap: '8px', marginTop: '12px' }}>
            <button
              onClick={async () => {
                // Test connection for non-mock providers
                if (currentProviderType !== 'mock') {
                  console.log('Testing connection for provider:', currentProviderType);
                  console.log('Provider config:', {
                    hasApiKey: !!providerConfig.apiKey,
                    apiKeyStart: providerConfig.apiKey?.substring(0, 10) + '...',
                    model: providerConfig.model,
                    endpoint: providerConfig.endpoint
                  });
                  
                  try {
                    const isConnected = await aiService.testProviderConnection(currentProviderType, providerConfig);
                    console.log('Connection test result:', isConnected);
                    
                    if (!isConnected) {
                      alert(`❌ Failed to connect to ${currentProviderType}.

Possible issues:
• Check your API key is correct
• Verify you have access to the service
• For GitHub Models: may be in limited beta
• For Groq: get free API key at console.groq.com
• Try a different provider (Groq/DeepSeek are free)

Check browser console for detailed error messages.`);
                      return;
                    } else {
                      alert(`✅ Successfully connected to ${currentProviderType}!`);
                    }
                  } catch (error) {
                    console.error('Connection test error:', error);
                    alert(`❌ Error testing connection: ${error}`);
                    return;
                  }
                }
                setShowSettings(false);
                logger.info('AI', 'AI provider configured:', currentProviderType);
              }}
              style={{
                background: '#0e639c',
                border: 'none',
                color: 'white',
                padding: '6px 12px',
                borderRadius: '4px',
                cursor: 'pointer',
                fontSize: '12px'
              }}
            >
              Guardar
            </button>
            <button
              onClick={() => setShowSettings(false)}
              style={{
                background: 'transparent',
                border: '1px solid #3c3c3c',
                color: '#cccccc',
                padding: '6px 12px',
                borderRadius: '4px',
                cursor: 'pointer',
                fontSize: '12px'
              }}
            >
              Cancelar
            </button>
          </div>
          </div>
        </div>
      )}

      {/* Messages */}
      <div style={{ 
        flex: 1, 
        overflowY: 'auto', 
        padding: '16px',
        display: 'flex',
        flexDirection: 'column',
        gap: '12px'
      }}>
        {messages.map((message) => (
          <div
            key={message.id}
            style={{
              display: 'flex',
              flexDirection: message.role === 'user' ? 'row-reverse' : 'row',
              gap: '8px',
              alignItems: 'flex-start'
            }}
          >
            <div style={{
              width: '32px',
              height: '32px',
              borderRadius: '16px',
              background: message.role === 'user' ? '#0e639c' : 
                         message.role === 'system' ? '#10b981' : '#6b7280',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              fontSize: '14px',
              flexShrink: 0
            }}>
              {message.role === 'user' ? '👤' : message.role === 'system' ? '🤖' : '🤖'}
            </div>
            
            <div style={{
              background: message.role === 'user' ? '#0e639c20' : 
                         message.role === 'system' ? '#10b98120' : '#3c3c3c',
              padding: '8px 12px',
              borderRadius: '8px',
              maxWidth: '80%',
              fontSize: '13px',
              lineHeight: '1.4'
            }}>
              <div style={{ 
                whiteSpace: 'pre-wrap'
              }}>
                {renderMarkdown(message.content)}
              </div>
              
              {message.context && (
                <div style={{
                  fontSize: '11px',
                  color: '#969696',
                  marginTop: '6px',
                  fontStyle: 'italic'
                }}>
                  📁 {message.context.fileName}
                </div>
              )}
              
              <div style={{
                fontSize: '10px',
                color: '#6b7280',
                marginTop: '4px'
              }}>
                {message.timestamp.toLocaleTimeString()}
              </div>
            </div>
          </div>
        ))}
        
        {isLoading && (
          <div style={{
            display: 'flex',
            gap: '8px',
            alignItems: 'center',
            color: '#6b7280'
          }}>
            <div style={{
              width: '32px',
              height: '32px',
              borderRadius: '16px',
              background: '#6b7280',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center'
            }}>
              🤖
            </div>
            <div style={{
              background: '#3c3c3c',
              padding: '8px 12px',
              borderRadius: '8px',
              fontSize: '13px'
            }}>
              <span>🤔 Pensando...</span>
            </div>
          </div>
        )}
        
        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div style={{ 
        padding: '16px', 
        borderTop: '1px solid #3c3c3c',
        background: '#252526'
      }}>
        {/* Context indicator */}
        <div style={{
          fontSize: '11px',
          color: '#6b7280',
          marginBottom: '8px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between'
        }}>
          <span>📎 Contexto: {getContextPreview()}</span>
          <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
            <button
              onClick={() => {
                const context = prompt('Añadir contexto manual:', manualContext);
                if (context !== null) setManualContext(context);
              }}
              style={{
                background: 'transparent',
                border: '1px solid #3c3c3c',
                color: '#cccccc',
                padding: '2px 6px',
                borderRadius: '3px',
                fontSize: '10px',
                cursor: 'pointer'
              }}
            >
              📎 Adjuntar
            </button>
          </div>
        </div>
        
        <div style={{ display: 'flex', gap: '8px', alignItems: 'flex-end' }}>
          <textarea
            ref={inputRef}
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Pregunta algo o usa /help para ver comandos..."
            style={{
              flex: 1,
              background: '#1e1e1e',
              border: '1px solid #3c3c3c',
              color: '#cccccc',
              padding: '12px 16px',
              borderRadius: '8px',
              resize: 'vertical',
              minHeight: '60px',
              maxHeight: '200px',
              fontSize: '14px',
              lineHeight: '1.5',
              fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Monaco, Inconsolata, "Roboto Mono", monospace'
            }}
            rows={3}
          />
          
          <button
            onClick={handleSendMessage}
            disabled={!inputValue.trim() || isLoading}
            style={{
              background: inputValue.trim() && !isLoading ? '#0e639c' : '#3c3c3c',
              border: 'none',
              color: 'white',
              padding: '12px 16px',
              borderRadius: '8px',
              cursor: inputValue.trim() && !isLoading ? 'pointer' : 'not-allowed',
              fontSize: '14px',
              height: '60px',
              minWidth: '60px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center'
            }}
          >
            {isLoading ? '⏳' : '📤'}
          </button>
        </div>
        
        <div style={{
          fontSize: '11px',
          color: '#6b7280',
          marginTop: '8px'
        }}>
        </div>
      </div>
      
      {/* Ollama Manager Dialog */}
      <OllamaManagerDialog
        isOpen={showOllamaManager}
        onClose={() => setShowOllamaManager(false)}
        onModelSelected={(modelName) => {
          setProviderConfig(prev => ({ ...prev, model: modelName }));
        }}
        currentModel={providerConfig.model}
      />
    </div>
  );
};