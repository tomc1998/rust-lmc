use assembler::lexer as lexer;

pub struct Operation {
  pub op_code: u8,
  pub operand: u16,
}

struct Label<'a> {
  loc: u8,
  name: &'a str,
}

/// First pass of parsing, retrieve labels and their locations
fn label_pass<'a>(token_list: &'a lexer::TokenList) -> Vec<Label<'a>> {
  let mut labels: Vec<Label> = Vec::new();
  let mut loc = 0u8;
  for t in &(token_list.tokens) {
    if t.label.is_some() {
      labels.push(Label{
        loc: loc,
        name: t.label.unwrap(),
      });
    }
    loc += 1;
  }
  return labels;
}

/// Second pass of parsing, parse out op codes and operands.  Just
/// linear search for labels, only 99 mailboxes so no need for
/// something like a search tree.
fn op_pass(token_list: &lexer::TokenList, labels: &Vec<Label>) -> Result<Vec<Operation>, String> {
  let mut operations = Vec::with_capacity(token_list.tokens.len());
  let mut curr_op: Operation;
  for t in &token_list.tokens {
    curr_op = Operation{op_code: 255, operand: 255};
    curr_op.op_code = match t.op {
      "ADD" => 1,
      "SUB" => 2,
      "STO" => 3,
      "LDA" => 5,
      "BR"  => 6,
      "BRZ" => 7,
      "BRP" => 8,
      "IN"  => {
        curr_op.operand = 1;
        9
      },
      "OUT" => {
        curr_op.operand = 2;
        9
      },
      "HLT" => {
        curr_op.operand = 0;
        0
      },
      "DAT" => {
        if t.operand.is_some() {
          let operand_str = t.operand.unwrap().trim();
          let operand_res = operand_str.parse::<u16>();
          if operand_res.is_ok() {
            curr_op.operand = operand_res.ok().unwrap() as u16;
            if curr_op.operand > 999 {
              curr_op.operand = 999;
              return Err("DAT value too large at line ".to_string() + &(t.lnum.to_string()));
            }
          }
          else {
            return Err("Err parsing DAT value at line ".to_string() + &(t.lnum.to_string()));
          }
        }
        else {
          curr_op.operand = 0;
        }
        10
      },
      _ => {
        return Err("Err parsing op code at line ".to_string() + &(t.lnum.to_string()));
      }
    };
    if curr_op.operand == 255 {
      if t.operand.is_none() {
        return Err("Err parsing, no operand at line ".to_string() + &(t.lnum.to_string()));
      }
      let operand = t.operand.unwrap();
      for l in labels {
        if l.name == operand {
          curr_op.operand = l.loc as u16;
        }
      }
      if curr_op.operand == 255 {
        return Err("Err parsing, label ".to_string() + operand +
                   " not found at line " + &(t.lnum.to_string()));
      }
    }
    operations.push(curr_op);
  }
  return Ok(operations);
}

pub fn parse(token_list: &lexer::TokenList) -> Result<Vec<Operation>, String> {
  let ops : Result<Vec<Operation>, String>;
  let labels: Vec<Label>;

  labels = label_pass(token_list);
  ops = op_pass(token_list, &labels);
  return ops;
}
