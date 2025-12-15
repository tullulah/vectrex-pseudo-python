import React, { useState, useEffect } from 'react';
import './GitPanel.css';
import { useProjectStore } from '../../state/projectStore';

interface GitChange {
  path: string;
  status: 'M' | 'A' | 'D' | '?';
  staged: boolean;
}

export const GitPanel: React.FC = () => {
  const [commitMessage, setCommitMessage] = useState('');
  const [changes, setChanges] = useState<GitChange[]>([]);
  const [loading, setLoading] = useState(false);
  const { vpyProject } = useProjectStore();

  // Load git status when component mounts or project changes
  useEffect(() => {
    if (!vpyProject?.projectFile) return;

    const loadGitStatus = async () => {
      setLoading(true);
      try {
        const git = (window as any).git;
        if (!git?.status) return;

        // Get project root directory
        const projectDir = vpyProject.projectFile.split(/[\\\/]/).slice(0, -1).join('/');
        
        const result = await git.status(projectDir);
        if (result.ok && result.files) {
          setChanges(result.files);
        }
      } catch (error) {
        console.error('Failed to load git status:', error);
      } finally {
        setLoading(false);
      }
    };

    loadGitStatus();
  }, [vpyProject]);

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

  const handleStageFile = async (path: string) => {
    try {
      const git = (window as any).git;
      if (!git?.stage) return;

      const projectDir = vpyProject?.projectFile.split(/[\\\/]/).slice(0, -1).join('/');
      if (!projectDir) return;
      
      const result = await git.stage({ projectDir, filePath: path });
      
      if (result.ok) {
        // Refresh git status
        const statusResult = await git.status(projectDir);
        if (statusResult.ok && statusResult.files) {
          setChanges(statusResult.files);
        }
      }
    } catch (error) {
      console.error('Failed to stage file:', error);
    }
  };

  const handleUnstageFile = async (path: string) => {
    try {
      const git = (window as any).git;
      if (!git?.unstage) return;

      const projectDir = vpyProject?.projectFile.split(/[\\\/]/).slice(0, -1).join('/');
      if (!projectDir) return;
      
      const result = await git.unstage({ projectDir, filePath: path });
      
      if (result.ok) {
        // Refresh git status
        const statusResult = await git.status(projectDir);
        if (statusResult.ok && statusResult.files) {
          setChanges(statusResult.files);
        }
      }
    } catch (error) {
      console.error('Failed to unstage file:', error);
    }
  };

  const handleDiscardFile = async (path: string) => {
    // Confirm before discarding
    const confirmed = window.confirm(`Are you sure you want to discard changes to ${path.split('/').pop()}?\n\nThis cannot be undone.`);
    if (!confirmed) return;

    try {
      const git = (window as any).git;
      if (!git?.discard) return;

      const projectDir = vpyProject?.projectFile.split(/[\\\/]/).slice(0, -1).join('/');
      if (!projectDir) return;
      
      const result = await git.discard({ projectDir, filePath: path });
      
      if (result.ok) {
        // Refresh git status
        const statusResult = await git.status(projectDir);
        if (statusResult.ok && statusResult.files) {
          setChanges(statusResult.files);
        }
      } else {
        alert(`Failed to discard changes: ${result.error}`);
      }
    } catch (error) {
      console.error('Failed to discard file:', error);
      alert(`Error discarding changes: ${error}`);
    }
  };

  const handleCommit = async () => {
    if (!commitMessage.trim() || stagedChanges.length === 0) return;

    try {
      const git = (window as any).git;
      if (!git?.commit) return;

      const projectDir = vpyProject?.projectFile.split(/[\\\/]/).slice(0, -1).join('/');
      if (!projectDir) return;
      
      const result = await git.commit({ projectDir, message: commitMessage });
      
      if (result.ok) {
        setCommitMessage('');
        // Refresh git status
        const statusResult = await git.status(projectDir);
        if (statusResult.ok && statusResult.files) {
          setChanges(statusResult.files);
        }
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
                  <button
                    className="git-change-action"
                    onClick={() => handleUnstageFile(file.path)}
                    title="Unstage"
                  >
                    −
                  </button>
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
    </div>
  );
};
