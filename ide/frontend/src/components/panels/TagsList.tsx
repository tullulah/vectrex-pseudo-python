import React, { useState, useEffect } from 'react';
import '../panels/GitPanel.css';

interface Tag {
  name: string;
}

interface TagsListProps {
  projectDir: string;
  onTagDeleted?: () => void;
}

export const TagsList: React.FC<TagsListProps> = ({ projectDir, onTagDeleted }) => {
  const [tags, setTags] = useState<Tag[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [newTagName, setNewTagName] = useState('');
  const [newTagMessage, setNewTagMessage] = useState('');
  const [creating, setCreating] = useState(false);

  const loadTags = async () => {
    try {
      setLoading(true);
      const git = (window as any).git;
      const result = await git.tagList(projectDir);

      if (!result.ok) {
        setError(result.error || 'Failed to load tags');
        return;
      }

      setTags(result.tags || []);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (projectDir) {
      loadTags();
    }
  }, [projectDir]);

  const handleCreateTag = async () => {
    if (!newTagName.trim()) {
      alert('Tag name cannot be empty');
      return;
    }

    try {
      setCreating(true);
      const git = (window as any).git;
      const result = await git.tag({
        projectDir,
        tagName: newTagName,
        message: newTagMessage || undefined,
      });

      if (result.ok) {
        alert(`Tag "${newTagName}" created successfully`);
        setNewTagName('');
        setNewTagMessage('');
        await loadTags();
      } else {
        alert(`Failed to create tag: ${result.error}`);
      }
    } catch (err) {
      alert(`Error creating tag: ${err}`);
    } finally {
      setCreating(false);
    }
  };

  const handleDeleteTag = async (tagName: string) => {
    const confirmed = window.confirm(`Delete tag "${tagName}"?`);
    if (!confirmed) return;

    try {
      const git = (window as any).git;
      const result = await git.deleteTag({ projectDir, tagName });

      if (result.ok) {
        alert(`Tag "${tagName}" deleted successfully`);
        await loadTags();
        onTagDeleted?.();
      } else {
        alert(`Failed to delete tag: ${result.error}`);
      }
    } catch (err) {
      alert(`Error deleting tag: ${err}`);
    }
  };

  return (
    <div>
      <div style={{ marginBottom: '12px', paddingBottom: '12px', borderBottom: '1px solid #444' }}>
        <div style={{ fontSize: '12px', marginBottom: '8px', fontWeight: 'bold' }}>Create New Tag</div>
        <input
          type="text"
          className="git-commit-message"
          placeholder="Tag name"
          value={newTagName}
          onChange={(e) => setNewTagName(e.target.value)}
          disabled={creating}
          style={{ minHeight: 'auto', padding: '6px 8px', fontSize: '12px', marginBottom: '4px' }}
        />
        <textarea
          className="git-commit-message"
          placeholder="Tag message (optional)"
          value={newTagMessage}
          onChange={(e) => setNewTagMessage(e.target.value)}
          disabled={creating}
          rows={2}
          style={{ fontSize: '12px', marginBottom: '4px', minHeight: '40px' }}
        />
        <button
          className="git-commit-button"
          disabled={creating || !newTagName.trim()}
          onClick={handleCreateTag}
        >
          ✓ Create Tag
        </button>
      </div>

      <div className="git-history-list" style={{ maxHeight: '300px', overflow: 'auto' }}>
        {loading && <div className="git-history-loading">Loading tags...</div>}
        {error && <div className="git-history-error">Error: {error}</div>}

        {!loading && !error && tags.length === 0 && (
          <div className="git-history-empty">No tags found</div>
        )}

        {!loading && !error && tags.map((tag) => (
          <div
            key={tag.name}
            className="git-history-item"
            style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}
          >
            <div style={{ flex: 1 }}>
              <div className="git-history-message">🏷️ {tag.name}</div>
            </div>
            <button
              className="git-panel-action-btn"
              onClick={() => handleDeleteTag(tag.name)}
              style={{ marginLeft: '8px', whiteSpace: 'nowrap', backgroundColor: '#d9534f', borderColor: '#d9534f' }}
            >
              Delete
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};
