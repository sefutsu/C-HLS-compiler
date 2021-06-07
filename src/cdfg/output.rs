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

    write!(f, "always @(posedge ap_clk) begin\n")?;
    write!(f, "\tif(ap_rst) begin\n")?;
    for r in self.regs.iter() {
      write!(f, "\t\t{} <= 32'h{}\n", r.name, r.ini)?;
    }
    write!(f, "\t\tap_done <= 0;\n\t\tap_ready <= 1;\n\t\tap_return <= 0;\n\t\tap_fsm <= 0;\n")?;
    write!(f, "\tend else begin\n")?;
    write!(f, "\t\tcase(ap_fsm)\n")?;
    // fsm = 0を特別に定義する
    write!(f, "\t\t\t0: begin\n\t\t\t\tif(ap_start) begin\n")?;
    for arg in self.args.iter() {
      write!(f, "\t\t\t\t\t{0} <= ap_{0};\n", arg)?;
    }
    write!(f, "\t\t\t\t\tap_ready <= 0;\n\t\t\t\t\tap_done <= 0;\n\t\t\t\t\tap_fsm <= 1;\n")?;
    // 各状態の定義
    let mut vgraph: Vec<(i32, CDFGNode)> = self.graph.clone().into_iter().collect();
    vgraph.sort();
    let vnode: Vec<CDFGNode> = vgraph.into_iter().map(|x| x.1).collect();
    for node in vnode.iter() {
      write!(f, "{}", node)?;
    }
    // default case
    write!(f, "\t\t\tdefault: begin\n\t\t\t\tap_fsm <= 0;\n\t\t\tend\n")?;
    write!(f, "\t\tendcase\n\tend\nend\n")?;
    write!(f, "endmodule")
  }
}

impl fmt::Display for CDFGNode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}\n", self.idx)
  }
}