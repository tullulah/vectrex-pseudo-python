use vectrex_lang::{lexer, parser, ast};

fn parse(src:&str)->ast::Module { let toks=lexer::lex(src).expect("lex"); parser::parse_with_filename(&toks, "<test>").expect("parse") }

#[test]
fn chained_comparisons() { let m=parse("def main():\n    1 < 2 < 3\n"); assert!(matches!(m.items[0], ast::Item::Function(_))); }

#[test]
fn logic_and_or() { parse("def main():\n    (1<2) and 3<4 or 5<6\n"); }

#[test]
fn for_while_if_switch() { let src = r#"def main():
    let x = 0
    for i in range(0,10,2):
        x
    while x < 10:
        x
    if x:
        x
    elif 0:
        x
    else:
        x
    switch x:
        case 1:
            x
        case 2:
            x
        default:
            x
"#; parse(src); }

#[test]
fn qualified_call_turns_into_identifier_with_underscores() { let m=parse("def main():\n    foo.bar.baz(1)\n"); if let ast::Item::Function(f)=&m.items[0] { if let ast::Stmt::Expr(ast::Expr::Call { name, .. }) = &f.body[0] { assert_eq!(name, "foo_bar_baz"); } else { panic!("unexpected stmt"); } } }

#[test]
fn multiple_vectorlists_and_order() { let src = r#"vectorlist a:
    ORIGIN
vectorlist b:
    ORIGIN
META TITLE = "X"
CONST A = 1
var B = 2
vectorlist c:
    ORIGIN

def main():
    A
"#; let m=parse(src); let vls = m.items.iter().filter(|i| matches!(i, ast::Item::VectorList{..})).count(); assert_eq!(vls,3); }

#[test]
fn intensity_arithmetic_constant_folding() { let m=parse("vectorlist s:\n    INTENSITY 2+3*4\n"); if let ast::Item::VectorList { entries, .. } = &m.items[0] { match entries[0] { ast::VlEntry::Intensity(v) => assert_eq!(v, 14), _=>panic!() } } }

#[test]
fn move_and_rect_signed_numbers() { parse("vectorlist s:\n    MOVE -5 6\n    RECT -1 -2 3 4\n"); }

#[test]
fn blank_lines_inside_vectorlist() { parse("vectorlist s:\n    ORIGIN\n\n    ORIGIN\n"); }
