//! VPy Music Resource format (.vmus)
//!
//! Music resources stored as JSON that can be compiled
//! into efficient ASM/binary data for Vectrex PSG.

use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Music resource file extension
pub const VMUS_EXTENSION: &str = "vmus";

/// Root structure of a .vmus file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicResource {
    /// File format version
    #[serde(default = "default_version")]
    pub version: String,
    /// Music name (used for symbol generation)
    pub name: String,
    /// Author information
    #[serde(default)]
    pub author: String,
    /// Tempo in BPM
    #[serde(default = "default_tempo")]
    pub tempo: u16,
    /// Ticks per beat (24 = quarter note, 12 = eighth note)
    #[serde(default = "default_ticks_per_beat")]
    #[serde(rename = "ticksPerBeat")]
    pub ticks_per_beat: u16,
    /// Total ticks in the track
    #[serde(default)]
    #[serde(rename = "totalTicks")]
    pub total_ticks: u32,
    /// Note events
    #[serde(default)]
    pub notes: Vec<NoteEvent>,
    /// Noise events
    #[serde(default)]
    pub noise: Vec<NoiseEvent>,
    /// Loop start tick
    #[serde(default)]
    #[serde(rename = "loopStart")]
    pub loop_start: u32,
    /// Loop end tick
    #[serde(default)]
    #[serde(rename = "loopEnd")]
    pub loop_end: u32,
}

fn default_version() -> String {
    "1.0".to_string()
}

fn default_tempo() -> u16 {
    120
}

fn default_ticks_per_beat() -> u16 {
    24
}

/// A note event (tone on PSG channels A/B/C)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteEvent {
    /// Unique identifier
    pub id: String,
    /// MIDI note number (0-127, 60=C4)
    pub note: u8,
    /// Start tick
    pub start: u32,
    /// Duration in ticks
    pub duration: u32,
    /// Velocity/volume (0-15)
    pub velocity: u8,
    /// PSG channel (0=A, 1=B, 2=C)
    pub channel: u8,
}

/// A noise event (for percussion/effects)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseEvent {
    /// Unique identifier
    pub id: String,
    /// Start tick
    pub start: u32,
    /// Duration in ticks
    pub duration: u32,
    /// Noise period (0-31, lower=higher pitch)
    pub period: u8,
    /// Channel mask (bit flags: 1=A, 2=B, 4=C)
    pub channels: u8,
}

