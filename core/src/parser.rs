use anyhow::{bail, Result};
use crate::ast::*;
use crate::lexer::{Token, TokenKind};

// Public entrypoint (filename-aware only)
pub fn parse_with_filename(tokens: &[Token], filename: &str) -> Result<Module> {
    let mut p = Parser { tokens, pos: 0, filename: filename.to_string() };
    p.parse_module()
}

// Small constant folder used for vectorlist numeric arguments.
fn const_eval(e: &Expr) -> Option<i32> {
    use crate::ast::BinOp;
    match e {
        Expr::Number(n) => Some(*n),
        Expr::Binary { op, left, right } => {
            let l = const_eval(left)?; let r = const_eval(right)?;
            let v = match op {
                BinOp::Add => l.wrapping_add(r),
                BinOp::Sub => l.wrapping_sub(r),
                BinOp::Mul => l.wrapping_mul(r),
                BinOp::Div => if r!=0 { l.wrapping_div(r) } else { return None },
                BinOp::Mod => if r!=0 { l.wrapping_rem(r) } else { return None },
                BinOp::Shl => l.wrapping_shl((r & 0xF) as u32),
                BinOp::Shr => ((l as u32) >> (r & 0xF)) as i32,
                BinOp::BitAnd => l & r,
                BinOp::BitOr => l | r,
                BinOp::BitXor => l ^ r,
            };
            Some(v & 0xFFFF)
        }
        Expr::Not(inner) => const_eval(inner).map(|v| if (v & 0xFFFF)==0 {1} else {0}),
        Expr::BitNot(inner) => const_eval(inner).map(|v| !v & 0xFFFF),
        _ => None,
    }
}

struct Parser<'a> { tokens: &'a [Token], pos: usize, filename: String }

impl<'a> Parser<'a> {
    fn parse_module(&mut self) -> Result<Module> {
        let mut items = Vec::new();
        let mut meta = ModuleMeta::default();
    while !self.check(TokenKind::Eof) {
            // skip structural noise
            while self.match_kind(&TokenKind::Newline) {}
            while self.match_kind(&TokenKind::Dedent) {}
            if self.check(TokenKind::Eof) { break; }
            if self.match_kind(&TokenKind::Const) || self.match_ident_case("CONST") {
                let name = self.identifier()?;
                self.consume(TokenKind::Equal)?;
                let value = self.expression()?;
                self.consume(TokenKind::Newline)?;
                if name.eq_ignore_ascii_case("TITLE") { if let Expr::StringLit(s)=&value { meta.title_override = Some(s.clone()); } }
                items.push(Item::Const { name, value });
                continue;
            }
            if self.match_kind(&TokenKind::Var) || self.match_ident_case("VAR") {
                let name = self.identifier()?;
                self.consume(TokenKind::Equal)?;
                let value = self.expression()?;
                self.consume(TokenKind::Newline)?;
                items.push(Item::GlobalLet { name, value });
                continue;
            }
            if self.match_kind(&TokenKind::Meta) || self.match_ident_case("META") {
                let key = self.identifier()?;
                self.consume(TokenKind::Equal)?;
                let value = self.expression()?;
                self.consume(TokenKind::Newline)?;
                if let Expr::StringLit(s)=&value { meta.metas.insert(key.to_uppercase(), s.clone()); }
                if key.eq_ignore_ascii_case("TITLE") { if let Expr::StringLit(s)=&value { meta.title_override = Some(s.clone()); } }
                else if key.eq_ignore_ascii_case("MUSIC") { if let Expr::StringLit(s)=&value { meta.music_override = Some(s.clone()); } }
                else if key.eq_ignore_ascii_case("COPYRIGHT") { if let Expr::StringLit(s)=&value { meta.copyright_override = Some(s.clone()); } }
                continue;
            }
            if self.match_kind(&TokenKind::VectorList) || self.match_ident_case("VECTORLIST") {
                // if keyword matched as identifier the token already consumed. If actual keyword token kind consumed above.
                let vl = self.parse_vectorlist()?; items.push(vl); continue;
            }
            if self.check(TokenKind::Def) { items.push(self.function()?); continue; }
            return self.err_here(&format!("Unexpected token {:?} at top-level", self.peek().kind));
        }
        Ok(Module { items, meta })
    }

