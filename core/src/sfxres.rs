//! VPy Sound Effects Resource format (.vsfx)
//!
//! Sound effects stored as JSON with envelope and oscillator parameters.
//! Designed for simple arcade-style sounds (explosions, lasers, pickups).
//! Based on SFXR/BFXR parameter model, adapted for AY-3-8910 PSG.

use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Sound effects resource file extension
pub const VSFX_EXTENSION: &str = "vsfx";

/// Root structure of a .vsfx file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SfxResource {
    /// File format version
    #[serde(default = "default_version")]
    pub version: String,
    
    /// Effect name (used for symbol generation)
    pub name: String,
    
    /// Effect category/preset type
    #[serde(default)]
    pub category: SfxCategory,
    
    /// Duration in milliseconds (50-2000ms typical)
    #[serde(default = "default_duration")]
    pub duration_ms: u16,
    
    /// Waveform/oscillator settings
    #[serde(default)]
    pub oscillator: Oscillator,
    
    /// Amplitude envelope (ADSR-like)
    #[serde(default)]
    pub envelope: Envelope,
    
    /// Pitch envelope (for sweeps)
    #[serde(default)]
    pub pitch: PitchEnvelope,
    
    /// Noise settings
    #[serde(default)]
    pub noise: NoiseSettings,
    
    /// Arpeggio/vibrato effects
    #[serde(default)]
    pub modulation: Modulation,
}

fn default_version() -> String { "1.0".to_string() }
fn default_duration() -> u16 { 200 }

/// Preset categories for quick sound generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SfxCategory {
    #[default]
    Custom,
    Laser,
    Explosion,
    Powerup,
    Hit,
    Jump,
    Blip,
    Coin,
}

/// Oscillator/waveform settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Oscillator {
    /// Base frequency in Hz (110-880 typical, maps to PSG period)
    #[serde(default = "default_frequency")]
    pub frequency: u16,
    
    /// PSG channel to use (0=A, 1=B, 2=C)
    #[serde(default)]
    pub channel: u8,
    
    /// Duty cycle simulation via rapid on/off (0-100%)
    /// PSG only does square waves, but we can fake PWM
    #[serde(default = "default_duty")]
    pub duty: u8,
}

fn default_frequency() -> u16 { 440 }
fn default_duty() -> u8 { 50 }

impl Default for Oscillator {
    fn default() -> Self {
        Self {
            frequency: 440,
            channel: 0,
            duty: 50,
        }
    }
}

/// Amplitude envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    /// Attack time in ms (0-500)
    #[serde(default)]
    pub attack: u16,
    
    /// Decay time in ms (0-500)
    #[serde(default = "default_decay")]
    pub decay: u16,
    
    /// Sustain level (0-15, PSG volume)
    #[serde(default = "default_sustain")]
    pub sustain: u8,
    
    /// Release time in ms (0-1000)
    #[serde(default = "default_release")]
    pub release: u16,
    
    /// Peak volume (0-15)
    #[serde(default = "default_peak")]
    pub peak: u8,
}

fn default_decay() -> u16 { 50 }
fn default_sustain() -> u8 { 8 }
fn default_release() -> u16 { 100 }
fn default_peak() -> u8 { 15 }

impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack: 0,
            decay: 50,
            sustain: 8,
            release: 100,
            peak: 15,
        }
    }
}

/// Pitch envelope for frequency sweeps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PitchEnvelope {
    /// Enable pitch sweep
    #[serde(default)]
    pub enabled: bool,
    
    /// Start frequency multiplier (0.5 = half, 2.0 = double)
    #[serde(default = "default_start_mult")]
    pub start_mult: f32,
    
    /// End frequency multiplier
    #[serde(default = "default_end_mult")]
    pub end_mult: f32,
    
    /// Sweep curve (0=linear, positive=exponential up, negative=exponential down)
    #[serde(default)]
    pub curve: i8,
}

fn default_start_mult() -> f32 { 1.0 }
fn default_end_mult() -> f32 { 1.0 }

