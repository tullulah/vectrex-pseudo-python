/// <reference types="vite/client" />
// Allow importing Monaco worker bundles with ?worker query
declare module 'monaco-editor/esm/vs/editor/editor.worker?worker' {
  const WorkerFactory: { new(): Worker };
  export default WorkerFactory;
}

// Electron API types
interface Window {
  electron: {
    runCommand: (command: string) => Promise<{
      success: boolean;
      output: string;
      exitCode: number;
    }>;
  };
  aiProxy: {
    request: (request: {
      provider: 'anthropic' | 'deepseek';
      apiKey: string;
      endpoint: string;
      method: string;
      body: any;
      headers?: Record<string, string>;
    }) => Promise<{
      success: boolean;
      data?: any;
      error?: string;
      status?: number;
    }>;
  };
}
