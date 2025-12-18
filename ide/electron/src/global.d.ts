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

interface PyPilotAPI {
  createSession(projectPath: string, name?: string): Promise<{ success: boolean; session?: any; error?: string }>;
  getSessions(projectPath: string): Promise<{ success: boolean; sessions?: any[]; error?: string }>;
  getActiveSession(projectPath: string): Promise<{ success: boolean; session?: any; error?: string }>;
  switchSession(sessionId: number): Promise<{ success: boolean; session?: any; error?: string }>;
  renameSession(sessionId: number, newName: string): Promise<{ success: boolean; error?: string }>;
  deleteSession(sessionId: number): Promise<{ success: boolean; error?: string }>;
  saveMessage(sessionId: number, role: string, content: string, metadata?: any): Promise<{ success: boolean; message?: any; error?: string }>;
  getMessages(sessionId: number): Promise<{ success: boolean; messages?: any[]; error?: string }>;
  clearMessages(sessionId: number): Promise<{ success: boolean; error?: string }>;
  getMessageCount(sessionId: number): Promise<{ success: boolean; count?: number; error?: string }>;
}

interface Window { 
  electronAPI?: ElectronAPI;
  mcp?: MCPAPI;
  pypilot?: PyPilotAPI;
  __editorStore__?: any;
  __projectStore__?: any;
  __debugStore__?: any;
}
