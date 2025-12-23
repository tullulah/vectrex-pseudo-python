#!/usr/bin/env python3
"""Replace compile_to_asm in sfxres.rs with AYFX generator"""

import re

# Read the file
with open('core/src/sfxres.rs', 'r') as f:
    content = f.read()

# Find the compile_to_asm function
pattern = r'(    pub fn compile_to_asm\(&self\) -> String \{)(.*?)(^    \})'
match = re.search(pattern, content, re.MULTILINE | re.DOTALL)

if not match:
    print("ERROR: Could not find compile_to_asm function")
    exit(1)

print(f"Found compile_to_asm function at position {match.start()}")

# New AYFX generator
new_function = '''    pub fn compile_to_asm(&self) -> String {
        let label = format!("_{}_SFX", self.name.to_uppercase().replace(" ", "_").replace("-", "_"));
        
        // Calculate PSG period from frequency
        // PSG clock = 1.5 MHz, period = clock / (32 * freq)
        let base_period = if self.oscillator.frequency > 0 {
            (1_500_000u32 / (32 * self.oscillator.frequency as u32)).min(4095) as u16
        } else {
            440 // Default to A4 (440Hz) = period 106
        };
        
        // Duration in frames (50 FPS for Vectrex)
        let total_frames = (self.duration_ms as u32 * 50 / 1000).max(1) as usize;
        
        // Envelope timing
        let attack_frames = ((self.envelope.attack as u32 * 50 / 1000).max(1) as f32).min(total_frames as f32 * 0.3) as usize;
        let decay_frames = ((self.envelope.decay as u32 * 50 / 1000).max(1) as f32).min(total_frames as f32 * 0.3) as usize;
        let release_frames = ((self.envelope.release as u32 * 50 / 1000).max(1) as f32).min(total_frames as f32 * 0.3) as usize;
        let sustain_frames = total_frames.saturating_sub(attack_frames + decay_frames + release_frames);
        
        let mut asm = String::new();
        asm.push_str(&format!("{}:\n", label));
        asm.push_str(&format!("    ; SFX: {} ({})\n", self.name, 
            format!("{:?}", self.category).to_lowercase()));
        asm.push_str(&format!("    ; Duration: {}ms ({}fr), Freq: {}Hz, Channel: {}\n",
            self.duration_ms, total_frames, self.oscillator.frequency, self.oscillator.channel));
        
        // Generate AYFX frame-by-frame
        let mut last_period: Option<u16> = None;
        let mut last_noise: Option<u8> = None;
        
        for frame in 0..total_frames {
            // Calculate envelope volume (ADSR)
            let volume = if frame < attack_frames {
                // Attack phase: 0 -> peak
                ((frame as f32 / attack_frames as f32) * self.envelope.peak as f32) as u8
            } else if frame < attack_frames + decay_frames {
                // Decay phase: peak -> sustain
                let decay_progress = (frame - attack_frames) as f32 / decay_frames as f32;
                let vol_diff = self.envelope.peak.saturating_sub(self.envelope.sustain) as f32;
                self.envelope.peak.saturating_sub((decay_progress * vol_diff) as u8)
            } else if frame < attack_frames + decay_frames + sustain_frames {
                // Sustain phase: constant
                self.envelope.sustain
            } else {
                // Release phase: sustain -> 0
                let release_progress = (frame - attack_frames - decay_frames - sustain_frames) as f32 / release_frames as f32;
                ((1.0 - release_progress) * self.envelope.sustain as f32) as u8
            };
            
            // Calculate pitch sweep if enabled
            let mut current_period = base_period;
            if self.pitch.enabled && total_frames > 1 {
                let t = frame as f32 / (total_frames - 1) as f32;
                let mult = self.pitch.start_mult + (self.pitch.end_mult - self.pitch.start_mult) * t;
                current_period = ((base_period as f32) * mult) as u16;
                current_period = current_period.max(1).min(4095);
            }
            
            // Build flag byte
            let mut flag: u8 = volume & 0x0F; // Bits 0-3: volume
            
            // VRelease optimization: only include data when it changes
            let include_tone = last_period != Some(current_period);
            let include_noise = self.noise.enabled && last_noise != Some(self.noise.period);
            
            if include_tone {
                flag |= 0x20; // Bit 5: tone data present
            }
            if include_noise {
                flag |= 0x40; // Bit 6: noise data present
            }
            
            // Bit 4: disable tone (never for simple SFX)
            // Bit 7: disable noise (set if noise not enabled)
            if !self.noise.enabled {
                flag |= 0x80;
            }
            
            // Emit frame data
            asm.push_str(&format!("    FCB ${:02X}         ; Frame {} - flags (vol={}, tone={}, noise={})\n",
                flag, frame, volume,
                if include_tone { "Y" } else { "N" },
                if include_noise { "Y" } else { "N" }
            ));
            
            // Emit tone frequency if changed (big-endian for M6809 LDX)
            if include_tone {
                let high = (current_period >> 8) & 0xFF;
                let low = current_period & 0xFF;
                asm.push_str(&format!("    FCB ${:02X}, ${:02X}  ; Tone period = {} (big-endian)\n",
                    high, low, current_period));
                last_period = Some(current_period);
            }
            
            // Emit noise period if changed
            if include_noise {
                asm.push_str(&format!("    FCB ${:02X}         ; Noise period\n", self.noise.period));
                last_noise = Some(self.noise.period);
            }
        }
        
        // End marker
        asm.push_str("    FCB $D0, $20    ; End of effect marker\n");
        asm.push_str("\n");
        
        asm
    }'''

# Replace
new_content = content[:match.start()] + new_function + content[match.end():]

# Write back
with open('core/src/sfxres.rs', 'w') as f:
    f.write(new_content)

print("✓ Replaced compile_to_asm with AYFX generator")
print("✓ Format: frame-by-frame with envelope + pitch sweep + end marker")
