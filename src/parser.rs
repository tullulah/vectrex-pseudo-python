use anyhow::{bail, Result};
use crate::ast::*;
use crate::lexer::{Token, TokenKind};

// parse: entry point converting token slice into AST Module.
pub fn parse(tokens: &[Token]) -> Result<Module> {
    let mut p = Parser { tokens, pos: 0 };
    let mut items = Vec::new();
    while !p.check(TokenKind::EOF) {
        items.push(p.function()?);
    }
    Ok(Module { items })
}

// Parser: recursive descent parser state.
struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    // function: parse a function definition.
    fn function(&mut self) -> Result<Item> {
        self.consume(TokenKind::Def)?;
        let name = self.identifier()?;
        self.consume(TokenKind::LParen)?;
        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                params.push(self.identifier()?);
                if self.match_kind(&TokenKind::Comma) {
                    continue;
                }
                break;
            }
        }
        self.consume(TokenKind::RParen)?;
        self.consume(TokenKind::Colon)?;
        self.consume(TokenKind::Newline)?;
        self.consume(TokenKind::Indent)?;
        let mut body = Vec::new();
        while !self.match_kind(&TokenKind::Dedent) {
            body.push(self.statement()?);
        }
        Ok(Item::Function(Function { name, params, body }))
    }

    // statement: dispatch to specific statement forms.
    fn statement(&mut self) -> Result<Stmt> {
        if self.match_kind(&TokenKind::For) {
            return self.for_stmt();
        }
        if self.match_kind(&TokenKind::While) {
            return self.while_stmt();
        }
        if self.match_kind(&TokenKind::If) {
            return self.if_stmt();
        }
        if self.match_kind(&TokenKind::Return) {
            return self.return_stmt();
        }
        if self.match_kind(&TokenKind::Break) {
            self.consume(TokenKind::Newline)?;
            return Ok(Stmt::Break);
        }
        if self.match_kind(&TokenKind::Continue) {
            self.consume(TokenKind::Newline)?;
            return Ok(Stmt::Continue);
        }
        if let Some(name) = self.try_identifier() {
            if self.match_kind(&TokenKind::Equal) {
                let expr = self.expression()?;
                self.consume(TokenKind::Newline)?;
                return Ok(Stmt::Assign { target: name, value: expr });
            } else {
                self.unread_identifier(name);
            }
        }
        let expr = self.expression()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Expr(expr))
    }

    // while_stmt: parse a while loop.
    fn while_stmt(&mut self) -> Result<Stmt> {
        let cond = self.expression()?;
        self.consume(TokenKind::Colon)?;
        self.consume(TokenKind::Newline)?;
        self.consume(TokenKind::Indent)?;
        let mut body = Vec::new();
        while !self.match_kind(&TokenKind::Dedent) {
            body.push(self.statement()?);
        }
        Ok(Stmt::While { cond, body })
    }

    // return_stmt: parse return with optional expression.
    fn return_stmt(&mut self) -> Result<Stmt> {
        if self.check(TokenKind::Newline) {
            self.consume(TokenKind::Newline)?;
            return Ok(Stmt::Return(None));
        }
        let expr = self.expression()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Return(Some(expr)))
    }

    // for_stmt: parse range-based for loop.
    fn for_stmt(&mut self) -> Result<Stmt> {
        let var = self.identifier()?;
        self.consume(TokenKind::In)?;
        self.consume(TokenKind::Range)?;
        self.consume(TokenKind::LParen)?;
        let start = self.expression()?;
        self.consume(TokenKind::Comma)?;
        let end = self.expression()?;
        let step = if self.match_kind(&TokenKind::Comma) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenKind::RParen)?;
        self.consume(TokenKind::Colon)?;
        self.consume(TokenKind::Newline)?;
        self.consume(TokenKind::Indent)?;
        let mut body = Vec::new();
        while !self.match_kind(&TokenKind::Dedent) {
            body.push(self.statement()?);
        }
        Ok(Stmt::For { var, start, end, step, body })
    }

    // if_stmt: parse if/elif/else chain.
    fn if_stmt(&mut self) -> Result<Stmt> {
        let cond = self.expression()?;
        self.consume(TokenKind::Colon)?;
        self.consume(TokenKind::Newline)?;
        self.consume(TokenKind::Indent)?;
        let mut body = Vec::new();
        while !self.match_kind(&TokenKind::Dedent) {
            body.push(self.statement()?);
        }
        let mut elifs = Vec::new();
        while self.match_kind(&TokenKind::Elif) {
            let econd = self.expression()?;
            self.consume(TokenKind::Colon)?;
            self.consume(TokenKind::Newline)?;
            self.consume(TokenKind::Indent)?;
            let mut ebody = Vec::new();
            while !self.match_kind(&TokenKind::Dedent) {
                ebody.push(self.statement()?);
            }
            elifs.push((econd, ebody));
        }
        let else_body = if self.match_kind(&TokenKind::Else) {
            self.consume(TokenKind::Colon)?;
            self.consume(TokenKind::Newline)?;
            self.consume(TokenKind::Indent)?;
            let mut e = Vec::new();
            while !self.match_kind(&TokenKind::Dedent) {
                e.push(self.statement()?);
            }
            Some(e)
        } else {
            None
        };
        Ok(Stmt::If { cond, body, elifs, else_body })
    }

    // expression: parse top-level expression (logic_or).
    fn expression(&mut self) -> Result<Expr> { self.logic_or() }

    // logic_or: parse left-associative 'or' chain.
    fn logic_or(&mut self) -> Result<Expr> {
        let mut node = self.logic_and()?;
        while self.match_kind(&TokenKind::Or) {
            let rhs = self.logic_and()?;
            node = Expr::Logic { op: crate::ast::LogicOp::Or, left: Box::new(node), right: Box::new(rhs) };
        }
        Ok(node)
    }

    // logic_and: parse left-associative 'and' chain.
    fn logic_and(&mut self) -> Result<Expr> {
        let mut node = self.bit_or()?;
        while self.match_kind(&TokenKind::And) {
            let rhs = self.bit_or()?;
            node = Expr::Logic { op: crate::ast::LogicOp::And, left: Box::new(node), right: Box::new(rhs) };
        }
        Ok(node)
    }

    // bit_or: handle '|'
    fn bit_or(&mut self) -> Result<Expr> {
        let mut node = self.bit_xor()?;
        while self.match_kind(&TokenKind::Pipe) {
            let rhs = self.bit_xor()?;
            node = Expr::Binary { op: crate::ast::BinOp::BitOr, left: Box::new(node), right: Box::new(rhs) };
        }
        Ok(node)
    }

    // bit_xor: handle '^'
    fn bit_xor(&mut self) -> Result<Expr> {
        let mut node = self.bit_and()?;
        while self.match_kind(&TokenKind::Caret) {
            let rhs = self.bit_and()?;
            node = Expr::Binary { op: crate::ast::BinOp::BitXor, left: Box::new(node), right: Box::new(rhs) };
        }
        Ok(node)
    }

    // bit_and: handle '&'
    fn bit_and(&mut self) -> Result<Expr> {
        let mut node = self.shift()?;
        while self.match_kind(&TokenKind::Amp) {
            let rhs = self.shift()?;
            node = Expr::Binary { op: crate::ast::BinOp::BitAnd, left: Box::new(node), right: Box::new(rhs) };
        }
        Ok(node)
    }

    // shift: handle << >> after comparison tier
    fn shift(&mut self) -> Result<Expr> {
        let mut node = self.comparison()?;
        while let Some(op) = if self.match_kind(&TokenKind::ShiftLeft) { Some(BinOp::Shl) } else if self.match_kind(&TokenKind::ShiftRight) { Some(BinOp::Shr) } else { None } {
            let rhs = self.comparison()?;
            node = Expr::Binary { op, left: Box::new(node), right: Box::new(rhs) };
        }
        Ok(node)
    }

    // comparison: parse comparison expressions with optional chaining (a < b < c).
    fn comparison(&mut self) -> Result<Expr> {
        let first = self.term()?;
        // If no comparison operator follows, return the term directly.
        if let Some(op0) = self.match_cmp_op() {
            // Collect subsequent operands and operators.
            let mut operands: Vec<Expr> = Vec::new();
            let mut ops: Vec<CmpOp> = Vec::new();
            operands.push(first); // operand0
            ops.push(op0);
            operands.push(self.term()?); // operand1
            while let Some(next_op) = self.match_cmp_op() {
                ops.push(next_op);
                operands.push(self.term()?);
            }
            // If only one op, just build a single Compare.
            if ops.len() == 1 {
                let left = operands.remove(0);
                let right = operands.remove(0);
                return Ok(Expr::Compare { op: ops[0], left: Box::new(left), right: Box::new(right) });
            }
            // Build left-associative chain: (a op0 b) AND (b op1 c) AND ...
            let mut chain: Option<Expr> = None;
            for i in 0..ops.len() {
                let cmp = Expr::Compare {
                    op: ops[i],
                    left: Box::new(operands[i].clone()),
                    right: Box::new(operands[i + 1].clone()),
                };
                chain = Some(if let Some(acc) = chain {
                    Expr::Logic { op: crate::ast::LogicOp::And, left: Box::new(acc), right: Box::new(cmp) }
                } else {
                    cmp
                });
            }
            return Ok(chain.unwrap());
        }
        Ok(first)
    }

    // term: parse addition/subtraction.
    fn term(&mut self) -> Result<Expr> {
        let mut node = self.factor()?;
        while let Some(op) = if self.match_kind(&TokenKind::Plus) {
            Some(crate::ast::BinOp::Add)
        } else if self.match_kind(&TokenKind::Minus) {
            Some(crate::ast::BinOp::Sub)
        } else {
            None
        } {
            let right = self.factor()?;
            node = Expr::Binary { op, left: Box::new(node), right: Box::new(right) };
        }
        Ok(node)
    }

    // factor: parse multiplication/division.
    fn factor(&mut self) -> Result<Expr> {
        let mut node = self.unary()?;
        while let Some(op) = if self.match_kind(&TokenKind::Star) { Some(BinOp::Mul) }
            else if self.match_kind(&TokenKind::Slash) { Some(BinOp::Div) }
            else if self.match_kind(&TokenKind::Percent) { Some(BinOp::Mod) }
            else { None } {
            let right = self.unary()?;
            node = Expr::Binary { op, left: Box::new(node), right: Box::new(right) };
        }
        Ok(node)
    }

    // unary: handle prefix not / + / -
    fn unary(&mut self) -> Result<Expr> {
        if self.match_kind(&TokenKind::Not) {
            let inner = self.unary()?;
            return Ok(Expr::Not(Box::new(inner)));
        }
        if self.match_kind(&TokenKind::Minus) {
            // Represent -X as 0 - X to avoid new AST variant.
            let rhs = self.unary()?;
            return Ok(Expr::Binary { op: crate::ast::BinOp::Sub, left: Box::new(Expr::Number(0)), right: Box::new(rhs) });
        }
        if self.match_kind(&TokenKind::Plus) {
            return self.unary();
        }
        if self.match_kind(&TokenKind::Tilde) {
            let inner = self.unary()?;
            return Ok(Expr::BitNot(Box::new(inner)));
        }
        self.primary()
    }

    // primary: parse literals, identifiers, calls, parenthesized expressions.
    fn primary(&mut self) -> Result<Expr> {
        if let Some(n) = self.match_number() {
            return Ok(Expr::Number(n));
        }
        if self.match_kind(&TokenKind::True) {
            return Ok(Expr::Number(1));
        }
        if self.match_kind(&TokenKind::False) {
            return Ok(Expr::Number(0));
        }
        if let Some(name) = self.match_identifier() {
            if self.match_kind(&TokenKind::LParen) {
                let mut args = Vec::new();
                if !self.check(TokenKind::RParen) {
                    loop {
                        args.push(self.expression()?);
                        if self.match_kind(&TokenKind::Comma) {
                            continue;
                        }
                        break;
                    }
                }
                self.consume(TokenKind::RParen)?;
                return Ok(Expr::Call { name, args });
            }
            return Ok(Expr::Ident(name));
        }
        if self.match_kind(&TokenKind::LParen) {
            let e = self.expression()?;
            self.consume(TokenKind::RParen)?;
            return Ok(e);
        }
        bail!("Unexpected token {:?}", self.peek().kind)
    }

    // match_cmp_op: attempt to parse a comparison operator.
    fn match_cmp_op(&mut self) -> Option<crate::ast::CmpOp> {
        use crate::ast::CmpOp;
        let op = if self.match_kind(&TokenKind::EqEq) {
            Some(CmpOp::Eq)
        } else if self.match_kind(&TokenKind::NotEq) {
            Some(CmpOp::Ne)
        } else if self.match_kind(&TokenKind::Le) {
            Some(CmpOp::Le)
        } else if self.match_kind(&TokenKind::Ge) {
            Some(CmpOp::Ge)
        } else if self.match_kind(&TokenKind::Lt) {
            Some(CmpOp::Lt)
        } else if self.match_kind(&TokenKind::Gt) {
            Some(CmpOp::Gt)
        } else {
            None
        };
        op
    }


    // Token utilities below.
    fn match_number(&mut self) -> Option<i32> {
        if let TokenKind::Number(n) = &self.peek().kind {
            let n = *n;
            self.pos += 1;
            Some(n)
        } else {
            None
        }
    }
    fn match_identifier(&mut self) -> Option<String> {
        if let TokenKind::Identifier(s) = &self.peek().kind {
            let s = s.clone();
            self.pos += 1;
            Some(s)
        } else {
            None
        }
    }
    fn try_identifier(&mut self) -> Option<String> { self.match_identifier() }
    fn unread_identifier(&mut self, name: String) {
        self.pos -= 1;
        if let TokenKind::Identifier(s) = &self.tokens[self.pos].kind {
            assert_eq!(&name, s);
        }
    }
    fn identifier(&mut self) -> Result<String> { self.match_identifier().ok_or_else(|| anyhow::anyhow!("Expected identifier")) }
    fn consume(&mut self, kind: TokenKind) -> Result<()> {
        if std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&kind) {
            self.pos += 1;
            Ok(())
        } else {
            bail!("Expected {:?} got {:?}", kind, self.peek().kind)
        }
    }
    fn check(&self, kind: TokenKind) -> bool { std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&kind) }
    fn match_kind(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind.clone()) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn peek(&self) -> &Token { &self.tokens[self.pos] }
}
