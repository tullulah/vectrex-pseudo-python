use vectrex_lang::{lexer, parser};

fn parse_ok(src: &str) -> (usize, usize) {
    let tokens = lexer::lex(src).expect("lex");
    let module = parser::parse_with_filename(&tokens, "<test>").expect("parse");
    let vls = module.items.iter().filter(|i| matches!(i, vectrex_lang::ast::Item::VectorList{..})).count();
    let funs = module.items.iter().filter(|i| matches!(i, vectrex_lang::ast::Item::Function(_))).count();
    (vls, funs)
}

#[test]
fn vectorlist_before_function() {
    let src = r#"vectorlist demo:
    ORIGIN
    MOVE -1 -1
    RECT -1 -1 1 1

def main():
    vectrex_draw_vectorlist("demo")
"#;
    let (vls,funs)=parse_ok(src); assert_eq!(vls,1); assert_eq!(funs,1);
}

#[test]
fn function_before_vectorlist() {
    let src = r#"def main():
    vectrex_draw_vectorlist("demo")

vectorlist demo:
    ORIGIN
    INTENSITY 10
"#;
    let (vls,funs)=parse_ok(src); assert_eq!(vls,1); assert_eq!(funs,1);
}

#[test]
fn polygon_vertices_signed() {
    let src = r#"vectorlist poly:
    POLYGON 4 0 -16 16 0 0 16 -16 0
"#; let (vls,funs)=parse_ok(src); assert_eq!(vls,1); assert_eq!(funs,0);
}

#[test]
fn meta_and_consts_mix_order() {
    let src = r#"META TITLE = "X"
CONST A = 1
vectorlist v:
    ORIGIN
CONST B = 2
def main():
    B
"#; let (vls,funs)=parse_ok(src); assert_eq!(vls,1); assert_eq!(funs,1);
}
