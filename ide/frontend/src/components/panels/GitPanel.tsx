import React, { useState, useEffect } from 'react';
import './GitPanel.css';
import { useProjectStore } from '../../state/projectStore';
import { DiffViewer } from '../modals/DiffViewer';
import { CommitHistory } from './CommitHistory';

interface GitChange {
  path: string;
  status: 'M' | 'A' | 'D' | '?';
  staged: boolean;
}

interface GitBranch {
  name: string;
  current: boolean;
  isRemote: boolean;
}

export const GitPanel: React.FC = () => {
  const [commitMessage, setCommitMessage] = useState('');
  const [changes, setChanges] = useState<GitChange[]>([]);
  const [loading, setLoading] = useState(false);
  const [currentProjectDir, setCurrentProjectDir] = useState<string | null>(null);
  const [branches, setBranches] = useState<GitBranch[]>([]);
  const [currentBranch, setCurrentBranch] = useState<string | null>(null);
  const [showBranchDropdown, setShowBranchDropdown] = useState(false);
  const [selectedDiffFile, setSelectedDiffFile] = useState<string | null>(null);
  const [showCommitHistory, setShowCommitHistory] = useState(false);
  const { vpyProject } = useProjectStore();

  // Function to refresh git status
  const refreshGitStatus = async (projectDir: string) => {
    try {
      const git = (window as any).git;
      if (!git?.status) return;

      const result = await git.status(projectDir);
      if (result.ok && result.files) {
        setChanges(result.files);
      }
    } catch (error) {
      console.error('Failed to refresh git status:', error);
    }
  };

  // Function to refresh branches
  const refreshBranches = async (projectDir: string) => {
    try {
      const git = (window as any).git;
      if (!git?.branches) return;

      const result = await git.branches(projectDir);
      if (result.ok && result.branches) {
        setBranches(result.branches);
        setCurrentBranch(result.current);
      }
    } catch (error) {
      console.error('Failed to refresh branches:', error);
    }
  };

  // Load git status when component mounts or project changes
  useEffect(() => {
    // Clear changes when no project
    if (!vpyProject?.rootDir) {
      setChanges([]);
      setCurrentProjectDir(null);
      setCommitMessage('');
      setBranches([]);
      setCurrentBranch(null);
      setLoading(false);
      console.log('[GitPanel] Project closed, clearing changes');
      return;
    }

    // Only reload if project actually changed
    if (currentProjectDir === vpyProject.rootDir) {
      return;
    }

    // Clear and load new project
    console.log('[GitPanel] Project changed, loading:', vpyProject.rootDir);
    setCurrentProjectDir(vpyProject.rootDir);
    setChanges([]);
    setCommitMessage('');
    setBranches([]);
    setCurrentBranch(null);
    
    const loadGitData = async () => {
      setLoading(true);
      await refreshGitStatus(vpyProject.rootDir);
      await refreshBranches(vpyProject.rootDir);
      setLoading(false);
    };

    loadGitData();
  }, [vpyProject?.rootDir, currentProjectDir]);

  // Listen for file changes and auto-refresh git status
  useEffect(() => {
    if (!currentProjectDir) return;

    const files = (window as any).files;
    
    if (!files?.onFileChanged) return;

    // Subscribe to file changes
    const unsubscribe = files.onFileChanged((event: any) => {
      // Only refresh if it's a .vpy file change
      if (event.path.endsWith('.vpy') || event.path.endsWith('.asm')) {
        // Debounce: wait a bit for rapid file changes
        const timer = setTimeout(() => {
          refreshGitStatus(currentProjectDir);
        }, 500);
        
        return () => clearTimeout(timer);
      }
    });

    return unsubscribe;
  }, [currentProjectDir]);

  const stagedChanges = changes.filter(c => c.staged);
  const unstagedChanges = changes.filter(c => !c.staged);

  const getStatusLabel = (status: string) => {
    switch (status) {
      case 'M': return 'M';
      case 'A': return 'A';
      case 'D': return 'D';
      case '?': return '?';
      default: return '•';
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'M': return 'modified';
      case 'A': return 'added';
      case 'D': return 'deleted';
      case '?': return 'modified';
      default: return 'modified';
    }
  };

  const handleCheckoutBranch = async (branchName: string) => {
    if (!currentProjectDir || branchName === currentBranch) return;

    setShowBranchDropdown(false);
    
    try {
      const git = (window as any).git;
      if (!git?.checkout) return;

      const result = await git.checkout({ projectDir: currentProjectDir, branch: branchName });
      
      if (result.ok) {
        await refreshBranches(currentProjectDir);
        await refreshGitStatus(currentProjectDir);
      } else {
        alert(`Failed to checkout branch: ${result.error}`);
      }
    } catch (error) {
      console.error('Failed to checkout branch:', error);
      alert(`Error switching branch: ${error}`);
    }
  };

  const handlePush = async () => {
    if (!currentProjectDir || !currentBranch) return;

    try {
      const git = (window as any).git;
      if (!git?.push) return;

      setLoading(true);
      const result = await git.push({ projectDir: currentProjectDir, branch: currentBranch });
      
      if (result.ok) {
        alert('Changes pushed successfully');
        await refreshGitStatus(currentProjectDir);
      } else {
        alert(`Failed to push: ${result.error}`);
      }
    } catch (error) {
      console.error('Failed to push:', error);
      alert(`Error pushing changes: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handlePull = async () => {
    if (!currentProjectDir || !currentBranch) return;

    try {
      const git = (window as any).git;
      if (!git?.pull) return;

      setLoading(true);
      const result = await git.pull({ projectDir: currentProjectDir, branch: currentBranch });
      
      if (result.ok) {
        alert('Changes pulled successfully');
        await refreshGitStatus(currentProjectDir);
        await refreshBranches(currentProjectDir);
      } else {
        alert(`Failed to pull: ${result.error}`);
      }
    } catch (error) {
      console.error('Failed to pull:', error);
      alert(`Error pulling changes: ${error}`);
    } finally {
      setLoading(false);
    }

  const handleStageFile = async (path: string) => {
    if (!currentProjectDir) return;
    
    try {
      const git = (window as any).git;
      if (!git?.stage) return;
      
      const result = await git.stage({ projectDir: currentProjectDir, filePath: path });
      
      if (result.ok) {
        await refreshGitStatus(currentProjectDir);
      }
    } catch (error) {
      console.error('Failed to stage file:', error);
    }
  };

  const handleUnstageFile = async (path: string) => {
    if (!currentProjectDir) return;
    
    try {
      const git = (window as any).git;
      if (!git?.unstage) return;
      
      const result = await git.unstage({ projectDir: currentProjectDir, filePath: path });
      
      if (result.ok) {
        await refreshGitStatus(currentProjectDir);
      }
    } catch (error) {
      console.error('Failed to unstage file:', error);
    }
  };

  const handleDiscardFile = async (path: string) => {
    if (!currentProjectDir) return;
    
    // Confirm before discarding
    const confirmed = window.confirm(`Are you sure you want to discard changes to ${path.split('/').pop()}?\n\nThis cannot be undone.`);
    if (!confirmed) return;

    try {
      const git = (window as any).git;
      if (!git?.discard) return;
      
      const result = await git.discard({ projectDir: currentProjectDir, filePath: path });
      
      if (result.ok) {
        await refreshGitStatus(currentProjectDir);
      } else {
        alert(`Failed to discard changes: ${result.error}`);
      }
    } catch (error) {
      console.error('Failed to discard file:', error);
      alert(`Error discarding changes: ${error}`);
    }
  };

  const handleCommit = async () => {
    if (!currentProjectDir) return;
    if (!commitMessage.trim() || stagedChanges.length === 0) return;

    try {
      const git = (window as any).git;
      if (!git?.commit) return;
      
      const result = await git.commit({ projectDir: currentProjectDir, message: commitMessage });
      
      if (result.ok) {
        setCommitMessage('');
        await refreshGitStatus(currentProjectDir);
      } else {
        alert(`Commit failed: ${result.error}`);
      }
    } catch (error) {
      console.error('Failed to commit:', error);
      alert(`Commit error: ${error}`);
    }
  };

  return (
    <div className="git-panel">
      <div className="git-panel-header">
        <h3>Source Control</h3>
        
        <div className="git-panel-actions">
          <button
            className="git-panel-action-btn"
            onClick={handlePush}
            disabled={!currentBranch || loading}
            title="Push changes to remote"
          >
            ⬆️
          </button>
          <button
            className="git-panel-action-btn"
            onClick={handlePull}
            disabled={!currentBranch || loading}
            title="Pull changes from remote"
          >
            ⬇️
          </button>
          <button
            className="git-panel-action-btn"
            onClick={() => setShowCommitHistory(!showCommitHistory)}
            title="View commit history"
          >
            📜
          </button>
        </div>
        
        {/* Branch Selector Dropdown */}
        {currentBranch && (
          <div className="git-branch-selector">
            <button
              className="git-branch-button"
              onClick={() => setShowBranchDropdown(!showBranchDropdown)}
            >
              {currentBranch}
            </button>
            
            {showBranchDropdown && branches.length > 0 && (
              <div className="git-branch-dropdown">
                {/* Local Branches */}
                {branches.filter(b => !b.isRemote).length > 0 && (
                  <div className="git-branch-section">
                    {branches.filter(b => !b.isRemote).map((branch) => (
                      <button
                        key={branch.name}
                        className={`git-branch-option ${branch.current ? 'active' : ''}`}
                        onClick={() => handleCheckoutBranch(branch.name)}
                      >
                        {branch.current && '✓ '}
                        {branch.name}
                      </button>
                    ))}
                  </div>
                )}
                
                {/* Remote Branches */}
                {branches.filter(b => b.isRemote).length > 0 && (
                  <div className="git-branch-section">
                    <div className="git-branch-section-title">Remote</div>
                    {branches.filter(b => b.isRemote).map((branch) => (
                      <button
                        key={branch.name}
                        className={`git-branch-option remote ${branch.current ? 'active' : ''}`}
                        onClick={() => handleCheckoutBranch(branch.name)}
                      >
                        {branch.current && '✓ '}
                        {branch.name}
                      </button>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>
        )}
      </div>
      
      <div className="git-panel-content">
        <div className="git-commit-section">
          <textarea
            className="git-commit-message"
            placeholder="Message (Ctrl+Enter to commit)"
            value={commitMessage}
            onChange={(e) => setCommitMessage(e.target.value)}
            onKeyDown={(e) => {
              if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                handleCommit();
              }
            }}
            rows={3}
          />
          <button 
            className="git-commit-button" 
            disabled={!commitMessage.trim() || stagedChanges.length === 0}
            onClick={handleCommit}
          >
            ✓ Commit ({stagedChanges.length})
          </button>
        </div>

        {/* Staged Changes */}
        {stagedChanges.length > 0 && (
          <div className="git-changes-section">
            <div className="git-changes-header">
              <span>Staged Changes</span>
              <span className="git-changes-count">{stagedChanges.length}</span>
            </div>
            <div className="git-changes-list">
              {stagedChanges.map((file) => (
                <div 
                  key={file.path} 
                  className="git-change-item"
                  title={file.path}
                >
                  <span className={`git-change-status ${getStatusColor(file.status)}`}>
                    {getStatusLabel(file.status)}
                  </span>
                  <span className="git-change-path">{file.path.split('/').pop()}</span>
                  <div className="git-change-actions">
                    <button
                      className="git-change-action diff-btn"
                      onClick={() => setSelectedDiffFile(file.path)}
                      title="View diff"
                    >
                      ⧉
                    </button>
                    <button
                      className="git-change-action"
                      onClick={() => handleUnstageFile(file.path)}
                      title="Unstage"
                    >
                      −
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Unstaged Changes */}
        {unstagedChanges.length > 0 && (
          <div className="git-changes-section">
            <div className="git-changes-header">
              <span>Changes</span>
              <span className="git-changes-count">{unstagedChanges.length}</span>
            </div>
            <div className="git-changes-list">
              {unstagedChanges.map((file) => (
                <div 
                  key={file.path} 
                  className="git-change-item"
                  title={file.path}
                >
                  <span className={`git-change-status ${getStatusColor(file.status)}`}>
                    {getStatusLabel(file.status)}
                  </span>
                  <span className="git-change-path">{file.path.split('/').pop()}</span>
                  <div className="git-change-actions">
                    <button
                      className="git-change-action diff-btn"
                      onClick={() => setSelectedDiffFile(file.path)}
                      title="View diff"
                    >
                      ⧉
                    </button>
                    <button
                      className="git-change-action git-stage-btn"
                      onClick={() => handleStageFile(file.path)}
                      title="Stage"
                    >
                      +
                    </button>
                    <button
                      className="git-change-action git-discard-btn"
                      onClick={() => handleDiscardFile(file.path)}
                      title="Discard changes"
                    >
                      ↺
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* No changes */}
        {changes.length === 0 && !loading && (
          <div className="git-no-changes">No changes</div>
        )}

        {loading && (
          <div className="git-no-changes">Loading...</div>
        )}
      </div>

      {/* Diff Viewer Modal */}
      {selectedDiffFile && currentProjectDir && (
        <DiffViewer
          filePath={selectedDiffFile}
          projectDir={currentProjectDir}
          onClose={() => setSelectedDiffFile(null)}
        />
      )}

      {/* Commit History Modal */}
      {showCommitHistory && currentProjectDir && (
        <CommitHistory
          projectDir={currentProjectDir}
          onClose={() => setShowCommitHistory(false)}
        />
      )}
    </div>
  );
};
