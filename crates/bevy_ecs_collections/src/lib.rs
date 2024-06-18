pub mod prelude {
	pub(crate) use bevy::prelude::*;

	pub use crate::plugin::*;
	pub use crate::traits::*;
}

mod plugin {
	use crate::prelude::*;

	/// Convenience [Plugin] that sets up the necessary systems
	pub struct ECSCollectionPlugin;
}

mod traits {
	use crate::prelude::*;

	pub trait ECSCollection {
		type Item: std::hash::Hash + PartialEq;
	}

	/// e.g. [Vec]
	impl<Iter> ECSCollection for Iter
	where
		Iter: Iterator,
		Iter::Item: std::hash::Hash + PartialEq,
	{
		type Item = Iter::Item;
	}

	pub trait ECSExpandable {
		
	}
}
