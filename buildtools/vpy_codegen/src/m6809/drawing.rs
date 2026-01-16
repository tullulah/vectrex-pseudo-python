//! Drawing Geometric Shapes
//!
//! Builtins for drawing circles, rectangles, polygons, etc.

use std::collections::HashSet;
use vpy_parser::Expr;

/// DRAW_CIRCLE(xc, yc, diam) or DRAW_CIRCLE(xc, yc, diam, intensity)
pub fn emit_draw_circle(
    args: &[Expr],
    out: &mut String,
) {
    if args.len() != 3 && args.len() != 4 {
        out.push_str("    ; ERROR: DRAW_CIRCLE requires 3 or 4 arguments\n");
        return;
    }
    
    // Check if all args are constants - optimize as 16-gon inline
    if args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(xc), Expr::Number(yc), Expr::Number(diam)) = 
            (&args[0], &args[1], &args[2]) {
            let mut intensity: i32 = 0x5F;
            if args.len() == 4 {
                if let Expr::Number(i) = &args[3] {
                    intensity = *i;
                }
            }
            
            let segs = 16; // 16-sided polygon approximation
            let r = (*diam as f64) / 2.0;
            let mut verts: Vec<(i32, i32)> = Vec::new();
            
            for k in 0..segs {
                let ang = 2.0 * std::f64::consts::PI * (k as f64) / (segs as f64);
                let x = (*xc as f64) + r * ang.cos();
                let y = (*yc as f64) + r * ang.sin();
                verts.push((x.round() as i32, y.round() as i32));
            }
            
            // Emit inline code (like core does)
            if intensity == 0x5F {
                out.push_str("    JSR Intensity_5F\n");
            } else {
                out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF));
            }
            out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
            
            let (sx, sy) = verts[0];
            out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", 
                (sy & 0xFF), (sx & 0xFF)));
            
            for i in 0..segs {
                let (x0, y0) = verts[i];
                let (x1, y1) = verts[(i + 1) % segs];
                let dx = (x1 - x0) & 0xFF;
                let dy = (y1 - y0) & 0xFF;
                out.push_str("    CLR Vec_Misc_Count\n");
                out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", 
                    dy, dx));
            }
            out.push_str("    LDD #0\n    STD RESULT\n");
            return;
        }
    }
    
    // Variables - use runtime helper
    out.push_str("    ; ERROR: DRAW_CIRCLE with variables requires expressions module access\n");
    out.push_str("    ; Use constant values for now\n");
    out.push_str("    LDD #0\n    STD RESULT\n");
}

/// DRAW_RECT(x, y, width, height[, intensity])
pub fn emit_draw_rect(
    args: &[Expr],
    out: &mut String,
) {
    if args.len() != 4 && args.len() != 5 {
        out.push_str("    ; ERROR: DRAW_RECT requires 4 or 5 arguments\n");
        return;
    }
    
    // Check if all args are constants - optimize as 4 inline lines
    if args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(x), Expr::Number(y), Expr::Number(w), Expr::Number(h)) = 
            (&args[0], &args[1], &args[2], &args[3]) {
            let mut intensity: i32 = 0x5F;
            if args.len() == 5 {
                if let Expr::Number(i) = &args[4] {
                    intensity = *i;
                }
            }
            
            // Four corners
            let x0 = *x;
            let y0 = *y;
            let _x1 = x0 + w;  // Calculated but not used directly
            let _y1 = y0 + h;  // Calculated but not used directly
            
            // Emit inline code
            if intensity == 0x5F {
                out.push_str("    JSR Intensity_5F\n");
            } else {
                out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF));
            }
            out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
            
            // Move to start
            out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", 
                (y0 & 0xFF), (x0 & 0xFF)));
            
            // Draw 4 sides
            out.push_str("    CLR Vec_Misc_Count\n");
            out.push_str(&format!("    LDA #$00\n    LDB #${:02X}\n    JSR Draw_Line_d\n", 
                (w & 0xFF)));  // Right
            out.push_str("    CLR Vec_Misc_Count\n");
            out.push_str(&format!("    LDA #${:02X}\n    LDB #$00\n    JSR Draw_Line_d\n", 
                (h & 0xFF)));  // Down
            out.push_str("    CLR Vec_Misc_Count\n");
            let neg_w = (-(*w as i32)) & 0xFF;
            out.push_str(&format!("    LDA #$00\n    LDB #${:02X}\n    JSR Draw_Line_d\n", 
                neg_w));  // Left
            out.push_str("    CLR Vec_Misc_Count\n");
            let neg_h = (-(*h as i32)) & 0xFF;
            out.push_str(&format!("    LDA #${:02X}\n    LDB #$00\n    JSR Draw_Line_d\n", 
                neg_h));  // Up
            
            out.push_str("    LDD #0\n    STD RESULT\n");
            return;
        }
    }
    
    // Variables - use runtime helper
    out.push_str("    ; ERROR: DRAW_RECT with variables requires expressions module access\n");
    out.push_str("    ; Use constant values for now\n");
    out.push_str("    LDD #0\n    STD RESULT\n");
}

