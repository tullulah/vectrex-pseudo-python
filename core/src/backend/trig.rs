// Shared trig table generation to avoid duplication across backends.
// Provides SIN_TABLE, COS_TABLE, TAN_TABLE data strings and raw vectors.

pub struct TrigTables {
    pub sin: Vec<i16>,
    pub cos: Vec<i16>,
    pub tan: Vec<i16>,
}

pub fn generate_trig_tables() -> TrigTables {
    let mut sin_vals: Vec<i16> = Vec::new();
    for i in 0..128 { let ang = (i as f32) * std::f32::consts::TAU / 128.0; sin_vals.push((ang.sin()*127.0).round() as i16); }
    let mut cos_vals: Vec<i16> = Vec::new();
    for i in 0..128 { let ang = (i as f32) * std::f32::consts::TAU / 128.0; cos_vals.push((ang.cos()*127.0).round() as i16); }
    let mut tan_vals: Vec<i16> = Vec::new();
    for i in 0..128 { let ang = (i as f32) * std::f32::consts::TAU / 128.0; let t = ang.tan(); let v = if t.is_finite() { (t.clamp(-6.0, 6.0)*20.0).round() as i16 } else { 0 }; tan_vals.push(v); }
    TrigTables { sin: sin_vals, cos: cos_vals, tan: tan_vals }
}

pub fn emit_trig_tables<T: std::fmt::Write>(out: &mut T, word_directive: &str) {
    let tbl = generate_trig_tables();
    let write_table = |out: &mut T, label: &str, data: &Vec<i16>| {
        let _ = writeln!(out, "{}:", label);
        for v in data {
            let _ = writeln!(out, "    {} {}", word_directive, *v);
        }
    };
    write_table(out, "SIN_TABLE", &tbl.sin);
    write_table(out, "COS_TABLE", &tbl.cos);
    write_table(out, "TAN_TABLE", &tbl.tan);
}
