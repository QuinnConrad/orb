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
  /// let input = String::from("spell fireball(arcanum level, halfling radius) => arcanum {")
  ///                     + "\n    # This is an orb comment."
  ///                     + "\n }";
  /// let lex = orb::lex::Lexer::new();
  /// let tokens = lex.tokenize(&input).expect("Should tokenize");
  /// dbg!("{:?}", tokens.clone());
  /// let mut parser = orb::parser::Parser::new(tokens);
  /// let output = parser.parse_program();
  /// dbg!(&output);
  /// assert!(output.is_ok());
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

    match *self.peek() {
      Token::Keyword(KeywordKind::Spell) => return self.parse_spell_decl(),
      Token::Comment(mana) => {self.advance(); return Ok(Stmt::Comment(mana))},
      _ => todo!("{:?}", *self.peek()),
    }
  } // parse_statement


  fn parse_type(&mut self) -> Result<TypeKind, String> {
    let token = self.advance();
    match token {
      Token::Keyword(KeywordKind::Void) => Ok(TypeKind::Void),
      Token::Keyword(_) => Ok(TypeKind::TODO),
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
      if *self.peek() != Token::Punctuator(PunctKind::CloseParen) {
        self.consume(&Token::Punctuator(PunctKind::Comma))?;
      }
      args.push( (arg_type, arg_name) );
    }
    self.consume(&Token::Punctuator(PunctKind::CloseParen))?;
    self.consume(&Token::Punctuator(PunctKind::Rarrow))?;

    let ret_val = self.parse_type()?;

    self.consume(&Token::Punctuator(PunctKind::OpenBrace))?;
    let mut body = vec![];
    while *self.peek() != Token::Punctuator(PunctKind::CloseBrace) {
      body.push(self.parse_statement()?);
    }
    self.consume(&Token::Punctuator(PunctKind::CloseBrace))?;

    Ok(Stmt::SpellDecl {
      name,
      ret_val,
      args,
      body,
    })
  }
} // impl Parser
