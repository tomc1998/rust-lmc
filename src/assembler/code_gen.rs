use assembler::parser as parser;
use lmc;

pub fn gen(operations: &Vec<parser::Operation>, lmc: &mut lmc::LMC) {
  let mut val: u16;
  let mut o : &parser::Operation;
  for ii in 0..100 {
    if ii >= operations.len() {
      lmc.mem[ii] = 0;
      continue;
    }
    o = &operations[ii];
    val = (o.op_code as u16)*100 + o.operand as u16;
    if o.op_code == 10 {
      val = o.operand;
    }
    lmc.mem[ii] = val;
  }
}