    // --- vectorlist ---
    fn parse_vectorlist(&mut self) -> Result<Item> {
        let name = self.identifier()?;
        self.consume(TokenKind::Colon)?;
        self.consume(TokenKind::Newline)?;
        self.consume(TokenKind::Indent)?;
        let mut entries: Vec<VlEntry> = Vec::new();
        loop {
            while self.match_kind(&TokenKind::Newline) {}
            if self.check(TokenKind::Dedent) { self.match_kind(&TokenKind::Dedent); break; }
            if self.check(TokenKind::Eof) { break; }
            let cmd = match self.peek().kind.clone() { TokenKind::Identifier(s) => s, _ => break };
            let upper = cmd.to_ascii_uppercase();
            // consume identifier
            self.match_identifier();
            match upper.as_str() {
                "INTENSITY" => {
                    let expr = self.expression()?; if let Some(v)=const_eval(&expr) { entries.push(VlEntry::Intensity(v)); } else { return self.err_here("Expected number after INTENSITY"); }
                }
                "ORIGIN" => entries.push(VlEntry::Origin),
                "MOVE" => { let x=self.parse_signed_number()?; let y=self.parse_signed_number()?; entries.push(VlEntry::Move(x,y)); }
                "RECT" => {
                    let x1=self.parse_signed_number()?; let y1=self.parse_signed_number()?; let x2=self.parse_signed_number()?; let y2=self.parse_signed_number()?; entries.push(VlEntry::Rect(x1,y1,x2,y2));
                }
                "POLYGON" => {
                    // Count can be an expression, vertices must be literal signed ints (no binary ops across coords).
                    let cnt_expr = self.expression()?;
                    let n = if let Some(nn) = const_eval(&cnt_expr) { nn } else { return self.err_here("POLYGON expects count"); };
                    if !(2..=256).contains(&n) { return self.err_here("POLYGON count out of range"); }
                    let mut verts = Vec::new();
                    for _ in 0..n { let x = self.parse_signed_number()?; let y = self.parse_signed_number()?; verts.push((x,y)); }
                    entries.push(VlEntry::Polygon(verts));
                }
                "CIRCLE" => {
                    // CIRCLE cx cy r [segs]
                    let cx = self.parse_signed_number()?; let cy = self.parse_signed_number()?; let r = self.parse_signed_number()?;
                    let segs = if !self.check(TokenKind::Newline) { self.parse_signed_number().unwrap_or(16) } else { 16 };
                    let segs = segs.clamp(3,64);
                    entries.push(VlEntry::Circle { cx, cy, r, segs });
                }
                "ARC" => {
                    // ARC cx cy r startDeg sweepDeg [segs]
                    let cx = self.parse_signed_number()?; let cy = self.parse_signed_number()?; let r = self.parse_signed_number()?; let start = self.parse_signed_number()?; let sweep = self.parse_signed_number()?;
                    let segs = if !self.check(TokenKind::Newline) { self.parse_signed_number().unwrap_or(16) } else { 16 };
                    let segs = segs.clamp(2,128);
                    entries.push(VlEntry::Arc { cx, cy, r, start_deg: start, sweep_deg: sweep, segs });
                }
                "SPIRAL" => {
                    // SPIRAL cx cy r_start r_end turns [segs]
                    let cx = self.parse_signed_number()?; let cy = self.parse_signed_number()?; let rs = self.parse_signed_number()?; let re = self.parse_signed_number()?; let turns = self.parse_signed_number()?;
                    let segs = if !self.check(TokenKind::Newline) { self.parse_signed_number().unwrap_or(64) } else { 64 };
                    let segs = segs.clamp(4,256);
                    entries.push(VlEntry::Spiral { cx, cy, r_start: rs, r_end: re, turns, segs });
                }
                _ => return self.err_here(&format!("Unknown vectorlist command {}", cmd)),
            }
            if self.check(TokenKind::Newline) { self.match_kind(&TokenKind::Newline); }
        }
        Ok(Item::VectorList { name, entries })
    }

