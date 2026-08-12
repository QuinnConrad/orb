use crate::token::{NumKind, OperatorKind};

#[derive(Debug)]
pub enum Stmt {
  Let {
    var_type: TypeKind,
    name: String,
    val: Option<ExprKind>,
  },
  Expr(ExprKind),
  SpellDecl {
    name: String,
    ret_val: TypeKind,
    args: Vec<(TypeKind, String)>,
    body: Vec<Stmt>,
  },
  Comment(usize), // For gathering mana.
  VarDecl {
    name: String,
    var_type: TypeKind,
  },
  Return(Option<ExprKind>),
  Conditional {
    condition: ExprKind,
    consequent: Vec<Stmt>,
    alternative: Vec<Stmt>
  }

}

#[derive(Debug)]
pub enum TypeKind {
  Arcanum,
  Glyph,
  Halfling,
  Void,
  Custom(String),
  TODO,
}

#[derive(Debug)]
pub enum ExprKind {
  NumLiteral(NumKind),
  StrLiteral(String),
  BinOperation {
    lhs: Box<ExprKind>,
    op: OperatorKind,
    rhs: Box<ExprKind>
  },
  Variable(String),
  FunctionCall {
    name: String,
    args: Vec<ExprKind>,
  }
}
