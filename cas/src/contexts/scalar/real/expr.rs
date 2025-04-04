use bevy_ecs::component::Component;
use num::BigUint;

use super::Ident;

pub struct Exprs<Ident>(pub Vec<Expr<Ident>>);

impl<Ident> IntoIterator for Exprs<Ident> {
  type Item = Expr<Ident>;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

pub enum Expr<Ident> {
  Constant(ConstantNum),
  Ident(Ident),
  Unary(UnaryOp<Ident>),
  Binary(BinaryOp<Ident>),
}

#[derive(Clone)]
pub enum ConstantNum {
  Positive(BigUint),
  Pi,
  // Negative(BigUint),
  // Ratio {
  //   num: BigUint,
  //   denom: BigUint
  // }
}

impl<Var> From<ConstantNum> for Expr<Var> {
  fn from(value: ConstantNum) -> Self {
    Expr::Constant(value)
  }
}

impl<Var> From<BigUint> for Expr<Var> {
  fn from(value: BigUint) -> Self {
    Expr::Constant(ConstantNum::Positive(value))
  }
}

impl From<Ident> for Expr<Ident> {
  fn from(value: Ident) -> Self {
    Expr::Ident(value)
  }
}

impl<Var> From<BinaryOp<Var>> for Expr<Var> {
  fn from(value: BinaryOp<Var>) -> Self {
    Expr::Binary(value)
  }
}

pub enum UnaryOp<Var> {
  Neg(Box<Expr<Var>>),
}

pub enum BinaryOp<Var> {
  Add {
    lhs: Box<Expr<Var>>,
    rhs: Box<Expr<Var>>,
  },
  Mul {
    lhs: Box<Expr<Var>>,
    rhs: Box<Expr<Var>>,
  },
  Div {
    numerator: Box<Expr<Var>>,
    denominator: Box<Expr<Var>>,
  },
  Exp {
    base: Box<Expr<Var>>,
    exponent: Box<Expr<Var>>,
  },
}

#[derive(Component)]
pub struct Equation<Var> {
  lhs: Expr<Var>,
  rhs: Expr<Var>,
}
