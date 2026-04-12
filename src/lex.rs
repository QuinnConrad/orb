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
            "(?P<WORD>[a-zA-Z_][a-zA-Z0-9_]*)",
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
        if let Some(_) = caps.name("WORD") {
          tokens.push(decode_word(matched.as_str()));
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

        idx = matched.end();
      }
    }
    
    Ok(tokens)
  }

}

fn decode_word(word: &str) -> Token {
  match word {
    "alignment" => Token::Keyword(KeywordKind::Alignment),
    "arcanum" => Token::Keyword(KeywordKind::Arcanum),
    "changeling" => Token::Keyword(KeywordKind::Changeling),
    "evoke" => Token::Keyword(KeywordKind::Evoke),
    "glyph" => Token::Keyword(KeywordKind::Glyph),
    "great" => Token::Keyword(KeywordKind::Great),
    "halfling" => Token::Keyword(KeywordKind::Halfling),
    "impure" => Token::Keyword(KeywordKind::Impure),
    "otherwise" => Token::Keyword(KeywordKind::Otherwise),
    "perhaps" => Token::Keyword(KeywordKind::Perhaps),
    "potion" => Token::Keyword(KeywordKind::Potion),
    "pure" => Token::Keyword(KeywordKind::Pure),
    "rune" => Token::Keyword(KeywordKind::Rune),
    "spell" => Token::Keyword(KeywordKind::Spell),
    "transmute" => Token::Keyword(KeywordKind::Transmute),
    "void" => Token::Keyword(KeywordKind::Void),
    _ => Token::Identifier(word.to_string())
  }
}
