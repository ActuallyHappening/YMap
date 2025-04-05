use std::iter::Peekable;

use crate::{contexts::scalar::real::Error, prelude::*};

use latex_parser::Ident;

use super::{
  OpKind,
  ir1::{IR1Expr, IR1Flat},
};

/// Cancels unary [`OpKind::Neg`] and [`OpKind::Add`]
/// into -1 * ... or ...,
/// implicitely multiplies between expressions
#[derive(Debug, PartialEq)]
pub struct IR2Exprs {
  pub first: IR2Flat,
  pub pairs: Vec<(OpKind, IR2Flat)>,
}

#[derive(Debug, PartialEq)]
pub enum IR2Flat {
  Neg1,
  Num(BigUint),
  Ident(Ident),
  Bracketed(Box<IR2Exprs>),
}

impl IR2Flat {
  fn from_ir1_expr_flat(flat_expr: IR1Flat) -> Result<IR2Flat, Error> {
    match flat_expr {
      IR1Flat::Num(num) => Ok(IR2Flat::Num(num)),
      IR1Flat::Ident(ident) => Ok(IR2Flat::Ident(ident)),
      IR1Flat::Bracketed(exprs) => Ok(IR2Flat::Bracketed(Box::new(IR2Exprs::from_ir1(exprs)?))),
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

#[derive(Debug, PartialEq)]
enum ResolvedExpr {
  Single(IR2Flat),
  Negated(IR2Flat, OpKind, IR2Flat),
}

impl From<IR2Flat> for ResolvedExpr {
  fn from(value: IR2Flat) -> Self {
    ResolvedExpr::Single(value)
  }
}

/// Handles +-++-- cancellation.
/// Assumes no previous tokens, or an operator as the previous token.
fn resolve_expr<I>(tokens: &mut Peekable<I>) -> Result<ResolvedExpr, Error>
where
  I: Iterator<Item = IR1Expr>,
{
  let first = tokens.next().ok_or(Error::NoTokens)?;
  match first {
    IR1Expr::Expr(expr) => Ok(ResolvedExpr::Single(
      IR2Flat::from_ir1_expr_flat(expr)?.into(),
    )),
    // since this is the first, must be an unary
    IR1Expr::Op(op) => match op {
      // these can act as unary
      OpKind::Add | OpKind::Neg => {
        // handle +-++-- cancelling
        let mut current = Sign::from_op(op);
        while let Some(&IR1Expr::Op(OpKind::Add | OpKind::Neg)) = tokens.peek() {
          let Some(IR1Expr::Op(next_op)) = tokens.next() else {
            unreachable!()
          };
          current = current.combine(Sign::from_op(next_op));
        }

        // must be basic expr next
        let IR1Expr::Expr(flat) = tokens.next().ok_or(Error::NoTokens)? else {
          return Err(Error::CantListOperators);
        };
        let flat = IR2Flat::from_ir1_expr_flat(flat)?;
        match current {
          Sign::Positive => Ok(ResolvedExpr::Single(flat)),
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

#[test]
fn cancelling_add_neg() {
  let input: Vec<IR1Expr> = vec![
    OpKind::Add.into(),
    OpKind::Neg.into(),
    OpKind::Neg.into(),
    OpKind::Add.into(),
    IR1Flat::Ident(Ident::Pi).into(),
  ];
  let mut input = input.into_iter().peekable();
  let resolved = resolve_expr(&mut input).unwrap();
  assert_eq!(resolved, ResolvedExpr::Single(IR2Flat::Ident(Ident::Pi)));

  let input: Vec<IR1Expr> = vec![
    OpKind::Neg.into(),
    OpKind::Neg.into(),
    OpKind::Neg.into(),
    OpKind::Add.into(),
    IR1Flat::Ident(Ident::Pi).into(),
  ];
  let mut input = input.into_iter().peekable();
  let resolved = resolve_expr(&mut input).unwrap();
  assert_eq!(
    resolved,
    ResolvedExpr::Negated(IR2Flat::Neg1, OpKind::Mul, IR2Flat::Ident(Ident::Pi))
  );
}

/// Handles implicit expression multiplication,
/// assumes the previous token was an expression.
///
/// Recursive
fn resolve_op<I>(tokens: &mut I, pairs: &mut Vec<(OpKind, IR2Flat)>) -> Result<OpKind, Error>
where
  I: Iterator<Item = IR1Expr>,
{
  match tokens.next().ok_or(Error::NoTokens)? {
    IR1Expr::Op(op) => Ok(op),
    IR1Expr::Expr(flat) => {
      let flat = IR2Flat::from_ir1_expr_flat(flat)?;
      pairs.push((OpKind::Mul, flat));
      resolve_op(tokens, pairs)
    }
  }
}

impl IR2Exprs {
  pub fn from_ir1(tokens: impl IntoIterator<Item = IR1Expr>) -> Result<IR2Exprs, Error> {
    let mut tokens = tokens.into_iter().peekable();

    // Error::NoTokens if not a first
    let resolved_expr = resolve_expr(&mut tokens)?;

    let mut pairs: Vec<(OpKind, IR2Flat)> = Vec::new();

    let first = match resolved_expr {
      ResolvedExpr::Single(expr) => expr,
      ResolvedExpr::Negated(first, op, second) => {
        pairs.push((op, second));
        first
      }
    };

    // checks
    while tokens.peek().is_some() {
      // op
      let op = resolve_op(&mut tokens, &mut pairs)?;

      if tokens.peek().is_none() {
        continue;
      }

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
