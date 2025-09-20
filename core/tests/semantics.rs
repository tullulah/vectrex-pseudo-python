use vectrex_lang::ast::*;

#[test]
fn semantics_valid_decl_and_use() {
    let module = Module { items: vec![
        Item::Const { name: "C1".to_string(), value: Expr::Number(5) },
        Item::Function(Function { name: "main".to_string(), params: vec!["p".to_string()], body: vec![
            Stmt::Let { name: "x".to_string(), value: Expr::Ident("p".to_string()) },
            Stmt::Assign { target: "x".to_string(), value: Expr::Binary { op: BinOp::Add, left: Box::new(Expr::Ident("x".into())), right: Box::new(Expr::Ident("C1".into())) } },
            Stmt::Return(Some(Expr::Ident("x".into())))
        ]})
    ], meta: ModuleMeta::default() };
    // emit_asm should not panic
    let asm = vectrex_lang::codegen::emit_asm(&module, vectrex_lang::target::Target::Vectrex, &vectrex_lang::codegen::CodegenOptions {
        title: "t".into(),
        auto_loop: false,
        diag_freeze: false,
        force_extended_jsr: false,
        _bank_size: 0,
        per_frame_silence: false,
        debug_init_draw: false,
        blink_intensity: false,
        exclude_ram_org: false,
        fast_wait: false,
    });
    assert!(asm.contains("main"));
}

#[test]
#[should_panic(expected = "SemanticsError: uso de variable no declarada 'y'")]
fn semantics_undefined_use_panics() {
    let module = Module { items: vec![
        Item::Function(Function { name: "f".to_string(), params: vec![], body: vec![
            Stmt::Expr(Expr::Ident("y".into()))
        ]})
    ], meta: ModuleMeta::default() };
    let _ = vectrex_lang::codegen::emit_asm(&module, vectrex_lang::target::Target::Vectrex, &vectrex_lang::codegen::CodegenOptions {
        title: "t".into(), auto_loop: false, diag_freeze: false, force_extended_jsr: false, _bank_size: 0, per_frame_silence: false, debug_init_draw: false, blink_intensity: false, exclude_ram_org: false, fast_wait: false });
}
