use anyhow::{bail, Result};

// TokenKind: enumeration of all lexical tokens in the pseudo-Python language.

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Def, Identifier(String), Number(i32), Newline, Indent, Dedent,
    LParen, RParen, Colon, Comma,
    Plus, Minus, Star, Slash, Percent,
    Amp, Pipe, Caret, Tilde,
    ShiftLeft, ShiftRight,
    Equal, If, Elif, Else, For, In, Range, Return, While, Break, Continue, Let,
    Switch, Case, Default,
    And, Or, Not,
    StringLit(String),
    True, False,
    EqEq, NotEq, Lt, Le, Gt, Ge,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

// lex: convert source text into a token stream with indentation tracking.
pub fn lex(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut lines = input.lines().enumerate();
    let mut indent_stack: Vec<usize> = vec![0];

    while let Some((i, raw_line)) = lines.next() {
        let line_no = i + 1;
        let trimmed = raw_line.trim();
        // Skip blank or comment-only lines (treat as whitespace, no indentation changes)
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let indent = raw_line.chars().take_while(|c| *c == ' ').count();
        if indent % 4 != 0 {
            bail!("Indentation must be multiples of 4 (line {})", line_no);
        }
        let current = *indent_stack.last().unwrap();
        if indent > current {
            indent_stack.push(indent);
            tokens.push(Token { kind: TokenKind::Indent, line: line_no, col: 1 });
        }
        while indent < *indent_stack.last().unwrap() {
            indent_stack.pop();
            tokens.push(Token { kind: TokenKind::Dedent, line: line_no, col: 1 });
        }
        lex_line(raw_line.trim(), line_no, &mut tokens)?;
        tokens.push(Token { kind: TokenKind::Newline, line: line_no, col: raw_line.len() });
    }

    while indent_stack.len() > 1 {
        indent_stack.pop();
        tokens.push(Token { kind: TokenKind::Dedent, line: 0, col: 0 });
    }
    tokens.push(Token { kind: TokenKind::EOF, line: 0, col: 0 });
    Ok(tokens)
}

