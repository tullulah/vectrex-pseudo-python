import React, { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { emuCore } from '../../emulatorCoreSingleton';

export const DebugPanel: React.FC = () => {
  const { t } = useTranslation(['common']);
  const [debugMessages, setDebugMessages] = useState<string[]>([]);
  const [lastOutput, setLastOutput] = useState<string>('');

  // Poll for debug messages every 100ms
  useEffect(() => {
    const interval = setInterval(() => {
      if (emuCore.getDebugMessages) {
        const messages = emuCore.getDebugMessages();
        if (messages.length !== debugMessages.length) {
          setDebugMessages([...messages]);
        }
      }
      
      if (emuCore.getLastDebugOutput) {
        const current = emuCore.getLastDebugOutput();
        if (current !== lastOutput) {
          setLastOutput(current);
        }
      }
    }, 100);

    return () => clearInterval(interval);
  }, [debugMessages.length, lastOutput]);

  const clearMessages = () => {
    if (emuCore.clearDebugMessages) {
      emuCore.clearDebugMessages();
      setDebugMessages([]);
      setLastOutput('');
    }
  };

  return (
    <div style={{ padding: 8 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 8 }}>
        <strong>{t('panel.debug')}</strong>
        <button 
          onClick={clearMessages}
          style={{ 
            padding: '4px 8px', 
            fontSize: '12px',
            backgroundColor: '#444',
            color: 'white',
            border: '1px solid #666',
            borderRadius: '3px',
            cursor: 'pointer'
          }}
        >
          Clear
        </button>
      </div>
      
      {lastOutput && (
        <div style={{ 
          marginBottom: 8, 
          padding: 6, 
          backgroundColor: '#2a2a2a', 
          border: '1px solid #555',
          borderRadius: 3,
          fontFamily: 'monospace',
          fontSize: '13px',
          color: '#90EE90'
        }}>
          <strong>Last:</strong> {lastOutput}
        </div>
      )}
      
      <div style={{ 
        maxHeight: '300px', 
        overflowY: 'auto',
        border: '1px solid #555',
        backgroundColor: '#1a1a1a',
        borderRadius: 3
      }}>
        {debugMessages.length === 0 ? (
          <div style={{ 
            padding: 8, 
            color: '#888', 
            fontStyle: 'italic',
            textAlign: 'center'
          }}>
            No debug output yet.<br/>
            Use DEBUG_PRINT(value) or DEBUG_PRINT_LABELED("label", value) in VPy code.
          </div>
        ) : (
          debugMessages.map((message, index) => (
            <div 
              key={index} 
              style={{ 
                padding: '4px 8px',
                borderBottom: index < debugMessages.length - 1 ? '1px solid #333' : 'none',
                fontFamily: 'monospace',
                fontSize: '12px',
                color: '#E0E0E0'
              }}
            >
              <span style={{ color: '#666', marginRight: 8 }}>
                [{String(index + 1).padStart(3, '0')}]
              </span>
              {message}
            </div>
          ))
        )}
      </div>
      
      <div style={{ 
        marginTop: 8, 
        fontSize: '11px', 
        color: '#666',
        fontStyle: 'italic'
      }}>
        Debug output from VPy DEBUG_PRINT functions. Messages are captured from memory writes to $DFFF.
      </div>
    </div>
  );
};
