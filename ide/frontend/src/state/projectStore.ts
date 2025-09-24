import { create } from 'zustand';
import type { Project, FileNode } from '../types/models';

interface ProjectState {
  project?: Project;
  selected?: string; // path
  setProject: (rootPath: string, files: FileNode[]) => void;
  selectFile: (path: string) => void;
}

export const useProjectStore = create<ProjectState>((set) => ({
  project: undefined,
  selected: undefined,
  setProject: (rootPath, files) => set({ project: { rootPath, files } }),
  selectFile: (path) => set({ selected: path }),
}));
