use io;

pub struct LMC {
  pc  : u8,
  mar : u8,
  mdr : u16,
  acc : u16,
  neg : bool,
  op_code : u8,
  operand : u8,
  pub mem : [u16; 100],
}

impl LMC {
  pub fn new() -> LMC {
    LMC {
      pc  : 0,
      mar : 0,
      mdr : 0,
      acc : 0,
      neg: false,
      op_code : 0,
      operand : 0,
      mem: [0; 100],
    }
  }

  /// Stores the value in the MDR at the address in the MAR.
  pub fn store(&mut self) {
    self.mem[self.mar as usize] = self.mdr;
  }

  /// Fetches memory at the location in the MAR and places data into
  /// the MDR.
  pub fn fetch(&mut self) {
    self.mdr = self.mem[self.mar as usize];
  }

  /// Decode the number in the MDR, set op_code and operand.
  pub fn decode(&mut self) {
    self.op_code = (self.mdr / 100) as u8;
    self.operand = (self.mdr % 100) as u8;
  }

  /// Execute the operation in op_code using operand.
  /// Returns true by default, or false if the program should halt.
  pub fn execute(&mut self) -> bool {
    match self.op_code {
      // HLT
      0 => {
        println!("Halted.");
        return false;
      }

      // ADD
      1 => {
        self.mar = self.operand;
        self.fetch();
        self.acc += self.mdr;
      }

      // SUB
      2 => {
        self.mar = self.operand;
        self.fetch();
        self.acc += self.mdr;
      }

      // STO
      3 => {
        self.mar = self.operand;
        self.mdr = self.acc;
        self.store();
      }

      // LOAD
      5 => {
        self.mar = self.operand;
        self.fetch();
        self.acc = self.mdr;
      }

      // BRANCH
      6 => {
        self.pc = self.operand;
      }

      // BRANCH ON ZERO
      7 => {
        if self.acc == 0 {
          self.pc = self.operand;
        }
      }

      // BRANCH ON POSITIVE
      8 => {
        if self.neg {
          self.pc = self.operand;
        }
      }

      // INPUT / OUTPUT
      9 => {
        if self.operand == 1 {
          self.acc = io::read();
        }
        else if self.operand == 2 {
          io::write(self.acc);
        }
      }

      _ => {}
    }
    return true;
  }

  /// Steps the LMC. Returns true by default, or false if the LMC should halt.
  pub fn step(&mut self) -> bool {
    self.mar = self.pc;
    self.fetch();
    self.decode();
    // Increment PC before execute, if program branches we don't want
    // to increment after.
    self.pc += 1; 
    return self.execute();
  }
}
