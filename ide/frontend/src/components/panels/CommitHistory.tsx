import React, { useState, useEffect } from 'react';
import '../panels/GitPanel.css';

interface Commit {
  hash: string;
  fullHash: string;
  message: string;
  author: string;
  email: string;
  date: string;
  body: string;
}

interface CommitHistoryProps {
  projectDir: string;
  onClose: () => void;
  onRevert?: () => void;
}

export const CommitHistory: React.FC<CommitHistoryProps> = ({ projectDir, onClose, onRevert }) => {
  const [commits, setCommits] = useState<Commit[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedCommit, setSelectedCommit] = useState<Commit | null>(null);

  const handleRevert = async () => {
    if (!selectedCommit) return;

    const confirmed = window.confirm(`Revert commit ${selectedCommit.hash}?\n\n"${selectedCommit.message}"`);
    if (!confirmed) return;

    try {
      const git = (window as any).git;
      const result = await git.revert({ projectDir, commitHash: selectedCommit.fullHash });

      if (result.ok) {
        alert('Commit reverted successfully');
        onRevert?.();
        onClose();
      } else {
        alert(`Failed to revert: ${result.error}`);
      }
    } catch (err) {
      alert(`Error reverting commit: ${err}`);
    }
  };

  useEffect(() => {
    const loadCommits = async () => {
      try {
        setLoading(true);
        const git = (window as any).git;
        const result = await git.log({ projectDir, limit: 50 });

        if (!result.ok) {
          setError(result.error || 'Failed to load commits');
          return;
        }

        setCommits(result.commits || []);
        setError(null);
      } catch (err) {
        setError(String(err));
      } finally {
        setLoading(false);
      }
    };

    if (projectDir) {
      loadCommits();
    }
  }, [projectDir]);

  const formatDate = (dateStr: string): string => {
    try {
      const date = new Date(dateStr);
      return date.toLocaleDateString('es-ES', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
      });
    } catch {
      return dateStr;
    }
  };

  return (
    <div className="git-history-overlay" onClick={onClose}>
      <div className="git-history-modal" onClick={(e) => e.stopPropagation()}>
        <div className="git-history-header">
          <h2>Commit History</h2>
          <button className="git-history-close" onClick={onClose}>✕</button>
        </div>

        <div className="git-history-container">
          <div className="git-history-list">
            {loading && <div className="git-history-loading">Loading commits...</div>}
            {error && <div className="git-history-error">Error: {error}</div>}

            {!loading && !error && commits.length === 0 && (
              <div className="git-history-empty">No commits found</div>
            )}

            {!loading && !error && commits.map((commit) => (
              <div
                key={commit.fullHash}
                className={`git-history-item ${selectedCommit?.fullHash === commit.fullHash ? 'active' : ''}`}
                onClick={() => setSelectedCommit(commit)}
              >
                <div className="git-history-hash">{commit.hash}</div>
                <div className="git-history-message">{commit.message}</div>
                <div className="git-history-meta">
                  <span className="git-history-author">{commit.author}</span>
                  <span className="git-history-date">{formatDate(commit.date)}</span>
                </div>
              </div>
            ))}
          </div>

          {selectedCommit && (
            <div className="git-history-detail">
              <div className="git-history-detail-header">
                <h3>Commit Details</h3>
              </div>
              <div className="git-history-detail-content">
                <div className="git-history-detail-row">
                  <span className="git-history-detail-label">Hash:</span>
                  <span className="git-history-detail-value">{selectedCommit.fullHash}</span>
                </div>
                <div className="git-history-detail-row">
                  <span className="git-history-detail-label">Author:</span>
                  <span className="git-history-detail-value">{selectedCommit.author} &lt;{selectedCommit.email}&gt;</span>
                </div>
                <div className="git-history-detail-row">
                  <span className="git-history-detail-label">Date:</span>
                  <span className="git-history-detail-value">{formatDate(selectedCommit.date)}</span>
                </div>
                <div className="git-history-detail-row full">
                  <span className="git-history-detail-label">Message:</span>
                  <div className="git-history-detail-text">{selectedCommit.message}</div>
                </div>
                {selectedCommit.body && (
                  <div className="git-history-detail-row full">
                    <span className="git-history-detail-label">Body:</span>
                    <div className="git-history-detail-text">{selectedCommit.body}</div>
                  </div>
                )}
                <div className="git-history-detail-row" style={{ marginTop: '12px' }}>
                  <button
                    className="git-panel-action-btn"
                    onClick={handleRevert}
                    style={{ backgroundColor: '#d9534f', borderColor: '#d9534f' }}
                  >
                    ↶ Revert
                  </button>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
