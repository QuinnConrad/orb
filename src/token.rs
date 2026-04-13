#[derive(Debug)]
pub enum Token {
  Identifier(String),
  Punctuator(PunctKind),
  Operator(OperatorKind),
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
  Void,             // Void
  Whilst,           // While
  Persevere,        // continue
  Banish,           // break
  Transmute         // typedef
}

#[derive(Debug)]
pub enum PunctKind {
  Comma,
  Dot,
  Semicolon,
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
  OpenBracket,
  CloseBracket,
  At,
  Rarrow,
}

#[derive(Debug)]
pub enum OperatorKind {
  Assign,             // <=
  Plus, Minus,        // +, -
  Mult, Div, Mod,     // *, /, %
  BitAnd, BitOr,      // $&, $|
  BitNot, BitXor,     // $!, $^
  BitLeft, BitRight,   // $<, $>
  LogAnd, LogOr,      // &, |
  LogNot,             // !
  Eq, Ne,             // =, ~
  Lt, Gt, Le, Ge,     // <, >, =<, >=
  PlusAssign,         // <+
  MinusAssign,        // <-
  MultAssign,         // <*
  DivAssign,          // </
  ModAssign,          // <%
}
