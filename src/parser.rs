use crate::{
  token::{Token, KeywordKind, PunctKind, OperatorKind},
  ast::{Stmt, ExprKind, TypeKind},
};

pub struct Parser {
  tokens: Vec<Token>,
  idx: usize,
}

impl Parser {
  #[must_use]
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

  fn peek_next(&self) -> Token {
    if self.empty() {
      Token::Eof
    } else {
      self.tokens[self.idx + 1].clone()
    }
  } // peek_next

  fn advance(&mut self) -> Token {
    if !self.empty() {
      self.idx += 1;
    }
    self.prev()
  } // advance

  fn consume(&mut self, expected: &Token) -> Result<Token, String> {
    if self.peek() == expected {
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
    match *self.peek() {
      Token::Keyword(KeywordKind::Spell) => self.parse_spell_decl(),
      Token::Comment(mana) => {self.advance(); Ok(Stmt::Comment(mana))},
      Token::Keyword(KeywordKind::Arcanum |
                     KeywordKind::Glyph |
                     KeywordKind::Halfling) => self.parse_let(),
      Token::Keyword(KeywordKind::Evoke) => self.parse_return(),
      Token::Keyword(KeywordKind::Perhaps) => self.parse_conditional(),
      Token::Keyword(KeywordKind::Whilst) => self.parse_while(),
      Token::Identifier(_) => self.parse_identifier(),
      _ => todo!("{:?}", *self.peek()),
    }
  } // parse_statement


  fn parse_type(&mut self) -> Result<TypeKind, String> {
    let token = self.advance();
    match token {
      Token::Keyword(KeywordKind::Arcanum) => Ok(TypeKind::Arcanum),
      Token::Keyword(KeywordKind::Glyph) => Ok(TypeKind::Glyph),
      Token::Keyword(KeywordKind::Halfling) => Ok(TypeKind::Halfling),
      Token::Keyword(KeywordKind::Void) => Ok(TypeKind::Void),
      Token::Identifier(ident) => Ok(TypeKind::Custom(ident)),
      Token::Keyword(_) => Ok(TypeKind::TODO),
      _ => Err(format!("Expected type keyword or ident, got {token:?}")),
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
  } // parse_spell_decl

  fn parse_expr_stmt(&mut self) -> Result<Stmt, String> {
    let expr = self.parse_expr()?;

    if *self.peek() == Token::Punctuator(PunctKind::Semicolon) {
        self.advance();
    }

    Ok(Stmt::Expr(expr))
  } // parse_expr_stmt

  fn parse_func_call(&mut self, name: String) -> Result<ExprKind, String> {
    self.consume(&Token::Punctuator(PunctKind::OpenParen))?;

    let mut args = Vec::new();
    while *self.peek() != Token::Punctuator(PunctKind::CloseParen) {
        args.push(self.parse_expr()?);
        if *self.peek() != Token::Punctuator(PunctKind::CloseParen) {
            self.consume(&Token::Punctuator(PunctKind::Comma))?;
        }
    }
    self.consume(&Token::Punctuator(PunctKind::CloseParen))?;

    Ok(ExprKind::FunctionCall { name, args })
  } // parse_func_call


  fn parse_expr(&mut self) -> Result<ExprKind, String> {
    let lhs = self.parse_primary()?;
    self.parse_recur(lhs, 0)
  } // parse_expr

  fn parse_primary(&mut self) -> Result<ExprKind, String> {
    match self.advance() {
      Token::Num(n) => Ok(ExprKind::NumLiteral(n) ),
      Token::Str(str) => Ok(ExprKind::StrLiteral(str)),
      Token::Identifier(name) => {
        if *self.peek() == Token::Punctuator(PunctKind::OpenParen) {
          self.parse_func_call(name)
        }
        else {
          Ok(ExprKind::Variable(name))
        }
      },
      Token::Punctuator(PunctKind::OpenParen) => {
        let expr = self.parse_expr()?;
        self.consume(&Token::Punctuator(PunctKind::CloseParen))?;
        Ok(expr)
      },
      tok => Err(format!("Unexpected token: {tok:?}")),
    }
  } // parse_primary

  fn parse_recur(&mut self, mut lhs: ExprKind, min_precedence: i32) -> Result<ExprKind, String> {
    loop {
      let lookahead = self.peek().clone();
      let precedence = get_precedence(&lookahead);

      if precedence < min_precedence {
        break;
      }

      let op_token = self.advance();
      let Token::Operator(op) = op_token else {
        return Err(format!("Expected operator, found {op_token:?}"))
      };

      let mut rhs = self.parse_primary()?;

      let next_lookahead = self.peek();
      let next_precedence = get_precedence(next_lookahead);

      if next_precedence > precedence {
        rhs = self.parse_recur(rhs, precedence + 1)?;
      } else if is_right_associative(&op_token) && next_precedence == precedence {
        rhs = self.parse_recur(rhs, precedence)?;
      }

      lhs = ExprKind::BinOperation {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
      };
    }
    Ok(lhs)
  } // parse_recur




  fn parse_let(&mut self) -> Result<Stmt, String> {
    let var_type = self.parse_type()?;

    let name = match self.advance() {
        Token::Identifier(ident) => ident,
        tok => return Err(format!("Expected variable name, got {tok:?}")),
    };

    let val = if *self.peek() == Token::Operator(OperatorKind::Assign) {
        self.advance();
        Some(self.parse_expr()?)
    } else {
        None
    };

    let _ = self.consume(&Token::Punctuator(PunctKind::Semicolon));

    Ok(Stmt::Let {
      var_type,
      name,
      val,
    })
  } // parse_let

  fn parse_return(&mut self) -> Result<Stmt, String> {
    self.consume(&Token::Keyword(KeywordKind::Evoke))?;
    let mut value = None;
    if self.peek() != &Token::Punctuator(PunctKind::Semicolon) {
      value = Some(self.parse_expr()?);
    }
    let _ = self.consume(&Token::Punctuator(PunctKind::Semicolon));
    Ok(Stmt::Return(value))
  } // parse_return

  fn parse_identifier(&mut self) -> Result<Stmt, String> {
    let tok = self.peek();
    let Token::Identifier(_tok) = tok else {
      return Err(format!("Expected identifier; got {tok:?}"))
    };
    let next = self.peek_next();
    match next {
      Token::Identifier(_) => self.parse_let(),
      Token::Punctuator(PunctKind::OpenParen) => {
        let res = Ok(Stmt::Expr(self.parse_expr()?));
        let _ = self.consume(&Token::Punctuator(PunctKind::Semicolon));
        res
      },
      Token::Operator(_) => {
        let Token::Identifier(name) = self.advance() else { unreachable!() };
        let op = self.advance();
        let val = self.parse_expr()?;
        let _ = self.consume(&Token::Punctuator(PunctKind::Semicolon))?;
        if is_right_associative(&op) {
          let Token::Operator(op) = op else { unreachable!() };
          Ok(Stmt::Assignment {name, op, val})
        }
        else {
          Err(format!("Expected assignment but found {op:?}"))
        }
      }
      //TODO: assignment
      _ => Err(format!("Unexpected token: {next:?}")),
    }
  } // parse_identifier

  fn parse_conditional(&mut self) -> Result<Stmt, String> {
    let _ = self.consume(&Token::Keyword(KeywordKind::Perhaps))?;
    let _ = self.consume(&Token::Punctuator(PunctKind::OpenParen))?;
    let condition = self.parse_expr()?;
    let _ = self.consume(&Token::Punctuator(PunctKind::CloseParen))?;

    let _ = self.consume(&Token::Punctuator(PunctKind::OpenBrace))?;
    let mut consequent = Vec::new();
    while self.peek() != &Token::Punctuator(PunctKind::CloseBrace) {
      consequent.push(self.parse_statement()?);
    }
    let _ = self.consume(&Token::Punctuator(PunctKind::CloseBrace))?;

    let mut alternative = Vec::new();
    if self.peek() == &Token::Keyword(KeywordKind::Otherwise) {
      let _ = self.consume(&Token::Keyword(KeywordKind::Otherwise))?;
      let _ = self.consume(&Token::Punctuator(PunctKind::OpenBrace))?;
      while self.peek() != &Token::Punctuator(PunctKind::CloseBrace) {
        alternative.push(self.parse_statement()?);
      }
      let _ = self.consume(&Token::Punctuator(PunctKind::CloseBrace))?;
    }

    Ok(Stmt::Conditional {
      condition,
      consequent,
      alternative
    })
  } // parse_conditional

  fn parse_while(&mut self) -> Result<Stmt, String> {
    let _ = self.consume(&Token::Keyword(KeywordKind::Whilst))?;
    let _ = self.consume(&Token::Punctuator(PunctKind::OpenParen))?;
    let condition = self.parse_expr()?;
    let _ = self.consume(&Token::Punctuator(PunctKind::CloseParen))?;

    let _ = self.consume(&Token::Punctuator(PunctKind::OpenBrace))?;
    let mut body = Vec::new();
    while self.peek() != &Token::Punctuator(PunctKind::CloseBrace) {
      body.push(self.parse_statement()?);
    }
    let _ = self.consume(&Token::Punctuator(PunctKind::CloseBrace))?;

    Ok(Stmt::While {condition, body})
  } // parse_while

} // impl Parser


fn get_precedence(token: &Token) -> i32 {
  match token {
    Token::Operator(op) => match op {
      OperatorKind::Assign | OperatorKind::PlusAssign | OperatorKind::MinusAssign |
      OperatorKind::MultAssign | OperatorKind::DivAssign | OperatorKind::ModAssign => 1,

      OperatorKind::LogOr => 2,
      OperatorKind::LogAnd => 3,

      OperatorKind::Eq | OperatorKind::Ne |
        OperatorKind::Lt | OperatorKind::Gt |
        OperatorKind::Le | OperatorKind::Ge => 4,
      OperatorKind::BitOr => 5,
      OperatorKind::BitXor => 6,
      OperatorKind::BitAnd => 7,
      OperatorKind::BitLeft | OperatorKind::BitRight => 8,
      OperatorKind::Plus | OperatorKind::Minus => 9,
      OperatorKind::Mult | OperatorKind::Div | OperatorKind::Mod => 10,
      _ => -1,
    },
    _ => -1,
  }
} // get_precedence

fn is_right_associative(token: &Token) -> bool {
  match token {
  Token::Operator(op) => matches!(
    op,
    OperatorKind::Assign |
    OperatorKind::PlusAssign |
    OperatorKind::MinusAssign |
    OperatorKind::MultAssign |
    OperatorKind::DivAssign |
    OperatorKind::ModAssign
    ),
    _ => false,
  }
} // is_right_associtative
