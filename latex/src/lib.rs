#[allow(unused_imports)]
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

#[derive(Debug)]
pub struct LatexTokens(Vec<LatexToken>);

impl LatexTokens {
  pub fn parse_from_latex(latex: &str) -> Result<Self, Error> {
    parse_latex(latex).map(LatexTokens)
  }

  pub fn visit<T>(&self, visitor: &mut T)
  where
    T: TokenVisitor,
  {
    for token in &self.0 {
      token.visit(visitor);
    }
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

impl IntoIterator for LatexTokens {
  type Item = LatexToken;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LatexToken {
  Num(BigUint),
  Neg,
  Mul,
  Add,
  Eq,
  Exp,
  Ident(Ident),
  Bracketed(Bracketed),
  Frac(Frac),
}

impl From<BigUint> for LatexToken {
  fn from(num: BigUint) -> Self {
    LatexToken::Num(num)
  }
}

/// IDK why this needs to be sized btw
pub trait TokenVisitor: Sized {
  fn visit_num(&mut self, num: &BigUint) {
    _ = num;
  }
  fn visit_neg(&mut self) {}
  fn visit_mul(&mut self) {}
  fn visit_add(&mut self) {}
  fn visit_exp(&mut self) {}
  fn visit_eq(&mut self) {}
  fn visit_ident(&mut self, ident: &Ident) {
    _ = ident;
  }
  /// Default visits inner tokens
  fn visit_bracketed(&mut self, bracketed: &Bracketed) {
    for token in &bracketed.inner {
      token.visit(self);
    }
  }
  /// Default visits numerator and denominator
  fn visit_frac(&mut self, frac: &Frac) {
    for token in &frac.numerator {
      token.visit(self);
    }
    for token in &frac.denominator {
      token.visit(self);
    }
  }
}

impl LatexToken {
  pub fn visit<T>(&self, visitor: &mut T)
  where
    T: TokenVisitor,
  {
    match self {
      LatexToken::Neg => visitor.visit_neg(),
      LatexToken::Num(num) => visitor.visit_num(num),
      LatexToken::Mul => visitor.visit_mul(),
      LatexToken::Add => visitor.visit_add(),
      LatexToken::Exp => visitor.visit_exp(),
      LatexToken::Eq => visitor.visit_eq(),
      LatexToken::Ident(ident) => visitor.visit_ident(ident),
      LatexToken::Bracketed(bracketed) => visitor.visit_bracketed(bracketed),
      LatexToken::Frac(frac) => visitor.visit_frac(frac),
    }
  }
}

/// A symbol
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ident {
  Pi,
  AlphabeticChar(char),
}

impl std::fmt::Display for Ident {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Ident::Pi => write!(f, "π"),
      Ident::AlphabeticChar(char) => write!(f, "{}", char),
    }
  }
}

impl From<Ident> for LatexToken {
  fn from(identifier: Ident) -> Self {
    LatexToken::Ident(identifier)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bracket {
  Round,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frac {
  pub numerator: Vec<LatexToken>,
  pub denominator: Vec<LatexToken>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bracketed {
  pub bracket: Bracket,
  pub inner: Vec<LatexToken>,
}

impl From<Bracketed> for LatexToken {
  fn from(bracketed: Bracketed) -> Self {
    LatexToken::Bracketed(bracketed)
  }
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

  #[cfg(test)]
  fn assert_parsing_errors<T>(res: Result<(&str, T), VerboseError<&str>>, input: &str) -> T {
    match Error::handle_parsing_errors(res, input) {
      Ok(tokens) => tokens,
      Err(err) => panic!("Parsing error: {}", err),
    }
  }
}

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
  many1(alt((
    neg, num, mul, add, exp, eq, pi, identifier, brackets, frac,
  )))
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
      LatexToken::Ident(Ident::AlphabeticChar('x')),
      LatexToken::Ident(Ident::AlphabeticChar('y')),
      LatexToken::Ident(Ident::AlphabeticChar('z')),
      LatexToken::Neg,
      LatexToken::Num(BigUint::from(5u32))
    ]
  );
}

fn neg(input: &str) -> IResult<&str, LatexToken> {
  map(preceded(multispace0, tag("-")), |_str| LatexToken::Neg).parse(input)
}

fn add(input: &str) -> IResult<&str, LatexToken> {
  map(preceded(multispace0, tag("+")), |_str| LatexToken::Add).parse(input)
}

fn exp(input: &str) -> IResult<&str, LatexToken> {
  map(preceded(multispace0, tag("^")), |_str| LatexToken::Exp).parse(input)
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

fn identifier(input: &str) -> IResult<&str, LatexToken> {
  preceded(multispace0, alt((pi, alphanumeric_ident))).parse(input)
}

fn pi(input: &str) -> IResult<&str, LatexToken> {
  map(preceded(multispace0, tag(r"\pi")), |_str| {
    LatexToken::Ident(Ident::Pi)
  })
  .parse(input)
}

/// This is a bit manual, is there a better way?
fn alphanumeric_ident<'i, E>(input: &'i str) -> IResult<&'i str, LatexToken, E>
where
  E: ParseError<&'i str>,
{
  let char = input
    .chars()
    .next()
    .ok_or(E::from_error_kind(input, nom::error::ErrorKind::Char))
    .map_err(|err| nom::Err::Error(err))?;
  if char.is_alphabetic() {
    return Ok((&input[1..], LatexToken::Ident(Ident::AlphabeticChar(char))));
  } else {
    Err(nom::Err::Error(E::from_error_kind(
      input,
      nom::error::ErrorKind::Char,
    )))
  }
}

#[test]
fn latex_identifiers() {
  let input = r"   x y   z\pi";
  let tokens = Error::assert_parsing_errors(many1(identifier).parse(input).finish(), input);
  assert_eq!(
    tokens,
    vec![
      Ident::AlphabeticChar('x').into(),
      Ident::AlphabeticChar('y').into(),
      Ident::AlphabeticChar('z').into(),
      Ident::Pi.into()
    ]
  );
}

fn brackets(input: &str) -> IResult<&str, LatexToken> {
  delimited(ws(tag(r"\left(")), tokens, ws(tag(r"\right)")))
    .map(|tokens| {
      LatexToken::Bracketed(Bracketed {
        bracket: Bracket::Round,
        inner: tokens,
      })
    })
    .parse(input)
}

#[test]
fn latex_brackets() {
  let input = r"\left( 5 \cdot 7 \right)";
  let tokens = Error::assert_parsing_errors(tokens.parse(input).finish(), input);
  assert_eq!(
    tokens,
    vec![
      Bracketed {
        bracket: Bracket::Round,
        inner: vec![
          LatexToken::Num(5u32.into()),
          LatexToken::Mul,
          LatexToken::Num(7u32.into())
        ]
      }
      .into()
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
        Ident::AlphabeticChar('x').into(),
        Ident::AlphabeticChar('y').into()
      ],
      denominator: vec![LatexToken::Num(3u32.into()), Ident::Pi.into()],
    })
  );
}
