/// Both binary and unary
#[derive(Debug)]
enum OpKind {
  Neg,
  Add,
  Mul,
  Div,
  Exp,
}

pub use ir1::IR1Expr;
pub use ir2::IR2Exprs;

mod ir1;
mod ir2;
