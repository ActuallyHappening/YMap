use latex_parser::{Bracketed, Frac, Ident, LatexToken};
use num::BigUint;

use crate::contexts::scalar::real::Error;

use super::*;

/// Removes [`LatexToken::Eq`],
/// normalizes [`LatexToken::Frac`],
/// hasn't handled operator precedence yet
#[derive(Debug)]
pub enum IR1Expr {
  Op(OpKind),
  Expr(IR1ExprFlat),
}

#[derive(Debug)]
pub enum IR1ExprFlat {
  Num(BigUint),
  Ident(Ident),
  /// This has ultimate operator precedence
  Bracketed(Vec<IR1Expr>),
}

impl From<OpKind> for IR1Expr {
  fn from(op: OpKind) -> Self {
    IR1Expr::Op(op)
  }
}

impl From<IR1ExprFlat> for IR1Expr {
  fn from(token: IR1ExprFlat) -> Self {
    IR1Expr::Expr(token)
  }
}

impl IR1Expr {
  /// Calls [Self::from_latex_tokens] and errors if there is any eq nesting
  fn from_latex_not_nested(
    tokens: impl IntoIterator<Item = LatexToken>,
  ) -> Result<Vec<IR1Expr>, Error> {
    let inner = Self::from_latex_tokens(tokens)?;
    if inner.len() != 1 {
      return Err(Error::CantNestEq);
    }
    Ok(inner.into_iter().next().unwrap())
  }

  /// Split by [`LatexToken::Eq`].
  /// Therefore, len should be num of equals signs - 1.
  ///
  /// Assumes all brackets are equal
  pub fn from_latex_tokens(
    tokens: impl IntoIterator<Item = LatexToken>,
  ) -> Result<Vec<Vec<IR1Expr>>, Error> {
    // starts with something
    // which is the condition for .last_mut() to be unwrapped
    let mut ret: Vec<Vec<IR1Expr>> = Vec::new();
    ret.push(vec![]);

    for token in tokens {
      let current = ret.last_mut().unwrap();
      match token {
        LatexToken::Eq => {
          ret.push(Vec::new());
          continue;
        }
        LatexToken::Neg => current.push(OpKind::Neg.into()),
        LatexToken::Mul => current.push(OpKind::Mul.into()),
        LatexToken::Add => current.push(OpKind::Add.into()),
        LatexToken::Exp => current.push(OpKind::Exp.into()),
        LatexToken::Ident(ident) => current.push(IR1ExprFlat::Ident(ident).into()),
        LatexToken::Num(num) => current.push(IR1ExprFlat::Num(num).into()),
        LatexToken::Bracketed(Bracketed {
          bracket: latex_parser::Bracket::Round,
          inner,
        }) => {
          // transform only
          current.push(IR1ExprFlat::Bracketed(IR1Expr::from_latex_not_nested(inner)?).into());
        }
        LatexToken::Frac(Frac {
          numerator,
          denominator,
        }) => {
          current.push(IR1ExprFlat::Bracketed(IR1Expr::from_latex_not_nested(numerator)?).into());
          current.push(OpKind::Div.into());
          current.push(IR1ExprFlat::Bracketed(IR1Expr::from_latex_not_nested(denominator)?).into())
        }
      }
    }
    Ok(ret)
  }
}
