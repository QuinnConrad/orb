use regex::Regex;
use crate::token::{Token, NumKind, KeywordKind, OperatorKind, PunctKind};

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
            "(?P<OPERATOR>((>=)|(=<)|((<)?[=+*/%-])|(($)?[&|!><])|($^)|([=~])))",
            "(?P<COMMENT>(#[^\n]*))",
            r"(?P<PUNCT>(=>)|([,.\(\)\{\}\[\];@]))",
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
        else if let Some(_) = caps.name("OPERATOR") {
            tokens.push(decode_operator(matched.as_str()));
        }
        else if let Some(_) = caps.name("PUNCT") {
            tokens.push(decode_punct(matched.as_str()));
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
    "banish" => Token::Keyword(KeywordKind::Banish),
    "changeling" => Token::Keyword(KeywordKind::Changeling),
    "evoke" => Token::Keyword(KeywordKind::Evoke),
    "glyph" => Token::Keyword(KeywordKind::Glyph),
    "great" => Token::Keyword(KeywordKind::Great),
    "halfling" => Token::Keyword(KeywordKind::Halfling),
    "impure" => Token::Keyword(KeywordKind::Impure),
    "otherwise" => Token::Keyword(KeywordKind::Otherwise),
    "perhaps" => Token::Keyword(KeywordKind::Perhaps),
    "persevere" => Token::Keyword(KeywordKind::Persevere),
    "potion" => Token::Keyword(KeywordKind::Potion),
    "pure" => Token::Keyword(KeywordKind::Pure),
    "rune" => Token::Keyword(KeywordKind::Rune),
    "spell" => Token::Keyword(KeywordKind::Spell),
    "transmute" => Token::Keyword(KeywordKind::Transmute),
    "void" => Token::Keyword(KeywordKind::Void),
    "whilst" => Token::Keyword(KeywordKind::Whilst),
    _ => Token::Identifier(word.to_string())
  }
}

fn decode_punct(word: &str) -> Token {
  match word {
    "." => Token::Punctuator(PunctKind::Dot),
    "," => Token::Punctuator(PunctKind::Comma),
    ";" => Token::Punctuator(PunctKind::Semicolon),
    "(" => Token::Punctuator(PunctKind::OpenParen),
    ")" => Token::Punctuator(PunctKind::CloseParen),
    "{" => Token::Punctuator(PunctKind::OpenBrace),
    "}" => Token::Punctuator(PunctKind::CloseBrace),
    "[" => Token::Punctuator(PunctKind::OpenBracket),
    "]" => Token::Punctuator(PunctKind::CloseBracket),
    "@" => Token::Punctuator(PunctKind::At),
    "=>" => Token::Punctuator(PunctKind::Rarrow),
    _ => panic!("Unexpected Punctuator: {}", word.to_string())
  }
}

fn decode_operator(word: &str) -> Token {
  match word {
    "<=" => Token::Operator(OperatorKind::Assign),
    "+" => Token::Operator(OperatorKind::Plus),
    "-" => Token::Operator(OperatorKind::Minus),
    "*" => Token::Operator(OperatorKind::Mult),
    "/" => Token::Operator(OperatorKind::Div),
    "%" => Token::Operator(OperatorKind::Mod),
    "$&" => Token::Operator(OperatorKind::BitAnd),
    "$|" => Token::Operator(OperatorKind::BitOr),
    "$!" => Token::Operator(OperatorKind::BitNot),
    "$^" => Token::Operator(OperatorKind::BitXor),
    "$<" => Token::Operator(OperatorKind::BitLeft),
    "$>" => Token::Operator(OperatorKind::BitRight),
    "&" => Token::Operator(OperatorKind::LogAnd),
    "|" => Token::Operator(OperatorKind::LogOr),
    "!" => Token::Operator(OperatorKind::LogNot),
    "=" => Token::Operator(OperatorKind::Eq),
    "~" => Token::Operator(OperatorKind::Ne),
    "<" => Token::Operator(OperatorKind::Lt),
    ">" => Token::Operator(OperatorKind::Gt),
    "=<" => Token::Operator(OperatorKind::Le),
    ">=" => Token::Operator(OperatorKind::Ge),
    "<+" => Token::Operator(OperatorKind::PlusAssign),
    "<-" => Token::Operator(OperatorKind::MinusAssign),
    "<*" => Token::Operator(OperatorKind::MultAssign),
    "</" => Token::Operator(OperatorKind::DivAssign),
    "<%" => Token::Operator(OperatorKind::ModAssign),
    _ => panic!("Unexpected Operator: {}", word.to_string())
  }
}
