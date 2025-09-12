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
