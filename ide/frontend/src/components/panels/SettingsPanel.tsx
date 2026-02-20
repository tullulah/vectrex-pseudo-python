import React from 'react';
import { useSettings } from '../../state/settingsStore';
import './SettingsPanel.css';

export const SettingsPanel: React.FC = () => {
  const { compiler, setCompiler } = useSettings();

  return (
    <div className="settings-panel">
      <h2>Settings</h2>
      
      <div className="settings-section">
        <h3>Compiler</h3>
        <p className="settings-description">
          Select which compiler backend to use for building VPy projects.
        </p>
        
        <div className="settings-option">
          <label className="settings-radio">
            <input
              type="radio"
              name="compiler"
              value="buildtools"
              checked={compiler === 'buildtools'}
              onChange={() => setCompiler('buildtools')}
            />
            <div className="radio-content">
              <span className="radio-title">Buildtools (New)</span>
              <span className="radio-description">
                Modular 9-phase pipeline. Supports multibank ROMs and PDB debug symbols.
                Some edge cases may not compile correctly yet.
              </span>
            </div>
          </label>
          
          <label className="settings-radio">
            <input
              type="radio"
              name="compiler"
              value="core"
              checked={compiler === 'core'}
              onChange={() => setCompiler('core')}
            />
            <div className="radio-content">
              <span className="radio-title">Core (Legacy)</span>
              <span className="radio-description">
                Original compiler. Stable and well-tested. Always outputs a fixed
                32KB ROM. No PDB debug symbols. Recommended for most projects.
              </span>
            </div>
          </label>
        </div>
      </div>
      
      <div className="settings-info">
        <p>
          <strong>Note:</strong> Changes take effect on the next build.
          The compiler setting is saved in your browser's local storage.
        </p>
      </div>
    </div>
  );
};
