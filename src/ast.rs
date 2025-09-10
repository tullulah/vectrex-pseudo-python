#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module { pub items: Vec<Item> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item { Function(Function) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function { pub name: String, #[allow(dead_code)] pub params: Vec<String>, pub body: Vec<Stmt> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
	Assign { target: String, value: Expr },
	Let { name: String, value: Expr },
	For { var: String, start: Expr, end: Expr, step: Option<Expr>, body: Vec<Stmt> },
	While { cond: Expr, body: Vec<Stmt> },
	Break,
	Continue,
	Expr(Expr),
	If { cond: Expr, body: Vec<Stmt>, elifs: Vec<(Expr, Vec<Stmt>)>, else_body: Option<Vec<Stmt>> },
	Return(Option<Expr>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr { Number(i32), Ident(String), Call { name: String, args: Vec<Expr> }, Binary { op: BinOp, left: Box<Expr>, right: Box<Expr> }, Compare { op: CmpOp, left: Box<Expr>, right: Box<Expr> }, Logic { op: LogicOp, left: Box<Expr>, right: Box<Expr> }, Not(Box<Expr>), BitNot(Box<Expr>) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp { Add, Sub, Mul, Div, Mod, Shl, Shr, BitAnd, BitOr, BitXor }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp { Eq, Ne, Lt, Le, Gt, Ge }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOp { And, Or }