    // --- functions / statements ---
    fn function(&mut self) -> Result<Item> {
        self.consume(TokenKind::Def)?;
        let name = self.identifier()?;
        self.consume(TokenKind::LParen)?;
        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop { params.push(self.identifier()?); if self.match_kind(&TokenKind::Comma) { continue; } break; }
        }
        self.consume(TokenKind::RParen)?; self.consume(TokenKind::Colon)?; self.consume(TokenKind::Newline)?; self.consume(TokenKind::Indent)?;
        let mut body = Vec::new();
        while !self.match_kind(&TokenKind::Dedent) { body.push(self.statement()?); }
        Ok(Item::Function(Function { name, params, body }))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.match_kind(&TokenKind::Let) { let name=self.identifier()?; self.consume(TokenKind::Equal)?; let value=self.expression()?; self.consume(TokenKind::Newline)?; return Ok(Stmt::Let { name, value }); }
        if self.match_kind(&TokenKind::For) { return self.for_stmt(); }
        if self.match_kind(&TokenKind::While) { return self.while_stmt(); }
        if self.match_kind(&TokenKind::If) { return self.if_stmt(); }
        if self.match_kind(&TokenKind::Switch) { return self.switch_stmt(); }
        if self.match_kind(&TokenKind::Return) { return self.return_stmt(); }
        if self.match_kind(&TokenKind::Break) { self.consume(TokenKind::Newline)?; return Ok(Stmt::Break); }
        if self.match_kind(&TokenKind::Continue) { self.consume(TokenKind::Newline)?; return Ok(Stmt::Continue); }
        if let Some(name)=self.try_identifier() { if self.match_kind(&TokenKind::Equal) { let expr=self.expression()?; self.consume(TokenKind::Newline)?; return Ok(Stmt::Assign { target:name, value:expr }); } else { self.unread_identifier(name); } }
        let expr = self.expression()?; self.consume(TokenKind::Newline)?; Ok(Stmt::Expr(expr))
    }

    fn switch_stmt(&mut self) -> Result<Stmt> {
        let expr = self.expression()?; self.consume(TokenKind::Colon)?; self.consume(TokenKind::Newline)?; self.consume(TokenKind::Indent)?;
        let mut cases = Vec::new(); let mut default_block=None;
        while !self.match_kind(&TokenKind::Dedent) {
            if self.match_kind(&TokenKind::Case) { let cv=self.expression()?; self.consume(TokenKind::Colon)?; self.consume(TokenKind::Newline)?; self.consume(TokenKind::Indent)?; let mut body=Vec::new(); while !self.match_kind(&TokenKind::Dedent) { body.push(self.statement()?); } cases.push((cv,body)); }
            else if self.match_kind(&TokenKind::Default) { self.consume(TokenKind::Colon)?; self.consume(TokenKind::Newline)?; self.consume(TokenKind::Indent)?; let mut body=Vec::new(); while !self.match_kind(&TokenKind::Dedent) { body.push(self.statement()?); } default_block=Some(body); }
            else { bail!("Expected 'case' or 'default' in switch block"); }
        }
        Ok(Stmt::Switch { expr, cases, default: default_block })
    }

    fn while_stmt(&mut self) -> Result<Stmt> { let cond=self.expression()?; self.consume(TokenKind::Colon)?; self.consume(TokenKind::Newline)?; self.consume(TokenKind::Indent)?; let mut body=Vec::new(); while !self.match_kind(&TokenKind::Dedent){ body.push(self.statement()?);} Ok(Stmt::While { cond, body }) }
    fn return_stmt(&mut self) -> Result<Stmt> { if self.check(TokenKind::Newline) { self.consume(TokenKind::Newline)?; return Ok(Stmt::Return(None)); } let expr=self.expression()?; self.consume(TokenKind::Newline)?; Ok(Stmt::Return(Some(expr))) }
    fn for_stmt(&mut self) -> Result<Stmt> { let var=self.identifier()?; self.consume(TokenKind::In)?; self.consume(TokenKind::Range)?; self.consume(TokenKind::LParen)?; let start=self.expression()?; self.consume(TokenKind::Comma)?; let end=self.expression()?; let step= if self.match_kind(&TokenKind::Comma){Some(self.expression()?)} else {None}; self.consume(TokenKind::RParen)?; self.consume(TokenKind::Colon)?; self.consume(TokenKind::Newline)?; self.consume(TokenKind::Indent)?; let mut body=Vec::new(); while !self.match_kind(&TokenKind::Dedent){ body.push(self.statement()?);} Ok(Stmt::For { var, start, end, step, body }) }
    fn if_stmt(&mut self) -> Result<Stmt> { let cond=self.expression()?; self.consume(TokenKind::Colon)?; self.consume(TokenKind::Newline)?; self.consume(TokenKind::Indent)?; let mut body=Vec::new(); while !self.match_kind(&TokenKind::Dedent){ body.push(self.statement()?);} let mut elifs=Vec::new(); while self.match_kind(&TokenKind::Elif){ let ec=self.expression()?; self.consume(TokenKind::Colon)?; self.consume(TokenKind::Newline)?; self.consume(TokenKind::Indent)?; let mut ebody=Vec::new(); while !self.match_kind(&TokenKind::Dedent){ ebody.push(self.statement()?);} elifs.push((ec,ebody)); } let else_body= if self.match_kind(&TokenKind::Else){ self.consume(TokenKind::Colon)?; self.consume(TokenKind::Newline)?; self.consume(TokenKind::Indent)?; let mut eb=Vec::new(); while !self.match_kind(&TokenKind::Dedent){ eb.push(self.statement()?);} Some(eb)} else {None}; Ok(Stmt::If { cond, body, elifs, else_body }) }

    // --- expressions ---
    fn expression(&mut self) -> Result<Expr> { self.logic_or() }
    fn logic_or(&mut self) -> Result<Expr> { let mut node=self.logic_and()?; while self.match_kind(&TokenKind::Or){ let rhs=self.logic_and()?; node=Expr::Logic { op:LogicOp::Or, left:Box::new(node), right:Box::new(rhs)}; } Ok(node) }
    fn logic_and(&mut self) -> Result<Expr> { let mut node=self.bit_or()?; while self.match_kind(&TokenKind::And){ let rhs=self.bit_or()?; node=Expr::Logic { op:LogicOp::And, left:Box::new(node), right:Box::new(rhs)};} Ok(node) }
    fn bit_or(&mut self) -> Result<Expr> { let mut node=self.bit_xor()?; while self.match_kind(&TokenKind::Pipe){ let rhs=self.bit_xor()?; node=Expr::Binary { op:BinOp::BitOr, left:Box::new(node), right:Box::new(rhs)};} Ok(node) }
    fn bit_xor(&mut self) -> Result<Expr> { let mut node=self.bit_and()?; while self.match_kind(&TokenKind::Caret){ let rhs=self.bit_and()?; node=Expr::Binary { op:BinOp::BitXor, left:Box::new(node), right:Box::new(rhs)};} Ok(node) }
    fn bit_and(&mut self) -> Result<Expr> { let mut node=self.shift()?; while self.match_kind(&TokenKind::Amp){ let rhs=self.shift()?; node=Expr::Binary { op:BinOp::BitAnd, left:Box::new(node), right:Box::new(rhs)};} Ok(node) }
    fn shift(&mut self) -> Result<Expr> { let mut node=self.comparison()?; while let Some(op)= if self.match_kind(&TokenKind::ShiftLeft){Some(BinOp::Shl)} else if self.match_kind(&TokenKind::ShiftRight){Some(BinOp::Shr)} else {None} { let rhs=self.comparison()?; node=Expr::Binary { op, left:Box::new(node), right:Box::new(rhs)}; } Ok(node) }
    fn comparison(&mut self) -> Result<Expr> { let first=self.term()?; if let Some(op0)=self.match_cmp_op(){ let mut operands=vec![first]; let mut ops=vec![op0]; operands.push(self.term()?); while let Some(nop)=self.match_cmp_op(){ ops.push(nop); operands.push(self.term()?);} if ops.len()==1 { let left=operands.remove(0); let right=operands.remove(0); return Ok(Expr::Compare { op:ops[0], left:Box::new(left), right:Box::new(right)});} let mut chain=None; for i in 0..ops.len(){ let cmp=Expr::Compare { op:ops[i], left:Box::new(operands[i].clone()), right:Box::new(operands[i+1].clone())}; chain=Some(if let Some(acc)=chain { Expr::Logic { op:LogicOp::And, left:Box::new(acc), right:Box::new(cmp)} } else { cmp }); } return Ok(chain.unwrap()); } Ok(first) }
    fn term(&mut self) -> Result<Expr> { let mut node=self.factor()?; while let Some(op)= if self.match_kind(&TokenKind::Plus){Some(BinOp::Add)} else if self.match_kind(&TokenKind::Minus){Some(BinOp::Sub)} else {None} { let rhs=self.factor()?; node=Expr::Binary { op, left:Box::new(node), right:Box::new(rhs)};} Ok(node) }
    fn factor(&mut self) -> Result<Expr> { let mut node=self.unary()?; while let Some(op)= if self.match_kind(&TokenKind::Star){Some(BinOp::Mul)} else if self.match_kind(&TokenKind::Slash){Some(BinOp::Div)} else if self.match_kind(&TokenKind::Percent){Some(BinOp::Mod)} else {None} { let rhs=self.unary()?; node=Expr::Binary { op, left:Box::new(node), right:Box::new(rhs)};} Ok(node) }
    fn unary(&mut self) -> Result<Expr> {
        if self.match_kind(&TokenKind::Not) {
            let inner = self.unary()?;
            return Ok(Expr::Not(Box::new(inner)));
        } else if self.match_kind(&TokenKind::Minus) {
            let rhs = self.unary()?;
            return Ok(Expr::Binary { op: BinOp::Sub, left: Box::new(Expr::Number(0)), right: Box::new(rhs) });
        } else if self.match_kind(&TokenKind::Plus) {
            return self.unary();
        } else if self.match_kind(&TokenKind::Tilde) {
            let inner = self.unary()?;
            return Ok(Expr::BitNot(Box::new(inner)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        if let Some(n) = self.match_number() {
            return Ok(Expr::Number(n));
        } else if let Some(s) = self.match_string() {
            return Ok(Expr::StringLit(s));
        } else if self.match_kind(&TokenKind::True) {
            return Ok(Expr::Number(1));
        } else if self.match_kind(&TokenKind::False) {
            return Ok(Expr::Number(0));
        } else if let Some(first) = self.match_identifier() {
            let mut full = first;
            while self.match_kind(&TokenKind::Dot) {
                if let Some(seg) = self.match_identifier() {
                    full.push('_');
                    full.push_str(&seg);
                } else {
                    return self.err_here("Expected identifier after '.' in qualified name");
                }
            }
            if self.match_kind(&TokenKind::LParen) {
                let mut args = Vec::new();
                if !self.check(TokenKind::RParen) {
                    loop {
                        args.push(self.expression()?);
                        if self.match_kind(&TokenKind::Comma) { continue; }
                        break;
                    }
                }
                self.consume(TokenKind::RParen)?;
                return Ok(Expr::Call { name: full, args });
            }
            return Ok(Expr::Ident(full));
        } else if self.match_kind(&TokenKind::LParen) {
            let e = self.expression()?;
            self.consume(TokenKind::RParen)?;
            return Ok(e);
        }
        self.err_here(&format!("Unexpected token {:?}", self.peek().kind))
    }

    // --- helpers ---
    fn match_ident_case(&mut self, upper:&str) -> bool { if let Some(TokenKind::Identifier(s))=self.peek_kind().cloned(){ if s.eq_ignore_ascii_case(upper){ self.advance(); return true; } } false }
    fn match_cmp_op(&mut self) -> Option<CmpOp> {
        let k=&self.peek().kind; let op=match k { TokenKind::EqEq=>Some(CmpOp::Eq), TokenKind::NotEq=>Some(CmpOp::Ne), TokenKind::Lt=>Some(CmpOp::Lt), TokenKind::Le=>Some(CmpOp::Le), TokenKind::Gt=>Some(CmpOp::Gt), TokenKind::Ge=>Some(CmpOp::Ge), _=>None }; if op.is_some(){ self.pos+=1; } op }
    fn match_number(&mut self) -> Option<i32> { if let TokenKind::Number(n)=self.peek().kind { let v=n; self.pos+=1; Some(v) } else { None } }
    fn match_string(&mut self) -> Option<String> { if let TokenKind::StringLit(s)=&self.peek().kind { let v=s.clone(); self.pos+=1; Some(v) } else { None } }
    fn match_identifier(&mut self) -> Option<String> { if let TokenKind::Identifier(s)=&self.peek().kind { let v=s.clone(); self.pos+=1; Some(v) } else { None } }
    fn try_identifier(&mut self) -> Option<String> { self.match_identifier() }
    fn unread_identifier(&mut self, name:String) { self.pos-=1; if let TokenKind::Identifier(s)=&self.tokens[self.pos].kind { assert_eq!(&name,s); } }
    fn identifier(&mut self) -> Result<String> { if let Some(s)=self.match_identifier(){ Ok(s) } else { self.err_here("Expected identifier") } }
    fn parse_signed_number(&mut self) -> Result<i32> { let neg=self.match_kind(&TokenKind::Minus); if let TokenKind::Number(n)=self.peek().kind { let v=n; self.pos+=1; Ok(if neg { -v } else { v }) } else { self.err_here("Expected number") } }
    fn consume(&mut self, kind: TokenKind) -> Result<()> { if std::mem::discriminant(&self.peek().kind)==std::mem::discriminant(&kind) { self.pos+=1; Ok(()) } else { self.err_here(&format!("Expected {:?} got {:?}", kind, self.peek().kind)) } }
    fn check(&self, kind: TokenKind) -> bool { std::mem::discriminant(&self.peek().kind)==std::mem::discriminant(&kind) }
    fn match_kind(&mut self, kind:&TokenKind) -> bool { if self.check(kind.clone()) { self.pos+=1; true } else { false } }
    fn peek(&self) -> &Token { &self.tokens[self.pos] }
    fn peek_kind(&self) -> Option<&TokenKind> { self.tokens.get(self.pos).map(|t| &t.kind) }
    fn advance(&mut self) { if self.pos < self.tokens.len() { self.pos+=1; } }
    fn err_here<T>(&self, msg:&str) -> Result<T> { let tk=self.peek(); bail!("{}:{}:{}: error: {}", self.filename, tk.line, tk.col, msg) }
}
