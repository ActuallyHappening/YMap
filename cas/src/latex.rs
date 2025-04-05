use std::u128;

use nom::{
  Finish, Parser,
  branch::alt,
  bytes::{
    complete::{tag, take_while1},
    take_while,
  },
  character::complete::{alpha1, anychar, digit1, multispace0},
  combinator::{all_consuming, map},
  error::ParseError,
  multi::{many0, many1},
  sequence::{delimited, preceded},
};
use nom_language::error::VerboseError;
use num::BigUint;

use crate::prelude::*;

#[derive(Debug)]
pub struct LatexTokens(Vec<LatexToken>);

impl LatexTokens {
  pub fn parse_from_latex(latex: &str) -> Result<Self, Error> {
    parse_latex(latex).map(LatexTokens)
  }
}

/// Will error on invalid content at the end
pub fn parse_latex(input: &str) -> Result<Vec<LatexToken>, Error> {
  let res = all_consuming(ws(tokens)).parse(input).finish();
  let tokens = Error::handle_parsing_errors(res, input)?;
  Ok(tokens)
}

impl FromIterator<LatexToken> for LatexTokens {
  fn from_iter<T: IntoIterator<Item = LatexToken>>(iter: T) -> Self {
    LatexTokens(iter.into_iter().collect())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LatexToken {
  Neg,
  Num(BigUint),
  Mul,
  Eq,
  Pi,
  Frac(Frac),
  AlphabeticChar(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frac {
  pub numerator: Vec<LatexToken>,
  pub denominator: Vec<LatexToken>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Parsing error: {0}")]
  ParsingError(String),
}

pub type IResult<I, O, E = nom_language::error::VerboseError<I>> = Result<(I, O), nom::Err<E>>;

impl Error {
  fn handle_parsing_errors<T>(
    res: Result<(&str, T), VerboseError<&str>>,
    input: &str,
  ) -> Result<T, Error> {
    match res {
      Ok((leftover, tokens)) => {
        assert_eq!(leftover.len(), 0);
        Ok(tokens)
      }
      Err(err) => {
        let pretty_trace = nom_language::error::convert_error(input, err);
        Err(Error::ParsingError(pretty_trace))
      }
    }
  }

  fn assert_parsing_errors<T>(res: Result<(&str, T), VerboseError<&str>>, input: &str) -> T {
    match Error::handle_parsing_errors(res, input) {
      Ok(tokens) => tokens,
      Err(err) => panic!("Parsing error: {}", err),
    }
  }
}

// impl From<nom_language::error::VerboseError<&str>> for Error {
//   fn from(err: nom_language::error::VerboseError<&str>) -> Self {
//     Error::ParsingError {
//       debug: format!("{:?}", err),
//       display: err.to_string(),
//     }
//   }
// }

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
  F: Parser<&'a str, Output = O, Error = E>,
{
  delimited(multispace0, inner, multispace0)
}

/// May leave whitespace or invalid content at the end
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

#[test]
fn latex_tokens() {
  let input = "123 x y z -5";
  let tokens = Error::assert_parsing_errors(tokens(input).finish(), input);
  assert_eq!(
    tokens,
    vec![
      LatexToken::Num(BigUint::from(123u32)),
      LatexToken::AlphabeticChar('x'),
      LatexToken::AlphabeticChar('y'),
      LatexToken::AlphabeticChar('z'),
      LatexToken::Neg,
      LatexToken::Num(BigUint::from(5u32))
    ]
  );
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

fn is_free_char(char: char) -> bool {
  ('a'..='z').contains(&char) || ('A'..='Z').contains(&char)
}
fn vars(input: &str) -> IResult<&str, Vec<LatexToken>> {
  map(
    take_while1(|char: char| char.is_whitespace() || is_free_char(char)),
    |chars: &str| {
      chars
        .chars()
        .filter_map(|char| (!char.is_whitespace()).then(|| LatexToken::AlphabeticChar(char)))
        .collect()
    },
  )
  .parse(input)
}

#[test]
fn latex_vars() {
  let input = r"   x y   z";
  let tokens = Error::assert_parsing_errors(vars(input).finish(), input);
  assert_eq!(
    tokens,
    vec![
      LatexToken::AlphabeticChar('x'),
      LatexToken::AlphabeticChar('y'),
      LatexToken::AlphabeticChar('z')
    ]
  );
}

/// Will error on content in numerator or denominator
/// failing to parse
fn frac(input: &str) -> IResult<&str, LatexToken> {
  map(
    (
      ws(tag(r"\frac")),
      ws(delimited(tag("{"), ws(tokens), tag("}"))),
      ws(delimited(tag("{"), ws(tokens), tag("}"))),
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

#[test]
fn latex_frac() {
  let fraction = r"\frac{2xy}{ 3 \pi }";
  let res = Error::handle_parsing_errors(frac(fraction).finish(), fraction);
  let Ok(tokens) = res else {
    panic!("{}", res.unwrap_err());
  };

  assert_eq!(
    tokens,
    LatexToken::Frac(Frac {
      numerator: vec![
        LatexToken::Num(2u32.into()),
        LatexToken::AlphabeticChar('x'),
        LatexToken::AlphabeticChar('y')
      ],
      denominator: vec![LatexToken::Num(3u32.into()), LatexToken::Pi],
    })
  );
}
