use std::io::{self, Read};

pub fn read_whole_file(filename: &str) -> Result<String, io::Error> {
  use std::fs::File;
  let mut f = try!(File::open(filename));
  let mut buffer = String::new();
  try!(f.read_to_string(&mut buffer));
  return Ok(buffer);
}

pub fn read() -> u16 {
  println!("Please input a number: ");
  loop {
    use std::io;
    let mut buffer = String::new();

    let stdio_res = io::stdin().read_line(&mut buffer);
    if !(stdio_res.is_ok() && stdio_res.ok().is_some()) {
      println!("\nError reading input, try again.");
      continue;
    }
    let res = buffer.trim().parse::<u16>();
    if !res.is_ok() {
      println!("{}", res.err().unwrap());
      continue;
    }
    let input = res.ok();
    if input == None {
      println!("HEY");
      continue;
    }
    if input != None && input.is_some() {
      return input.unwrap();
    }
    else {
      println!("\nInvalid input, try again.");
    }
  }
}

pub fn write(val : u16) {
  println!("Output: {}", val);
}
