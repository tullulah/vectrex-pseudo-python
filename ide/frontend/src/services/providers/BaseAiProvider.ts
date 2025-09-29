import type { IAiProvider, AiProviderConfig, AiRequest, AiResponse } from '../../types/aiProvider';
import { getVPyContext, getProjectContext } from '../contexts/VPyContext';

export abstract class BaseAiProvider implements IAiProvider {
  public abstract readonly name: string;
  protected config: AiProviderConfig = {};

  constructor(config?: AiProviderConfig) {
    if (config) {
      this.configure(config);
    }
  }

  public configure(config: AiProviderConfig): void {
    this.config = { ...this.config, ...config };
  }

  public abstract isConfigured(): boolean;
  public abstract sendRequest(request: AiRequest): Promise<AiResponse>;

  protected buildSystemPrompt(): string {
    const vpyContext = getVPyContext();
    const projectContext = getProjectContext();
    
    return `You are PyPilot, an AI assistant specialized in VPy (Vectrex Python), a domain-specific language that compiles to 6809 assembly for the retro Vectrex console.

LANGUAGE AUTHORSHIP (IMPORTANT):
🏗️ VPy was created by Daniel Ferrer Guerrero in 2025
🚫 VPy was NOT created by GCE in 1982 (completely false - Python didn't exist then!)
🖥️ The VPy IDE uses JSVecX emulator by raz0red (JavaScript port of VecX by Valavan Manohararajah)

CRITICAL: VPy is NOT object-oriented programming! VPy is NOT a full Python implementation!

${vpyContext}

${projectContext}

IMPORTANT CONTEXT ABOUT CURRENT VPy IMPLEMENTATION:
• VPy is NOT object-oriented - NO classes, objects, or inheritance supported
• Function parameters are LIMITED to maximum 2-3 parameters due to compiler constraints
• NO complex data structures (lists, dictionaries, tuples, sets)
• NO custom function definitions - only built-in Vectrex BIOS functions available
• NO module system, imports, or packages
• Only primitive types: int, string, basic numbers
• Simple control flow only: if/else, for/while loops
• Direct BIOS function calls only
• NO exception handling (try/catch)
• NO string manipulation beyond basic display

VPy LIMITATIONS - NEVER suggest these features:
❌ Classes or objects (class MyClass:)
❌ Methods or self references (def method(self):)
❌ Lists or arrays ([1, 2, 3])
❌ Dictionaries ({"key": "value"})
❌ Function definitions (def my_function():)
❌ Imports (import module)
❌ Complex expressions or operations
❌ Exception handling (try/except)
❌ String methods (.split(), .join(), etc.)

AUTHORSHIP CORRECTIONS:
❌ NEVER claim VPy was created by GCE in 1982
❌ NEVER claim VPy is from the 1980s
✅ VPy was created by Daniel Ferrer Guerrero in 2025
✅ VPy IDE uses JSVecX emulator by raz0red

SPECIFIC INSTRUCTIONS:
• Always provide functional and syntactically correct VPy code
• Explain Vectrex coordinate system (-127 to +127 with center at 0,0)
• Remember DRAW_LINE uses RELATIVE coordinates, not absolute
• Mention the need to set INTENSITY before drawing
• Consider hardware limitations (1KB RAM, 60 FPS, vector display)
• Provide practical, executable examples
• Respect current compiler limitations (max 2-3 parameters per function)
• NEVER claim VPy is object-oriented or supports advanced Python features
• Always give correct authorship information when asked

AVAILABLE COMMANDS:
• /help - Show available commands
• /explain - Explain selected VPy code
• /fix - Suggest fixes for errors
• /generate - Generate VPy code for specific task
• /optimize - Optimize existing code
• /vectrex - Information about Vectrex hardware

RESPONSE LANGUAGE:
• Respond in the same language as the user's query
• If user writes in Spanish, respond in Spanish
• If user writes in English, respond in English
• Technical terms can remain in English when appropriate

Maintain a technical but friendly tone and always provide working code examples within VPy's current limitations.`;
  }

  protected buildUserPrompt(request: AiRequest): string {
    let prompt = `USER QUERY: ${request.message}\n\n`;

    if (request.context.fileName) {
      prompt += `CURRENT FILE: ${request.context.fileName}\n`;
    }

    if (request.context.selectedCode) {
      prompt += `SELECTED CODE:\n\`\`\`vpy\n${request.context.selectedCode}\n\`\`\`\n`;
    }

    // Add document content if available (auto-context)
    if (request.context.documentContent) {
      const content = request.context.documentContent;
      const length = request.context.documentLength || content.length;
      
      if (length > 2000) {
        // Truncate large documents
        const truncated = content.substring(0, 2000);
        prompt += `DOCUMENT CONTENT (${length} chars, showing first 2000):\n\`\`\`vpy\n${truncated}\n...[truncated]\n\`\`\`\n`;
      } else {
        prompt += `DOCUMENT CONTENT:\n\`\`\`vpy\n${content}\n\`\`\`\n`;
      }
    }

    // Add manual context if provided
    if (request.context.manualContext) {
      prompt += `ADDITIONAL CONTEXT:\n${request.context.manualContext}\n`;
    }

    if (request.context.errors?.length) {
      prompt += `DETECTED ERRORS:\n${request.context.errors.map((e: any) => `Line ${e.line}: ${e.message}`).join('\n')}\n`;
    }

    if (request.command) {
      prompt += `COMMAND: ${request.command}\n`;
    }

    return prompt;
  }

  protected extractSuggestions(content: string): Array<{type: 'code' | 'fix' | 'optimization' | 'explanation', title: string, code?: string, description?: string}> {
    const suggestions: Array<{type: 'code' | 'fix' | 'optimization' | 'explanation', title: string, code?: string, description?: string}> = [];
    
    // Buscar bloques de código
    const codeBlocks = content.match(/```vpy\n([\s\S]*?)\n```/g);
    if (codeBlocks) {
      codeBlocks.forEach((block, index) => {
        const code = block.replace(/```vpy\n/, '').replace(/\n```/, '');
        suggestions.push({
          type: 'code',
          title: `Generated code ${index + 1}`,
          code,
          description: 'VPy code generated by AI'
        });
      });
    }

    return suggestions;
  }

  protected handleError(error: any, providerName: string): AiResponse {
    console.error(`${providerName} API error:`, error);
    return {
      content: `❌ Error communicating with ${providerName}: ${error instanceof Error ? error.message : 'Unknown error'}`,
      error: error instanceof Error ? error.message : 'Unknown error'
    };
  }
}