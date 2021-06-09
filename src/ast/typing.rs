use crate::ast::*;
use std::collections::HashMap;

impl Program {
  pub fn typing(self) -> Self {
    Self { functions: self.functions.into_iter().map(|x| x.typing()).collect() }
  }
}

impl Function {
  fn typing(self) -> Self {
    let mut env: HashMap<String, Type> = HashMap::new();
    for (t, s) in self.args.clone().into_iter() {
      env.insert(s, t);
    }
    Self {
      ret_type: self.ret_type, 
      name: self.name, 
      args: self.args, 
      content: self.content.typing(&mut env)
    }
  }
}

impl Stat {
  fn typing(self, env: &mut HashMap<String, Type>) -> Self {
    match self {
      Self::Void => Self::Void,
      Self::Expression(e) => Self::Expression(Box::new(e.typing(env))),
      Self::Decl(t, s, o) => {
        let o = o.map(|x| x.typing(env));
        env.insert(s.clone(), t.clone());
        Self::Decl(t, s, o)
      },
      Self::Compound(v) => {
        let mut w: Vec<Stat> =  Vec::new();
        for s in v.into_iter() {
          w.push(s.typing(env));
        }
        Self::Compound(w)
      },
      Self::Return(e) => Self::Return(Box::new(e.typing(env))),
      Self::IfElse(e, s, t) => {
        let e = e.typing(env);
        let mut nenv = env.clone();
        let s = s.typing(&mut nenv);
        let mut nenv = env.clone();
        let t = t.typing(&mut nenv);
        Self::IfElse(Box::new(e), Box::new(s), Box::new(t))
      },
      Self::While(e, s) => {
        let e = e.typing(env);
        let mut nenv = env.clone();
        let s = s.typing(&mut nenv);
        Self::While(Box::new(e), Box::new(s))
      },
    }
  }
}

impl Expr {
  fn typing(self, env: &mut HashMap<String, Type>) -> Self {
    match self {
      Self::Id(_, s) => match env.get(&s) {
        None => panic!("variable \"{}\" is not defined", s),
        Some(t) => Self::Id(t.clone(), s),
      },
      Self::Op1(op, e) => Self::Op1(op, Box::new(e.typing(env))),
      Self::Op2(op, e1, e2) => Self::Op2(op, Box::new(e1.typing(env)), Box::new(e2.typing(env))),
      Self::Assign(s, e) => Self::Assign(s, Box::new(e.typing(env))),
      x => x,
    }
  }
}