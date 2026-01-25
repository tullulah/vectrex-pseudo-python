//! VPy Animation Resource format (.vanim)
//!
//! Animation resources stored as JSON that can be compiled
//! into efficient ASM/binary data for Vectrex.
//!
//! Supports multiple simultaneous animation instances with state machines
//! and controller integration for playable entities.

use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Animation resource file extension
pub const VANIM_EXTENSION: &str = "vanim";

/// Root structure of a .vanim file (v1.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VAnimAnimation {
    /// File format version ("1.0")
    pub version: String,
    /// Animation name (used for symbol generation)
    pub name: String,
    /// Author information
    #[serde(default)]
    pub author: String,
    /// Creation date
    #[serde(default)]
    pub created: String,
    /// Animation frames
    pub frames: Vec<VAnimFrame>,
    /// Animation states (state machine)
    #[serde(default)]
    pub states: std::collections::HashMap<String, VAnimState>,
    /// Controller configuration (for playable entities)
    #[serde(default)]
    pub controller: Option<VAnimController>,
}

/// Single animation frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VAnimFrame {
    /// Frame ID (unique within animation)
    pub id: String,
    /// Vector asset name to render
    #[serde(rename = "vectorName")]
    pub vector_name: String,
    /// Frame duration in ticks (60 ticks = 1 second)
    pub duration: u16,
    /// Intensity (0-127, default 127)
    #[serde(default = "default_intensity")]
    pub intensity: u8,
    /// Offset X from entity position (default 0)
    #[serde(default, rename = "offsetX")]
    pub offset_x: i16,
    /// Offset Y from entity position (default 0)
    #[serde(default, rename = "offsetY")]
    pub offset_y: i16,
    /// Mirror mode: 0=none, 1=X, 2=Y, 3=XY (default 0)
    #[serde(default)]
    pub mirror: u8,
}

fn default_intensity() -> u8 { 127 }

/// Animation state in state machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VAnimState {
    /// State name
    pub name: String,
    /// Frame IDs in this state (plays in sequence)
    pub frames: Vec<String>,
    /// Loop behavior: true=loop, false=hold last frame
    #[serde(default = "default_loop")]
    pub loop_state: bool,
    /// Transitions to other states
    #[serde(default)]
    pub transitions: Vec<VAnimTransition>,
}

fn default_loop() -> bool { true }

/// State transition trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VAnimTransition {
    /// Trigger type: "auto", "input_left", "input_right", "input_up", "input_down", "input_fire", "custom"
    pub trigger: String,
    /// Target state name
    #[serde(rename = "targetState")]
    pub target_state: String,
    /// Custom condition expression (for trigger="custom")
    #[serde(default)]
    pub condition: String,
}

/// Controller configuration for playable entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VAnimController {
    /// Mirror X when moving left (default true)
    #[serde(default = "default_true", rename = "mirrorOnLeft")]
    pub mirror_on_left: bool,
    /// State to switch to when moving left
    #[serde(rename = "stateLeft")]
    pub state_left: Option<String>,
    /// State to switch to when moving right
    #[serde(rename = "stateRight")]
    pub state_right: Option<String>,
    /// State to switch to when idle
    #[serde(rename = "stateIdle")]
    pub state_idle: Option<String>,
}

fn default_true() -> bool { true }

impl VAnimAnimation {
    /// Load animation from .vanim file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let anim: VAnimAnimation = serde_json::from_str(&content)?;
        
        // Validate version
        if anim.version != "1.0" {
            anyhow::bail!("Unsupported .vanim version: {} (expected 1.0)", anim.version);
        }
        
        // Validate frames exist
        if anim.frames.is_empty() {
            anyhow::bail!("Animation must have at least one frame");
        }
        
        // Validate state frames reference existing frame IDs
        for (state_name, state) in &anim.states {
            for frame_id in &state.frames {
                if !anim.frames.iter().any(|f| &f.id == frame_id) {
                    anyhow::bail!("State '{}' references non-existent frame: {}", state_name, frame_id);
                }
            }
        }
        
