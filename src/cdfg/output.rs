use crate::cdfg::*;
use std::fmt;

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for fun in self.functions.iter() {
      write!(f, "{}", fun)?;
    }
    Ok(())
  }
}

impl fmt::Display for Function {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "module {} (\n", self.name)?;
    write!(f, "\tinput ap_clk,\n\tinput ap_rst,\n\tinput ap_start,\n\toutput reg ap_done,\n\toutput ap_idle,\n\toutput reg ap_ready,\n")?;
    for arg in self.args.iter() {
      write!(f, "\tinput [31:0] ap_{},\n", arg)?;
    }
    write!(f, "\toutput reg [31:0] ap_return\n);\n\n")?;
    for r in self.regs.iter() {
      write!(f, "reg [31:0] {};\n", r.name)?;
    }
    // fsmのビット幅を求める
    let mut fsm_len = 1u8;
    while self.graph.contains_key(&(1 << fsm_len)){
      fsm_len += 1;
    }
    write!(f, "\nreg [{}:0] ap_fsm\n", fsm_len - 1)?;
    write!(f, "assign ap_idle = ap_fsm == 0;\n\n")?;
    Ok(())
  }
}