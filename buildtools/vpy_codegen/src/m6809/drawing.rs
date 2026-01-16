//! Drawing Geometric Shapes
//!
//! Builtins for drawing circles, rectangles, polygons, etc.

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
