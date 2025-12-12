//! VPy Vector Resource format (.vec)
//!
//! Vector graphics resources stored as JSON that can be compiled
//! into efficient ASM/binary data for Vectrex.

use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Vector resource file extension
#[allow(dead_code)]
pub const VEC_EXTENSION: &str = "vec";

/// Root structure of a .vec file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecResource {
    /// File format version
    #[serde(default = "default_version")]
    pub version: String,
    /// Resource name (used for symbol generation)
    pub name: String,
    /// Author information
    #[serde(default)]
    pub author: String,
    /// Creation date
    #[serde(default)]
    pub created: String,
    /// Canvas settings
    #[serde(default)]
    pub canvas: Canvas,
    /// Layers containing paths
    #[serde(default)]
    pub layers: Vec<Layer>,
    /// Animation definitions (optional)
    #[serde(default)]
    pub animations: Vec<Animation>,
    /// Metadata (hitbox, origin, tags)
    #[serde(default)]
    pub metadata: Metadata,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// Canvas settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
    /// Canvas width (default 256)
    #[serde(default = "default_canvas_size")]
    pub width: u16,
    /// Canvas height (default 256)
    #[serde(default = "default_canvas_size")]
    pub height: u16,
    /// Origin position: "center", "top-left", "bottom-left"
    #[serde(default = "default_origin")]
    pub origin: String,
}

fn default_canvas_size() -> u16 { 256 }
fn default_origin() -> String { "center".to_string() }

impl Default for Canvas {
    fn default() -> Self {
        Self {
            width: 256,
            height: 256,
            origin: "center".to_string(),
        }
    }
}

/// A layer containing multiple paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    /// Layer name
    pub name: String,
    /// Whether layer is visible
    #[serde(default = "default_true")]
    pub visible: bool,
    /// Paths in this layer
    #[serde(default)]
    pub paths: Vec<VecPath>,
}

fn default_true() -> bool { true }

/// A vector path (series of connected points)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecPath {
    /// Path name
    #[serde(default)]
    pub name: String,
    /// Beam intensity (0-127)
    #[serde(default = "default_intensity")]
    pub intensity: u8,
    /// Whether path is closed (connects back to start)
    #[serde(default)]
    pub closed: bool,
    /// Points in the path
    pub points: Vec<Point>,
}

fn default_intensity() -> u8 { 127 }

/// A point in 2D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: i16,
    pub y: i16,
    /// Optional intensity override for this specific point (0-255)
    /// If present, triggers Intensity_a call before drawing to this point
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intensity: Option<u8>,
}

/// Animation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    /// Animation name
    pub name: String,
    /// Frames in the animation
    pub frames: Vec<AnimFrame>,
}

/// A single animation frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimFrame {
    /// Layer to show for this frame
    pub layer: String,
    /// Frame duration in milliseconds
    #[serde(default = "default_duration")]
    pub duration: u16,
}

fn default_duration() -> u16 { 100 }

/// Resource metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// Hitbox rectangle
    #[serde(default)]
    pub hitbox: Option<Rect>,
    /// Origin/pivot point
    #[serde(default)]
    pub origin: Option<Point>,
    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
}

/// A rectangle
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}

