//! Render context for collecting vector lines to draw
//! Port of vectrexy/libs/emulator/include/emulator/EngineTypes.h (RenderContext)

use super::line::Line;

/* C++ Original:
struct RenderContext {
    std::vector<Line> lines; // Lines to draw this frame
};
*/
#[derive(Debug, Clone)]
pub struct RenderContext {
    // C++ Original: std::vector<Line> lines; // Lines to draw this frame
    pub lines: Vec<Line>,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self { lines: Vec::new() }
    }
}

impl RenderContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }
}