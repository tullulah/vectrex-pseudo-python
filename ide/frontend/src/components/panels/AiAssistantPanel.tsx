import React, { useState, useRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useEditorStore } from '../../state/editorStore';
import type { AiMessage, AiProvider, AiCommand, VectrexCommandInfo } from '../../types/ai';
import { aiService } from '../../services/aiService';
import { logger } from '../../utils/logger';

export const AiAssistantPanel: React.FC = () => {
  const { t } = useTranslation();
  const [messages, setMessages] = useState<AiMessage[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [aiProvider, setAiProvider] = useState<AiProvider>({
    name: 'Mock',
    enabled: false,
    apiKey: ''
  });
  const [showSettings, setShowSettings] = useState(false);
  
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const activeDocument = useEditorStore(s => s.active);
  const documents = useEditorStore(s => s.documents);

  // Auto-scroll to bottom when new messages arrive
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Sync provider with AI service
  useEffect(() => {
    aiService.setProvider(aiProvider);
  }, [aiProvider]);

  // Get current editor context
  const getCurrentContext = () => {
    if (!activeDocument) return {};
    
    const doc = documents.find(d => d.uri === activeDocument);
    if (!doc) return {};

    // Get selected text if any (mock for now)
    const selectedCode = ''; // TODO: Get from Monaco editor selection
    
    // Extract filename from URI or diskPath
    const fileName = doc.diskPath ? 
      doc.diskPath.split(/[/\\]/).pop() || doc.uri :
      doc.uri;
    
    return {
      fileName,
      selectedCode,
      language: 'vpy'
    };
  };

  // Add system message on first load
  useEffect(() => {
    if (messages.length === 0) {
      addMessage('system', `🤖 **VPy AI Assistant** activado

Soy tu asistente especializado en **Vectrex VPy development**. Puedo ayudarte con:

• 🔧 **Generar código VPy** - Describe lo que quieres crear
• 🐛 **Analizar errores** - Explica problemas en tu código  
• 📚 **Explicar sintaxis** - Aprende comandos VPy/Vectrex
• ⚡ **Optimizar código** - Mejora performance y legibilidad
• 🎮 **Ideas de juegos** - Sugiere mecánicas para Vectrex

**Comandos rápidos:**
\`/explain\` - Explica el código seleccionado
\`/fix\` - Sugiere fixes para errores
\`/generate\` - Genera código desde descripción
\`/optimize\` - Optimiza código seleccionado
\`/help\` - Ver todos los comandos

¿En qué puedo ayudarte hoy?`);
    }
  }, []);

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
• \`/examples\` - Ver ejemplos de código
• \`/clear\` - Limpiar conversación
• \`/settings\` - Configurar IA

**Ejemplo de uso:**
\`/generate una pelota que rebote en los bordes\`
\`/explain\` (con código seleccionado)
\`/fix\` (cuando hay errores en el panel)`);
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
        
      default:
        addMessage('assistant', `❌ Comando desconocido: \`${cmd}\`\n\nUsa \`/help\` para ver comandos disponibles.`);
    }
  };

  const sendToAI = async (message: string, context: any) => {
    try {
      const response = await aiService.sendRequest({
        message,
        context: {
          ...context,
          errors: [] // TODO: Get real errors from editor
        }
      });

      addMessage('assistant', response.content);
      
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
      const commands = aiService.getVectrexCommands();
      const commandsList = commands.map(cmd => `• **${cmd.name}** - ${cmd.description}`).join('\n');
      
      addMessage('assistant', `📚 **Comandos Vectrex disponibles:**

${commandsList}

**Uso:** \`/vectrex [comando]\`
**Ejemplo:** \`/vectrex MOVE\`

💡 Los comandos Vectrex usan coordenadas de -127 a +127 con (0,0) en el centro.`);
      return;
    }

    const cmdInfo = aiService.getVectrexCommand(command);
    
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
      const commands = aiService.getVectrexCommands();
      const commandNames = commands.map(cmd => cmd.name).join(', ');
      
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
          <span style={{ fontWeight: '600' }}>VPy AI Assistant</span>
          <span style={{ 
            fontSize: '11px', 
            background: aiProvider.enabled ? '#10b981' : '#6b7280',
            color: 'white',
            padding: '2px 6px',
            borderRadius: '10px'
          }}>
            {aiProvider.enabled ? 'Activo' : 'Mock'}
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
          padding: '16px', 
          borderBottom: '1px solid #3c3c3c',
          background: '#252526'
        }}>
          <div style={{ marginBottom: '12px', fontWeight: '600' }}>⚙️ Configuración IA</div>
          
          <div style={{ marginBottom: '8px' }}>
            <label style={{ display: 'block', fontSize: '12px', marginBottom: '4px' }}>
              Proveedor:
            </label>
            <select 
              value={aiProvider.name}
              onChange={(e) => setAiProvider(prev => ({ 
                ...prev, 
                name: e.target.value as AiProvider['name']
              }))}
              style={{
                background: '#1e1e1e',
                border: '1px solid #3c3c3c',
                color: '#cccccc',
                padding: '4px 8px',
                borderRadius: '4px',
                width: '100%'
              }}
            >
              <option value="OpenAI">OpenAI GPT</option>
              <option value="Anthropic">Anthropic Claude</option>
              <option value="Local">Local LLM</option>
              <option value="Mock">Mock (Testing)</option>
            </select>
          </div>
          
          <div style={{ marginBottom: '8px' }}>
            <label style={{ display: 'block', fontSize: '12px', marginBottom: '4px' }}>
              API Key:
            </label>
            <input
              type="password"
              value={aiProvider.apiKey}
              onChange={(e) => setAiProvider(prev => ({ ...prev, apiKey: e.target.value }))}
              placeholder="sk-..."
              style={{
                background: '#1e1e1e',
                border: '1px solid #3c3c3c',
                color: '#cccccc',
                padding: '4px 8px',
                borderRadius: '4px',
                width: '100%'
              }}
            />
          </div>
          
          <div style={{ display: 'flex', gap: '8px', marginTop: '12px' }}>
            <button
              onClick={() => {
                setAiProvider(prev => ({ 
                  ...prev, 
                  enabled: (prev.apiKey || '').length > 0 
                }));
                setShowSettings(false);
                logger.info('AI', 'AI provider configured:', aiProvider.name);
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
                whiteSpace: 'pre-wrap',
                fontFamily: message.content.includes('```') ? 'Consolas, monospace' : 'inherit'
              }}>
                {message.content}
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
              padding: '8px 12px',
              borderRadius: '6px',
              resize: 'none',
              minHeight: '20px',
              maxHeight: '100px',
              fontSize: '13px',
              lineHeight: '1.4'
            }}
            rows={1}
          />
          
          <button
            onClick={handleSendMessage}
            disabled={!inputValue.trim() || isLoading}
            style={{
              background: inputValue.trim() && !isLoading ? '#0e639c' : '#3c3c3c',
              border: 'none',
              color: 'white',
              padding: '8px 12px',
              borderRadius: '6px',
              cursor: inputValue.trim() && !isLoading ? 'pointer' : 'not-allowed',
              fontSize: '12px',
              height: '36px'
            }}
          >
            {isLoading ? '⏳' : '📤'}
          </button>
        </div>
        
        <div style={{
          fontSize: '11px',
          color: '#6b7280',
          marginTop: '6px'
        }}>
          Enter para enviar • Shift+Enter para nueva línea • /help para comandos
        </div>
      </div>
    </div>
  );
};