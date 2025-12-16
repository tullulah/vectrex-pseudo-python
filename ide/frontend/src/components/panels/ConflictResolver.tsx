import React, { useState, useEffect } from 'react';
import '../panels/GitPanel.css';

interface Conflict {
  filePath: string;
  resolved: boolean;
}

interface ConflictResolverProps {
  projectDir: string;
  onConflictResolved?: () => void;
  onMergeComplete?: () => void;
}

export const ConflictResolver: React.FC<ConflictResolverProps> = ({ 
  projectDir, 
  onConflictResolved, 
  onMergeComplete 
}) => {
  const [conflicts, setConflicts] = useState<Conflict[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedConflict, setSelectedConflict] = useState<string | null>(null);
  const [conflictContent, setConflictContent] = useState<string | null>(null);
  const [mergeMessage, setMergeMessage] = useState('Merge resolved');
  const [completing, setCompleting] = useState(false);

  useEffect(() => {
    loadConflicts();
  }, [projectDir]);

  const loadConflicts = async () => {
    try {
      setLoading(true);
      const git = (window as any).git;
      const result = await git.checkConflicts(projectDir);

      if (result.ok && result.conflicts) {
        setConflicts(result.conflicts.map((filePath: string) => ({
          filePath,
          resolved: false,
        })));
      } else {
        setConflicts([]);
      }
    } catch (err) {
      console.error('Error loading conflicts:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleViewConflict = async (filePath: string) => {
    try {
      const git = (window as any).git;
      const result = await git.getConflictDetails({ projectDir, filePath });

      if (result.ok) {
        setSelectedConflict(filePath);
        setConflictContent(result.content || '');
      } else {
        alert(`Error loading conflict: ${result.error}`);
      }
    } catch (err) {
      alert(`Error: ${err}`);
    }
  };

  const handleResolveConflict = async () => {
    if (!selectedConflict) return;

    // In a real scenario, the user would manually edit the file to resolve conflicts
    // Here we just mark it as resolved (staged)
    try {
      const git = (window as any).git;
      const result = await git.markResolved({ projectDir, filePath: selectedConflict });

      if (result.ok) {
        alert(`File "${selectedConflict}" marked as resolved`);
        setConflicts(prev =>
          prev.map(c => c.filePath === selectedConflict ? { ...c, resolved: true } : c)
        );
        setSelectedConflict(null);
        setConflictContent(null);
        onConflictResolved?.();
      } else {
        alert(`Failed to resolve: ${result.error}`);
      }
    } catch (err) {
      alert(`Error: ${err}`);
    }
  };

  const handleCompleteMerge = async () => {
    const allResolved = conflicts.every(c => c.resolved);
    if (!allResolved) {
      alert('Please resolve all conflicts before completing merge');
      return;
    }

    try {
      setCompleting(true);
      const git = (window as any).git;
      const result = await git.completeMerge({
        projectDir,
        message: mergeMessage,
      });

      if (result.ok) {
        alert('Merge completed successfully');
        onMergeComplete?.();
      } else {
        alert(`Failed to complete merge: ${result.error}`);
      }
    } catch (err) {
      alert(`Error: ${err}`);
    } finally {
      setCompleting(false);
    }
  };

  const unresolvedCount = conflicts.filter(c => !c.resolved).length;
  const resolvedCount = conflicts.filter(c => c.resolved).length;

  return (
    <div>
      <div style={{ marginBottom: '12px', paddingBottom: '12px', borderBottom: '1px solid #444' }}>
        <div style={{ fontSize: '13px', fontWeight: 'bold', marginBottom: '8px' }}>
          Merge Conflicts: {unresolvedCount} unresolved, {resolvedCount} resolved
        </div>
      </div>

      <div className="git-history-list" style={{ maxHeight: '300px', overflow: 'auto', marginBottom: '12px' }}>
        {loading && <div className="git-history-loading">Loading conflicts...</div>}

        {!loading && conflicts.length === 0 && (
          <div className="git-history-empty">No conflicts found</div>
        )}

        {!loading && conflicts.map(conflict => (
          <div
            key={conflict.filePath}
            className="git-history-item"
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              opacity: conflict.resolved ? 0.6 : 1,
            }}
          >
            <div style={{ flex: 1 }}>
              <div className="git-history-message">
                {conflict.resolved ? '✓' : '⚠️'} {conflict.filePath}
              </div>
            </div>
            {!conflict.resolved && (
              <button
                className="git-panel-action-btn"
                onClick={() => handleViewConflict(conflict.filePath)}
                style={{ marginLeft: '8px', whiteSpace: 'nowrap' }}
              >
                Edit
              </button>
            )}
            {conflict.resolved && (
              <span style={{ color: '#6f6', marginRight: '8px' }}>Resolved</span>
            )}
          </div>
        ))}
      </div>

      {selectedConflict && (
        <div style={{ marginBottom: '12px', paddingBottom: '12px', borderBottom: '1px solid #444' }}>
          <div style={{ fontSize: '12px', fontWeight: 'bold', marginBottom: '8px' }}>
            Conflict in: {selectedConflict}
          </div>
          <div
            style={{
              backgroundColor: '#1a1a1a',
              border: '1px solid #444',
              borderRadius: '4px',
              padding: '8px',
              fontSize: '11px',
              fontFamily: 'monospace',
              maxHeight: '200px',
              overflow: 'auto',
              marginBottom: '8px',
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-all',
            }}
          >
            {conflictContent}
          </div>
          <div style={{ display: 'flex', gap: '8px' }}>
            <button
              className="git-panel-action-btn"
              onClick={handleResolveConflict}
              style={{ backgroundColor: '#28a745', borderColor: '#28a745' }}
            >
              Mark as Resolved
            </button>
            <button
              className="git-panel-action-btn"
              onClick={() => {
                setSelectedConflict(null);
                setConflictContent(null);
              }}
            >
              Close
            </button>
          </div>
        </div>
      )}

      {unresolvedCount === 0 && conflicts.length > 0 && (
        <div style={{ borderTop: '1px solid #444', paddingTop: '12px' }}>
          <div style={{ fontSize: '12px', marginBottom: '8px', fontWeight: 'bold' }}>
            All conflicts resolved. Complete merge?
          </div>
          <input
            type="text"
            className="git-commit-message"
            placeholder="Merge commit message"
            value={mergeMessage}
            onChange={(e) => setMergeMessage(e.target.value)}
            disabled={completing}
            style={{ minHeight: 'auto', padding: '6px 8px', fontSize: '12px', marginBottom: '8px' }}
          />
          <button
            className="git-commit-button"
            disabled={completing || !mergeMessage.trim()}
            onClick={handleCompleteMerge}
            style={{ backgroundColor: '#28a745', borderColor: '#28a745' }}
          >
            ✓ Complete Merge
          </button>
        </div>
      )}
    </div>
  );
};
