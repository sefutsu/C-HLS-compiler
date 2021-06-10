#[macro_use]
extern crate lalrpop_util;
use std::io::Read;

lalrpop_mod!(pub c);

mod ast;
mod cdfg;

fn main() {
    let parser = c::ProgramParser::new();
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    match parser.parse(&buf) {
      Ok(s) => {
        eprintln!("Parsed: {:#?}", s);
        let typed = s.typing();
        eprintln!("Typed: {:#?}", typed);
        let cdfg = typed.to_cdfg();
        eprintln!("CDFG: {:#?}", cdfg);
        let optimized = cdfg.optimize();
        eprintln!("Optimized: {:#?}", optimized);
        println!("{}", optimized);
      }
      Err(e) => eprintln!("Parse error: {:#?}", e),
    }
  }