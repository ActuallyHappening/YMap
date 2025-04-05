use latex_parser::Ident;

use crate::{contexts::scalar::real::Error, prelude::*};

use super::{IR2Exprs, ir2::IR2ExprFlat};

/// Takes into account operator precedence
pub enum IR3Expr<Ident> {
  BinaryOp(IR3BinaryOp<Ident>),
  Flat(IR3Flat<Ident>),
}

impl From<IR3Flat<Ident>> for IR3Expr<Ident> {
  fn from(flat: IR3Flat<Ident>) -> Self {
    IR3Expr::Flat(flat)
  }
}

pub enum IR3Flat<Ident> {
  Neg1,
  Num(BigUint),
  Ident(Ident),
  Bracket(Vec<IR3Expr<Ident>>),
}

pub enum IR3BinaryOp<Ident> {
  Add {
    lhs: Box<IR3Expr<Ident>>,
    rhs: Box<IR3Expr<Ident>>,
  },
  Mul {
    lhs: Box<IR3Expr<Ident>>,
    rhs: Box<IR3Expr<Ident>>,
  },
  Div {
    lhs: Box<IR3Expr<Ident>>,
    rhs: Box<IR3Expr<Ident>>,
  },
  Exp {
    base: Box<IR3Expr<Ident>>,
    exponent: Box<IR3Expr<Ident>>,
  },
}

impl IR3Expr<Ident> {
  pub fn from_ir2(ir2: IR2Exprs) -> Result<Vec<Self>, Error> {
    let mut ops = ir2.ops.into_iter();
    let Some((expr, op)) = ops.next() else {
      // base case
      return Ok(vec![IR3Flat::from_ir2(ir2.last)?.into()]);
    };

    todo!()
  }
}

impl IR3Flat<Ident> {
  pub fn from_ir2(ir2: IR2ExprFlat) -> Result<Self, Error> {
    match ir2 {
      IR2ExprFlat::Neg1 => Ok(IR3Flat::Neg1),
      IR2ExprFlat::Num(num) => Ok(IR3Flat::Num(num)),
      IR2ExprFlat::Ident(ident) => Ok(IR3Flat::Ident(ident)),
      IR2ExprFlat::Bracketed(exprs) => Ok(IR3Flat::Bracket(IR3Expr::from_ir2(*exprs)?)),
    }
  }
}
