use std::iter::Peekable;

use crate::{contexts::scalar::real::Error, prelude::*};

use latex_parser::Ident;

use super::{
  OpKind,
  ir1::{IR1Expr, IR1ExprFlat},
};

/// Cancels unary [`OpKind::Neg`] and [`OpKind::Add`]
/// into -1 * ... or ...,
/// implicitely multiplies between expressions
#[derive(Debug)]
pub struct IR2Exprs {
  pub first: IR2Flat,
  pub pairs: Vec<(OpKind, IR2Flat)>,
}

#[derive(Debug)]
pub enum IR2Flat {
  Neg1,
  Num(BigUint),
  Ident(Ident),
  Bracketed(Box<IR2Exprs>),
}

impl IR2Flat {
  fn from_ir1_expr_flat(flat_expr: IR1ExprFlat) -> Result<IR2Flat, Error> {
    match flat_expr {
      IR1ExprFlat::Num(num) => Ok(IR2Flat::Num(num)),
      IR1ExprFlat::Ident(ident) => Ok(IR2Flat::Ident(ident)),
      IR1ExprFlat::Bracketed(exprs) => Ok(IR2Flat::Bracketed(Box::new(IR2Exprs::from_ir1(exprs)?))),
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

enum ResolvedExpr {
  Single(IR2Flat),
  Negated(IR2Flat, OpKind, IR2Flat),
}

impl From<IR2Flat> for ResolvedExpr {
  fn from(value: IR2Flat) -> Self {
    ResolvedExpr::Single(value)
  }
}

/// Handles +-++-- cancellation
fn resolve_expr<I>(tokens: &mut Peekable<I>) -> Result<ResolvedExpr, Error>
where
  I: Iterator<Item = IR1Expr>,
{
  let first = tokens.next().ok_or(Error::NoTokens)?;
  match first {
    IR1Expr::Expr(expr) => Ok(IR2Flat::from_ir1_expr_flat(expr)?.into()),
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

        // must be basic expr next
        let IR1Expr::Expr(flat) = tokens.next().ok_or(Error::NoTokens)? else {
          return Err(Error::CantListOperators);
        };
        let flat = IR2Flat::from_ir1_expr_flat(flat)?;
        match current {
          Sign::Positive => Ok(flat.into()),
          Sign::Negative => {
            // add -1 * to output
            Ok(ResolvedExpr::Negated(IR2Flat::Neg1, OpKind::Mul, flat))
          }
        }
      }
      // these can't act as unary
      OpKind::Mul | OpKind::Div | OpKind::Exp => return Err(Error::CantListOperators),
    },
  }
}

enum ResolvedOp {
  Op(OpKind),
  ImplicitMultiplication(OpKind, IR2Flat, Box<ResolvedOp>),
}

/// Handles implicit expression multiplication,
/// assumes the previous token was an expression.
///
/// Recursive
fn resolve_op<I>(tokens: &mut I) -> Result<ResolvedOp, Error>
where
  I: Iterator<Item = IR1Expr>,
{
  match tokens.next().ok_or(Error::NoTokens)? {
    IR1Expr::Op(op) => Ok(ResolvedOp::Op(op)),
    IR1Expr::Expr(flat) => {
      let flat = IR2Flat::from_ir1_expr_flat(flat)?;
      Ok(ResolvedOp::ImplicitMultiplication(
        OpKind::Mul,
        flat,
        Box::new(resolve_op(tokens)?),
      ))
    }
  }
}

impl IR2Exprs {
  pub fn from_ir1(tokens: impl IntoIterator<Item = IR1Expr>) -> Result<IR2Exprs, Error> {
    let mut tokens = tokens.into_iter().peekable();
    let mut pairs: Vec<(OpKind, IR2Flat)> = Vec::new();

    // Error::NoTokens if not a first
    let first = resolve_expr(&mut tokens)?;
    let first = match first {
      ResolvedExpr::Single(flat) => flat,
      ResolvedExpr::Negated(one, op, two) => {
        pairs.push((op, two));
        one
      }
    };

    // checks
    while tokens.peek().is_some() {
      // op
      let resolved_op = resolve_op(&mut tokens)?;
      fn resolve_op_closure(pairs: &mut Vec<(OpKind, IR2Flat)>, resolved_op: ResolvedOp) -> OpKind {
        match resolved_op {
          ResolvedOp::Op(op) => op,
          ResolvedOp::ImplicitMultiplication(op, flat, next) => {
            pairs.push((op, flat));
            resolve_op_closure(pairs, *next)
          }
        }
      }
      let op = resolve_op_closure(&mut pairs, resolved_op);

      // expr
      let expr = resolve_expr(&mut tokens)?;
      let expr = match expr {
        ResolvedExpr::Single(flat) => flat,
        ResolvedExpr::Negated(one, op, two) => {
          pairs.push((op, two));
          one
        }
      };

      pairs.push((op, expr));
    }

    Ok(IR2Exprs { first, pairs })
  }
}
