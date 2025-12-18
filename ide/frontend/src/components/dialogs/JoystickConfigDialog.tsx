import React, { useEffect, useState } from 'react';
import { useJoystickStore } from '../../state/joystickStore';

export const JoystickConfigDialog: React.FC = () => {
  const {
    isConfigOpen,
    setConfigOpen,
    connectedGamepads,
    updateGamepads,
    gamepadIndex,
    gamepadName,
    selectGamepad,
    axisXIndex,
    axisYIndex,
    axisXInverted,
    axisYInverted,
    deadzone,
    buttonMappings,
    setAxisXIndex,
    setAxisYIndex,
    setAxisXInverted,
    setAxisYInverted,
    setDeadzone,
    setButtonMapping,
    resetConfig,
  } = useJoystickStore();

  const [listeningForButton, setListeningForButton] = useState<number | null>(null);
  const [currentAxisValues, setCurrentAxisValues] = useState<number[]>([]);

  // Poll gamepads
  useEffect(() => {
    if (!isConfigOpen) return;

    const interval = setInterval(() => {
      const gamepads = navigator.getGamepads();
      const connected = Array.from(gamepads).filter((g): g is Gamepad => g !== null);
      updateGamepads(connected);

      // Update axis values for visualization
      if (gamepadIndex !== null && connected[gamepadIndex]) {
        setCurrentAxisValues(Array.from(connected[gamepadIndex].axes));
      }
    }, 100);

    return () => clearInterval(interval);
  }, [isConfigOpen, gamepadIndex, updateGamepads]);

  // Listen for button presses when mapping
  useEffect(() => {
    if (listeningForButton === null || !isConfigOpen) return;

    const interval = setInterval(() => {
      const gamepads = navigator.getGamepads();
      if (gamepadIndex !== null && gamepads[gamepadIndex]) {
        const gamepad = gamepads[gamepadIndex];
        const pressedButton = gamepad.buttons.findIndex(b => b.pressed);
        if (pressedButton !== -1) {
          setButtonMapping(listeningForButton, pressedButton);
          setListeningForButton(null);
        }
      }
    }, 50);

    return () => clearInterval(interval);
  }, [listeningForButton, gamepadIndex, isConfigOpen, setButtonMapping]);

  if (!isConfigOpen) return null;

  const vectrexButtonNames = ['Button 1', 'Button 2', 'Button 3', 'Button 4'];

  const getButtonName = (index: number) => {
    // Common gamepad button names
    const names: Record<number, string> = {
      0: 'A / Cross',
      1: 'B / Circle',
      2: 'X / Square',
      3: 'Y / Triangle',
      4: 'LB / L1',
      5: 'RB / R1',
      6: 'LT / L2',
      7: 'RT / R2',
      8: 'Select / Share',
      9: 'Start / Options',
      10: 'L3 (Left Stick)',
      11: 'R3 (Right Stick)',
      12: 'D-Pad Up',
      13: 'D-Pad Down',
      14: 'D-Pad Left',
      15: 'D-Pad Right',
    };
    return names[index] || `Button ${index}`;
  };

  const applyDeadzone = (value: number, dz: number) => {
    return Math.abs(value) < dz ? 0 : value;
  };

  return (
    <div style={{
      position: 'fixed',
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
      background: 'rgba(0,0,0,0.7)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      zIndex: 10000,
    }}>
      <div style={{
        background: '#1e1e1e',
        border: '1px solid #3c3c3c',
        borderRadius: 8,
        width: '90%',
        maxWidth: 700,
        maxHeight: '90vh',
        overflow: 'auto',
        padding: 20,
        color: '#cccccc',
      }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 20 }}>
          <h2 style={{ margin: 0 }}>🎮 Joystick Configuration</h2>
          <button
            onClick={() => setConfigOpen(false)}
            style={{
              background: 'transparent',
              border: 'none',
              color: '#cccccc',
              fontSize: 24,
              cursor: 'pointer',
              padding: 0,
            }}
          >
            ×
          </button>
        </div>

        {/* Gamepad Selection */}
        <div style={{ marginBottom: 20 }}>
          <h3 style={{ fontSize: 14, marginBottom: 10, color: '#ffffff' }}>Select Gamepad</h3>
          {connectedGamepads.length === 0 ? (
            <p style={{ color: '#ff6b6b', fontSize: 13 }}>
              ⚠️ No gamepads detected. Connect a gamepad and press any button.
            </p>
          ) : (
            <select
              value={gamepadIndex ?? ''}
              onChange={(e) => {
                const idx = parseInt(e.target.value);
                const gp = connectedGamepads[idx];
                if (gp) selectGamepad(idx, gp.id);
              }}
              style={{
                width: '100%',
                padding: 8,
                background: '#252526',
                border: '1px solid #3c3c3c',
                color: '#cccccc',
                borderRadius: 4,
              }}
            >
              <option value="">-- Select Gamepad --</option>
              {connectedGamepads.map((gp, idx) => (
                <option key={idx} value={idx}>
                  {gp.id} (Index {idx})
                </option>
              ))}
            </select>
          )}
          {gamepadName && (
            <p style={{ fontSize: 12, color: '#888', marginTop: 5 }}>
              Selected: {gamepadName}
            </p>
          )}
        </div>

        {gamepadIndex !== null && connectedGamepads[gamepadIndex] && (
          <>
            {/* Analog Stick Configuration */}
            <div style={{ marginBottom: 20, padding: 15, background: '#252526', borderRadius: 6 }}>
              <h3 style={{ fontSize: 14, marginBottom: 10, color: '#ffffff' }}>Analog Stick (Vectrex Joystick)</h3>
              
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 15 }}>
                <div>
                  <label style={{ display: 'block', fontSize: 12, marginBottom: 5 }}>X Axis:</label>
                  <select
                    value={axisXIndex}
                    onChange={(e) => setAxisXIndex(parseInt(e.target.value))}
                    style={{
                      width: '100%',
                      padding: 6,
                      background: '#1e1e1e',
                      border: '1px solid #3c3c3c',
                      color: '#cccccc',
                      borderRadius: 4,
                      fontSize: 12,
                    }}
                  >
                    {connectedGamepads[gamepadIndex].axes.map((_, idx) => (
                      <option key={idx} value={idx}>Axis {idx}</option>
                    ))}
                  </select>
                  <label style={{ display: 'flex', alignItems: 'center', marginTop: 5, fontSize: 12 }}>
                    <input
                      type="checkbox"
                      checked={axisXInverted}
                      onChange={(e) => setAxisXInverted(e.target.checked)}
                      style={{ marginRight: 5 }}
                    />
                    Invert X
                  </label>
                </div>

                <div>
                  <label style={{ display: 'block', fontSize: 12, marginBottom: 5 }}>Y Axis:</label>
                  <select
                    value={axisYIndex}
                    onChange={(e) => setAxisYIndex(parseInt(e.target.value))}
                    style={{
                      width: '100%',
                      padding: 6,
                      background: '#1e1e1e',
                      border: '1px solid #3c3c3c',
                      color: '#cccccc',
                      borderRadius: 4,
                      fontSize: 12,
                    }}
                  >
                    {connectedGamepads[gamepadIndex].axes.map((_, idx) => (
                      <option key={idx} value={idx}>Axis {idx}</option>
                    ))}
                  </select>
                  <label style={{ display: 'flex', alignItems: 'center', marginTop: 5, fontSize: 12 }}>
                    <input
                      type="checkbox"
                      checked={axisYInverted}
                      onChange={(e) => setAxisYInverted(e.target.checked)}
                      style={{ marginRight: 5 }}
                    />
                    Invert Y
                  </label>
                </div>
              </div>

              {/* Deadzone */}
              <div style={{ marginTop: 15 }}>
                <label style={{ display: 'block', fontSize: 12, marginBottom: 5 }}>
                  Deadzone: {(deadzone * 100).toFixed(0)}%
                </label>
                <input
                  type="range"
                  min="0"
                  max="0.5"
                  step="0.01"
                  value={deadzone}
                  onChange={(e) => setDeadzone(parseFloat(e.target.value))}
                  style={{ width: '100%' }}
                />
              </div>

              {/* Axis Visualizer */}
              {currentAxisValues.length > 0 && (
                <div style={{ marginTop: 15, padding: 10, background: '#1e1e1e', borderRadius: 4 }}>
                  <div style={{ fontSize: 11, marginBottom: 5 }}>Live Preview:</div>
                  <div style={{ display: 'flex', gap: 10, fontSize: 11 }}>
                    <div>
                      X: {applyDeadzone(
                        (axisXInverted ? -1 : 1) * (currentAxisValues[axisXIndex] || 0),
                        deadzone
                      ).toFixed(2)}
                    </div>
                    <div>
                      Y: {applyDeadzone(
                        (axisYInverted ? -1 : 1) * (currentAxisValues[axisYIndex] || 0),
                        deadzone
                      ).toFixed(2)}
                    </div>
                  </div>
                </div>
              )}
            </div>

            {/* Button Mappings */}
            <div style={{ marginBottom: 20 }}>
              <h3 style={{ fontSize: 14, marginBottom: 10, color: '#ffffff' }}>Button Mapping (Vectrex: 4 buttons)</h3>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 10 }}>
                {vectrexButtonNames.map((name, idx) => {
                  const vectrexBtn = idx + 1;
                  const mapping = buttonMappings.find(m => m.vectrexButton === vectrexBtn);
                  const isListening = listeningForButton === vectrexBtn;

                  return (
                    <div
                      key={vectrexBtn}
                      style={{
                        padding: 10,
                        background: isListening ? '#2a4a2a' : '#252526',
                        border: isListening ? '2px solid #4ec94e' : '1px solid #3c3c3c',
                        borderRadius: 4,
                      }}
                    >
                      <div style={{ fontSize: 12, fontWeight: 'bold', marginBottom: 5 }}>{name}</div>
                      <div style={{ fontSize: 11, color: '#888', marginBottom: 8 }}>
                        {mapping
                          ? `Mapped to: ${getButtonName(mapping.gamepadButton)}`
                          : 'Not mapped'}
                      </div>
                      <button
                        onClick={() => setListeningForButton(vectrexBtn)}
                        disabled={isListening}
                        style={{
                          padding: '4px 10px',
                          background: isListening ? '#4ec94e' : '#0e639c',
                          border: 'none',
                          color: '#fff',
                          borderRadius: 3,
                          cursor: isListening ? 'default' : 'pointer',
                          fontSize: 11,
                          width: '100%',
                        }}
                      >
                        {isListening ? 'Press gamepad button...' : 'Set Button'}
                      </button>
                    </div>
                  );
                })}
              </div>
            </div>
          </>
        )}

        {/* Actions */}
        <div style={{ display: 'flex', gap: 10, justifyContent: 'flex-end', paddingTop: 15, borderTop: '1px solid #3c3c3c' }}>
          <button
            onClick={resetConfig}
            style={{
              padding: '8px 16px',
              background: '#e74c3c',
              border: 'none',
              color: '#fff',
              borderRadius: 4,
              cursor: 'pointer',
              fontSize: 13,
            }}
          >
            Reset to Default
          </button>
          <button
            onClick={() => setConfigOpen(false)}
            style={{
              padding: '8px 16px',
              background: '#0e639c',
              border: 'none',
              color: '#fff',
              borderRadius: 4,
              cursor: 'pointer',
              fontSize: 13,
            }}
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
