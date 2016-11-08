mod lexer;
mod parser;
mod code_gen;
use lmc;

/// Returns false for assembly error, true for success
pub fn assemble(src: &str, lmc: &mut lmc::LMC) -> bool {
  let token_list_res: Result<lexer::TokenList, String>;
  let token_list: lexer::TokenList;
  let operations_res: Result<Vec<parser::Operation>, String>;
  let operations: Vec<parser::Operation>;

  token_list_res = lexer::lex(src);
  if !token_list_res.is_ok() {
    println!("{}", token_list_res.err().unwrap_or("".to_string()));
    return false;
  }
  token_list = token_list_res.ok().unwrap();

  operations_res = parser::parse(&token_list);
  if !operations_res.is_ok() {
    println!("{}", operations_res.err().unwrap_or("".to_string()));
    return false;
  }
  operations = operations_res.ok().unwrap();

  code_gen::gen(&operations, lmc);
  return true;
}
