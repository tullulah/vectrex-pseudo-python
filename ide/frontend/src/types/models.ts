// Domain model interfaces for the VPy IDE

export interface FileNode {
  path: string;
  name: string;
  isDir: boolean;
  children?: FileNode[];
}

export interface Project {
  rootPath: string;
  files: FileNode[];
}

export interface DocumentModel {
  uri: string;
  language: 'vpy';
  content: string;
  dirty: boolean;
  diagnostics: DiagnosticModel[];
  // Absolute path on disk if backed by a real file. Undefined for in-memory docs.
  diskPath?: string;
  // Last known modification time (ms epoch) from filesystem when loaded/saved.
  mtime?: number;
  // (Optional) snapshot of content at last save/load for advanced dirty detection.
  lastSavedContent?: string;
}

export interface DiagnosticModel {
  message: string;
  severity: 'error' | 'warning' | 'info';
  line: number;
  column: number;
}

export interface DebugState {
  registers: Record<string,string>;
  pc: number;
  cycles: number;
  variables: Array<{ name: string; value: string }>;
  constants: Array<{ name: string; value: string }>;
}

export interface EmulatorState {
  status: 'running' | 'stopped';
  lastFrameTimestamp?: number;
}
