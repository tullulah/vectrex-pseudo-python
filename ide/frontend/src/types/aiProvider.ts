// Interfaces and types for AI providers
export interface AiProvider {
  name: string;
  apiKey?: string;
  endpoint?: string;
  model?: string;
  enabled: boolean;
}

export interface AiRequest {
  message: string;
  context: {
    fileName?: string;
    selectedCode?: string;
    documentContent?: string;
    documentLength?: number;
    manualContext?: string;
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
  availableTools?: Array<{
    name: string;
    description: string;
    inputSchema?: any;
  }>;
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
  usage?: {
    promptTokens: number;
    completionTokens: number;
    totalTokens: number;
  };
}

// Base interface for all AI providers
export interface IAiProvider {
  name: string;
  isConfigured(): boolean;
  configure(config: AiProviderConfig): void;
  sendRequest(request: AiRequest): Promise<AiResponse>;
  getModels?(): Promise<string[]>;
  testConnection?(): Promise<boolean>;
}

export interface AiProviderConfig {
  apiKey?: string;
  endpoint?: string;
  model?: string;
  temperature?: number;
  maxTokens?: number;
}

// Provider types
export type AiProviderType = 'openai' | 'anthropic' | 'deepseek' | 'github' | 'groq' | 'ollama' | 'mock';

export interface VectrexCommandInfo {
  name: string;
  syntax: string;
  description: string;
  example: string;
  category: 'movement' | 'drawing' | 'text' | 'intensity' | 'control';
}

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