/// DRAW_POLYGON(points_array[, intensity]) - points_array is array of [x,y] pairs
/// Simplified version: DRAW_POLYGON(x0, y0, x1, y1, x2, y2[, intensity])
pub fn emit_draw_polygon(
    args: &[Expr],
    out: &mut String,
) {
    if args.len() < 6 {
        out.push_str("    ; ERROR: DRAW_POLYGON requires at least 6 arguments (3 points)\n");
        return;
    }
    
    // Check if all args are constants - optimize inline
    if args.iter().all(|a| matches!(a, Expr::Number(_))) {
        let mut intensity: i32 = 0x5F;
        let num_coords = if args.len() % 2 == 0 { 
            args.len() 
        } else { 
            // Last arg is intensity
            if let Expr::Number(i) = &args[args.len() - 1] {
                intensity = *i;
            }
            args.len() - 1
        };
        
        let num_points = num_coords / 2;
        let mut verts: Vec<(i32, i32)> = Vec::new();
        
        for i in 0..num_points {
            if let (Expr::Number(x), Expr::Number(y)) = 
                (&args[i * 2], &args[i * 2 + 1]) {
                verts.push((*x, *y));
            }
        }
        
        // Emit inline code
        if intensity == 0x5F {
            out.push_str("    JSR Intensity_5F\n");
        } else {
            out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF));
        }
        out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
        
        let (sx, sy) = verts[0];
        out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", 
            (sy & 0xFF), (sx & 0xFF)));
        
        for i in 0..num_points {
            let (x0, y0) = verts[i];
            let (x1, y1) = verts[(i + 1) % num_points];
            let dx = (x1 - x0) & 0xFF;
            let dy = (y1 - y0) & 0xFF;
            out.push_str("    CLR Vec_Misc_Count\n");
            out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", 
                dy, dx));
        }
        out.push_str("    LDD #0\n    STD RESULT\n");
        return;
    }
    
    // Variables - TODO: implement runtime helper (complex: requires array handling)
    out.push_str("    ; ERROR: DRAW_POLYGON with variables not yet implemented\n");
    out.push_str("    LDD #0\n    STD RESULT\n");
}

