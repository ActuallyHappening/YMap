/// Both binary and unary
#[derive(Debug)]
enum OpKind {
  Neg,
  Add,
  Mul,
  Div,
  Exp,
}

pub use ir1::IR1Expr;
pub use ir2::IR2Exprs;

mod ir1;
mod ir2 {
  use std::iter::Peekable;

  use crate::{contexts::scalar::real::Error, prelude::*};

  use latex_parser::Ident;

  use super::{
    OpKind,
    ir1::{IR1Expr, IR1ExprFlat},
  };

  /// Cancels unary [`OpKind::Neg`] and [`OpKind::Add`]
  /// into -1 * ... or ...
  #[derive(Debug)]
  pub struct IR2Exprs {
    ops: Vec<(IR2ExprFlat, OpKind)>,
    last: IR2ExprFlat,
  }

  #[derive(Debug)]
  pub enum IR2ExprFlat {
    Neg1,
    Num(BigUint),
    Ident(Ident),
    Bracketed(Box<IR2Exprs>),
  }

  impl IR2ExprFlat {
    fn from_ir1_expr_flat(flat_expr: IR1ExprFlat) -> Result<IR2ExprFlat, Error> {
      match flat_expr {
        IR1ExprFlat::Num(num) => Ok(IR2ExprFlat::Num(num)),
        IR1ExprFlat::Ident(ident) => Ok(IR2ExprFlat::Ident(ident)),
        IR1ExprFlat::Bracketed(exprs) => {
          Ok(IR2ExprFlat::Bracketed(Box::new(IR2Exprs::from_ir1(exprs)?)))
        }
      }
    }
  }

  #[derive(Clone, Copy)]
  enum Sign {
    Positive,
    Negative,
  }

  impl Sign {
    fn combine(self, other: Sign) -> Sign {
      match (self, other) {
        (Sign::Positive, Sign::Positive) => Sign::Positive,
        (Sign::Positive, Sign::Negative) => Sign::Negative,
        (Sign::Negative, Sign::Positive) => Sign::Negative,
        (Sign::Negative, Sign::Negative) => Sign::Positive,
      }
    }

    /// Only on Add and Neg
    fn from_op(op: OpKind) -> Sign {
      match op {
        OpKind::Add => Sign::Positive,
        OpKind::Neg => Sign::Negative,
        _ => unreachable!(),
      }
    }
  }

  fn next_flat_expr(token: IR1Expr) -> Result<IR1ExprFlat, Error> {
    match token {
      IR1Expr::Op(_op) => Err(Error::CantListOperators),
      IR1Expr::Expr(expr) => Ok(expr),
    }
  }

  fn next_op(token: IR1Expr) -> Result<OpKind, Error> {
    match token {
      IR1Expr::Op(op) => Ok(op),
      IR1Expr::Expr(_flat) => Err(Error::CantListExpressions),
    }
  }

  enum Extract1Pair<I>
  where
    I: Iterator<Item = IR1Expr>,
  {
    ExprOp {
      left: Peekable<I>,
      expr: IR2ExprFlat,
      op: OpKind,
    },
    FinalExpr(IR2ExprFlat),
  }

  fn extract_one<I>(mut tokens: Peekable<I>) -> Result<Extract1Pair<I>, Error>
  where
    I: Iterator<Item = IR1Expr>,
  {
    let first = tokens.next().ok_or(Error::NoTokens)?;
    match first {
      IR1Expr::Expr(expr) => {
        let expr = IR2ExprFlat::from_ir1_expr_flat(expr)?;
        let Some(next_token) = tokens.next() else {
          return Ok(Extract1Pair::FinalExpr(expr));
        };
        let op = next_op(next_token)?;
        Ok(Extract1Pair::ExprOp {
          left: tokens,
          expr,
          op,
        })
      }
      // since this is the first, must be an unary
      IR1Expr::Op(op) => match op {
        // these can act as unary
        OpKind::Add | OpKind::Neg => {
          // handle +-++-- cancelling
          let current = Sign::from_op(op);
          while let Some(&IR1Expr::Op(OpKind::Add | OpKind::Neg)) = tokens.peek() {
            let Some(IR1Expr::Op(next_op)) = tokens.next() else {
              unreachable!()
            };
            current.combine(Sign::from_op(next_op));
          }

          match current {
            Sign::Positive => {
              // everything cancelled, no extra tokens need to be inserted
              extract_one(tokens)
            }
            Sign::Negative => {
              // add -1 * to output
              Ok(Extract1Pair::ExprOp {
                left: tokens,
                expr: IR2ExprFlat::Neg1,
                op: OpKind::Mul,
              })
            }
          }
        }
        // these can't act as unary
        OpKind::Mul | OpKind::Div | OpKind::Exp => return Err(Error::CantListOperators),
      },
    }
  }

  impl IR2Exprs {
    pub fn from_ir1(tokens: impl IntoIterator<Item = IR1Expr>) -> Result<IR2Exprs, Error> {
      let mut tokens = tokens.into_iter().peekable();
      let mut pairs: Vec<(IR2ExprFlat, OpKind)> = Vec::new();

      loop {
        let res = extract_one(tokens)?;
        match res {
          Extract1Pair::ExprOp { left, expr, op } => {
            tokens = left;
            pairs.push((expr, op));
          }
          Extract1Pair::FinalExpr(last) => {
            // WHAAT how does the compiler know this
            // will always diverge? Is it genius?
            return Ok(IR2Exprs { ops: pairs, last });
          }
        }
      }
    }
  }
}
