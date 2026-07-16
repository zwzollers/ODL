#[derive(Debug)]
pub struct AST {
    pub items: Vec<TopStructre>,
}

#[derive(Debug)]
pub enum TopStructre {
    Module(Box<Module>),
    Struct(Box<Struct>),
    Enum(Box<Enum>),
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub port_list: Vec<Box<Port>>,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Port {
    pub name: String,
    pub dir: PortDir,
    pub ty: String,
}

#[derive(Debug)]
pub enum PortDir {
    In,
    Out,
}

#[derive(Debug)]
pub struct ModuleInst {
    pub module: String,
    pub port_inst: Vec<Box<PortInst>>,
}

#[derive(Debug)]
pub struct PortInst {
    pub ident: String,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub enum Statement {
    Declaration(Box<Declaration>),
    Assignment(Box<Assignment>),
    Scope(Box<Scope>),
    If(Box<If>),
    For(Box<For>),
    While(Box<While>),
    ModuleInst(Box<ModuleInst>),
    Error,
}

#[derive(Debug)]
pub struct Scope {
    pub statements: Vec<Statement>,
    pub exports: Vec<Box<Export>>,
}

#[derive(Debug)]
pub struct Assignment {
    pub name: String,
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Declaration {
    pub assignment: Box<Assignment>,
}

#[derive(Debug)]
pub struct Export {
    pub ident: String,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub enum Term {
    Ident(String),
    Num(f64),
    String(String),
    List(Vec<Box<Expr>>),
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub items: Vec<Box<StructItem>>
}

#[derive(Debug)]
pub struct StructItem {
    pub name: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub items: Vec<String>
}

#[derive(Debug)]
pub struct If {
    pub condition_list: Vec<Box<Expr>>,
    pub scope_list: Vec<Box<Scope>>,
}

#[derive(Debug)]
pub struct For {
    pub iter_idx: String,
    pub iter: Box<Expr>,
    pub scope: Box<Scope>,
}

#[derive(Debug)]
pub struct While {
    pub cond: Box<Expr>,
    pub scope: Box<Scope>,
}

#[derive(Debug)]
pub enum Expr {
    Term(Term),
    Paren(Box<Expr>),

    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),

    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),

    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),

    LogicAnd(Box<Expr>, Box<Expr>),

    LogicOr(Box<Expr>, Box<Expr>),
}