/// DRAW_CIRCLE_SEG(nseg, xc, yc, diam[, intensity]) - Circle with variable segments
pub fn emit_draw_circle_seg(
    args: &[Expr],
    out: &mut String,
) {
    if args.len() != 4 && args.len() != 5 {
        out.push_str("    ; ERROR: DRAW_CIRCLE_SEG requires 4 or 5 arguments\n");
        return;
    }
    
    // Check if all args are constants
    if args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(nseg), Expr::Number(xc), Expr::Number(yc), Expr::Number(diam)) = 
            (&args[0], &args[1], &args[2], &args[3]) {
            let mut intensity: i32 = 0x5F;
            if args.len() == 5 {
                if let Expr::Number(i) = &args[4] {
                    intensity = *i;
                }
            }
            
            let segs = (*nseg).clamp(3, 64) as usize;
            let r = (*diam as f64) / 2.0;
            let mut verts: Vec<(i32, i32)> = Vec::new();
            
            for k in 0..segs {
                let ang = 2.0 * std::f64::consts::PI * (k as f64) / (segs as f64);
                let x = (*xc as f64) + r * ang.cos();
                let y = (*yc as f64) + r * ang.sin();
                verts.push((x.round() as i32, y.round() as i32));
            }
            
            // Emit inline code
            if intensity == 0x5F {
                out.push_str("    JSR Intensity_5F\n");
            } else {
                out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF));
            }
            out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
            
            let (sx, sy) = verts[0];
            out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", 
                (sy & 0xFF), (sx & 0xFF)));
            
            for i in 0..segs {
                let (x0, y0) = verts[i];
                let (x1, y1) = verts[(i + 1) % segs];
                let dx = (x1 - x0) & 0xFF;
                let dy = (y1 - y0) & 0xFF;
                out.push_str("    CLR Vec_Misc_Count\n");
                out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", 
                    dy, dx));
            }
            out.push_str("    LDD #0\n    STD RESULT\n");
            return;
        }
    }
    
    out.push_str("    ; ERROR: DRAW_CIRCLE_SEG with variables not yet implemented\n");
    out.push_str("    LDD #0\n    STD RESULT\n");
}

/// DRAW_ARC(nseg, xc, yc, radius, start_deg, sweep_deg[, intensity]) - Open arc
pub fn emit_draw_arc(
    args: &[Expr],
    out: &mut String,
) {
    if args.len() != 6 && args.len() != 7 {
        out.push_str("    ; ERROR: DRAW_ARC requires 6 or 7 arguments\n");
        return;
    }
    
    // Check if all args are constants
    if args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(nseg), Expr::Number(xc), Expr::Number(yc), 
                Expr::Number(rad), Expr::Number(startd), Expr::Number(sweepd)) = 
            (&args[0], &args[1], &args[2], &args[3], &args[4], &args[5]) {
            let mut intensity: i32 = 0x5F;
            if args.len() == 7 {
                if let Expr::Number(i) = &args[6] {
                    intensity = *i;
                }
            }
            
            let segs = (*nseg).clamp(1, 96) as usize;
            let start = (*startd as f64) * std::f64::consts::PI / 180.0;
            let sweep = (*sweepd as f64) * std::f64::consts::PI / 180.0;
            let r = (*rad as f64).clamp(4.0, 110.0);
            
            let mut verts: Vec<(i32, i32)> = Vec::new();
            for k in 0..=segs {
                let t = k as f64 / segs as f64;
                let ang = start + sweep * t;
                let x = ((*xc as f64) + r * ang.cos()).clamp(-120.0, 120.0);
                let y = ((*yc as f64) + r * ang.sin()).clamp(-120.0, 120.0);
                verts.push((x.round() as i32, y.round() as i32));
            }
            
            // Emit inline code
            if intensity == 0x5F {
                out.push_str("    JSR Intensity_5F\n");
            } else {
                out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF));
            }
            out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
            
            let (sx, sy) = verts[0];
            out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", 
                (sy & 0xFF), (sx & 0xFF)));
            
            for i in 0..segs {
                let (x0, y0) = verts[i];
                let (x1, y1) = verts[i + 1];
                let dx = (x1 - x0) & 0xFF;
                let dy = (y1 - y0) & 0xFF;
                out.push_str("    CLR Vec_Misc_Count\n");
                out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", 
                    dy, dx));
            }
            out.push_str("    LDD #0\n    STD RESULT\n");
            return;
        }
    }
    
    out.push_str("    ; ERROR: DRAW_ARC with variables not yet implemented\n");
    out.push_str("    LDD #0\n    STD RESULT\n");
}

