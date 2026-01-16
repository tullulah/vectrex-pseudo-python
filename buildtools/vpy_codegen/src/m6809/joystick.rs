//! Joystick Input Runtime Helpers
//!
//! Handles analog joystick input from Vectrex hardware

use std::collections::HashSet;

/// Emit joystick runtime helpers
/// Only emits helpers that are actually used in the code (tree shaking)
pub fn emit_runtime_helpers(out: &mut String, needed: &HashSet<String>) {
    // J1X_BUILTIN: Joystick 1 X axis (-1, 0, +1)
    if needed.contains("J1X_BUILTIN") {
        out.push_str("J1X_BUILTIN:\n");
    out.push_str("    ; Read J1_X from $CF00 and return -1/0/+1\n");
    out.push_str("    LDB $CF00      ; Joy_1_X (unsigned byte 0-255)\n");
    out.push_str("    CMPB #108      ; Compare with lower threshold\n");
    out.push_str("    BLO .J1X_LEFT  ; Branch if <108 (left)\n");
    out.push_str("    CMPB #148      ; Compare with upper threshold\n");
    out.push_str("    BHI .J1X_RIGHT ; Branch if >148 (right)\n");
    out.push_str("    ; Center (108-148)\n");
    out.push_str("    LDD #0\n");
    out.push_str("    RTS\n");
    out.push_str(".J1X_LEFT:\n");
    out.push_str("    LDD #-1\n");
    out.push_str("    RTS\n");
    out.push_str(".J1X_RIGHT:\n");
        out.push_str("    LDD #1\n");
        out.push_str("    RTS\n\n");
    }
    
    // J1Y_BUILTIN: Joystick 1 Y axis (-1, 0, +1)
    if needed.contains("J1Y_BUILTIN") {
        out.push_str("J1Y_BUILTIN:\n");
    out.push_str("    ; Read J1_Y from $CF01 and return -1/0/+1\n");
    out.push_str("    LDB $CF01      ; Joy_1_Y (unsigned byte 0-255)\n");
    out.push_str("    CMPB #108      ; Compare with lower threshold\n");
    out.push_str("    BLO .J1Y_DOWN  ; Branch if <108 (down)\n");
    out.push_str("    CMPB #148      ; Compare with upper threshold\n");
    out.push_str("    BHI .J1Y_UP    ; Branch if >148 (up)\n");
    out.push_str("    ; Center (108-148)\n");
    out.push_str("    LDD #0\n");
    out.push_str("    RTS\n");
    out.push_str(".J1Y_DOWN:\n");
    out.push_str("    LDD #-1\n");
    out.push_str("    RTS\n");
    out.push_str(".J1Y_UP:\n");
        out.push_str("    LDD #1\n");
        out.push_str("    RTS\n\n");
    }
    
    // J2X_BUILTIN: Joystick 2 X axis (-1, 0, +1)
    if needed.contains("J2X_BUILTIN") {
        out.push_str("J2X_BUILTIN:\n");
    out.push_str("    ; Read J2_X from $CF02 and return -1/0/+1\n");
    out.push_str("    LDB $CF02      ; Joy_2_X (unsigned byte 0-255)\n");
    out.push_str("    CMPB #108      ; Compare with lower threshold\n");
    out.push_str("    BLO .J2X_LEFT  ; Branch if <108 (left)\n");
    out.push_str("    CMPB #148      ; Compare with upper threshold\n");
    out.push_str("    BHI .J2X_RIGHT ; Branch if >148 (right)\n");
    out.push_str("    ; Center (108-148)\n");
    out.push_str("    LDD #0\n");
    out.push_str("    RTS\n");
    out.push_str(".J2X_LEFT:\n");
    out.push_str("    LDD #-1\n");
    out.push_str("    RTS\n");
    out.push_str(".J2X_RIGHT:\n");
        out.push_str("    LDD #1\n");
        out.push_str("    RTS\n\n");
    }
    
    // J2Y_BUILTIN: Joystick 2 Y axis (-1, 0, +1)
    if needed.contains("J2Y_BUILTIN") {
        out.push_str("J2Y_BUILTIN:\n");
    out.push_str("    ; Read J2_Y from $CF03 and return -1/0/+1\n");
    out.push_str("    LDB $CF03      ; Joy_2_Y (unsigned byte 0-255)\n");
    out.push_str("    CMPB #108      ; Compare with lower threshold\n");
    out.push_str("    BLO .J2Y_DOWN  ; Branch if <108 (down)\n");
    out.push_str("    CMPB #148      ; Compare with upper threshold\n");
    out.push_str("    BHI .J2Y_UP    ; Branch if >148 (up)\n");
    out.push_str("    ; Center (108-148)\n");
    out.push_str("    LDD #0\n");
    out.push_str("    RTS\n");
    out.push_str(".J2Y_DOWN:\n");
    out.push_str("    LDD #-1\n");
    out.push_str("    RTS\n");
        out.push_str(".J2Y_UP:\n");
        out.push_str("    LDD #1\n");
        out.push_str("    RTS\n\n");
    }
}
