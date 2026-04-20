use crate::{
  token::{Token, KeywordKind},
  ast::{Stmt, ExprKind, TypeKind},
};

pub struct Parser {
  tokens: Vec<Token>,
  idx: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {tokens, idx: 0}
  }

  fn peek(&self) -> &Token {
    &self.tokens[self.idx]
  }

  fn empty(&self) -> bool {
    *self.peek() == Token::Eof
  }

  fn prev(&self) -> Token {
    self.tokens[self.idx - 1].clone()
  } // prev

  fn advance(&mut self) -> Token {
    if !self.empty() {
      self.idx += 1;
    }
    self.prev()
  } // advance

  fn consume(&mut self, expected: Token) -> Result<Token, String> {
    if *self.peek() == expected {
      Ok(self.advance())
    }
    else {
      Err(format!("Expected {:?} but was {:?}", expected, self.peek()))
    }
  } // consume

  pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
    let mut program = Vec::new();

    while !self.empty() {
      program.push(self.parse_statement()?);
    }

    Ok(program)
  } // parse_program

  fn parse_statement(&mut self) -> Result<Stmt, String> {
    let token = self.peek().clone();

    if token == Token::Keyword(KeywordKind::Spell) {
      println!("flibbity flibbity floo");
    }
    todo!()
  } // parse_statement

  fn parse_spell_decl(&mut self) -> Result<Stmt, String> {
    todo!()
  }
} // impl Parser