/// DRAW_FILLED_RECT(x, y, width, height[, intensity]) - Filled rectangle with scanlines
pub fn emit_draw_filled_rect(
    args: &[Expr],
    out: &mut String,
) {
    if args.len() != 4 && args.len() != 5 {
        out.push_str("    ; ERROR: DRAW_FILLED_RECT requires 4 or 5 arguments\n");
        return;
    }
    
    // Check if all args are constants
    if args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(x), Expr::Number(y), Expr::Number(w), Expr::Number(h)) = 
            (&args[0], &args[1], &args[2], &args[3]) {
            let mut intensity: i32 = 0x5F;
            if args.len() == 5 {
                if let Expr::Number(i) = &args[4] {
                    intensity = *i;
                }
            }
            
            let x0 = *x;
            let y0 = *y;
            let width = *w;
            let height = *h;
            
            // Emit inline code
            if intensity == 0x5F {
                out.push_str("    JSR Intensity_5F\n");
            } else {
                out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF));
            }
            out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
            
            // Draw horizontal scanlines from top to bottom
            let num_lines = height.abs().min(64); // Limit scanlines
            for i in 0..num_lines {
                let y_offset = if height >= 0 { i } else { -i };
                let curr_y = (y0 + y_offset) & 0xFF;
                
                // Move to start of scanline
                out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", 
                    curr_y, (x0 & 0xFF)));
                
                // Draw horizontal line
                out.push_str("    CLR Vec_Misc_Count\n");
                out.push_str(&format!("    LDA #$00\n    LDB #${:02X}\n    JSR Draw_Line_d\n", 
                    (width & 0xFF)));
            }
            
            out.push_str("    LDD #0\n    STD RESULT\n");
            return;
        }
    }
    
    out.push_str("    ; ERROR: DRAW_FILLED_RECT with variables not yet implemented\n");
    out.push_str("    LDD #0\n    STD RESULT\n");
}

/// DRAW_ELLIPSE(xc, yc, rx, ry[, intensity]) - Ellipse approximation
pub fn emit_draw_ellipse(
    args: &[Expr],
    out: &mut String,
) {
    if args.len() != 4 && args.len() != 5 {
        out.push_str("    ; ERROR: DRAW_ELLIPSE requires 4 or 5 arguments\n");
        return;
    }
    
    // Check if all args are constants
    if args.iter().all(|a| matches!(a, Expr::Number(_))) {
        if let (Expr::Number(xc), Expr::Number(yc), Expr::Number(rx), Expr::Number(ry)) = 
            (&args[0], &args[1], &args[2], &args[3]) {
            let mut intensity: i32 = 0x5F;
            if args.len() == 5 {
                if let Expr::Number(i) = &args[4] {
                    intensity = *i;
                }
            }
            
            let segs = 24; // 24-sided polygon approximation
            let rx_f = *rx as f64;
            let ry_f = *ry as f64;
            let mut verts: Vec<(i32, i32)> = Vec::new();
            
            for k in 0..segs {
                let ang = 2.0 * std::f64::consts::PI * (k as f64) / (segs as f64);
                let x = (*xc as f64) + rx_f * ang.cos();
                let y = (*yc as f64) + ry_f * ang.sin();
                verts.push((x.round() as i32, y.round() as i32));
            }
            
            // Emit inline code
            if intensity == 0x5F {
                out.push_str("    JSR Intensity_5F\n");
            } else {
                out.push_str(&format!("    LDA #${:02X}\n    JSR Intensity_a\n", intensity & 0xFF));
            }
            out.push_str("    LDA #$D0\n    TFR A,DP\n    JSR Reset0Ref\n");
            
            let (sx, sy) = verts[0];
            out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Moveto_d\n", 
                (sy & 0xFF), (sx & 0xFF)));
            
            for i in 0..segs {
                let (x0, y0) = verts[i];
                let (x1, y1) = verts[(i + 1) % segs];
                let dx = (x1 - x0) & 0xFF;
                let dy = (y1 - y0) & 0xFF;
                out.push_str("    CLR Vec_Misc_Count\n");
                out.push_str(&format!("    LDA #${:02X}\n    LDB #${:02X}\n    JSR Draw_Line_d\n", 
                    dy, dx));
            }
            out.push_str("    LDD #0\n    STD RESULT\n");
            return;
        }
    }
    
    out.push_str("    ; ERROR: DRAW_ELLIPSE with variables not yet implemented\n");
    out.push_str("    LDD #0\n    STD RESULT\n");
}

