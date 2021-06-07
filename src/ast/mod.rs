#[derive(Debug, Clone)]
pub enum Type {
  I32, U32,
}

#[derive(Debug, Clone)]
pub enum Op1 {
  Neg, Lognot,
}

#[derive(Debug, Clone)]
pub enum Op2 {
  Add, Sub, Mul, Div,
  And, Or, Xor, Logand, Logor,
  Eq, Ne, Lt, Le,
  Lshift, Rshift,
}

#[derive(Debug, Clone)]
pub struct FunType {
  pub ret: Type,
  pub args: Vec<Type>,
}

#[derive(Debug, Clone)]
pub enum Expr {
  Int(i64),
  Id(String),
  Op1(Op1, Box<Expr>),
  Op2(Op2, Box<Expr>, Box<Expr>),
  Assign(String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stat {
  Void,
  Expression(Box<Expr>),
  Decl(Type, String, Option<Expr>),
  Compound(Vec<Stat>),
  Return(Box<Expr>),
  IfElse(Box<Expr>, Box<Stat>, Box<Stat>),
  While(Box<Expr>, Box<Stat>),
}

#[derive(Debug, Clone)]
pub struct Function {
  pub ret_type: Type,
  pub name: String,
  pub args: Vec<(Type, String)>,
  pub content: Stat,
}

#[derive(Debug, Clone)]
pub struct Program {
  pub functions: Vec<Function>,
}

mod to_cdfg;
