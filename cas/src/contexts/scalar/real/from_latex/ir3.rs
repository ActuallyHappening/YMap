use latex_parser::Ident;

use crate::{contexts::scalar::real::Error, prelude::*};

use super::IR2Exprs;

/// Takes into account operator precedence
pub enum IR3Expr<Ident> {
  BinaryOp(IR3BinaryOp<Ident>),
  Flat(IR3Flat<Ident>),
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
  pub fn from_ir2(ir2: IR2Exprs) -> Result<Self, Error> {
    
    
    todo!()
  }
}
