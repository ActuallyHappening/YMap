pub mod prelude {
	pub(crate) use bevy::prelude::*;

	pub use crate::bevy_utils::*;
}

mod bevy_utils {
	use crate::prelude::*;

	/// Shortcut for accessing [Mesh] and [StandardMaterial] [Assets],
	/// and the [AssetServer].
	///
	/// See also MM
	#[allow(clippy::upper_case_acronyms)]
	#[derive(bevy::ecs::system::SystemParam)]
	pub struct MMA<'w> {
		pub meshs: ResMut<'w, Assets<Mesh>>,
		pub mats: ResMut<'w, Assets<StandardMaterial>>,
		pub ass: Res<'w, AssetServer>,
	}

	impl<'w> MMA<'w> {
		/// Converts the [SystemParam](bevy::ecs::system::SystemParam) [MMA] into the mutable reference [MMR].
		pub fn reborrow<'a>(&'a mut self) -> MMAR<'a>
		where
			'w: 'a,
		{
			MMAR {
				meshs: self.meshs.reborrow(),
				mats: self.mats.reborrow(),
			}
		}
	}

	/// Shortcut for accessing [Mesh] and [StandardMaterial] [Assets] as a [SystemParam](bevy::ecs::system::SystemParam)
	///
	/// See also [MMA]
	#[allow(clippy::upper_case_acronyms)]
	#[derive(bevy::ecs::system::SystemParam)]
	pub struct MM<'w> {
		pub meshs: ResMut<'w, Assets<Mesh>>,
		pub mats: ResMut<'w, Assets<StandardMaterial>>,
	}

	/// Mutable reference type, useful for extracted functions
	#[allow(clippy::upper_case_acronyms)]
	pub struct MMAR<'w> {
		pub meshs: Mut<'w, Assets<Mesh>>,
		pub mats: Mut<'w, Assets<StandardMaterial>>,
	}

	impl<'w> MMAR<'w> {
		/// Useful in converting `&mut MMR` into [MMR]
		pub fn reborrow<'a>(&'a mut self) -> MMAR<'a>
		where
			'w: 'a,
		{
			MMAR {
				meshs: self.meshs.reborrow(),
				mats: self.mats.reborrow(),
			}
		}
	}
}
