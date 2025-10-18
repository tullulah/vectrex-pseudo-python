use vectrex_lang::{lex, parse_with_filename, target::Target};

#[test]
fn smoke_minimal_compile() {
    let src = r#"CONST TITLE = "SMOKE"

vectorlist demo:
    INTENSITY 0x40
    MOVE 10 20

def main():
    let x = 1
    let y = x + 2
    y = y + 3
    return y
"#;
    let tokens = lex(src).expect("lex ok");
    assert!(tokens.len() > 0, "tokens generated");
    let module = parse_with_filename(&tokens, "smoke.vpy").expect("parse ok");
    assert!(module.items.len() >= 2, "expect at least vectorlist and function");
    // Emit 6809 asm
    let opts = vectrex_lang::codegen::CodegenOptions {
        title: "SMOKE".to_string(),
        auto_loop: true,
        diag_freeze: false,
        force_extended_jsr: false,
        _bank_size: 0,
        per_frame_silence: false,
        debug_init_draw: false,
        blink_intensity: false,
        exclude_ram_org: false,
        fast_wait: false, source_path: None,
    };
    let asm = vectrex_lang::codegen::emit_asm(&module, Target::Vectrex, &opts);
    assert!(asm.contains("MAIN:"), "asm should contain function label MAIN");
    assert!(asm.to_ascii_uppercase().contains("VECTREX_VECTOR_PHASE_BEGIN") == false, "no unused wrappers emitted by default");
}