impl Default for PitchEnvelope {
    fn default() -> Self {
        Self {
            enabled: false,
            start_mult: 1.0,
            end_mult: 1.0,
            curve: 0,
        }
    }
}

/// Noise generator settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseSettings {
    /// Enable noise mixing
    #[serde(default)]
    pub enabled: bool,
    
    /// Noise period (0-31, lower = higher pitch)
    #[serde(default = "default_noise_period")]
    pub period: u8,
    
    /// Noise volume (0-15)
    #[serde(default = "default_noise_volume")]
    pub volume: u8,
    
    /// Noise envelope (independent decay)
    #[serde(default)]
    pub decay_ms: u16,
}

fn default_noise_period() -> u8 { 15 }
fn default_noise_volume() -> u8 { 12 }

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            period: 15,
            volume: 12,
            decay_ms: 100,
        }
    }
}

/// Modulation effects (arpeggio, vibrato)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Modulation {
    /// Arpeggio enabled (rapid note changes)
    #[serde(default)]
    pub arpeggio: bool,
    
    /// Arpeggio semitones (e.g., [0, 4, 7] for major chord)
    #[serde(default)]
    pub arpeggio_notes: Vec<i8>,
    
    /// Arpeggio speed (ms per note)
    #[serde(default = "default_arp_speed")]
    pub arpeggio_speed: u16,
    
    /// Vibrato enabled
    #[serde(default)]
    pub vibrato: bool,
    
    /// Vibrato depth in semitones
    #[serde(default)]
    pub vibrato_depth: u8,
    
    /// Vibrato speed (Hz)
    #[serde(default = "default_vibrato_speed")]
    pub vibrato_speed: u8,
}

fn default_arp_speed() -> u16 { 50 }
fn default_vibrato_speed() -> u8 { 8 }

