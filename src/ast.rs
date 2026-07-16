#[derive(Debug, Clone, PartialEq)]
pub enum TypeRef {
    Void, 
    Integer,    // i64
    Float,      // f64
    Bool,       // bool
    Char,       // char
    Byte,       // u8
    String, 
    Untrusted, 
    Result(Box<TypeRef>), // e.g., i64!
    Array(Box<TypeRef>),
    Tuple(Vec<TypeRef>),
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub param_type: TypeRef,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Neq, Gt, Lt, Gte, Lte,
    And, Or,
    BitAnd, BitOr, BitXor, Shl, Shr,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,  // !
    Neg,  // - (tekli eksi)
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64), Float(f64), Str(String), Bool(bool), Char(char),
}

/// String interpolation parçası: "text ${expr} text"
#[derive(Debug, Clone)]
pub enum InterpolPart {
    Lit(String),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Identifier(String),
    Wildcard,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct InfraConfig {
    pub timeout_ms: u64,
}

#[derive(Debug, Clone)]
pub struct InfraCall {
    pub service: String,
    pub method: String,
    pub args: Vec<Expr>,
    pub config: InfraConfig,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Literal(Literal),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    /// Call(name, args, awaited) — awaited = `call` keyword kullanıldı mı
    Call(String, Vec<Expr>, bool),
    Spawn(Box<Expr>),
    Await(Box<Expr>),
    InlineRust(String),
    Infra(InfraCall),
    JsonField(Box<Expr>, String),
    
    ArrayLiteral(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    
    TupleLiteral(Vec<Expr>),
    TupleIndex(Box<Expr>, usize),
    
    Interpolation(Vec<InterpolPart>),
    
    MethodCall(Box<Expr>, String, Vec<Expr>),
    StructLiteral(String, Vec<(String, Expr)>),
    FieldAccess(Box<Expr>, String),
    
    Try(Box<Expr>), // expr?
    Catch(Box<Expr>, Box<Expr>), // expr catch default_expr
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub name: String,
    pub value: Expr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Assign,      // =
    AddAssign,   // +=
    SubAssign,   // -=
    MulAssign,   // *=
    DivAssign,   // /=
    ModAssign,   // %=
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStmt),
    Const { name: String, value: Expr },
    Assign { name: String, op: AssignOp, value: Expr },
    IndexAssign { name: String, index: Expr, op: AssignOp, value: Expr },
    Match { expr: Expr, arms: Vec<MatchArm> },
    If { condition: Expr, then_block: Block, else_block: Option<Block> },
    While { condition: Expr, body: Block },
    For { var: String, start: Expr, end: Expr, step: Option<Expr>, body: Block },
    ScopeBlock { name: String, body: Block },
    ValidateBlock { target: String, schema: String, on_fail: Box<Block>, success_scope: Box<Block> },
    ExprStmt(Expr), 
    Return(Option<Expr>),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Purity {
    Deterministic, Nondeterministic,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub purity: Purity,
    pub params: Vec<Param>,
    pub return_type: TypeRef,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Param>,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Function(FunctionDef),
    Struct(StructDef),
    Import(Vec<String>),
    Module(String, Vec<TopLevel>),
}