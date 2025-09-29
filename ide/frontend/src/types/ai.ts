// Types para el sistema de IA integrado

export interface AiMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: Date;
  context?: {
    fileName?: string;
    selectedCode?: string;
    errorContext?: string;
    lineNumber?: number;
    columnNumber?: number;
  };
}

export interface AiProvider {
  name: 'OpenAI' | 'Anthropic' | 'Local' | 'Mock';
  endpoint?: string;
  apiKey?: string;
  enabled: boolean;
  model?: string;
}

export interface AiRequest {
  message: string;
  context: {
    fileName?: string;
    selectedCode?: string;
    language: string;
    errors?: Array<{
      line: number;
      column: number;
      message: string;
      severity: string;
    }>;
    projectFiles?: string[];
  };
  command?: string;
}

export interface AiResponse {
  content: string;
  suggestions?: Array<{
    type: 'code' | 'fix' | 'optimization' | 'explanation';
    title: string;
    code?: string;
    description?: string;
  }>;
  error?: string;
}

// Comandos disponibles en el AI Assistant
export type AiCommand = 
  | '/help'
  | '/explain'
  | '/fix'
  | '/generate'
  | '/optimize'
  | '/vectrex'
  | '/examples'
  | '/clear'
  | '/settings';

export interface VectrexCommandInfo {
  name: string;
  syntax: string;
  description: string;
  example: string;
  category: 'movement' | 'drawing' | 'text' | 'intensity' | 'control';
}

// Contexto especializado para VPy/Vectrex
export interface VpyContext {
  currentFile?: string;
  selectedCode?: string;
  errors?: Array<{
    line: number;
    message: string;
    type: 'syntax' | 'semantic' | 'runtime';
  }>;
  availableCommands: VectrexCommandInfo[];
  compilationOutput?: string;
}