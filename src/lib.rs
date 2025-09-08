pub mod app_tracing;

pub mod prelude {
	pub use color_eyre::eyre::{eyre, WrapErr as _};
	pub use tracing::{debug, error, info, trace, warn};
}

pub mod windowing;

use wgpu::{Backends, RequestAdapterOptions};
use winit::event_loop::{ControlFlow, EventLoop};

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

	let event_loop = EventLoop::new().wrap_err("Couldn't create event loop")?;

	// ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
	// dispatched any events. This is ideal for games and similar applications.
	event_loop.set_control_flow(ControlFlow::Poll);

	// // ControlFlow::Wait pauses the event loop if no events are available to process.
	// // This is ideal for non-game applications that only update in response to user
	// // input, and uses significantly less power/CPU time than ControlFlow::Poll.
	// event_loop.set_control_flow(ControlFlow::Wait);

	let mut app = windowing::App::default();
	event_loop.run_app(&mut app).wrap_err("Event loop errored")?;

	// let target = todo!();
	// let surface = instance
	// 	.create_surface(target)
	// 	.wrap_err("Couldn't create surface")?;

	Ok(())
}
