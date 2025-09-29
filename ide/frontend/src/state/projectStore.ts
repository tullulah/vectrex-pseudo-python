import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Project, FileNode } from '../types/models';

interface WorkspaceEntry {
  path: string;
  name: string;
  lastOpened: number;
}

interface ProjectState {
  // Current workspace
  project?: Project;
  selected?: string; // path
  workspaceName?: string;
  
  // Recent workspaces
  recentWorkspaces: WorkspaceEntry[];
  lastWorkspacePath?: string; // Remember last opened workspace
  
  // Actions
  setProject: (rootPath: string, files: FileNode[], name?: string) => void;
  selectFile: (path: string) => void;
  addRecentWorkspace: (path: string, name: string) => void;
  clearWorkspace: () => void;
  clearRecentWorkspaces: () => void;
  hasWorkspace: () => boolean;
  refreshWorkspace: () => Promise<void>;
  restoreLastWorkspace: () => Promise<void>;
}

export const useProjectStore = create<ProjectState>()(
  persist(
    (set, get) => ({
      // Initial state
      project: undefined,
      selected: undefined,
      workspaceName: undefined,
      recentWorkspaces: [],
      lastWorkspacePath: undefined,
      
      // Actions
      setProject: (rootPath, files, name) => {
        const workspaceName = name || rootPath.split(/[/\\]/).pop() || 'Workspace';
        console.log('ProjectStore: Setting project', { rootPath, workspaceName, filesCount: files.length });
        console.log('ProjectStore: Files structure:', files);
        set({ 
          project: { rootPath, files }, 
          workspaceName,
          selected: undefined,
          lastWorkspacePath: rootPath // Save as last workspace
        });
        get().addRecentWorkspace(rootPath, workspaceName);
      },
      
      selectFile: (path) => set({ selected: path }),
      
      addRecentWorkspace: (path, name) => {
        const recent = get().recentWorkspaces;
        const filtered = recent.filter(w => w.path !== path);
        const newEntry: WorkspaceEntry = { path, name, lastOpened: Date.now() };
        console.log('ProjectStore: Adding recent workspace', newEntry);
        set({ 
          recentWorkspaces: [newEntry, ...filtered].slice(0, 10) // Keep last 10
        });
        console.log('ProjectStore: Recent workspaces now:', get().recentWorkspaces);
      },
      
      clearWorkspace: () => set({ 
        project: undefined, 
        selected: undefined, 
        workspaceName: undefined 
      }),
      
      clearRecentWorkspaces: () => {
        console.log('ProjectStore: Clearing recent workspaces');
        set({ recentWorkspaces: [] });
      },
      
      hasWorkspace: () => !!get().project,
      
      refreshWorkspace: async () => {
        const current = get().project;
        if (!current) return;
        
        try {
          const result = await (window as any).files?.readDirectory?.(current.rootPath);
          if (result?.files) {
            console.log('ProjectStore: Refreshing workspace files');
            set({ 
              project: { 
                rootPath: current.rootPath, 
                files: result.files 
              } 
            });
          }
        } catch (error) {
          console.error('ProjectStore: Failed to refresh workspace:', error);
        }
      },
      
      restoreLastWorkspace: async () => {
        const lastPath = get().lastWorkspacePath;
        if (!lastPath) return;
        
        try {
          console.log('ProjectStore: Restoring last workspace:', lastPath);
          const result = await (window as any).files?.readDirectory?.(lastPath);
          if (result?.files) {
            const workspaceName = lastPath.split(/[/\\]/).pop() || 'Workspace';
            set({ 
              project: { rootPath: lastPath, files: result.files }, 
              workspaceName,
              selected: undefined 
            });
            console.log('ProjectStore: Successfully restored workspace:', workspaceName);
          } else {
            console.warn('ProjectStore: Failed to restore workspace - directory not accessible');
            // Clear invalid last workspace
            set({ lastWorkspacePath: undefined });
          }
        } catch (error) {
          console.error('ProjectStore: Error restoring last workspace:', error);
          // Clear invalid last workspace
          set({ lastWorkspacePath: undefined });
        }
      },
    }),
    {
      name: 'vpy-workspace-storage',
      partialize: (state) => ({ 
        recentWorkspaces: state.recentWorkspaces,
        lastWorkspacePath: state.lastWorkspacePath
      })
    }
  )
);
