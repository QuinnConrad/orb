
#[derive(Debug)]
pub enum Token {
  Identifier(String),
  Punctuator(String),
  Keyword(KeywordKind),
  Str(String),
  Num(NumKind),
  Comment(usize),
  Eof,
}

#[derive(Debug)]
pub enum NumKind {
  Int(i64),
  Float(f64),
}

#[derive(Debug)]
pub enum KeywordKind {
  Spell,            // Function
  Arcanum,          // int
  Halfling,         // float
  Glyph,            // char
  Potion,           // struct
  Rune,             // enum
  Alignment,        // bool
  Changeling,       // union
  Impure,           // signed
  Pure,             // unsigned
  Great,            // long
  Perhaps,          // if
  Otherwise,        // else
  Evoke,            // return
  Void,             // void
  Transmute         // typedef
}
