import React, { useState, useEffect } from 'react';
import '../panels/GitPanel.css';

interface Remote {
  name: string;
  url: string;
  type: string;
}

interface RemotesListProps {
  projectDir: string;
  onRemoteAdded?: () => void;
}

export const RemotesList: React.FC<RemotesListProps> = ({ projectDir, onRemoteAdded }) => {
  const [remotes, setRemotes] = useState<Remote[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [newRemoteName, setNewRemoteName] = useState('');
  const [newRemoteUrl, setNewRemoteUrl] = useState('');
  const [adding, setAdding] = useState(false);

  const loadRemotes = async () => {
    try {
      setLoading(true);
      const git = (window as any).git;
      const result = await git.remoteList(projectDir);

      if (!result.ok) {
        setError(result.error || 'Failed to load remotes');
        return;
      }

      setRemotes(result.remotes || []);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (projectDir) {
      loadRemotes();
    }
  }, [projectDir]);

  const handleAddRemote = async () => {
    if (!newRemoteName.trim() || !newRemoteUrl.trim()) {
      alert('Remote name and URL cannot be empty');
      return;
    }

    try {
      setAdding(true);
      const git = (window as any).git;
      const result = await git.addRemote({
        projectDir,
        name: newRemoteName,
        url: newRemoteUrl,
      });

      if (result.ok) {
        alert(`Remote "${newRemoteName}" added successfully`);
        setNewRemoteName('');
        setNewRemoteUrl('');
        await loadRemotes();
        onRemoteAdded?.();
      } else {
        alert(`Failed to add remote: ${result.error}`);
      }
    } catch (err) {
      alert(`Error adding remote: ${err}`);
    } finally {
      setAdding(false);
    }
  };

  const handleRemoveRemote = async (remoteName: string) => {
    const confirmed = window.confirm(`Remove remote "${remoteName}"?`);
    if (!confirmed) return;

    try {
      const git = (window as any).git;
      const result = await git.removeRemote({ projectDir, name: remoteName });

      if (result.ok) {
        alert(`Remote "${remoteName}" removed successfully`);
        await loadRemotes();
        onRemoteAdded?.();
      } else {
        alert(`Failed to remove remote: ${result.error}`);
      }
    } catch (err) {
      alert(`Error removing remote: ${err}`);
    }
  };

  return (
    <div>
      <div style={{ marginBottom: '12px', paddingBottom: '12px', borderBottom: '1px solid #444' }}>
        <div style={{ fontSize: '12px', marginBottom: '8px', fontWeight: 'bold' }}>Add Remote</div>
        <input
          type="text"
          className="git-commit-message"
          placeholder="Remote name (e.g., origin, upstream)"
          value={newRemoteName}
          onChange={(e) => setNewRemoteName(e.target.value)}
          disabled={adding}
          style={{ minHeight: 'auto', padding: '6px 8px', fontSize: '12px', marginBottom: '4px' }}
        />
        <input
          type="text"
          className="git-commit-message"
          placeholder="Remote URL (e.g., git@github.com:user/repo.git)"
          value={newRemoteUrl}
          onChange={(e) => setNewRemoteUrl(e.target.value)}
          disabled={adding}
          style={{ minHeight: 'auto', padding: '6px 8px', fontSize: '12px', marginBottom: '4px' }}
        />
        <button
          className="git-commit-button"
          disabled={adding || !newRemoteName.trim() || !newRemoteUrl.trim()}
          onClick={handleAddRemote}
        >
          ✓ Add Remote
        </button>
      </div>

      <div className="git-history-list" style={{ maxHeight: '300px', overflow: 'auto' }}>
        {loading && <div className="git-history-loading">Loading remotes...</div>}
        {error && <div className="git-history-error">Error: {error}</div>}

        {!loading && !error && remotes.length === 0 && (
          <div className="git-history-empty">No remotes configured</div>
        )}

        {!loading && !error && remotes.map((remote) => (
          <div
            key={remote.name}
            className="git-history-item"
            style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}
          >
            <div style={{ flex: 1 }}>
              <div className="git-history-message">🔗 {remote.name}</div>
              <div style={{ fontSize: '11px', color: '#999', marginTop: '4px', wordBreak: 'break-all' }}>
                {remote.url}
              </div>
            </div>
            <button
              className="git-panel-action-btn"
              onClick={() => handleRemoveRemote(remote.name)}
              style={{ marginLeft: '8px', whiteSpace: 'nowrap', backgroundColor: '#d9534f', borderColor: '#d9534f' }}
            >
              Remove
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};
