enum OpKind {
  Neg,
  Add,
  Mul,
  Div,
  Exp,
}

mod ir1;

mod ir2 {
  use crate::{contexts::scalar::real::Error, prelude::*};

  use latex_parser::Ident;

  use super::{OpKind, ir1::IR1TokenExpr};

  /// Eats unary operators, ensuring all binary
  /// operators are correctly arranged
  pub(super) struct IR2Exprs {
    ops: Vec<(IR2ExprFlat, OpKind)>,
    last: Option<IR2ExprFlat>,
  }

  /// Eats unary operators like [`OpKind::Neg`]
  pub(super) enum IR2ExprFlat {
    Num(BigUint),
    Unary(IR2Unary),
    Ident(Ident),
  }

  enum IR2Unary {
    Neg(Vec<IR2ExprFlat>),
  }

  impl IR2Exprs {
    fn from_ir1_tokens(token: impl IntoIterator<Item = IR1TokenExpr>) -> Result<IR2Exprs, Error> {
      todo!()
    }
  }
}
