use std::env;
mod lmc;
mod io;
mod assembler;

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() < 2 {
    println!("");
    println!("Please give the name of your ASM file as a command line argument:");
    println!("lmc <filename>");
    println!("");
    return;
  }
  let mut lmc : lmc::LMC = lmc::LMC::new();
  let src : String = io::read_whole_file(&args[1]).ok().unwrap();
  assembler::assemble(&src, &mut lmc);
  loop {
    if !lmc.step() {
      break;
    }
  }
}
