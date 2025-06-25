use std::any::TypeId;

use crate::{prelude::*, storage::GenericStorage};

pub enum AttrKey {
	IsIgnored,
	Storage,
}

pub enum Attr {
	IsIgnored,
	Storage(TypeId),
}
