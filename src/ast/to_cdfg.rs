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
    let mut regs: Vec<cdfg::RegData> = 
      self.args.iter().map(|x| cdfg::RegData::new(x.0.clone(), x.1.clone(), 0i64)).collect();
    let mut wires: Vec<cdfg::WireData> = Vec::new();
    let mut graph: HashMap<i32, cdfg::CDFGNode> = HashMap::new();
    let mut state = 1;
    self.content.to_cdfg(&mut regs, &mut wires, &mut state, &mut graph);
    let args = self.args.into_iter().map(|x| x.1).collect();
    cdfg::Function { args, regs, wires, graph }
  }
}

impl ast::Stat {
  fn to_cdfg(self, regs: &mut Vec<cdfg::RegData>, wires: &mut Vec<cdfg::WireData>, state: &mut i32, graph: &mut HashMap<i32, cdfg::CDFGNode>) {
    match self {
      Self::Void => (),
      Self::Expression(e) => {
        let node = cdfg::CDFGNode::new(*state, Self::Expression(e));
        graph.insert(*state, node);
        *state += 1;
      },
      Self::Decl(t, n, o) => {
        let mut r = cdfg::RegData::new(t.clone(), n.clone(), 0i64);
        match o {
          None => (),
          Some(ast::Expr::Int(i)) => r.ini = i,
          Some(e) => {
            let node = cdfg::CDFGNode::new(*state, Self::Expression(Box::new(ast::Expr::Assign(n, Box::new(e)))));
            graph.insert(*state, node);
            *state += 1;
          }
        }
        regs.push(r);
      },
      Self::Return(e) => {
        let mut node = cdfg::CDFGNode::new(*state, Self::Expression(Box::new(ast::Expr::Assign("ap_return".to_string(), e))));
        node.set_next(cdfg::Next::Jump(0));
        graph.insert(*state, node);
        *state += 1;
      }
      Self::IfElse(e, s, t) => {
        let mut node = cdfg::CDFGNode::new(*state, Self::Void);
        *state += 1;
        let sstate = *state;
        s.to_cdfg(regs, wires, state, graph);
        let tstate = *state;
        t.to_cdfg(regs, wires, state, graph);

        node.set_next(cdfg::Next::Branch(e, sstate, tstate));
        graph.insert(node.idx, node);
        graph.get_mut(&(tstate - 1)).unwrap().set_next(cdfg::Next::Jump(*state));
      }
      _ => (),
    }
  }
}