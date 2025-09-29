import React from 'react';
import { useTranslation } from 'react-i18next';
import './ConflictDialog.css';

interface ConflictDialogProps {
  isOpen: boolean;
  filePath: string;
  currentMTime: number;
  expectedMTime?: number;
  onReload: () => void;
  onForce: () => void;
  onCancel?: () => void;
}

export const ConflictDialog: React.FC<ConflictDialogProps> = ({
  isOpen,
  filePath,
  currentMTime,
  expectedMTime,
  onReload,
  onForce,
  onCancel
}) => {
  const { t } = useTranslation();

  if (!isOpen) return null;

  const fileName = filePath.split('/').pop() || filePath.split('\\').pop() || filePath;

  return (
    <div className="conflict-dialog-overlay" onClick={onCancel}>
      <div className="conflict-dialog" onClick={(e) => e.stopPropagation()}>
        <div className="conflict-header">
          <h3>{t('dialog.conflict.title', 'File Conflict Detected')}</h3>
        </div>
        
        <div className="conflict-content">
          <div className="conflict-message">
            <p>
              {t('dialog.conflict.message', 'The file "{{fileName}}" has been modified externally during compilation.', { fileName })}
            </p>
          </div>
          
          <div className="conflict-details">
            <div className="time-info">
              <div className="time-row">
                <span className="time-label">{t('dialog.conflict.currentTime', 'Current file time:')}</span>
                <span className="time-value">{new Date(currentMTime).toLocaleString()}</span>
              </div>
              <div className="time-row">
                <span className="time-label">{t('dialog.conflict.expectedTime', 'Expected time:')}</span>
                <span className="time-value">{expectedMTime ? new Date(expectedMTime).toLocaleString() : t('dialog.conflict.unknown', 'unknown')}</span>
              </div>
            </div>
          </div>
          
          <div className="conflict-explanation">
            <p>{t('dialog.conflict.explanation', 'This means the file was changed by another program while you were editing it.')}</p>
          </div>
        </div>
        
        <div className="conflict-actions">
          <button 
            className="conflict-btn conflict-btn-primary" 
            onClick={onReload}
            title={t('dialog.conflict.reloadTooltip', 'Load the external changes and lose your current changes')}
          >
            🔄 {t('dialog.conflict.reload', 'Reload External Changes')}
          </button>
          
          <button 
            className="conflict-btn conflict-btn-warning" 
            onClick={onForce}
            title={t('dialog.conflict.forceTooltip', 'Keep your changes and overwrite the external file')}
          >
            ⚠️ {t('dialog.conflict.force', 'Keep My Changes')}
          </button>
          
          {onCancel && (
            <button 
              className="conflict-btn conflict-btn-secondary" 
              onClick={onCancel}
              title={t('dialog.conflict.cancelTooltip', 'Cancel the operation')}
            >
              ❌ {t('dialog.conflict.cancel', 'Cancel')}
            </button>
          )}
        </div>
      </div>
    </div>
  );
};