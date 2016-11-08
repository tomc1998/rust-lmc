mod lexer;
use lmc;

pub fn assemble(src: &str, lmc: &mut lmc::LMC) {
  let token_list_res = lexer::lex(src);
  let token_list: lexer::TokenList;

  if token_list_res.is_ok() {
    token_list = token_list_res.ok().unwrap();
    for token in &(token_list.tokens) {
      println!("{} {} {}", token.label.unwrap_or(""), token.op, token.operand.unwrap_or(""));
    }
  }
  else {
    println!("{}", token_list_res.err().unwrap_or("".to_string()));
  }
}
