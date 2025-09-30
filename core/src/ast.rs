#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
	pub items: Vec<Item>,
	pub meta: ModuleMeta,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModuleMeta {
	pub title_override: Option<String>,
	pub metas: std::collections::HashMap<String,String>,
	pub music_override: Option<String>,
	pub copyright_override: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item { 
    Function(Function), 
    Const { name: String, value: Expr }, 
    GlobalLet { name: String, value: Expr }, 
    VectorList { name: String, entries: Vec<VlEntry> },
    ExprStatement(Expr),  // Para permitir expresiones ejecutables en top-level
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VlEntry {
	Intensity(i32),
	Origin,
	Move(i32,i32),
	Rect(i32,i32,i32,i32),
	Polygon(Vec<(i32,i32)>),
	Circle { cx:i32, cy:i32, r:i32, segs:i32 },
	Arc { cx:i32, cy:i32, r:i32, start_deg:i32, sweep_deg:i32, segs:i32 },
	Spiral { cx:i32, cy:i32, r_start:i32, r_end:i32, turns:i32, segs:i32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function { pub name: String, #[allow(dead_code)] pub params: Vec<String>, pub body: Vec<Stmt> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
	Assign { target: AssignTarget, value: Expr },
	Let { name: String, value: Expr },
	For { var: String, start: Expr, end: Expr, step: Option<Expr>, body: Vec<Stmt> },
	While { cond: Expr, body: Vec<Stmt> },
	Break,
	Continue,
	Expr(Expr),
	If { cond: Expr, body: Vec<Stmt>, elifs: Vec<(Expr, Vec<Stmt>)>, else_body: Option<Vec<Stmt>> },
	Switch { expr: Expr, cases: Vec<(Expr, Vec<Stmt>)>, default: Option<Vec<Stmt>> },
	Return(Option<Expr>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentInfo { pub name: String, pub line: usize, pub col: usize }

// Nuevo: información de asignación con span para el identificador del LHS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignTarget { pub name: String, pub line: usize, pub col: usize }

// Información de llamadas con span del identificador (primer segmento calificado).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallInfo { pub name: String, pub line: usize, pub col: usize, pub args: Vec<Expr> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
	Number(i32),
	StringLit(String),
	Ident(IdentInfo),
	Call(CallInfo),
	Binary { op: BinOp, left: Box<Expr>, right: Box<Expr> },
	Compare { op: CmpOp, left: Box<Expr>, right: Box<Expr> },
	Logic { op: LogicOp, left: Box<Expr>, right: Box<Expr> },
	Not(Box<Expr>),
	BitNot(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp { Add, Sub, Mul, Div, Mod, Shl, Shr, BitAnd, BitOr, BitXor }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp { Eq, Ne, Lt, Le, Gt, Ge }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOp { And, Or }
