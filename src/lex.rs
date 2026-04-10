use regex::Regex;
use crate::token::{Token, NumKind};

pub struct Lexer {
  rules: Regex,
}

#[derive(Debug)]
pub enum LexerErr {
  UnexpectedCharErr(usize),
  UnmatchedErr,
  eee,
}

impl Lexer {
  pub fn new() -> Self {
    let pattern = vec![
            "(?P<IDENT>[a-zA-Z_][a-zA-Z0-9_]*)",
            "(?P<RARROW>=>)",
            "(?P<LARROW><[-+*/%=])",
            "(?P<COMMENT>(#[^\n]*)|(##.*##))",
            //"(?P<PUNCT>;;)",
            //"(?P<KEYWORD>::)",
            "(?P<STR>\"(\\.|[^\"])*\")",
            r"(?P<NUM>(-)?[0-9]+(\.[0-9]+)?)",
            "(?P<WHITESPACE>[ \n\t]+)",
    ].join("|");
    Self { rules: Regex::new(&pattern).unwrap() }
  }

  pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, LexerErr> {
    let mut tokens = Vec::new();
    let mut idx = 0;
    while idx < input.len() {
      if let Some(matched) = self.rules.find_at(input, idx) {
        if matched.start() != idx {
          return Err(LexerErr::UnexpectedCharErr(idx));
        }
        let caps = self.rules.captures(&input[idx..]).unwrap();
        if let Some(_) = caps.name("IDENT") {
          tokens.push(Token::Identifier(matched.as_str().to_string()));
        }
        else if let Some(_) = caps.name("STR") {
          tokens.push(Token::Str(matched.as_str().to_string()));
        }
        else if let Some(_) = caps.name("NUM") {
          let num_str = matched.as_str();
          let kind = if num_str.contains('.') {
            let val = num_str.parse::<f64>().map_err(|_| LexerErr::eee)?;
            NumKind::Float(val)
          } else {
            let val = num_str.parse::<i64>().map_err(|_| LexerErr::eee)?;
            NumKind::Int(val)
          };
          tokens.push(Token::Num(kind));
        }


        idx = matched.end();
      }
    }
    
    Ok(tokens)
  }

}
