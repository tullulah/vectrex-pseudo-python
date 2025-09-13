interface ElectronAPI {
  lspStart(): Promise<void>;
  lspSend(payload: string): Promise<void>;
  onLspMessage(cb: (json: string) => void): void;
  onLspStdout(cb: (line: string) => void): void;
  onLspStderr(cb: (line: string) => void): void;
}
interface Window { electronAPI?: ElectronAPI }
