use crate::prelude::*;

mod latex {
  use std::u128;

  use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0},
    combinator::{all_consuming, map},
    multi::{many0, many1},
    sequence::{delimited, preceded},
  };
  use num::BigUint;

  use crate::prelude::*;

  #[derive(Debug, Clone, PartialEq, Eq)]
  pub enum LatexToken {
    Neg,
    Num(BigUint),
    Mul,
    Eq,
    Pi,
    Var(char),
    Frac(Frac),
  }

  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct Frac {
    pub numerator: Vec<LatexToken>,
    pub denominator: Vec<LatexToken>,
  }

  #[derive(thiserror::Error, Debug)]
  pub enum Error {
    #[error("Parsing error: {display}")]
    ParsingError { debug: String, display: String },
  }

  impl From<nom::error::Error<&str>> for Error {
    fn from(err: nom::error::Error<&str>) -> Self {
      Error::ParsingError {
        debug: format!("{:?}", err),
        display: err.to_string(),
      }
    }
  }

  pub fn parse_latex(input: &str) -> Result<Vec<LatexToken>, Error> {
    let (leftover, tokens) = all_consuming(tokens).parse(input).finish()?;
    assert_eq!(leftover.len(), 0);
    Ok(tokens)
  }

  fn tokens(input: &str) -> IResult<&str, Vec<LatexToken>> {
    map(
      many1(alt((
        map(neg, |t| vec![t]),
        map(num, |t| vec![t]),
        map(mul, |t| vec![t]),
        map(eq, |t| vec![t]),
        map(pi, |t| vec![t]),
        vars,
        map(frac, |t| vec![t]),
      ))),
      |vec_of_vecs| vec_of_vecs.into_iter().flatten().collect(),
    )
    .parse(input)
  }

  fn neg(input: &str) -> IResult<&str, LatexToken> {
    map(preceded(multispace0, tag("-")), |_str| LatexToken::Neg).parse(input)
  }

  #[test]
  fn latex_neg() {
    let (_remaining, t) = all_consuming(neg).parse("  \t -").unwrap();
    assert_eq!(t, LatexToken::Neg);
  }

  fn num(input: &str) -> IResult<&str, LatexToken> {
    map(preceded(multispace0, digit1), |str: &str| {
      LatexToken::Num(str.parse().expect("BigUint to parse from only digits"))
    })
    .parse(input)
  }

  #[test]
  fn latex_num() {
    let bigint = {
      // way bigger than u128 can hold!
      let a = u128::MAX;
      let b = u128::MAX / 2;
      BigUint::from(a) + BigUint::from(b)
    };

    let (_remaining, t) = all_consuming(num)
      .parse("  \t 510423550381407695195061911147652317182")
      .unwrap();
    assert_eq!(t, LatexToken::Num(bigint));
  }

  fn mul(input: &str) -> IResult<&str, LatexToken> {
    map(
      preceded(multispace0, alt((tag(r#"\cdot"#), tag(r#"\ast"#)))),
      |_str| LatexToken::Mul,
    )
    .parse(input)
  }

  #[test]
  fn latex_mul() {
    let (_remaining, t) = all_consuming(mul).parse(r"      \cdot").unwrap();
    assert_eq!(t, LatexToken::Mul);
  }

  fn eq(input: &str) -> IResult<&str, LatexToken> {
    map(preceded(multispace0, tag("=")), |_str| LatexToken::Eq).parse(input)
  }

  fn pi(input: &str) -> IResult<&str, LatexToken> {
    map(preceded(multispace0, tag(r"\pi")), |_str| LatexToken::Pi).parse(input)
  }

  fn vars(input: &str) -> IResult<&str, Vec<LatexToken>> {
    map(preceded(multispace0, alpha1), |chars: &str| {
      chars.chars().map(|char| LatexToken::Var(char)).collect()
    })
    .parse(input)
  }

  #[test]
  fn latex_vars() {
    let (_remaining, tokens) = all_consuming(vars).parse(r"   x y z").unwrap();
    assert_eq!(
      tokens,
      vec![
        LatexToken::Var('x'),
        LatexToken::Var('y'),
        LatexToken::Var('z')
      ]
    );
  }

  fn frac(input: &str) -> IResult<&str, LatexToken> {
    map(
      (
        delimited(multispace0, tag(r"\frac"), multispace0),
        delimited(multispace0, tokens, multispace0),
        delimited(multispace0, tokens, multispace0),
      ),
      |(_frac, numerator, denominator)| {
        LatexToken::Frac(Frac {
          numerator,
          denominator,
        })
      },
    )
    .parse(input)
  }
}

pub mod scalar {
  use crate::prelude::*;

  pub mod real {
    //! Simplifies expressions and solves equations
    //! in the context of real scalar numbers,
    //! e.g. 2y=3 => y=3/2
    use crate::{contexts::latex::LatexToken, prelude::*};
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

    // impl TryFrom<LatexToken> for Equation<>

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
  }
}
