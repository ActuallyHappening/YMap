pub mod app_tracing;

pub mod prelude {
	pub use color_eyre::eyre::{eyre, WrapErr as _};
	pub use tracing::{debug, error, info, trace, warn};
}

pub mod windowing;

use wgpu::{
	rwh::{HasDisplayHandle, HasRawDisplayHandle, HasWindowHandle},
	Adapter, Backends, Instance, RequestAdapterOptions, Surface, SurfaceTarget, WindowHandle,
};
use winit::{
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};

#[derive(Default)]
pub struct App {
	window: Option<Window>,
	fatal_error: Option<color_eyre::Report>,
	instance: Option<wgpu::Instance>,
	surface: Option<wgpu::Surface<'static>>,
}

impl App {
	pub fn instance(instance: &mut Option<wgpu::Instance>) -> color_eyre::Result<&mut Instance> {
		if instance.is_some() {
			return Ok(instance.as_mut().unwrap());
		}
		let descriptor = wgpu::InstanceDescriptor::default();
		let new_instance = wgpu::Instance::new(&descriptor);
		*instance = Some(new_instance);
		Ok(instance.as_mut().unwrap())
	}

	pub fn surface(&mut self) -> color_eyre::Result<&mut wgpu::Surface<'static>> {
		if self.surface.is_some() {
			return Ok(self.surface.as_mut().unwrap());
		}
		let instance = App::instance(&mut self.instance)?;
		let window = self.window.as_mut().ok_or(eyre!("No window yet"))?;
		{
			for adapter in instance.enumerate_adapters(Backends::PRIMARY) {
				let adapter = adapter.get_info();
				info!(
					adapter.name,
					adapter.driver, adapter.driver_info, "Scanned an adapter (e.g. native GPU & library)"
				);
			}
		}
		{
			// let request_options = RequestAdapterOptions {
			// 	power_preference: wgpu::PowerPreference::None,
			// 	force_fallback_adapter: false,
			// 	compatible_surface: None,
			// };
			// let adapter = tokio::block_on(instance.request_adapter(&request_options))?;
			// {
			// 	let adapter = adapter.get_info();
			// 	info!(
			// 		adapter.name,
			// 		adapter.driver, adapter.driver_info, "Using this adapter (e.g. native GPU & library)"
			// 	);
			// }
		}

		{
			// Borrowing rules are too restrictive here?
			// fn check<T: WindowHandle + 'static>(t: T) -> T {
			// 	t
			// }
			// let b = Box::new(window as &mut dyn wgpu::WindowHandle) as Box<dyn WindowHandle>;
			// let b = check(b);
			// let surface_target: SurfaceTarget<'static> = wgpu::SurfaceTarget::Window(b);
			// let surface: Surface<'static> = instance
			// 	.create_surface(surface_target)
			// 	.wrap_err("Couldn't create wgpu surface")?;
		}

		let surface_target = wgpu::SurfaceTargetUnsafe::RawHandle {
			raw_display_handle: window.display_handle().unwrap().as_raw(),
			raw_window_handle: window.window_handle().unwrap().as_raw(),
		};
		// WHY unsafe?
		// Borrowing rules are annoying to get around with dyn-traits in wgpu, bevy does this here:
		// https://github.com/bevyengine/bevy/blob/1a346870288cb0f8b742e4a85fba0370842fc848/crates/bevy_render/src/view/window/mod.rs#L314-L325
		let surface: Surface<'static> = unsafe { instance.create_surface_unsafe(surface_target) }
			.wrap_err("Couldn't create WGPU surface")?;
		self.surface = Some(surface);
		Ok(self.surface.as_mut().unwrap())
	}
}

use crate::prelude::*;
pub async fn main() -> color_eyre::Result<()> {
	info!("Hello, world!");

	let event_loop = EventLoop::new().wrap_err("Couldn't create event loop")?;

	// ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
	// dispatched any events. This is ideal for games and similar applications.
	event_loop.set_control_flow(ControlFlow::Poll);

	// // ControlFlow::Wait pauses the event loop if no events are available to process.
	// // This is ideal for non-game applications that only update in response to user
	// // input, and uses significantly less power/CPU time than ControlFlow::Poll.
	// event_loop.set_control_flow(ControlFlow::Wait);

	let mut app = App::default();
	event_loop
		.run_app(&mut app)
		.wrap_err("Event loop errored")?;

	// let target = todo!();
	// let surface = instance
	// 	.create_surface(target)
	// 	.wrap_err("Couldn't create surface")?;

	Ok(())
}
