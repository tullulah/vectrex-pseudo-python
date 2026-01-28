import React, { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useEditorStore } from '../../state/editorStore';
import { useDebugStore } from '../../state/debugStore';

interface BreakpointInfo {
  address: number;
  uri: string;
  line: number;
  enabled: boolean;
}

export const BreakpointsPanel: React.FC = () => {
  const { t } = useTranslation(['common']);
  const breakpoints = useEditorStore((s) => s.breakpoints);
  const toggleBreakpoint = useEditorStore((s) => s.toggleBreakpoint);
  const clearAllBreakpoints = useEditorStore((s) => s.clearAllBreakpoints);
  const [breakpointList, setBreakpointList] = useState<BreakpointInfo[]>([]);
  const [disabledBreakpoints, setDisabledBreakpoints] = useState<Set<string>>(new Set());

  // Convert breakpoints to list with file info
  useEffect(() => {
    const list: BreakpointInfo[] = [];
    Object.entries(breakpoints).forEach(([uri, lines]) => {
      const fileName = uri.split('/').pop() || uri;
      (lines as Set<number>).forEach((line: number) => {
        const key = `${uri}:${line}`;
        list.push({
          address: 0, // Address will be resolved from PDB if needed
          uri,
          line,
          enabled: !disabledBreakpoints.has(key)
        });
      });
    });
    
    // Sort by file name, then line number
    list.sort((a, b) => {
      const fileA = a.uri.split('/').pop() || '';
      const fileB = b.uri.split('/').pop() || '';
      if (fileA !== fileB) return fileA.localeCompare(fileB);
      return a.line - b.line;
    });
    
    setBreakpointList(list);
  }, [breakpoints, disabledBreakpoints]);

  const handleRemove = (uri: string, line: number) => {
    toggleBreakpoint(uri, line);
  };

  const handleToggleEnabled = (uri: string, line: number) => {
    const key = `${uri}:${line}`;
    setDisabledBreakpoints(prev => {
      const next = new Set(prev);
      if (next.has(key)) {
        next.delete(key);
        // Re-enable in emulator
        if ((window as any).__emulatorDebug?.addBreakpoint) {
          const pdbData = useDebugStore.getState().pdbData;
          if (pdbData && pdbData.asmAddressMap) {
            const asmLine = pdbData.lineMap[uri]?.[line];
            if (asmLine !== undefined) {
              const address = pdbData.asmAddressMap[asmLine];
              if (address !== undefined) {
                (window as any).__emulatorDebug.addBreakpoint(address);
              }
            }
          }
        }
      } else {
        next.add(key);
        // Disable in emulator
        if ((window as any).__emulatorDebug?.removeBreakpoint) {
          const pdbData = useDebugStore.getState().pdbData;
          if (pdbData && pdbData.asmAddressMap) {
            const asmLine = pdbData.lineMap[uri]?.[line];
            if (asmLine !== undefined) {
              const address = pdbData.asmAddressMap[asmLine];
              if (address !== undefined) {
                (window as any).__emulatorDebug.removeBreakpoint(address);
              }
            }
          }
        }
      }
      return next;
    });
  };

  const handleClearAll = () => {
    if (confirm(t('breakpoints.confirmClearAll') || 'Remove all breakpoints?')) {
      clearAllBreakpoints();
      setDisabledBreakpoints(new Set());
    }
  };

  const handleGoToLocation = (uri: string, line: number) => {
    // Open file and go to line
    const Monaco = (window as any).monaco;
    if (Monaco) {
      // Find editor instance
      const editors = Monaco.editor.getEditors();
      if (editors && editors.length > 0) {
        const editor = editors[0];
        editor.revealLineInCenter(line);
        editor.setPosition({ lineNumber: line, column: 1 });
        editor.focus();
      }
    }
  };

  const totalCount = breakpointList.length;
  const enabledCount = breakpointList.filter(bp => bp.enabled).length;

  return (
    <div style={{ 
      padding: '8px',
      height: '100%',
      display: 'flex',
      flexDirection: 'column',
      backgroundColor: '#1e1e1e',
      color: '#d4d4d4'
    }}>
      {/* Header */}
      <div style={{ 
        display: 'flex', 
        justifyContent: 'space-between', 
        alignItems: 'center', 
        marginBottom: '8px',
        paddingBottom: '8px',
        borderBottom: '1px solid #404040'
      }}>
        <strong style={{ fontSize: '13px' }}>
          {t('panel.breakpoints') || 'Breakpoints'} ({enabledCount}/{totalCount})
        </strong>
        <button 
          onClick={handleClearAll}
          disabled={totalCount === 0}
          style={{ 
            padding: '4px 8px', 
            fontSize: '11px',
            backgroundColor: totalCount > 0 ? '#c23030' : '#444',
            color: totalCount > 0 ? 'white' : '#888',
            border: '1px solid #666',
            borderRadius: '3px',
            cursor: totalCount > 0 ? 'pointer' : 'not-allowed'
          }}
          title={t('breakpoints.clearAll') || 'Clear All Breakpoints'}
        >
          Clear All
        </button>
      </div>

      {/* Breakpoint List */}
      <div style={{ 
        flex: 1, 
        overflowY: 'auto',
        fontSize: '12px'
      }}>
        {breakpointList.length === 0 ? (
          <div style={{ 
            padding: '16px', 
            textAlign: 'center', 
            color: '#888',
            fontStyle: 'italic'
          }}>
            {t('breakpoints.noBreakpoints') || 'No breakpoints set. Click in the gutter to add.'}
          </div>
        ) : (
          breakpointList.map((bp, idx) => {
            const fileName = bp.uri.split('/').pop() || bp.uri;
            const key = `${bp.uri}:${bp.line}`;
            
            return (
              <div 
                key={key}
                style={{ 
                  display: 'flex',
                  alignItems: 'center',
                  padding: '6px 8px',
                  marginBottom: '2px',
                  backgroundColor: idx % 2 === 0 ? '#252526' : '#2d2d30',
                  borderRadius: '3px',
                  opacity: bp.enabled ? 1 : 0.5
                }}
              >
                {/* Enabled/Disabled Checkbox */}
                <input 
                  type="checkbox" 
                  checked={bp.enabled}
                  onChange={() => handleToggleEnabled(bp.uri, bp.line)}
                  style={{ 
                    marginRight: '8px',
                    cursor: 'pointer',
                    width: '14px',
                    height: '14px'
                  }}
                  title={bp.enabled ? 'Disable breakpoint' : 'Enable breakpoint'}
                />

                {/* File and Line Info */}
                <div 
                  style={{ 
                    flex: 1,
                    cursor: 'pointer',
                    fontFamily: 'monospace'
                  }}
                  onClick={() => handleGoToLocation(bp.uri, bp.line)}
                  title={`${bp.uri}:${bp.line}`}
                >
                  <span style={{ color: '#4EC9B0' }}>{fileName}</span>
                  <span style={{ color: '#888' }}>:</span>
                  <span style={{ color: '#B5CEA8' }}>{bp.line}</span>
                </div>

                {/* Delete Button */}
                <button 
                  onClick={() => handleRemove(bp.uri, bp.line)}
                  style={{ 
                    marginLeft: '8px',
                    padding: '2px 6px',
                    fontSize: '11px',
                    backgroundColor: 'transparent',
                    color: '#c23030',
                    border: '1px solid #c23030',
                    borderRadius: '3px',
                    cursor: 'pointer'
                  }}
                  title="Remove breakpoint"
                >
                  ✕
                </button>
              </div>
            );
          })
        )}
      </div>

      {/* Footer with keyboard shortcuts */}
      <div style={{ 
        marginTop: '8px',
        paddingTop: '8px',
        borderTop: '1px solid #404040',
        fontSize: '11px',
        color: '#888'
      }}>
        <div>💡 Click gutter to toggle • F9 to toggle at cursor</div>
      </div>
    </div>
  );
};
