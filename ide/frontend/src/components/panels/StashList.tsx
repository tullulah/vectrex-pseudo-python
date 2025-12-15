import React, { useState, useEffect } from 'react';
import '../panels/GitPanel.css';

interface Stash {
  index: number;
  hash: string;
  fullHash: string;
  message: string;
  date: string;
}

interface StashListProps {
  projectDir: string;
  onStashPopped?: () => void;
}

export const StashList: React.FC<StashListProps> = ({ projectDir, onStashPopped }) => {
  const [stashes, setStashes] = useState<Stash[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadStashes = async () => {
      try {
        setLoading(true);
        const git = (window as any).git;
        const result = await git.stashList(projectDir);

        if (!result.ok) {
          setError(result.error || 'Failed to load stashes');
          return;
        }

        setStashes(result.stashes || []);
        setError(null);
      } catch (err) {
        setError(String(err));
      } finally {
        setLoading(false);
      }
    };

    if (projectDir) {
      loadStashes();
    }
  }, [projectDir]);

  const handlePopStash = async (index: number) => {
    try {
      const git = (window as any).git;
      const result = await git.stashPop({ projectDir, index });

      if (result.ok) {
        alert('Stash applied successfully');
        // Reload stash list
        const listResult = await git.stashList(projectDir);
        if (listResult.ok) {
          setStashes(listResult.stashes || []);
        }
        onStashPopped?.();
      } else {
        alert(`Failed to apply stash: ${result.error}`);
      }
    } catch (err) {
      alert(`Error applying stash: ${err}`);
    }
  };

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
    <div>
      <div className="git-history-list" style={{ maxHeight: '400px', overflow: 'auto' }}>
        {loading && <div className="git-history-loading">Loading stashes...</div>}
        {error && <div className="git-history-error">Error: {error}</div>}

        {!loading && !error && stashes.length === 0 && (
          <div className="git-history-empty">No stashes found</div>
        )}

        {!loading && !error && stashes.map((stash) => (
          <div
            key={stash.index}
            className="git-history-item"
            style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}
          >
            <div style={{ flex: 1 }}>
              <div className="git-history-hash">{stash.hash}</div>
              <div className="git-history-message">{stash.message}</div>
              <div className="git-history-meta">
                <span className="git-history-date">{formatDate(stash.date)}</span>
              </div>
            </div>
            <button
              className="git-panel-action-btn"
              onClick={() => handlePopStash(stash.index)}
              style={{ marginLeft: '8px', whiteSpace: 'nowrap' }}
            >
              Apply
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};
