use regex::Regex;
use crate::token::{Token, NumKind, KeywordKind, OperatorKind, PunctKind};

pub struct Lexer {
  rules: Regex,
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

impl Lexer {
  #[must_use]
  pub fn new() -> Self {
    let pattern = [
            r"(?P<COMMENT>([#][^\n]*))",
            "(?P<WORD>[a-zA-Z_][a-zA-Z0-9_]*)",
            r"(?P<PUNCT>(=>)|([,.\(\)\{\}\[\];@]))",
            r"(?P<OPERATOR>((>=)|(=<)|((<)?[=+*/%-])|((\$)?[&|!><^])|($^)|([=~])))",
            r"(?P<NUM>(-)?[0-9]+(\.[0-9]+)?)",
            "(?P<STR>\"(\\.|[^\"])*\")",
            "(?P<WHITESPACE>[ \n\t]+)"].join("|");
    Self { rules: Regex::new(&pattern).unwrap() }
  }

  /// turns a string into a vector of tokens.
  ///
  /// ```
  /// let mut lex = orb::lex::Lexer::new();
  /// let input = "spell smite(pure arcanum level) => halfling;";
  /// assert!(lex.tokenize(input).is_ok())
  /// ```
  pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut idx = 0;
    while idx < input.len() {
      if let Some(matched) = self.rules.find_at(input, idx) {
        if matched.start() != idx {
          return Err(format!("Unexpected error at {idx}"));
        }
        let caps = self.rules.captures(&input[idx..]).unwrap();
        if caps.name("WORD").is_some() {
          tokens.push(decode_word(matched.as_str()));
        }
        else if caps.name("STR").is_some() {
          tokens.push(Token::Str(matched.as_str().to_string()));
        }
        else if caps.name("NUM").is_some() {
          let num_str = matched.as_str();
          let kind = if num_str.contains('.') {
            let val = num_str.parse::<f64>().map_err(|_| format!("Error parsing number."))?;
            NumKind::Float(val)
          } else {
            let val = num_str.parse::<i64>().map_err(|_| format!("Error parsing number"))?;
            NumKind::Int(val)
          };
          tokens.push(Token::Num(kind));
        }
        else if caps.name("COMMENT").is_some() {
          tokens.push(Token::Comment(matched.len()));
        }
        else if caps.name("OPERATOR").is_some() {
            tokens.push(decode_operator(matched.as_str()));
        }
        else if caps.name("PUNCT").is_some() {
            tokens.push(decode_punct(matched.as_str()));
        }

        idx = matched.end();
      }
    }
    tokens.push(Token::Eof);
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
    _ => unreachable!("Unexpected Punctuator: {word}")
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
    _ => unreachable!("Unexpected Operator: {word}")
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty() {
    let lex: Lexer = Lexer::new();
    let input = "";
    let tokens = lex.tokenize(input);
    if let Ok(tokens) = tokens {
      assert_eq!(tokens.len(), 1);
      assert_eq!(tokens[0], Token::Eof);
    }
    else {
      panic!("Could not tokenize input");
    }
  }

  #[test]
  fn test_numbers() {
    let lex: Lexer = Lexer::new();
    let input = "0 1 3.14 1.618";
    let tokens = lex.tokenize(input);
    if let Ok(mut tokens) = tokens {
      dbg!(&tokens);
      let _ = tokens.pop();
      assert_eq!(tokens.len(), 4);
      assert_eq!(tokens[0], Token::Num(NumKind::Int(0)));
      assert_eq!(tokens[1], Token::Num(NumKind::Int(1)));
      assert_eq!(tokens[2], Token::Num(NumKind::Float(3.14)));
      assert_eq!(tokens[3], Token::Num(NumKind::Float(1.618)));
    }
    else {
      panic!("Could not tokenize input");
    }
  }

  #[test]
  fn test_operators() {
    let lex: Lexer = Lexer::new();
    let input = "<= + - * / % $& $| $! $^ $< $> &\
                | ! = ~ < > =< >= <+ <- <* </ <%";
    let tokens = lex.tokenize(input);
    if let Ok(tokens) = tokens {
      dbg!(&tokens);
      for i in 0..tokens.len() - 1 {
        for j in i + 1..tokens.len() - 1 {
          assert_ne!(tokens[i], tokens[j]);
        }
        assert!(matches!(tokens[i], Token::Operator(_)));
      }
      assert_eq!(tokens.len(), 27);
    }
    else {
      dbg!("{tokens:?}");
      panic!("Could not tokenize input");
    }
  }
}