impl SfxResource {
    /// Load a .vsfx resource from a file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let resource: SfxResource = serde_json::from_str(&content)?;
        Ok(resource)
    }
    
    /// Save the resource to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Create a new empty SFX resource
    pub fn new(name: &str) -> Self {
        Self {
            version: "1.0".to_string(),
            name: name.to_string(),
            category: SfxCategory::Custom,
            duration_ms: 200,
            oscillator: Oscillator::default(),
            envelope: Envelope::default(),
            pitch: PitchEnvelope::default(),
            noise: NoiseSettings::default(),
            modulation: Modulation::default(),
        }
    }
    
    /// Create preset: Laser shot
    pub fn preset_laser() -> Self {
        Self {
            name: "laser".to_string(),
            category: SfxCategory::Laser,
            duration_ms: 150,
            oscillator: Oscillator {
                frequency: 880,
                channel: 0,
                duty: 50,
            },
            envelope: Envelope {
                attack: 0,
                decay: 0,
                sustain: 12,
                release: 100,
                peak: 15,
            },
            pitch: PitchEnvelope {
                enabled: true,
                start_mult: 2.0,
                end_mult: 0.5,
                curve: -2,
            },
            noise: NoiseSettings::default(),
            modulation: Modulation::default(),
            ..Default::default()
        }
    }
    
    /// Create preset: Explosion
    pub fn preset_explosion() -> Self {
        Self {
            name: "explosion".to_string(),
            category: SfxCategory::Explosion,
            duration_ms: 400,
            oscillator: Oscillator {
                frequency: 110,
                channel: 0,
                duty: 50,
            },
            envelope: Envelope {
                attack: 5,
                decay: 50,
                sustain: 4,
                release: 300,
                peak: 15,
            },
            pitch: PitchEnvelope {
                enabled: true,
                start_mult: 1.5,
                end_mult: 0.3,
                curve: -3,
            },
            noise: NoiseSettings {
                enabled: true,
                period: 8,
                volume: 15,
                decay_ms: 350,
            },
            modulation: Modulation::default(),
            ..Default::default()
        }
    }
    
    /// Create preset: Powerup/Coin
    pub fn preset_powerup() -> Self {
        Self {
            name: "powerup".to_string(),
            category: SfxCategory::Powerup,
            duration_ms: 200,
            oscillator: Oscillator {
                frequency: 440,
                channel: 0,
                duty: 50,
            },
            envelope: Envelope {
                attack: 0,
                decay: 20,
                sustain: 10,
                release: 100,
                peak: 15,
            },
            pitch: PitchEnvelope {
                enabled: true,
                start_mult: 0.8,
                end_mult: 1.5,
                curve: 2,
            },
            noise: NoiseSettings::default(),
            modulation: Modulation {
                arpeggio: true,
                arpeggio_notes: vec![0, 4, 7, 12],
                arpeggio_speed: 40,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    /// Create preset: Hit/Damage
    pub fn preset_hit() -> Self {
        Self {
            name: "hit".to_string(),
            category: SfxCategory::Hit,
            duration_ms: 100,
            oscillator: Oscillator {
                frequency: 220,
                channel: 0,
                duty: 50,
            },
            envelope: Envelope {
                attack: 0,
                decay: 10,
                sustain: 6,
                release: 50,
                peak: 15,
            },
            pitch: PitchEnvelope::default(),
            noise: NoiseSettings {
                enabled: true,
                period: 12,
                volume: 14,
                decay_ms: 80,
            },
            modulation: Modulation::default(),
            ..Default::default()
        }
    }
    
    /// Create preset: Jump
    pub fn preset_jump() -> Self {
        Self {
            name: "jump".to_string(),
            category: SfxCategory::Jump,
            duration_ms: 180,
            oscillator: Oscillator {
                frequency: 330,
                channel: 0,
                duty: 50,
            },
            envelope: Envelope {
                attack: 0,
                decay: 30,
                sustain: 8,
                release: 100,
                peak: 14,
            },
            pitch: PitchEnvelope {
                enabled: true,
                start_mult: 0.6,
                end_mult: 1.3,
                curve: 1,
            },
            noise: NoiseSettings::default(),
            modulation: Modulation::default(),
            ..Default::default()
        }
    }
    
    /// Create preset: Blip (menu selection)
    pub fn preset_blip() -> Self {
        Self {
            name: "blip".to_string(),
            category: SfxCategory::Blip,
            duration_ms: 50,
            oscillator: Oscillator {
                frequency: 660,
                channel: 0,
                duty: 50,
            },
            envelope: Envelope {
                attack: 0,
                decay: 5,
                sustain: 10,
                release: 30,
                peak: 12,
            },
            pitch: PitchEnvelope::default(),
            noise: NoiseSettings::default(),
            modulation: Modulation::default(),
            ..Default::default()
        }
    }
    
    /// Compile to ASM data for embedding in ROM
    pub fn compile_to_asm(&self) -> String {
        let label = format!("_{}_SFX", self.name.to_uppercase().replace(" ", "_"));
        
        // Calculate PSG period from frequency
        // PSG clock = 1.5 MHz, period = clock / (32 * freq)
        let psg_period = if self.oscillator.frequency > 0 {
            1_500_000u32 / (32 * self.oscillator.frequency as u32)
        } else {
            0
        };
        let psg_period = psg_period.min(4095) as u16; // 12-bit max
        
        // Duration in frames (50 FPS for Vectrex)
        let duration_frames = (self.duration_ms as u32 * 50 / 1000).max(1) as u8;
        
        // Envelope times in frames
        let attack_frames = (self.envelope.attack as u32 * 50 / 1000).min(255) as u8;
        let decay_frames = (self.envelope.decay as u32 * 50 / 1000).min(255) as u8;
        let release_frames = (self.envelope.release as u32 * 50 / 1000).min(255) as u8;
        
        // Pitch sweep data (fixed-point 8.8)
        let start_mult_fp = (self.pitch.start_mult * 256.0) as u16;
        let end_mult_fp = (self.pitch.end_mult * 256.0) as u16;
        
        let mut asm = String::new();
        asm.push_str(&format!("{}:\n", label));
        asm.push_str(&format!("    ; SFX: {} ({})\n", self.name, format!("{:?}", self.category).to_lowercase()));
        
        // Header: flags + duration
        let flags: u8 = 
            (if self.pitch.enabled { 0x01 } else { 0 }) |
            (if self.noise.enabled { 0x02 } else { 0 }) |
            (if self.modulation.arpeggio { 0x04 } else { 0 }) |
            (if self.modulation.vibrato { 0x08 } else { 0 });
        
        asm.push_str(&format!("    FCB ${:02X}        ; flags (pitch={}, noise={}, arp={}, vib={})\n",
            flags,
            if self.pitch.enabled { 1 } else { 0 },
            if self.noise.enabled { 1 } else { 0 },
            if self.modulation.arpeggio { 1 } else { 0 },
            if self.modulation.vibrato { 1 } else { 0 }
        ));
        asm.push_str(&format!("    FCB {}         ; duration (frames)\n", duration_frames));
        asm.push_str(&format!("    FCB {}          ; channel\n", self.oscillator.channel));
        
        // Oscillator
        asm.push_str(&format!("    FDB {}        ; base period (PSG)\n", psg_period));
        
        // Envelope (ADSR)
        asm.push_str(&format!("    FCB {}, {}, {}, {} ; A, D, S, R (frames/level)\n",
            attack_frames, decay_frames, self.envelope.sustain, release_frames));
        asm.push_str(&format!("    FCB {}         ; peak volume\n", self.envelope.peak));
        
        // Pitch envelope (if enabled)
        if self.pitch.enabled {
            asm.push_str(&format!("    FDB ${:04X}     ; pitch start mult (8.8 fixed)\n", start_mult_fp));
            asm.push_str(&format!("    FDB ${:04X}     ; pitch end mult (8.8 fixed)\n", end_mult_fp));
            asm.push_str(&format!("    FCB {}          ; pitch curve\n", self.pitch.curve as i8));
        }
        
        // Noise (if enabled)
        if self.noise.enabled {
            let noise_decay_frames = (self.noise.decay_ms as u32 * 50 / 1000).min(255) as u8;
            asm.push_str(&format!("    FCB {}         ; noise period\n", self.noise.period));
            asm.push_str(&format!("    FCB {}         ; noise volume\n", self.noise.volume));
            asm.push_str(&format!("    FCB {}         ; noise decay (frames)\n", noise_decay_frames));
        }
        
        // Arpeggio (if enabled)
        if self.modulation.arpeggio && !self.modulation.arpeggio_notes.is_empty() {
            let arp_speed_frames = (self.modulation.arpeggio_speed as u32 * 50 / 1000).max(1).min(255) as u8;
            asm.push_str(&format!("    FCB {}          ; arpeggio note count\n", self.modulation.arpeggio_notes.len()));
            asm.push_str(&format!("    FCB {}          ; arpeggio speed (frames)\n", arp_speed_frames));
            for note in &self.modulation.arpeggio_notes {
                asm.push_str(&format!("    FCB {}          ; arpeggio semitone\n", *note));
            }
        }
        
        asm.push_str("\n");
        asm
    }
}

impl Default for SfxResource {
    fn default() -> Self {
        Self::new("untitled")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_laser() {
        let sfx = SfxResource::preset_laser();
        assert_eq!(sfx.category, SfxCategory::Laser);
        assert!(sfx.pitch.enabled);
        assert!(!sfx.noise.enabled);
    }

    #[test]
    fn test_preset_explosion() {
        let sfx = SfxResource::preset_explosion();
        assert_eq!(sfx.category, SfxCategory::Explosion);
        assert!(sfx.noise.enabled);
    }

    #[test]
    fn test_compile_to_asm() {
        let sfx = SfxResource::preset_blip();
        let asm = sfx.compile_to_asm();
        assert!(asm.contains("_BLIP_SFX:"));
        assert!(asm.contains("FCB")); // Has byte data
    }

    #[test]
    fn test_json_roundtrip() {
        let original = SfxResource::preset_powerup();
        let json = serde_json::to_string(&original).unwrap();
        let parsed: SfxResource = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, original.name);
        assert_eq!(parsed.duration_ms, original.duration_ms);
    }
}
