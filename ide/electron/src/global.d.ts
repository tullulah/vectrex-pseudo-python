interface ElectronAPI {
  lspStart(): Promise<void>;
  lspSend(payload: string): Promise<void>;
  onLspMessage(cb: (json: string) => void): void;
  onLspStdout(cb: (line: string) => void): void;
  onLspStderr(cb: (line: string) => void): void;
}

interface MCPAPI {
  request(request: any): Promise<any>;
}

interface Window { 
  electronAPI?: ElectronAPI;
  mcp?: MCPAPI;
  __editorStore__?: any;
  __projectStore__?: any;
  __debugStore__?: any;
}
