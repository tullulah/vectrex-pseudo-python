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
pub struct Function { 
	pub name: String, 
	pub line: usize,  // Starting line number of function definition
	#[allow(dead_code)] pub params: Vec<String>, 
	pub body: Vec<Stmt> 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
	Assign { target: AssignTarget, value: Expr, source_line: usize },
	Let { name: String, value: Expr, source_line: usize },
	For { var: String, start: Expr, end: Expr, step: Option<Expr>, body: Vec<Stmt>, source_line: usize },
	While { cond: Expr, body: Vec<Stmt>, source_line: usize },
	Break { source_line: usize },
	Continue { source_line: usize },
	Expr(Expr, usize), // (expression, line)
	If { cond: Expr, body: Vec<Stmt>, elifs: Vec<(Expr, Vec<Stmt>)>, else_body: Option<Vec<Stmt>>, source_line: usize },
	Switch { expr: Expr, cases: Vec<(Expr, Vec<Stmt>)>, default: Option<Vec<Stmt>>, source_line: usize },
	Return(Option<Expr>, usize), // (value, line)
	// Operadores de asignación compuesta: var += expr
	CompoundAssign { target: AssignTarget, op: BinOp, value: Expr, source_line: usize },
}

impl Stmt {
	/// Get the source line number for any statement
	pub fn source_line(&self) -> usize {
		match self {
			Stmt::Assign { source_line, .. } => *source_line,
			Stmt::Let { source_line, .. } => *source_line,
			Stmt::For { source_line, .. } => *source_line,
			Stmt::While { source_line, .. } => *source_line,
			Stmt::Break { source_line } => *source_line,
			Stmt::Continue { source_line } => *source_line,
			Stmt::Expr(_, source_line) => *source_line,
			Stmt::If { source_line, .. } => *source_line,
			Stmt::Switch { source_line, .. } => *source_line,
			Stmt::Return(_, source_line) => *source_line,
			Stmt::CompoundAssign { source_line, .. } => *source_line,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentInfo { pub name: String, pub source_line: usize, pub col: usize }

// Nuevo: información de asignación con span para el identificador del LHS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignTarget { pub name: String, pub source_line: usize, pub col: usize }

// Información de llamadas con span del identificador (primer segmento calificado).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallInfo { pub name: String, pub source_line: usize, pub col: usize, pub args: Vec<Expr> }

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
pub enum BinOp { Add, Sub, Mul, Div, FloorDiv, Mod, Shl, Shr, BitAnd, BitOr, BitXor }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp { Eq, Ne, Lt, Le, Gt, Ge }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOp { And, Or }
