pub mod app_tracing;

pub mod prelude {
	pub use color_eyre::eyre::WrapErr as _;
	pub use tracing::{debug, error, info, trace, warn};
}

use wgpu::{Backends, RequestAdapterOptions};

use crate::prelude::*;
pub async fn main() -> color_eyre::Result<()> {
	info!("Hello, world!");

	let descriptor = wgpu::InstanceDescriptor::default();
	let instance = wgpu::Instance::new(&descriptor);

	{
		for adapter in instance.enumerate_adapters(Backends::PRIMARY) {
			let adapter = adapter.get_info();
			info!(
				adapter.name,
				adapter.driver, adapter.driver_info, "Scanned an adapter (e.g. native GPU & library)"
			);
		}
	}
	let adapter = {
		let request_options = RequestAdapterOptions {
			power_preference: wgpu::PowerPreference::None,
			force_fallback_adapter: false,
			compatible_surface: None,
		};
		let adapter = instance.request_adapter(&request_options).await?;
		{
			let adapter = adapter.get_info();
			info!(
				adapter.name,
				adapter.driver, adapter.driver_info, "Using this adapter (e.g. native GPU & library)"
			);
		}
	};

	// let target = todo!();
	// let surface = instance
	// 	.create_surface(target)
	// 	.wrap_err("Couldn't create surface")?;

	Ok(())
}
