/// Both binary and unary
#[derive(Debug)]
pub enum OpKind {
  Neg,
  Add,
  Mul,
  Div,
  Exp,
}

pub use ir1::IR1Expr;
pub use ir2::IR2Exprs;

mod ir1;
mod ir2;
mod ir3;
mod from_ir2 {
  //! non trivial

  use latex_parser::Ident;

  use crate::{
    contexts::scalar::real::{ContextOneVarEq, Error, expr::Exprs},
    prelude::*,
  };

  use super::IR2Exprs;

  impl Exprs<Ident> {
    fn from_ir2(context: &ContextOneVarEq<Ident>, ir2: IR2Exprs) -> Result<Self, Error> {
      todo!()
    }
  }
}
