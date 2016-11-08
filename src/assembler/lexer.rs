/// Struct represents a line of assembly split into parts
pub struct SrcLine<'a> {
  pub op: &'a str,
  pub operand: Option<&'a str>,
  pub label: Option<&'a str>,
  pub lnum: u32,
}

pub struct TokenList<'a> {
  pub tokens: Vec<SrcLine<'a>>,
}

/// List of valid op codes
const OP_CODES: [&'static str; 10] = ["ADD", "SUB", "LDA", "STO", "BR", "BRZ", "BRP", "IN", "OUT", "HLT"];

/// Function checks if a string is a valid opcode
fn check_if_op(l: &str) -> bool {
  for oc in OP_CODES.iter() {
    if *oc == l {
      return true;
    }
  }
  return false;
}

/// Lex a single line of ASM.
#[inline(always)]
fn lex_line(line: &str, lnum: u32) -> Result<SrcLine, &str> {
  let lexemes: Vec<&str> = line.split_whitespace().collect();
  let mut src_line : SrcLine; 
  if lexemes.len() == 0 {
    return Err("Assembler error, parsing empty line.");
  }
  if lexemes.len() > 3 {
    return Err("Unexpected symbols before EOL.");
  }

  src_line = SrcLine{op: "", operand: None, label: None, lnum: lnum};
  if check_if_op(lexemes[0]) { // Standard op, no label
    src_line.op = lexemes[0];
    if lexemes.len() == 2 {
      src_line.operand = Some(lexemes[1]);
    }
  }
  else { // Either a DAT op or label first
    if lexemes.len() == 1 {
      return Err("Premature EOL.");
    }
    else if lexemes[0] == "DAT" { // DAT
      src_line.op = lexemes[0];
      src_line.label = Some(lexemes[1]);
    }
    else { // Label first then op
      if !check_if_op(lexemes[1]) {
        return Err("Bad OP code");
      }
      src_line.label = Some(lexemes[0]);
      src_line.op = lexemes[1];
    }
    // Operand will always be index 2 if DAT or label
    if lexemes.len() == 3 {
      src_line.operand = Some(lexemes[2]);
    }
  }
  return Ok(src_line);
}

/// Parses a source file, returns a list of tokens
pub fn lex(source: &str) -> Result<TokenList, String> {
  let mut token_list = TokenList{tokens: Vec::new()};
  let mut line_lex_res : Result<SrcLine, &str>;
  let mut line_num = 0u32;

  for line in source.lines() {
    line_num += 1;
    if line.trim() == "" { continue; }
    line_lex_res = lex_line(line, line_num);
    if !line_lex_res.is_ok() {
      return Err(line_lex_res.err().unwrap_or("Unknown error.").to_string()
                 + " LINE "
                 + &(line_num.to_string())
      );
    }
    token_list.tokens.push(line_lex_res.ok().unwrap());
  }
  return Ok(token_list);
}

