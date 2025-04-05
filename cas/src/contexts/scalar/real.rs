//! Simplifies expressions and solves equations
//! in the context of real scalar numbers,
//! e.g. 2y=3 => y=3/2
use crate::prelude::*;
use bevy_ecs::{entity::Entity, world::World};
use num::bigint::BigUint;

pub struct RealScalarContext {
  world: World,
  start: Entity,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {}

impl RealScalarContext {
  pub fn from_latext_eq(latex: String) -> Result<Self, Error> {
    todo!()
  }
}



/// Lowest atom
pub enum Num {
  /// NB: not zero!
  NonZero(BigUint),
  Zero,
}

pub enum Expr<Var> {
  Lit(BigUint),
  Var(Var),
  Unary(UnaryOp<Var>),
  Binary(BinaryOp<Var>),
}

pub struct UnaryOp<Var> {
  op: UnaryOperator,
  operand: Box<Expr<Var>>,
}

pub enum UnaryOperator {
  Neg,
}

pub struct BinaryOp<Var> {
  lhs: Box<Expr<Var>>,
  op: BinaryOperator,
  rhs: Box<Expr<Var>>,
}

pub enum BinaryOperator {
  Add,
  Mul,
  Frac,
}

pub struct Equation<Var> {
  lhs: Expr<Var>,
  rhs: Expr<Var>,
}

pub mod pass {
  use crate::prelude::*;

  pub mod eqs {
    use crate::prelude::*;

    pub fn single_mul_divide_to_solve() {}
  }
}
