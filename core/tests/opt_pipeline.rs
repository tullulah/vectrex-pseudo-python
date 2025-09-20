use vectrex_lang::ast::*;
use vectrex_lang::codegen::debug_optimize_module_for_tests;

// S4: Constant folding test
#[test]
fn constant_folding_add_mul_identities() {
    let f = Function { name: "main".into(), params: vec![], body: vec![
        Stmt::Let { name: "a".into(), value: Expr::Binary { op: BinOp::Add, left: Box::new(Expr::Number(0)), right: Box::new(Expr::Number(5)) } },
        Stmt::Let { name: "b".into(), value: Expr::Binary { op: BinOp::Mul, left: Box::new(Expr::Number(1)), right: Box::new(Expr::Ident("a".into())) } },
        Stmt::Return(Some(Expr::Ident("b".into())))
    ]};
    let m = Module { items: vec![Item::Function(f)], meta: ModuleMeta::default() };
    let opt = debug_optimize_module_for_tests(&m);
    // After full optimization chain both lets get folded & DSE leaves only Return(Number(5)).
    if let Item::Function(fun) = &opt.items[0] {
        assert_eq!(fun.body.len(), 1, "expected single return after DSE");
        match &fun.body[0] { Stmt::Return(Some(Expr::Number(5))) => {}, _ => panic!("expected Return(Number(5)) got {:?}", fun.body[0]) }
    } else { panic!("expected function") }
}

// S4: Dead store elimination test
#[test]
fn dead_store_elimination_basic() {
    // x assigned then overwritten before any read; first assign should be removed.
    let f = Function { name: "f".into(), params: vec![], body: vec![
        Stmt::Let { name: "x".into(), value: Expr::Number(1) }, // dead
        Stmt::Assign { target: "x".into(), value: Expr::Number(2) },
        Stmt::Return(Some(Expr::Ident("x".into())))
    ]};
    let m = Module { items: vec![Item::Function(f)], meta: ModuleMeta::default() };
    let opt = debug_optimize_module_for_tests(&m); // pipeline incluye dead_store_elim
    if let Item::Function(fun) = &opt.items[0] {
        // Both the initial let and the store are dead after propagation; only Return(Number(2)).
        assert_eq!(fun.body.len(), 1, "unexpected stmt count after DSE: {:?}", fun.body);
        match &fun.body[0] { Stmt::Return(Some(Expr::Number(2))) => {}, _ => panic!("expected Return(Number(2))") }
    }
}
