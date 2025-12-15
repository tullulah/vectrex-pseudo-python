import React, { useState } from 'react';
import '../../styles/Dialog.css';

interface CreateBranchDialogProps {
  projectDir: string;
  currentBranch: string;
  onClose: () => void;
  onBranchCreated?: () => void;
}

export const CreateBranchDialog: React.FC<CreateBranchDialogProps> = ({
  projectDir,
  currentBranch,
  onClose,
  onBranchCreated,
}) => {
  const [branchName, setBranchName] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleCreate = async () => {
    if (!branchName.trim()) {
      setError('Branch name cannot be empty');
      return;
    }

    // Validate branch name
    if (!/^[a-zA-Z0-9._\/-]+$/.test(branchName)) {
      setError('Invalid branch name. Use alphanumeric, dots, slashes, and hyphens');
      return;
    }

    try {
      setLoading(true);
      const git = (window as any).git;
      
      // Create branch from current branch
      const result = await git.createBranch?.({ projectDir, branch: branchName, fromBranch: currentBranch });
      
      if (result?.ok) {
        onBranchCreated?.();
        onClose();
      } else {
        setError(result?.error || 'Failed to create branch');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') handleCreate();
    if (e.key === 'Escape') onClose();
  };

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog-modal" onClick={(e) => e.stopPropagation()}>
        <div className="dialog-header">
          <h2>Create New Branch</h2>
          <button className="dialog-close" onClick={onClose}>✕</button>
        </div>

        <div className="dialog-content">
          <div className="dialog-field">
            <label>Branch Name:</label>
            <input
              type="text"
              value={branchName}
              onChange={(e) => {
                setBranchName(e.target.value);
                setError(null);
              }}
              onKeyPress={handleKeyPress}
              placeholder="feature/my-feature"
              disabled={loading}
              autoFocus
            />
          </div>

          <div className="dialog-info">
            <span>Creating from: <code>{currentBranch}</code></span>
          </div>

          {error && <div className="dialog-error">{error}</div>}
        </div>

        <div className="dialog-actions">
          <button
            className="dialog-cancel-btn"
            onClick={onClose}
            disabled={loading}
          >
            Cancel
          </button>
          <button
            className="dialog-confirm-btn"
            onClick={handleCreate}
            disabled={loading || !branchName.trim()}
          >
            {loading ? 'Creating...' : 'Create'}
          </button>
        </div>
      </div>
    </div>
  );
};