        Ok(anim)
    }
    
    /// Compile animation to M6809 assembly with optional custom name
    pub fn compile_to_asm_with_name(&self, override_name: Option<&str>) -> String {
        let asset_name = override_name.unwrap_or(&self.name);
        let label_base = asset_name.to_uppercase().replace(" ", "_").replace("-", "_");
        
        let mut out = String::new();
        
        // Header comment
        out.push_str(&format!("; ===== ANIMATION: {} =====\n", asset_name));
        out.push_str(&format!("; Version: {}\n", self.version));
        out.push_str(&format!("; Frames: {}\n", self.frames.len()));
        out.push_str(&format!("; States: {}\n", self.states.len()));
        out.push_str("\n");
        
        // Main animation data structure
        out.push_str(&format!("_{}_ANIM:\n", label_base));
        out.push_str(&format!("    FCB {}          ; num_frames\n", self.frames.len()));
        out.push_str(&format!("    FCB {}          ; num_states\n", self.states.len()));
        
        // Emit controller flags (1 byte)
        let controller_byte = if let Some(ref ctrl) = self.controller {
            let mut flags = 0u8;
            if ctrl.mirror_on_left { flags |= 0x01; }
            flags
        } else {
            0
        };
        out.push_str(&format!("    FCB ${:02X}        ; controller_flags (bit 0: mirror_on_left)\n", controller_byte));
        
        // Frame table pointer
        out.push_str(&format!("    FDB _{}_FRAMES    ; Pointer to frame table\n", label_base));
        
        // State table pointer (or 0x0000 if no states)
        if self.states.is_empty() {
            out.push_str("    FDB $0000       ; No states\n");
        } else {
            out.push_str(&format!("    FDB _{}_STATES    ; Pointer to state table\n", label_base));
        }
        
        out.push_str("\n");
        
        // Emit frame table
        out.push_str(&format!("; Frame table for {}\n", asset_name));
        out.push_str(&format!("_{}_FRAMES:\n", label_base));
        
        for (idx, frame) in self.frames.iter().enumerate() {
            let frame_label = format!("{}_{}", label_base, frame.id.to_uppercase().replace(" ", "_").replace("-", "_"));
            out.push_str(&format!("    ; Frame {}: {} (duration={})\n", idx, frame.id, frame.duration));
            out.push_str(&format!("_{}_FRAME:\n", frame_label));
            
            // Frame data structure: vector_name_ptr (2 bytes), duration (2 bytes), intensity (1 byte), 
            // offset_x (2 bytes), offset_y (2 bytes), mirror (1 byte) = 10 bytes per frame
            let vector_label = format!("_{}_VECTORS", frame.vector_name.to_uppercase().replace(" ", "_").replace("-", "_"));
            out.push_str(&format!("    FDB {}     ; Pointer to vector asset\n", vector_label));
            out.push_str(&format!("    FDB {}          ; Duration (ticks)\n", frame.duration));
            out.push_str(&format!("    FCB {}          ; Intensity\n", frame.intensity));
            out.push_str(&format!("    FDB {}          ; Offset X\n", frame.offset_x as u16));
            out.push_str(&format!("    FDB {}          ; Offset Y\n", frame.offset_y as u16));
            out.push_str(&format!("    FCB {}          ; Mirror mode\n", frame.mirror));
        }
        
        out.push_str("\n");
        
        // Emit state table if states exist
        if !self.states.is_empty() {
            out.push_str(&format!("; State table for {}\n", asset_name));
            out.push_str(&format!("_{}_STATES:\n", label_base));
            
            for (state_name, state) in &self.states {
                let state_label = format!("{}_{}", label_base, state_name.to_uppercase().replace(" ", "_").replace("-", "_"));
                out.push_str(&format!("    ; State: {}\n", state_name));
                out.push_str(&format!("_{}_STATE:\n", state_label));
                
                // State data: num_frames (1 byte), loop (1 byte), frame_indices... (1 byte each)
                out.push_str(&format!("    FCB {}          ; num_frames in state\n", state.frames.len()));
                out.push_str(&format!("    FCB {}          ; loop_state (0=no, 1=yes)\n", if state.loop_state { 1 } else { 0 }));
                
                // Emit frame indices
                for frame_id in &state.frames {
                    // Find frame index
                    let frame_idx = self.frames.iter().position(|f| &f.id == frame_id).unwrap_or(0);
                    out.push_str(&format!("    FCB {}          ; Frame index: {}\n", frame_idx, frame_id));
                }
            }
            
            out.push_str("\n");
        }
        
        out
    }
    
    /// Compile animation to M6809 assembly (uses animation name from file)
    pub fn compile_to_asm(&self) -> String {
        self.compile_to_asm_with_name(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_animation() {
        let json = r#"{
            "version": "1.0",
            "name": "player_walk",
            "frames": [
                {
                    "id": "frame1",
                    "vectorName": "player_walk_1",
                    "duration": 5
                },
                {
                    "id": "frame2",
                    "vectorName": "player_walk_2",
                    "duration": 5
                }
            ]
        }"#;
        
        let anim: VAnimAnimation = serde_json::from_str(json).unwrap();
        assert_eq!(anim.version, "1.0");
        assert_eq!(anim.name, "player_walk");
        assert_eq!(anim.frames.len(), 2);
        assert_eq!(anim.frames[0].intensity, 127); // default
    }
    
    #[test]
    fn test_compile_to_asm() {
        let json = r#"{
            "version": "1.0",
            "name": "test_anim",
            "frames": [
                {
                    "id": "frame1",
                    "vectorName": "sprite1",
                    "duration": 10,
                    "intensity": 100
                }
            ]
        }"#;
        
        let anim: VAnimAnimation = serde_json::from_str(json).unwrap();
        let asm = anim.compile_to_asm();
        
        assert!(asm.contains("_TEST_ANIM_ANIM:"));
        assert!(asm.contains("_TEST_ANIM_FRAMES:"));
        assert!(asm.contains("FCB 1")); // num_frames
    }
}