impl VecResource {
    /// Load a .vec resource from a file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let resource: VecResource = serde_json::from_str(&content)?;
        Ok(resource)
    }
    
    /// Save the resource to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Create a new empty resource
    pub fn new(name: &str) -> Self {
        Self {
            version: "1.0".to_string(),
            name: name.to_string(),
            author: String::new(),
            created: String::new(),
            canvas: Canvas::default(),
            layers: vec![Layer {
                name: "default".to_string(),
                visible: true,
                paths: Vec::new(),
            }],
            animations: Vec::new(),
            metadata: Metadata::default(),
        }
    }
    
    /// Get all visible paths flattened
    pub fn visible_paths(&self) -> Vec<&VecPath> {
        self.layers.iter()
            .filter(|l| l.visible)
            .flat_map(|l| l.paths.iter())
            .collect()
    }
    
    /// Get total point count
    pub fn point_count(&self) -> usize {
        self.layers.iter()
            .flat_map(|l| l.paths.iter())
            .map(|p| p.points.len())
            .sum()
    }
    
    // Helper: format i8 value for ASM (compatible with both native and lwasm)
    // lwasm requires hex format $XX for negative values, no spaces after commas
    fn format_byte(value: i8) -> String {
        format!("${:02X}", value as u8)
    }
    
    // Helper: format two bytes for FCB (lwasm compatibility: no space after comma)
    fn format_fcb2(v1: i8, v2: i8) -> String {
        format!("{},{}", Self::format_byte(v1), Self::format_byte(v2))
    }
    
    /// Compile to Vectrex-compatible ASM data using Draw_Sync_List format (Malban optimized)
    /// Format: FCB intensity, y, x, [<0=draw | 0=move | 1=next_seg], dy, dx, ..., FCB 1, [repeat], FCB 2 (end)
    pub fn compile_to_asm(&self) -> String {
        let mut asm = String::new();
        let symbol_name = self.name.to_uppercase().replace("-", "_").replace(" ", "_");
        
        asm.push_str(&format!("; Generated from {}.vec (Malban Draw_Sync_List format)\n", self.name));
        asm.push_str(&format!("; Total paths: {}, points: {}\n", 
            self.visible_paths().len(), self.point_count()));
        asm.push_str("\n");
        
        // Single path per file - no multi-path complexity
        if self.visible_paths().is_empty() {
            asm.push_str(&format!("_{}_VECTORS:\n", symbol_name));
            asm.push_str("    FCB 2               ; end marker\n");
            return asm;
        }
        
        let path = &self.visible_paths()[0]; // Take only the first path
        asm.push_str(&format!("_{}_VECTORS:\n", symbol_name));
        
        // Malban Draw_Sync_List format:
        // First segment: FCB intensity, y, x (absolute position)
        // Then for each line: FCB <0 (draw) or 0 (move), dy, dx
        // Between segments: FCB 1 (next segment marker)
        // End: FCB 2
        if path.points.len() < 2 {
            asm.push_str("    FCB 2               ; end marker (< 2 points)\n");
        } else {
            let default_intensity = path.intensity;
            let mut segments: Vec<(usize, usize, u8)> = Vec::new(); // (start_idx, end_idx, intensity)
            let mut current_intensity = path.points[0].intensity.unwrap_or(default_intensity);
            let mut segment_start = 0;
            
            // Detect segments by intensity changes (intensity=0 means beam off/move)
            for i in 1..path.points.len() {
                if let Some(new_intensity) = path.points[i].intensity {
                    if new_intensity != current_intensity {
                        // End current segment
                        segments.push((segment_start, i - 1, current_intensity));
                        segment_start = i;
                        current_intensity = new_intensity;
                    }
                }
            }
            // Add final segment
            segments.push((segment_start, path.points.len() - 1, current_intensity));
            
            // Generate BIOS Draw_VLc format data (simple, proven format)
            // Format: FCB intensity, FCB y,x (start), FCB count, [FCB dy,dx]*count
            for (seg_idx, (start, end, intensity)) in segments.iter().enumerate() {
                let p0 = &path.points[*start];
                let y0 = p0.y.clamp(-127, 127) as i8;
                let x0 = p0.x.clamp(-127, 127) as i8;
                let line_count = (end - start) as u8;
                
                // Header: intensity, position, count
                asm.push_str(&format!("    FCB {}              ; seg{}: intensity\n", *intensity, seg_idx));
                asm.push_str(&format!("    FCB {},{}          ; seg{}: position (y={}, x={})\n", 
                    Self::format_byte(y0), Self::format_byte(x0), seg_idx, y0, x0));
                asm.push_str(&format!("    FCB {}              ; seg{}: {} lines\n", line_count, seg_idx, line_count));
                
                // Generate deltas (dy, dx pairs)
                for j in *start..*end {
                    let p_from = &path.points[j];
                    let p_to = &path.points[j + 1];
                    
                    let dx = (p_to.x - p_from.x).clamp(-127, 127) as i8;
                    let dy = (p_to.y - p_from.y).clamp(-127, 127) as i8;
                    
                    asm.push_str(&format!("    FCB {}          ; line {} delta (dy={}, dx={})\n", 
                        Self::format_fcb2(dy, dx), j - start, dy, dx));
                }
            }
        }
        
        asm
    }
    
    /// Compile to binary vectorlist format
    #[allow(dead_code)]
    pub fn compile_to_binary(&self) -> Vec<u8> {
        let mut data = Vec::new();
        
        for path in self.visible_paths() {
            data.push(path.points.len() as u8);
            data.push(path.intensity);
            
            for point in &path.points {
                let x = point.x.clamp(-127, 127) as i8;
                let y = point.y.clamp(-127, 127) as i8;
                data.push(y as u8);
                data.push(x as u8);
            }
            
            data.push(if path.closed { 0x01 } else { 0x00 });
        }
        
        data
    }
}

/// Compile a .vec file to ASM
#[allow(dead_code)]
pub fn compile_vec_to_asm(input: &Path, output: &Path) -> Result<()> {
    let resource = VecResource::load(input)?;
    let asm = resource.compile_to_asm();
    std::fs::write(output, asm)?;
    Ok(())
}

/// Compile a .vec file to binary
#[allow(dead_code)]
pub fn compile_vec_to_binary(input: &Path, output: &Path) -> Result<()> {
    let resource = VecResource::load(input)?;
    let binary = resource.compile_to_binary();
    std::fs::write(output, binary)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_resource() {
        let res = VecResource::new("test-sprite");
        assert_eq!(res.name, "test-sprite");
        assert_eq!(res.layers.len(), 1);
    }
    
    #[test]
    fn test_compile_to_asm() {
        let mut res = VecResource::new("ship");
        res.layers[0].paths.push(VecPath {
            name: "hull".to_string(),
            intensity: 127,
            closed: true,
            points: vec![
                Point { x: 0, y: 20, intensity: None },
                Point { x: -10, y: -10, intensity: None },
                Point { x: 10, y: -10, intensity: None },
            ],
        });
        
        let asm = res.compile_to_asm();
        assert!(asm.contains("_SHIP_HULL_VECTORS:"));
        assert!(asm.contains("FCB 3"));  // 3 points
        assert!(asm.contains("FCB 127")); // intensity
    }
}
