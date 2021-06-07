use crate::ast;
use crate::cdfg;
use std::collections::HashMap;

impl ast::Program {
  pub fn to_cdfg(self) -> cdfg::Program {
    cdfg::Program { functions: self.functions.into_iter().map(|f| f.to_cdfg()).collect() }
  }
}

impl ast::Function {
  fn to_cdfg(self) -> cdfg::Function {
    let regs: Vec<cdfg::RegData> = 
      self.args.iter().map(|x| cdfg::RegData::new(x.0.clone(), x.1.clone(), 0i64)).collect();
    let wires: Vec<cdfg::WireData> = Vec::new();
    let graph: HashMap<i32, cdfg::CDFGNode> = HashMap::new();
    let args = self.args.into_iter().map(|x| x.1).collect();
    let mut fun = cdfg::Function { args, regs, wires, graph };
    let mut state = 1;
    self.content.to_cdfg(&mut fun, &mut state);
    fun
  }
}

impl ast::Stat {
  fn to_cdfg(self, fun: &mut cdfg::Function, state: &mut i32) {
    match self {
      Self::Void => (),
      Self::Expression(e) => {
        let node = cdfg::CDFGNode::new(*state, Self::Expression(e));
        fun.add_node(node);
        *state += 1;
      },
      Self::Decl(t, n, o) => {
        let mut r = cdfg::RegData::new(t.clone(), n.clone(), 0i64);
        match o {
          None => (),
          Some(ast::Expr::Int(i)) => r.ini = i,
          Some(e) => {
            let node = cdfg::CDFGNode::new(*state, Self::Expression(Box::new(ast::Expr::Assign(n, Box::new(e)))));
            fun.add_node(node);
            *state += 1;
          }
        }
        fun.regs.push(r);
      },
      Self::Return(e) => {
        let mut node = cdfg::CDFGNode::new(*state, ast::Stat::Return(e));
        node.set_next(cdfg::Next::Jump(0));
        fun.add_node(node);
        *state += 1;
      }
      Self::IfElse(e, s, t) => {
        let mut node = cdfg::CDFGNode::new(*state, Self::Void);
        *state += 1;
        s.to_cdfg(fun, state);
        let mut s_end = cdfg::CDFGNode::new(*state, Self::Void);
        *state += 1;
        t.to_cdfg(fun, state);

        node.set_next(cdfg::Next::Branch(e, node.idx + 1, s_end.idx + 1));
        s_end.set_next(cdfg::Next::Jump(*state));
        fun.add_node(node);
        fun.add_node(s_end);
      }
      Self::While(e, s) => {
        let mut node = cdfg::CDFGNode::new(*state, Self::Void);
        *state += 1;
        s.to_cdfg(fun, state);
        let mut while_end = cdfg::CDFGNode::new(*state, Self::Void);
        *state += 1;
        while_end.set_next(cdfg::Next::Jump(node.idx));
        node.set_next(cdfg::Next::Branch(e, node.idx + 1, while_end.idx + 1));
        fun.add_node(node);
        fun.add_node(while_end);
      }
      Self::Compound(v) => {
        for s in v.into_iter() {
          s.to_cdfg(fun, state);
        }
      }
    }
  }
}