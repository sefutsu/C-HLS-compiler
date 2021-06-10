use crate::ast;
use crate::cdfg::*;
use std::collections::{HashMap, HashSet};

mod id {
  static mut COUNTER: u32 = 0;
  pub fn generate(x: &str) -> String {
    unsafe {
      COUNTER += 1;
      format!("ap_{}_{}", x, COUNTER)
    }
  }
}

mod alpha {
  use crate::ast::*;
  impl Expr {
    fn alpha(self, x: &str) -> Self {
      match self {
        Self::Int(i) => Self::Int(i),
        Self::Id(t, s) => if s == *x {
          Self::Id(t, x.to_string())
        } else {
          Self::Id(t, s)
        },
        Self::Op1(op, e) => Self::Op1(op, Box::new(e.alpha(x))),
        Self::Op2(op, e1, e2) => Self::Op2(op, Box::new(e1.alpha(x)), Box::new(e2.alpha(x))),
        Self::Assign(s, e) => Self::Assign(s, Box::new(e.alpha(x))),
      }
    }
  }
  impl Stat {
    fn alpha(self, x: &str) -> Self {
      match self {
        Self::Expression(e) => Self::Expression(Box::new(e.alpha(x))),
        Self::Compound(v) => Self::Compound(v.into_iter().map(|s| s.alpha(x)).collect()),
        Self::Return(e) => Self::Return(Box::new(e.alpha(x))),
        s => s,
      }
    }
  }
}

fn get_graph_inputs(graph: &HashMap<i32, CDFGNode>) -> HashMap<i32, Option<HashSet<i32>>> {
  let mut res: HashMap<i32, Option<HashSet<i32>>> = HashMap::new();
  for idx in graph.keys() {
    res.insert(*idx, Some(HashSet::new()));
  }
  // 1は0から来る特別ノード
  res.insert(1, None);
  for node in graph.values() {
    match node.next {
      Next::Jump(i) => {
        let oo = res.get_mut(&i);
        match oo {
          None => (),
          Some(o) => {
            match o {
              None => (),
              Some(hs) => { hs.insert(node.idx); () }
            }
          }
        }
      }
      Next::Branch(_, i, j) => {
        res.insert(i, None);
        res.insert(j, None);
      },
    }
  }
  res
}

impl Program {
  pub fn optimize(self) -> Self {
    Self { functions: self.functions.into_iter().map(|x| x.optimize()).collect() }
  }
}
impl Function {
  fn optimize(mut self) -> Self {
    let graph_inputs = get_graph_inputs(&self.graph);
    let mut deletable: Vec<(i32, i32)> = Vec::new();
    for (i, o) in graph_inputs.into_iter() {
      if let Some(v) = o {
        match v.len() {
          0 => { self.graph.remove(&i); () }
          1 => { deletable.push((v.into_iter().next().unwrap(), i))}
          _ => (),
        }
      }
    }
    while !deletable.is_empty() {
      // iとjを統合 (i < j)
      let (i, j) = deletable.pop().unwrap();
      let mut ni = self.graph.remove(&i).unwrap();
      let mut nj = self.graph.remove(&j).unwrap();
      match ni.src {
        ast::Stat::Expression(asg) => {
          match *asg {
            ast::Expr::Assign(s, e) => {
              let gwire = id::generate(&s);
            },
            _ => unreachable!()
          }
        },
        ast::Stat::Compound(v) => {
          
        }
        _ => unreachable!()
      }
    }
    Self { name: self.name, args: self.args, regs:self.regs, wires: self.wires, graph: self.graph }
  }
}

