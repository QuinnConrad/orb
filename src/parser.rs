use crate::{
  token::{Token, KeywordKind, PunctKind},
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

  fn consume(&mut self, expected: &Token) -> Result<Token, String> {
    if *self.peek() == *expected {
      Ok(self.advance())
    }
    else {
      Err(format!("Expected {:?} but was {:?}", expected, self.peek()))
    }
  } // consume


  ///
  /// ```
  /// let input = "spell smite() => arcanum";
  /// let lex = orb::lex::Lexer::new();
  /// let tokens = lex.tokenize(input).expect("Should tokenize");
  /// let mut parser = orb::parser::Parser::new(tokens);
  /// dbg!(parser.parse_program());
  /// ```
  pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
    let mut program = Vec::new();

    while !self.empty() {
      program.push(self.parse_statement()?);
    }

    Ok(program)
  } // parse_program


  fn parse_statement(&mut self) -> Result<Stmt, String> {
    let token = self.peek().clone();

    match self.peek().clone() {
      Token::Keyword(KeywordKind::Spell) => return self.parse_spell_decl(),
      _ => todo!(),
    }
    unreachable!()
  } // parse_statement


  fn parse_type(&mut self) -> Result<TypeKind, String> {
    let token = self.advance();
    match token {
      Token::Keyword(KeywordKind::Void) => Ok(TypeKind::Void),
      Token::Keyword(KeywordKind::Arcanum) => Ok(TypeKind::TODO),
      _ => Err(format!("Expected type keyword, got {:?}", token)),
    }
  } // parse_type

  fn parse_spell_decl(&mut self) -> Result<Stmt, String> {
    self.consume(&Token::Keyword(KeywordKind::Spell))?;

    let name = match self.advance() {
      Token::Identifier(n) => n,
      e => return Err(format!("Expected spell name, got {e:?}")),
    };
    self.consume(&Token::Punctuator(PunctKind::OpenParen))?;

    let mut args = Vec::new();
    while *self.peek() != Token::Punctuator(PunctKind::CloseParen) {
      let arg_type = self.parse_type()?;
      let arg_name = match self.advance() {
        Token::Identifier(n) => n,
        e => return Err(format!("Expected argument name, got {e:?}")),
      };
      args.push( (arg_type, arg_name) );
    }
    self.consume(&Token::Punctuator(PunctKind::CloseParen))?;
    self.consume(&Token::Punctuator(PunctKind::Rarrow))?;

    let ret_val = self.parse_type()?;



    let body = vec![];
    //let args = vec![];

    Ok(Stmt::SpellDecl {
      name,
      ret_val,
      args,
      body,
    })
  }
} // impl Parser
