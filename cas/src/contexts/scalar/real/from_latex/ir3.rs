use std::iter::Peekable;

use latex_parser::Ident;

use crate::{contexts::scalar::real::Error, prelude::*};

use super::{IR2Exprs, OpKind, ir2::IR2Flat};

/// Takes into account operator precedence
#[derive(Debug)]
pub enum IR3Expr<Ident> {
	BinaryOp(IR3BinaryOp<Ident>),
	Flat(IR3Flat<Ident>),
}

impl From<IR3Flat<Ident>> for IR3Expr<Ident> {
	fn from(flat: IR3Flat<Ident>) -> Self {
		IR3Expr::Flat(flat)
	}
}

impl From<IR3BinaryOp<Ident>> for IR3Expr<Ident> {
	fn from(binary: IR3BinaryOp<Ident>) -> Self {
		IR3Expr::BinaryOp(binary)
	}
}

#[derive(Debug)]
pub enum IR3Flat<Ident> {
	Neg1,
	Num(BigUint),
	Ident(Ident),
	Bracket(Box<IR3Expr<Ident>>),
}

#[derive(Debug)]
pub enum IR3BinaryOp<Ident> {
	Add {
		lhs: Box<IR3Expr<Ident>>,
		rhs: Box<IR3Expr<Ident>>,
	},
	Mul {
		lhs: Box<IR3Expr<Ident>>,
		rhs: Box<IR3Expr<Ident>>,
	},
	Div {
		lhs: Box<IR3Expr<Ident>>,
		rhs: Box<IR3Expr<Ident>>,
	},
	Exp {
		base: Box<IR3Expr<Ident>>,
		exponent: Box<IR3Expr<Ident>>,
	},
}

impl IR3BinaryOp<Ident> {
	pub fn new(lhs: IR3Expr<Ident>, op: OpKind, rhs: IR3Expr<Ident>) -> Self {
		match op {
			OpKind::Add => IR3BinaryOp::Add {
				lhs: Box::new(lhs),
				rhs: Box::new(rhs),
			},
			OpKind::Neg => IR3BinaryOp::Add {
				lhs: Box::new(lhs),
				rhs: Box::new(
					IR3BinaryOp::Mul {
						lhs: Box::new(IR3Flat::Neg1.into()),
						rhs: Box::new(rhs),
					}
					.into(),
				),
			},
			OpKind::Mul => IR3BinaryOp::Mul {
				lhs: Box::new(lhs),
				rhs: Box::new(rhs),
			},
			OpKind::Div => IR3BinaryOp::Div {
				lhs: Box::new(lhs),
				rhs: Box::new(rhs),
			},
			OpKind::Exp => IR3BinaryOp::Exp {
				base: Box::new(lhs),
				exponent: Box::new(rhs),
			},
		}
	}
}

impl OpKind {
	fn precedence(self) -> NonZero<u8> {
		match self {
			Self::Add | Self::Neg => u8!(1),
			Self::Mul | Self::Div => u8!(2),
			Self::Exp => u8!(3),
		}
	}

	fn is_higher_or_eq_precedence(self, other: Self) -> bool {
		self.precedence() >= other.precedence()
	}
}

impl IR3Expr<Ident> {
	fn recursive_from_ir2(
		prev: IR3Expr<Ident>,
		op1: OpKind,
		current: IR3Flat<Ident>,
		mut next: Peekable<impl Iterator<Item = (OpKind, IR3Flat<Ident>)>>,
	) -> IR3Expr<Ident> {
		let Some((op2, next_expr)) = next.next() else {
			// base case, ended
			return IR3BinaryOp::new(prev, op1, current.into()).into();
		};

		if op1.is_higher_or_eq_precedence(op2) {
			let lhs = IR3BinaryOp::new(prev, op1, current.into()).into();
			IR3Expr::recursive_from_ir2(lhs, op2, next_expr, next)
		} else {
			let rhs = IR3Expr::recursive_from_ir2(current.into(), op2, next_expr, next);
			IR3BinaryOp::new(prev, op1, rhs).into()
		}
	}

	pub fn from_ir2(ir2: IR2Exprs) -> Self {
		let IR2Exprs { first, pairs } = ir2;
		let first = IR3Flat::from_ir2(first).into();
		let mut pairs = pairs
			.into_iter()
			.map(|(op, expr)| (op, IR3Flat::from_ir2(expr)))
			.peekable();

		let Some((op, expr)) = pairs.next() else {
			// base case, single expr
			return first;
		};

		IR3Expr::recursive_from_ir2(first, op, expr, pairs)
	}
}

impl IR3Flat<Ident> {
	pub fn from_ir2(ir2: IR2Flat) -> Self {
		match ir2 {
			IR2Flat::Neg1 => IR3Flat::Neg1,
			IR2Flat::Num(num) => IR3Flat::Num(num),
			IR2Flat::Ident(ident) => IR3Flat::Ident(ident),
			IR2Flat::Bracketed(exprs) => IR3Flat::Bracket(Box::new(IR3Expr::from_ir2(*exprs))),
		}
	}
}
