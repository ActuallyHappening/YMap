//! Simplifies expressions and solves equations
//! in the context of real scalar numbers,
//! e.g. 2y=3 => y=3/2
use std::{collections::HashMap, hash::Hash};

use crate::prelude::*;
use bevy_ecs::{entity::Entity, world::World};
use num::bigint::BigUint;

pub type Identifier = latex_parser::Identifier;

pub struct RealScalarContext {
  context: Context<Identifier>,
  world: World,
  start: Entity,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(
    "You must define all your variables and constants before using them, this one wasn't defined: {0}"
  )]
  UndefinedIdent(Identifier),

  #[error("Couldn't parse your math: {0}")]
  ParseLatex(#[from] latex_parser::Error),
}

impl RealScalarContext {
  pub fn from_latext_eq(latex: String) -> Result<Self, Error> {
    let latex = latex_parser::LatexTokens::parse_from_latex(&latex)?;
    
    let variables = latex.0.iter().filter(|t| matches!(t, latex_parser::LatexToken::Identifier()))
    
    todo!()
  }
}

pub struct Context<Var> {
  solve_for: Var,
  constants: HashMap<Var, ConstantExpr>,
}

#[derive(Clone)]
pub enum VariableStatus {
  SolveFor,
  Constant(ConstantExpr),
}

impl<Var> Context<Var>
where
  Var: PartialEq + Eq + Hash,
{
  pub fn lookup_ident(&self, ident: Var) -> Option<VariableStatus> {
    if ident == self.solve_for {
      return Some(VariableStatus::SolveFor);
    } else {
      self
        .constants
        .get(&ident)
        .cloned()
        .map(|expr| VariableStatus::Constant(expr))
    }
  }
}

/// Lowest atom
pub enum Num {
  /// NB: not zero!
  NonZero(BigUint),
  Zero,
}

#[derive(Clone)]
pub enum ConstantExpr {
  Positive(BigUint),
  Pi,
  // Negative(BigUint),
  // Ratio {
  //   num: BigUint,
  //   denom: BigUint
  // }
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
