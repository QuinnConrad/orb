use orb::*;
use token::Token;
pub fn string_to_tok(str: String) -> Result<Vec<Token>, String> {
  let mut lex = lex::Lexer::new();
  lex.tokenize(&str)
}

use ast::*;
pub fn string_to_ast(str: String) -> Result<Vec<Stmt>, String> {
  let toks = string_to_tok(str)?;
  let mut parser = parser::Parser::new(toks);
  parser.parse_program()
}
