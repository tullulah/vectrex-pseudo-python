use vectrex_lang::{lexer, parser};

fn expect_err(src:&str, needle:&str){ let toks=lexer::lex(src).expect("lex"); let e=parser::parse_with_filename(&toks, "file.vpy").err().expect("should err"); let msg = format!("{e}"); assert!(msg.contains(needle),"expected '{needle}' in '{msg}'"); }

#[test]
fn unknown_vectorlist_command() { expect_err("vectorlist s:\n    FOO\n", "Unknown vectorlist command"); }

#[test]
fn polygon_bad_count_expr() { expect_err("vectorlist s:\n    POLYGON x 0 0\n", "POLYGON expects count"); }

#[test]
fn polygon_vertex_non_number() { expect_err("vectorlist s:\n    POLYGON 2 0 A\n", "Expected number"); }

#[test]
fn unexpected_top_level() { expect_err("def\n", "Expected identifier"); }

#[test]
fn missing_indent_in_vectorlist() { expect_err("vectorlist s: \nORIGIN\n", "Expected Indent"); }
