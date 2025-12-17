# Vectrex Hardware Reference

## Display System:
- Vector CRT display (not raster/pixel-based)
- Electron beam draws lines directly
- Intensity controls line brightness (CRITICAL: use ≤127 for safe display)
- No frame buffer - real-time drawing

## Safe Intensity Values (ALWAYS USE THESE):
- **127 (0x7F)**: Maximum safe brightness (bright, clear lines)
- **80 (0x50)**: Medium brightness (recommended for most graphics)
- **64 (0x40)**: Low-medium brightness (good for background elements)
- **48 (0x30)**: Dim (subtle effects)
- **0**: Invisible (beam off)

⚠️ **NEVER use intensity values above 127** - values like 150, 200, 255 cause:
- CRT phosphor oversaturation
- Lines become invisible or distorted
- Potential burn-in damage on real hardware
- Emulator may show incorrect behavior

## Memory Map:
- 0x0000-0xBFFF: Cartridge ROM space
- 0xC800-0xCFFF: System RAM (1KB)
- 0xD000-0xD7FF: 6522 VIA (I/O)
- 0xE000-0xFFFF: System ROM (BIOS)

## Sound System:
- General Instruments AY-3-8912 PSG
- 3 tone channels + 1 noise channel
- Memory-mapped at 0xD000-0xD001

## Input System:
- 4 controller ports
- Analog joystick (X/Y axes)
- 4 digital buttons per controller
- Light pen support

## BIOS Functions:
- F312: Move beam to absolute position
- F3DF: Draw line with relative displacement
- F373: Set beam intensity
- F37A: Print text using ROM character set
- F36B: Reset coordinate origin

## Hardware Constraints:
- 1KB RAM total (0xC800-0xCFFF)
- 8K ROM BIOS (0xE000-0xFFFF)
- Motorola 6809 CPU @ 1.5 MHz
- Vector display with X/Y deflection
- 4-channel sound via AY-3-8912 PSG

## 🚨 CRITICAL Vectrex Coordinate System:
- **Screen center is (0, 0)** - NOT top-left corner!
- **Valid range: -127 to +127** for both X and Y axes
- **X axis**: -127 (far left) → 0 (center) → +127 (far right)
- **Y axis**: -127 (bottom) → 0 (center) → +127 (top)
- **Examples**:
  - Top-left corner: (-127, 127)
  - Top-right corner: (127, 127)
  - Bottom-left corner: (-127, -127)
  - Bottom-right corner: (127, -127)
  - Center: (0, 0)
