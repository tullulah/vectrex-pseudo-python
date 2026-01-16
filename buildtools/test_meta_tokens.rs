use vpy_parser::lexer::lex;

fn main() {
    let code = "META TITLE = \"My Game\"\n";
    match lex(code) {
        Ok(tokens) => {
            for token in &tokens {
                println!("{:?}: {:?}", token.kind, token.line);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
