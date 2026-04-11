use regex::Regex;
use crate::token::{Token, NumKind, KeywordKind};

pub struct Lexer {
  rules: Regex,
}

#[derive(Debug)]
pub enum LexerErr {
  UnexpectedCharErr(usize),
  UnmatchedErr,
  ParsingErr,
}

impl Lexer {
  pub fn new() -> Self {
    let pattern = vec![
            "(?P<KEYWORD>(perhaps|otherwise))",
            "(?P<IDENT>[a-zA-Z_][a-zA-Z0-9_]*)",
            "(?P<RARROW>=>)",
            "(?P<LARROW><[-+*/%=])",
            "(?P<COMMENT>(#[^\n]*))",
            //"(?P<PUNCT>;;)",
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
            let val = num_str.parse::<f64>().map_err(|_| LexerErr::ParsingErr)?;
            NumKind::Float(val)
          } else {
            let val = num_str.parse::<i64>().map_err(|_| LexerErr::ParsingErr)?;
            NumKind::Int(val)
          };
          tokens.push(Token::Num(kind));
        }
        else if let Some(_) = caps.name("COMMENT") {
          tokens.push(Token::Comment(matched.len()));
        }
        else if let Some(_) = caps.name("KEYWORD") {
          let matched_str = matched.as_str();
          let kind = match matched_str {
            "perhaps" => KeywordKind::Perhaps,
            "otherwise" => KeywordKind::Otherwise,
            _ => panic!(),
          };
          tokens.push(Token::Keyword(kind));
        }


        idx = matched.end();
      }
    }
    
    Ok(tokens)
  }

}
