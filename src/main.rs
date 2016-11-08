mod lmc;
mod io;
mod assembler;

fn main() {
  let mut lmc : lmc::LMC = lmc::LMC::new();
  let src : String = io::read_whole_file("test.txt").ok().unwrap();
  assembler::assemble(&src, &mut lmc);
  loop {
    if !lmc.step() {
      break;
    }
  }
}
