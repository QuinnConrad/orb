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

}

#[derive(Debug)]
pub enum TypeKind {
  Void,
  TODO,
}

#[derive(Debug)]
pub enum ExprKind {
  NumLiteral(NumKind),
  StrLiteral,
  BinOperator {
    lhs: Box<ExprKind>,
    op: OperatorKind,
    rhs: Box<ExprKind>
  },
  Identifier(String),
}
