import React, { useState, useEffect } from 'react';
import '../modals/DiffViewer.css';

interface DiffLine {
  type: 'header' | 'added' | 'removed' | 'context';
  content: string;
  lineNumber?: number;
}

interface DiffViewerProps {
  filePath: string;
  projectDir: string;
  onClose: () => void;
}

export const DiffViewer: React.FC<DiffViewerProps> = ({ filePath, projectDir, onClose }) => {
  const [diffLines, setDiffLines] = useState<DiffLine[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadDiff = async () => {
      try {
        setLoading(true);
        const git = (window as any).git;
        const result = await git.diff({ projectDir, path: filePath });
        
        if (!result.ok) {
          setError(result.error || 'Failed to load diff');
          return;
        }

        // Parse unified diff format
        const lines = result.data.split('\n');
        const parsed: DiffLine[] = [];

        lines.forEach((line: string) => {
          if (line.startsWith('+++') || line.startsWith('---') || line.startsWith('@@')) {
            parsed.push({
              type: 'header',
              content: line,
            });
          } else if (line.startsWith('+')) {
            parsed.push({
              type: 'added',
              content: line.substring(1),
            });
          } else if (line.startsWith('-')) {
            parsed.push({
              type: 'removed',
              content: line.substring(1),
            });
          } else {
            parsed.push({
              type: 'context',
              content: line,
            });
          }
        });

        setDiffLines(parsed);
        setError(null);
      } catch (err) {
        setError(String(err));
      } finally {
        setLoading(false);
      }
    };

    loadDiff();
  }, [filePath]);

  return (
    <div className="diff-viewer-overlay" onClick={onClose}>
      <div className="diff-viewer-modal" onClick={(e) => e.stopPropagation()}>
        <div className="diff-viewer-header">
          <h2>Diff: {filePath}</h2>
          <button className="diff-viewer-close" onClick={onClose}>✕</button>
        </div>

        <div className="diff-viewer-content">
          {loading && <div className="diff-viewer-loading">Loading diff...</div>}
          {error && <div className="diff-viewer-error">Error: {error}</div>}
          
          {!loading && !error && (
            <pre className="diff-viewer-text">
              {diffLines.map((line, idx) => (
                <div
                  key={idx}
                  className={`diff-line diff-${line.type}`}
                >
                  {line.type === 'header' ? (
                    <span className="diff-header-text">{line.content}</span>
                  ) : line.type === 'added' ? (
                    <span className="diff-added-text">+ {line.content}</span>
                  ) : line.type === 'removed' ? (
                    <span className="diff-removed-text">- {line.content}</span>
                  ) : (
                    <span className="diff-context-text">  {line.content}</span>
                  )}
                </div>
              ))}
            </pre>
          )}
        </div>
      </div>
    </div>
  );
};
