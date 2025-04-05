//! Simplifies expressions and solves equations
//! in the context of real scalar numbers,
//! e.g. 2y=3 => y=3/2
use std::{
  collections::{HashMap, HashSet},
  hash::Hash,
};

use crate::prelude::*;
use bevy_ecs::{bundle::Bundle, component::Component, entity::Entity, world::World};
use expr::{ConstantNum, Equation};
use latex_parser::{Bracketed, Frac, Ident, LatexToken, LatexTokens};
use num::bigint::BigUint;

pub struct RealScalarStorage {
  context: ContextOneVarEq<Ident>,
  world: World,
  start: Entity,
}

impl RealScalarStorage {
  pub fn from_latext_eq(latex: String) -> Result<Self, Error> {
    let latex = latex_parser::LatexTokens::parse_from_latex(&latex)?;

    let context = ContextOneVarEq::infer_variable(&latex);
    // let eq = Equation::from_tokens(&context, &latex);

    let mut world = World::new();

    // let first_line = world.spawn(Line {
    //   eq:
    //   is_eq: IsEquation,
    // });

    // RealScalarContext {
    //   solve_for,
    //   world,
    // }

    todo!()
  }
}

#[derive(Component)]
struct IsEquation;

#[derive(Bundle)]
struct Line {
  eq: Equation<Ident>,
  is_eq: IsEquation,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(
    "You must define all your variables and constants before using them, this one wasn't defined: {0}"
  )]
  UndefinedIdent(Ident),

  #[error("Couldn't parse your math: {0}")]
  ParseLatex(#[from] latex_parser::Error),

  #[error("This program only supports solving equations of one variable at the moment")]
  MultipleVariables(HashSet<Ident>),

  #[error("This program doesn't just simplify your expressions, make it an equation!")]
  NoVariables,

  #[error("Keep typing ...")]
  NoTokens,

  #[error("You can't just list multiple numbers, add them or something idk")]
  TwoNumbersTogether(BigUint, BigUint),

  #[error("You gotta write stuff around the = sign bro")]
  EmptyAroundEq,

  #[error("Why are you putting equals signs there? Don't nest them please!")]
  CantNestEq,
}

pub enum OneVariableEquation {
  NoVariables,
  Ok { solve_for: Ident },
  ErrMultipleVars(HashSet<Ident>),
}

impl latex_parser::TokenVisitor for OneVariableEquation {
  fn visit_ident(&mut self, ident: &latex_parser::Ident) {
    match self {
      OneVariableEquation::NoVariables => {
        *self = OneVariableEquation::Ok {
          solve_for: ident.clone(),
        }
      }
      OneVariableEquation::Ok { solve_for } => {
        if solve_for != ident {
          *self = OneVariableEquation::ErrMultipleVars({
            let mut set = HashSet::new();
            set.insert(solve_for.clone());
            set.insert(ident.clone());
            set
          })
        }
      }
      OneVariableEquation::ErrMultipleVars(vars) => {
        vars.insert(ident.clone());
      }
    }
  }
}

pub struct ContextOneVarEq<Var> {
  solve_for: Var,
  constants: HashMap<Var, ConstantNum>,
}

impl ContextOneVarEq<Ident> {
  pub fn infer_variable(tokens: &LatexTokens) -> Result<Self, Error> {
    let mut visitor = OneVariableEquation::NoVariables;
    tokens.visit(&mut visitor);
    let solve_for = match visitor {
      OneVariableEquation::NoVariables => Err(Error::NoVariables),
      OneVariableEquation::Ok { solve_for } => Ok(solve_for),
      OneVariableEquation::ErrMultipleVars(vars) => Err(Error::MultipleVariables(vars)),
    }?;
    Ok(Self {
      solve_for,
      constants: HashMap::new(),
    })
  }
}

#[derive(Clone)]
pub enum VariableStatus {
  SolveFor,
  Constant(ConstantNum),
}

impl<Var> ContextOneVarEq<Var>
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

pub use from_latex::*;

mod expr;
mod from_latex;

pub mod pass {
  use crate::prelude::*;

  pub mod eqs {
    use crate::prelude::*;

    pub fn single_mul_divide_to_solve() {}
  }
}