// lex_line: tokenize a single logical line (whitespace-trimmed) without indentation.
fn lex_line(line: &str, line_no: usize, out: &mut Vec<Token>) -> Result<()> {
    let chars: Vec<char> = line.chars().collect();
    let mut idx = 0;
    while idx < chars.len() {
        let c = chars[idx];
        match c {
            ' ' => {
                idx += 1;
            }
            '(' => {
                out.push(tok(TokenKind::LParen, line_no, idx));
                idx += 1;
            }
            ')' => {
                out.push(tok(TokenKind::RParen, line_no, idx));
                idx += 1;
            }
            ':' => {
                out.push(tok(TokenKind::Colon, line_no, idx));
                idx += 1;
            }
            ',' => {
                out.push(tok(TokenKind::Comma, line_no, idx));
                idx += 1;
            }
            '+' => {
                out.push(tok(TokenKind::Plus, line_no, idx));
                idx += 1;
            }
            '-' => {
                out.push(tok(TokenKind::Minus, line_no, idx));
                idx += 1;
            }
            '#' => { break; }
            '*' => {
                out.push(tok(TokenKind::Star, line_no, idx));
                idx += 1;
            }
            '%' => { out.push(tok(TokenKind::Percent, line_no, idx)); idx += 1; }
            '/' => {
                out.push(tok(TokenKind::Slash, line_no, idx));
                idx += 1;
            }
            '&' => {
                out.push(tok(TokenKind::Amp, line_no, idx));
                idx += 1;
            }
            '|' => {
                out.push(tok(TokenKind::Pipe, line_no, idx));
                idx += 1;
            }
            '^' => {
                out.push(tok(TokenKind::Caret, line_no, idx));
                idx += 1;
            }
            '~' => {
                out.push(tok(TokenKind::Tilde, line_no, idx));
                idx += 1;
            }
            '=' => {
                if idx + 1 < chars.len() && chars[idx + 1] == '=' {
                    out.push(tok(TokenKind::EqEq, line_no, idx));
                    idx += 2;
                } else {
                    out.push(tok(TokenKind::Equal, line_no, idx));
                    idx += 1;
                }
            }
            '!' => {
                if idx + 1 < chars.len() && chars[idx + 1] == '=' {
                    out.push(tok(TokenKind::NotEq, line_no, idx));
                    idx += 2;
                } else {
                    bail!("Unexpected '!' (did you mean != ?) line {}", line_no);
                }
            }
            '<' => {
                if idx + 1 < chars.len() && chars[idx + 1] == '<' {
                    out.push(tok(TokenKind::ShiftLeft, line_no, idx));
                    idx += 2;
                } else if idx + 1 < chars.len() && chars[idx + 1] == '=' {
                    out.push(tok(TokenKind::Le, line_no, idx));
                    idx += 2;
                } else {
                    out.push(tok(TokenKind::Lt, line_no, idx));
                    idx += 1;
                }
            }
            '>' => {
                if idx + 1 < chars.len() && chars[idx + 1] == '>' {
                    out.push(tok(TokenKind::ShiftRight, line_no, idx));
                    idx += 2;
                } else if idx + 1 < chars.len() && chars[idx + 1] == '=' {
                    out.push(tok(TokenKind::Ge, line_no, idx));
                    idx += 2;
                } else {
                    out.push(tok(TokenKind::Gt, line_no, idx));
                    idx += 1;
                }
            }
            '0'..='9' => {
                let start = idx;
                if chars[idx] == '0' && idx + 1 < chars.len() && (chars[idx+1] == 'x' || chars[idx+1] == 'X') {
                    idx += 2;
                    let hs = idx;
                    while idx < chars.len() && chars[idx].is_ascii_hexdigit() { idx += 1; }
                    let num = i32::from_str_radix(&line[hs..idx], 16).unwrap_or(0);
                    out.push(tok(TokenKind::Number(num), line_no, start));
                } else if chars[idx] == '0' && idx + 1 < chars.len() && (chars[idx+1] == 'b' || chars[idx+1] == 'B') {
                    idx += 2;
                    let bs = idx;
                    while idx < chars.len() && (chars[idx] == '0' || chars[idx] == '1') { idx += 1; }
                    let num = i32::from_str_radix(&line[bs..idx], 2).unwrap_or(0);
                    out.push(tok(TokenKind::Number(num), line_no, start));
                } else {
                    while idx < chars.len() && chars[idx].is_ascii_digit() { idx += 1; }
                    let num: i32 = line[start..idx].parse().unwrap();
                    out.push(tok(TokenKind::Number(num), line_no, start));
                }
            }
            '"' => {
                let start_col = idx;
                idx += 1; // skip opening quote
                let lit_start = idx;
                while idx < chars.len() && chars[idx] != '"' { idx += 1; }
                if idx >= chars.len() { bail!("Unterminated string literal line {} col {}", line_no, start_col + 1); }
                let s: String = chars[lit_start..idx].iter().collect();
                out.push(tok(TokenKind::StringLit(s), line_no, start_col));
                idx += 1; // skip closing quote
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = idx;
                while idx < chars.len() && (chars[idx].is_alphanumeric() || chars[idx] == '_') {
                    idx += 1;
                }
                let ident = &line[start..idx];
                let kind = match ident {
                    "def" => TokenKind::Def,
                    "if" => TokenKind::If,
                    "elif" => TokenKind::Elif,
                    "else" => TokenKind::Else,
                    "for" => TokenKind::For,
                    "let" => TokenKind::Let,
                    "while" => TokenKind::While,
                    "break" => TokenKind::Break,
                    "continue" => TokenKind::Continue,
                    "switch" => TokenKind::Switch,
                    "case" => TokenKind::Case,
                    "default" => TokenKind::Default,
                    "in" => TokenKind::In,
                    "range" => TokenKind::Range,
                    "return" => TokenKind::Return,
                    "and" => TokenKind::And,
                    "or" => TokenKind::Or,
                    "not" => TokenKind::Not,
                    "True" => TokenKind::True,
                    "False" => TokenKind::False,
                    _ => TokenKind::Identifier(ident.to_string()),
                };
                out.push(tok(kind, line_no, start));
            }
            _ => bail!("Unexpected char '{}' line {} col {}", c, line_no, idx + 1),
        }
    }
    Ok(())
}

// tok: convenience constructor for tokens.
fn tok(kind: TokenKind, line: usize, col: usize) -> Token {
    Token { kind, line, col }
}
