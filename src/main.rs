mod lmc;
mod io;
mod assembler;

/// Program loads the number 3, adds it to some user input, then outputs the result (6).
/// ASM program as follows:
/// 00 IN
/// 01 ADD three
/// 02 OUT
/// 03 DAT three 003
fn load_test_program(lmc : &mut lmc::LMC) {
  lmc.mem[0] = 901;
  lmc.mem[1] = 103;
  lmc.mem[2] = 902;
  lmc.mem[3] = 003;
}

fn main() {
  let mut lmc : lmc::LMC = lmc::LMC::new();
  let src : String = io::read_whole_file("test.txt").ok().unwrap();
  assembler::assemble(&src, &mut lmc);
  load_test_program(&mut lmc);
  loop {
    if !lmc.step() {
      break;
    }
  }
}