/// DRAW_SPRITE(x, y, sprite_name) - Draw bitmap sprite (placeholder)
pub fn emit_draw_sprite(
    args: &[Expr],
    out: &mut String,
) {
    if args.len() != 3 {
        out.push_str("    ; ERROR: DRAW_SPRITE requires 3 arguments\n");
        return;
    }
    
    // DRAW_SPRITE is complex (requires bitmap data, scanline conversion)
    // For now, placeholder implementation
    out.push_str("    ; TODO: DRAW_SPRITE implementation\n");
    out.push_str("    ; Requires bitmap asset system and raster conversion\n");
    out.push_str("    LDD #0\n    STD RESULT\n");
}

/// Emit runtime helpers for drawing builtins
/// Only emits helpers that are actually used in the code (tree shaking)
pub fn emit_runtime_helpers(out: &mut String, needed: &HashSet<String>) {
    // DRAW_CIRCLE_RUNTIME: Draw circle with runtime parameters
    if needed.contains("DRAW_CIRCLE_RUNTIME") {
        out.push_str("DRAW_CIRCLE_RUNTIME:\n");
        out.push_str("    ; Input: DRAW_CIRCLE_XC, DRAW_CIRCLE_YC, DRAW_CIRCLE_DIAM, DRAW_CIRCLE_INTENSITY\n");
        out.push_str("    ; Draw 16-sided polygon approximation\n");
        out.push_str("    \n");
        out.push_str("    ; Read parameters BEFORE DP change\n");
        out.push_str("    LDB DRAW_CIRCLE_INTENSITY\n");
        out.push_str("    PSHS B              ; Save intensity\n");
        out.push_str("    LDB DRAW_CIRCLE_DIAM\n");
        out.push_str("    SEX                 ; Sign-extend to 16-bit\n");
        out.push_str("    LSRA                ; Divide by 2 = radius\n");
        out.push_str("    RORB\n");
        out.push_str("    STD DRAW_CIRCLE_TEMP   ; Save radius\n");
        out.push_str("    LDB DRAW_CIRCLE_XC\n");
        out.push_str("    SEX\n");
        out.push_str("    STD DRAW_CIRCLE_TEMP+2 ; Save xc\n");
        out.push_str("    LDB DRAW_CIRCLE_YC\n");
        out.push_str("    SEX\n");
        out.push_str("    STD DRAW_CIRCLE_TEMP+4 ; Save yc\n");
        out.push_str("    \n");
        out.push_str("    ; Setup BIOS\n");
        out.push_str("    LDA #$D0\n");
        out.push_str("    TFR A,DP\n");
        out.push_str("    JSR Reset0Ref\n");
        out.push_str("    \n");
        out.push_str("    ; Set intensity\n");
        out.push_str("    PULS A\n");
        out.push_str("    CMPA #$5F\n");
        out.push_str("    BEQ .DCR_INT_5F\n");
        out.push_str("    JSR Intensity_a\n");
        out.push_str("    BRA .DCR_AFTER_INT\n");
        out.push_str(".DCR_INT_5F:\n");
        out.push_str("    JSR Intensity_5F\n");
        out.push_str(".DCR_AFTER_INT:\n");
        out.push_str("    \n");
        out.push_str("    ; TODO: Generate 16 vertices with trig (simplified version uses 8-gon)\n");
        out.push_str("    ; For now, draw octagon approximation\n");
        out.push_str("    ; Move to start position (xc + radius, yc)\n");
        out.push_str("    LDD DRAW_CIRCLE_TEMP   ; radius\n");
        out.push_str("    ADDD DRAW_CIRCLE_TEMP+2 ; xc + radius\n");
        out.push_str("    TFR B,B\n");
        out.push_str("    PSHS B              ; Save X\n");
        out.push_str("    LDD DRAW_CIRCLE_TEMP+4 ; yc\n");
        out.push_str("    TFR B,A             ; Y to A\n");
        out.push_str("    PULS B              ; X to B\n");
        out.push_str("    JSR Moveto_d\n");
        out.push_str("    \n");
        out.push_str("    ; Simple octagon: 8 segments with fixed deltas\n");
        out.push_str("    ; This is simplified - full implementation would use SIN_TABLE\n");
        out.push_str("    LDD DRAW_CIRCLE_TEMP   ; radius\n");
        out.push_str("    TFR B,A             ; Use low byte only\n");
        out.push_str("    \n");
        out.push_str("    ; Segment 1: move (0, -r)\n");
        out.push_str("    CLR Vec_Misc_Count\n");
        out.push_str("    NEGA                ; -radius\n");
        out.push_str("    LDB #0\n");
        out.push_str("    JSR Draw_Line_d\n");
        out.push_str("    \n");
        out.push_str("    ; ... (simplified - full version would iterate all 16 segments)\n");
        out.push_str("    ; For now return (minimal octagon)\n");
        out.push_str("    RTS\n\n");
    }
    
    // DRAW_RECT_RUNTIME: Draw rectangle with runtime parameters
    if needed.contains("DRAW_RECT_RUNTIME") {
        out.push_str("DRAW_RECT_RUNTIME:\n");
        out.push_str("    ; Input: DRAW_RECT_X, DRAW_RECT_Y, DRAW_RECT_WIDTH, DRAW_RECT_HEIGHT, DRAW_RECT_INTENSITY\n");
        out.push_str("    ; Draws 4 sides of rectangle\n");
        out.push_str("    \n");
        out.push_str("    ; Save parameters to stack before DP change\n");
        out.push_str("    LDB DRAW_RECT_INTENSITY\n");
        out.push_str("    PSHS B\n");
        out.push_str("    LDB DRAW_RECT_HEIGHT\n");
        out.push_str("    PSHS B\n");
        out.push_str("    LDB DRAW_RECT_WIDTH\n");
        out.push_str("    PSHS B\n");
        out.push_str("    LDB DRAW_RECT_Y\n");
        out.push_str("    PSHS B\n");
        out.push_str("    LDB DRAW_RECT_X\n");
        out.push_str("    PSHS B\n");
        out.push_str("    \n");
        out.push_str("    ; Setup BIOS\n");
        out.push_str("    LDA #$D0\n");
        out.push_str("    TFR A,DP\n");
        out.push_str("    JSR Reset0Ref\n");
        out.push_str("    \n");
        out.push_str("    ; Set intensity\n");
        out.push_str("    LDA 4,S             ; intensity\n");
        out.push_str("    JSR Intensity_a\n");
        out.push_str("    \n");
        out.push_str("    ; Move to starting position (x, y)\n");
        out.push_str("    LDA 1,S             ; y\n");
        out.push_str("    LDB ,S              ; x\n");
        out.push_str("    JSR Moveto_d_7F\n");
        out.push_str("    \n");
        out.push_str("    ; Draw right side\n");
        out.push_str("    CLR Vec_Misc_Count\n");
        out.push_str("    LDA #0\n");
        out.push_str("    LDB 2,S             ; width\n");
        out.push_str("    JSR Draw_Line_d\n");
        out.push_str("    \n");
        out.push_str("    ; Draw down side\n");
        out.push_str("    CLR Vec_Misc_Count\n");
        out.push_str("    LDA 3,S             ; height\n");
        out.push_str("    NEGA                ; -height\n");
        out.push_str("    LDB #0\n");
        out.push_str("    JSR Draw_Line_d\n");
        out.push_str("    \n");
        out.push_str("    ; Draw left side\n");
        out.push_str("    CLR Vec_Misc_Count\n");
        out.push_str("    LDA #0\n");
        out.push_str("    LDB 2,S             ; width\n");
        out.push_str("    NEGB                ; -width\n");
        out.push_str("    JSR Draw_Line_d\n");
        out.push_str("    \n");
        out.push_str("    ; Draw up side\n");
        out.push_str("    CLR Vec_Misc_Count\n");
        out.push_str("    LDA 2,S             ; height\n");
        out.push_str("    NEGA                ; -height\n");
        out.push_str("    LDB #0\n");
        out.push_str("    JSR Draw_Line_d\n");
        out.push_str("    \n");
        out.push_str("    LEAS 5,S            ; Clean stack\n");
        out.push_str("    RTS\n\n");
    }
}
