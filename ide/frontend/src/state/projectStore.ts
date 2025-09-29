import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Project, FileNode } from '../types/models';
import { logger } from '../utils/logger';

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
        logger.debug('Project', 'Setting project', { rootPath, workspaceName, filesCount: files.length });
        logger.debug('Project', 'Files structure:', files);
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
        logger.debug('Project', 'Adding recent workspace', newEntry);
        set({ 
          recentWorkspaces: [newEntry, ...filtered].slice(0, 10) // Keep last 10
        });
        logger.debug('Project', 'Recent workspaces now:', get().recentWorkspaces);
      },
      
      clearWorkspace: () => set({ 
        project: undefined, 
        selected: undefined, 
        workspaceName: undefined 
      }),
      
      clearRecentWorkspaces: () => {
        logger.debug('Project', 'Clearing recent workspaces');
        set({ recentWorkspaces: [] });
      },
      
      hasWorkspace: () => !!get().project,
      
      refreshWorkspace: async () => {
        const current = get().project;
        if (!current) return;
        
        try {
          const result = await (window as any).files?.readDirectory?.(current.rootPath);
          if (result?.files) {
            logger.debug('Project', 'Refreshing workspace files');
            set({ 
              project: { 
                rootPath: current.rootPath, 
                files: result.files 
              } 
            });
          }
        } catch (error) {
          logger.error('Project', 'Failed to refresh workspace:', error);
        }
      },
      
      restoreLastWorkspace: async () => {
        const lastPath = get().lastWorkspacePath;
        if (!lastPath) return;
        
        try {
          logger.debug('Project', 'Restoring last workspace:', lastPath);
          const result = await (window as any).files?.readDirectory?.(lastPath);
          if (result?.files) {
            const workspaceName = lastPath.split(/[/\\]/).pop() || 'Workspace';
            set({ 
              project: { rootPath: lastPath, files: result.files }, 
              workspaceName,
              selected: undefined 
            });
            logger.info('Project', 'Successfully restored workspace:', workspaceName);
          } else {
            logger.warn('Project', 'Failed to restore workspace - directory not accessible');
            // Clear invalid last workspace
            set({ lastWorkspacePath: undefined });
          }
        } catch (error) {
          logger.error('Project', 'Error restoring last workspace:', error);
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
