use std::iter::Peekable;

use latex_parser::Ident;

use crate::{contexts::scalar::real::Error, prelude::*};

use super::{IR2Exprs, OpKind, ir2::IR2Flat};

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

impl From<IR3BinaryOp<Ident>> for IR3Expr<Ident> {
  fn from(binary: IR3BinaryOp<Ident>) -> Self {
    IR3Expr::BinaryOp(binary)
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

impl IR3BinaryOp<Ident> {
  pub fn new(lhs: IR3Expr<Ident>, op: OpKind, rhs: IR3Expr<Ident>) -> Self {
    match op {
      OpKind::Add => IR3BinaryOp::Add {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
      },
      OpKind::Neg => IR3BinaryOp::Add {
        lhs: Box::new(lhs),
        rhs: Box::new(
          IR3BinaryOp::Mul {
            lhs: Box::new(IR3Flat::Neg1.into()),
            rhs: Box::new(rhs),
          }
          .into(),
        ),
      },
      OpKind::Mul => IR3BinaryOp::Mul {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
      },
      OpKind::Div => IR3BinaryOp::Div {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
      },
      OpKind::Exp => IR3BinaryOp::Exp {
        base: Box::new(lhs),
        exponent: Box::new(rhs),
      },
    }
  }
}

impl OpKind {
  fn precedence(self) -> NonZero<u8> {
    match self {
      Self::Add | Self::Neg => u8!(1),
      Self::Mul | Self::Div => u8!(2),
      Self::Exp => u8!(3),
    }
  }

  fn is_higher_or_eq_precedence(self, other: Self) -> bool {
    self.precedence() >= other.precedence()
  }
}

impl IR3Expr<Ident> {
  fn recursive_from_ir2(
    prev: IR3Expr<Ident>,
    op1: OpKind,
    current: IR3Flat<Ident>,
    mut next: Peekable<impl Iterator<Item = (OpKind, IR3Flat<Ident>)>>,
  ) -> IR3Expr<Ident> {
    let Some((op2, next_expr)) = next.next() else {
      // base case, ended
      return IR3BinaryOp::new(prev, op1, current.into()).into();
    };

    if op1.is_higher_or_eq_precedence(op2) {
      // e.g.     [prev]   *       2       +      [next]
      //                 [op1] [current] [op2]
      //
      // becomes ([prev]   *       2)      +      [next]
      //         [      newprev     ]   [newop1]
      //         [        prev      ]   [  op1 ]  [ op2 ]  [ next_expr ]
      let newprev = IR3BinaryOp::new(prev.into(), op1, current.into()).into();
      let newop1 = op2;
      let newcurrent = next_expr;
      IR3BinaryOp::new(
        newprev,
        op2,
        IR3Expr::recursive_from_ir2(newprev, newop1, newcurrent, next),
      )
      .into()
    } else {
      // e.g. [prev] + 2 * [next]
      // becomes [prev] * (2 + [next])
      todo!()
    }
  }

  pub fn from_ir2(ir2: IR2Exprs) -> Result<Vec<Self>, Error> {
    let (first, ops) = ir2.resolve();
    let mut ops = ops.into_iter();

    let Some((op, expr)) = ops.next() else {
      // base case, single expr
      return Ok(vec![IR3Flat::from_ir2(first)?.into()]);
    };

    // get next
    let Some((next_expr, next_op)) = ops.next() else {
      // no choice in operator precedence
      return Ok(vec![
        IR3BinaryOp::new(
          IR3Flat::from_ir2(first)?.into(),
          op,
          IR3Flat::from_ir2(expr)?.into(),
        )
        .into(),
      ]);
    };

    todo!()
  }
}

impl IR3Flat<Ident> {
  pub fn from_ir2(ir2: IR2Flat) -> Result<Self, Error> {
    match ir2 {
      IR2Flat::Neg1 => Ok(IR3Flat::Neg1),
      IR2Flat::Num(num) => Ok(IR3Flat::Num(num)),
      IR2Flat::Ident(ident) => Ok(IR3Flat::Ident(ident)),
      IR2Flat::Bracketed(exprs) => Ok(IR3Flat::Bracket(IR3Expr::from_ir2(*exprs)?)),
    }
  }
}
