use crate::ast;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Next {
  Jump(i32),
  Branch(Box<ast::Expr>, i32, i32),
}

#[derive(Debug, Clone)]
pub struct CDFGNode {
  pub idx: i32,
  pub src: ast::Stat,
  pub next: Next,
}

impl CDFGNode {
  pub fn new(idx: i32, src: ast::Stat) -> Self {
    Self {idx, src, next: Next::Jump(idx + 1)}
  }
  pub fn set_next(&mut self, n: Next) {
    self.next = n;
  }
}

#[derive(Debug, Clone)]
pub struct RegData {
  pub t: ast::Type,
  pub name: String,
  pub ini: i64,
}

impl RegData {
  pub fn new(t: ast::Type, name: String, ini: i64) -> Self {
    Self {t, name, ini}
  }
}

#[derive(Debug, Clone)]
pub struct WireData {
  pub t: ast::Type,
  pub name: String,
  pub value: ast::Expr,
}

#[derive(Debug, Clone)]
pub struct Function {
  pub name: String,
  pub args: Vec<String>,
  pub regs: Vec<RegData>,
  pub wires: Vec<WireData>,
  pub graph: HashMap<i32, CDFGNode>,
}

impl Function {
  pub fn add_node(&mut self, node: CDFGNode) {
    self.graph.insert(node.idx, node);
  }
}

#[derive(Debug, Clone)]
pub struct Program {
  pub functions: Vec<Function>,
}

mod output;