impl MusicResource {
    /// Load a .vmus resource from a file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let resource: MusicResource = serde_json::from_str(&content)?;
        Ok(resource)
    }
    
    /// Save the resource to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Create a new empty music resource
    pub fn new(name: &str) -> Self {
        Self {
            version: "1.0".to_string(),
            name: name.to_string(),
            author: String::new(),
            tempo: 120,
            ticks_per_beat: 24,
            total_ticks: 384, // 4 beats default
            notes: Vec::new(),
            noise: Vec::new(),
            loop_start: 0,
            loop_end: 384,
        }
    }
    
    /// MIDI note to PSG frequency (Vectrex uses 1.5 MHz clock)
    /// Formula: freq = 1500000 / (32 * midi_freq_hz)
    fn midi_to_psg_period(midi: u8) -> u16 {
        // MIDI note to Hz: 440 * 2^((note - 69) / 12)
        let note = midi as f64;
        let freq_hz = 440.0 * 2.0_f64.powf((note - 69.0) / 12.0);
        
        // PSG period = 1.5MHz / (32 * freq)
        let period = (1_500_000.0 / (32.0 * freq_hz)) as u16;
        period.clamp(0, 4095) // 12-bit period
    }
    
    /// Compile to Vectrex PSG ASM data
    /// Compila a ASM usando el nombre del asset (filename sin extensión), NO el nombre del JSON
    pub fn compile_to_asm(&self, asset_name: &str) -> String {
        let mut asm = String::new();
        let symbol_name = asset_name.to_uppercase().replace("-", "_").replace(" ", "_");
        
        asm.push_str(&format!("; Generated from {}.vmus (internal name: {})\n", asset_name, self.name));
        asm.push_str(&format!("; Tempo: {} BPM, Total events: {}\n", 
            self.tempo, self.notes.len() + self.noise.len()));
        asm.push_str("\n");
        
        // Music data structure:
        // Header: tempo (u16), ticks_per_beat (u16), total_ticks (u32)
        // Events: sorted by start time, each event has type + data
        
        asm.push_str(&format!("_{}_MUSIC:\n", symbol_name));
        asm.push_str(&format!("    FDB {}              ; tempo (BPM)\n", self.tempo));
        asm.push_str(&format!("    FDB {}              ; ticks per beat\n", self.ticks_per_beat));
        asm.push_str(&format!("    FDB ${:04X},${:04X}  ; total ticks (32-bit)\n", 
            (self.total_ticks >> 16) as u16, (self.total_ticks & 0xFFFF) as u16));
        asm.push_str(&format!("    FDB {}              ; num events\n", 
            self.notes.len() + self.noise.len()));
        
        // Combine and sort events by start time
        let mut events: Vec<(u32, EventType)> = Vec::new();
        
        for note in &self.notes {
            events.push((note.start, EventType::Note(note.clone())));
        }
        
        for noise in &self.noise {
            events.push((noise.start, EventType::Noise(noise.clone())));
        }
        
        events.sort_by_key(|(start, _)| *start);
        
        // Emit events
        asm.push_str("\n    ; Event data\n");
        for (_, event) in events {
            match event {
                EventType::Note(note) => {
                    let period = Self::midi_to_psg_period(note.note);
                    asm.push_str(&format!("    FCB $01             ; NOTE event\n"));
                    asm.push_str(&format!("    FDB ${:04X},${:04X}  ; start tick\n", 
                        (note.start >> 16) as u16, (note.start & 0xFFFF) as u16));
                    asm.push_str(&format!("    FDB ${:04X},${:04X}  ; duration\n", 
                        (note.duration >> 16) as u16, (note.duration & 0xFFFF) as u16));
                    asm.push_str(&format!("    FCB {}              ; channel\n", note.channel));
                    asm.push_str(&format!("    FDB ${:04X}         ; PSG period\n", period));
                    asm.push_str(&format!("    FCB {}              ; velocity\n", note.velocity));
                },
                EventType::Noise(noise) => {
                    asm.push_str(&format!("    FCB $02             ; NOISE event\n"));
                    asm.push_str(&format!("    FDB ${:04X},${:04X}  ; start tick\n", 
                        (noise.start >> 16) as u16, (noise.start & 0xFFFF) as u16));
                    asm.push_str(&format!("    FDB ${:04X},${:04X}  ; duration\n", 
                        (noise.duration >> 16) as u16, (noise.duration & 0xFFFF) as u16));
                    asm.push_str(&format!("    FCB {}              ; period\n", noise.period));
                    asm.push_str(&format!("    FCB {}              ; channels mask\n", noise.channels));
                }
            }
            asm.push_str("\n");
        }
        
        // Loop points
        asm.push_str(&format!("    FDB ${:04X},${:04X}  ; loop start\n", 
            (self.loop_start >> 16) as u16, (self.loop_start & 0xFFFF) as u16));
        asm.push_str(&format!("    FDB ${:04X},${:04X}  ; loop end\n", 
            (self.loop_end >> 16) as u16, (self.loop_end & 0xFFFF) as u16));
        
        asm
    }
    
    /// Compile to binary music format
    pub fn compile_to_binary(&self) -> Vec<u8> {
        let mut data = Vec::new();
        
        // Header
        data.extend_from_slice(&self.tempo.to_be_bytes());
        data.extend_from_slice(&self.ticks_per_beat.to_be_bytes());
        data.extend_from_slice(&self.total_ticks.to_be_bytes());
        
        let num_events = (self.notes.len() + self.noise.len()) as u16;
        data.extend_from_slice(&num_events.to_be_bytes());
        
        // Combine and sort events
        let mut events: Vec<(u32, EventType)> = Vec::new();
        
        for note in &self.notes {
            events.push((note.start, EventType::Note(note.clone())));
        }
        
        for noise in &self.noise {
            events.push((noise.start, EventType::Noise(noise.clone())));
        }
        
        events.sort_by_key(|(start, _)| *start);
        
        // Emit events
        for (_, event) in events {
            match event {
                EventType::Note(note) => {
                    data.push(0x01); // NOTE type
                    data.extend_from_slice(&note.start.to_be_bytes());
                    data.extend_from_slice(&note.duration.to_be_bytes());
                    data.push(note.channel);
                    
                    let period = Self::midi_to_psg_period(note.note);
                    data.extend_from_slice(&period.to_be_bytes());
                    data.push(note.velocity);
                },
                EventType::Noise(noise) => {
                    data.push(0x02); // NOISE type
                    data.extend_from_slice(&noise.start.to_be_bytes());
                    data.extend_from_slice(&noise.duration.to_be_bytes());
                    data.push(noise.period);
                    data.push(noise.channels);
                }
            }
        }
        
        // Loop points
        data.extend_from_slice(&self.loop_start.to_be_bytes());
        data.extend_from_slice(&self.loop_end.to_be_bytes());
        
        data
    }
}

#[derive(Debug, Clone)]
enum EventType {
    Note(NoteEvent),
    Noise(NoiseEvent),
}

/// Compile a .vmus file to ASM
pub fn compile_vmus_to_asm(input: &Path, output: &Path) -> Result<()> {
    let resource = MusicResource::load(input)?;
    // Use filename stem as asset name
    let asset_name = input.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unnamed");
    let asm = resource.compile_to_asm(asset_name);
    std::fs::write(output, asm)?;
    Ok(())
}

/// Compile a .vmus file to binary
pub fn compile_vmus_to_binary(input: &Path, output: &Path) -> Result<()> {
    let resource = MusicResource::load(input)?;
    let binary = resource.compile_to_binary();
    std::fs::write(output, binary)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_music() {
        let music = MusicResource::new("test-song");
        assert_eq!(music.name, "test-song");
        assert_eq!(music.tempo, 120);
    }
    
    #[test]
    fn test_midi_to_psg() {
        // Middle C (MIDI 60) = 261.63 Hz
        let period = MusicResource::midi_to_psg_period(60);
        // Expected: 1500000 / (32 * 261.63) ≈ 179
        assert!(period >= 175 && period <= 185);
        
        // A4 (MIDI 69) = 440 Hz
        let period_a4 = MusicResource::midi_to_psg_period(69);
        // Expected: 1500000 / (32 * 440) ≈ 106
        assert!(period_a4 >= 100 && period_a4 <= 110);
    }
    
    #[test]
    fn test_compile_to_asm() {
        let mut music = MusicResource::new("test");
        music.notes.push(NoteEvent {
            id: "note1".to_string(),
            note: 60, // Middle C
            start: 0,
            duration: 48,
            velocity: 12,
            channel: 0,
        });
        
        let symbol_name = music.name.to_uppercase().replace("-", "_").replace(" ", "_");
        let asm = music.compile_to_asm(&symbol_name);
        assert!(asm.contains("_TEST_MUSIC:"));
        assert!(asm.contains("FDB 120")); // tempo
        assert!(asm.contains("FCB $01")); // NOTE event
    }
}
