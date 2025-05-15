/// Both binary and unary
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpKind {
	Neg,
	Add,
	Mul,
	Div,
	Exp,
}

pub use ir1::IR1Expr;
pub use ir2::IR2Exprs;
pub use ir3::IR3Expr;

mod ir1;
mod ir2;
mod ir3;
mod from_ir2 {
	//! non trivial

	use latex_parser::Ident;

	use crate::{
		contexts::scalar::real::{ContextOneVarEq, Error, expr::Exprs},
		prelude::*,
	};

	use super::{IR2Exprs, IR3Expr};

	impl Exprs<Ident> {
		fn from_ir3(context: &ContextOneVarEq<Ident>, irr: IR3Expr<Ident>) -> Result<Self, Error> {
			todo!()
		}
	}
}
