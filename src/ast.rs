use token::{NumKind, OperatorKind}


pub enum Stmt {
  Let {
    var_type: Option<TypeKind>,
    name: String,
    val: ExprKind,
  },
  Expr(ExprKind),
  SpellDecl {
    name: String,
    ret_val: TypeKind,
    args: Vec<TypeKind>,
    body: Vec<Stmt>,
  },
  Comment(usize), // For gathering mana.
  VarDecl {
    name: String,
    var_type: TypeKind,
  },

}

pub enum TypeKind {
  Void,
  TODO,
}

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
