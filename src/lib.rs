pub mod app_tracing;

pub mod prelude {
	pub use color_eyre::eyre::{bail, eyre, WrapErr as _};
	pub use tracing::{debug, error, info, trace, warn};
}

pub mod windowing;

use std::sync::Arc;

use wgpu::{
	rwh::{HasDisplayHandle, HasRawDisplayHandle, HasWindowHandle},
	Adapter, Backends, DeviceType, Features, Instance, RequestAdapterOptions, Surface, SurfaceTarget,
	WindowHandle,
};
use winit::{
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};

#[derive(Default)]
pub enum App {
	#[default]
	Initial,
	FatalError(color_eyre::Report),
	Setup(SetupApp),
}

pub struct SetupApp {
	window: Arc<winit::window::Window>,
	size: winit::dpi::PhysicalSize<u32>,

	instance: wgpu::Instance,
	surface: wgpu::Surface<'static>,
	adapter: wgpu::Adapter,
	device: wgpu::Device,
	queue: wgpu::Queue,

	surface_format: wgpu::TextureFormat,
}

impl App {
	pub fn instance() -> color_eyre::Result<wgpu::Instance> {
		let descriptor = wgpu::InstanceDescriptor::default();
		let instance = wgpu::Instance::new(&descriptor);
		Ok(instance)
	}

	pub fn surface(
		instance: &wgpu::Instance,
		window: Arc<winit::window::Window>,
	) -> color_eyre::Result<wgpu::Surface<'static>> {
		let surface: Surface<'static> = instance
			.create_surface(window)
			.wrap_err("Couldn't create WGPU surface <https://docs.rs/wgpu/latest/wgpu/struct.Instance.html#method.create_surface>")?;

		Ok(surface)
	}

	pub fn adapter(
		instance: &wgpu::Instance,
		surface: &wgpu::Surface,
	) -> color_eyre::Result<wgpu::Adapter> {
		// logging
		{
			let all_adapters = instance.enumerate_adapters(Backends::PRIMARY);
			for adapter in &all_adapters {
				let adapter = adapter.get_info();
				debug!(
					adapter.name,
					adapter.driver, adapter.driver_info, "Scanned an adapter (e.g. native GPU & library)"
				);
			}
		}

		let request_options = RequestAdapterOptions {
			power_preference: wgpu::PowerPreference::None,
			force_fallback_adapter: false,
			compatible_surface: Some(&surface),
		};
		let adapter = async move { instance.request_adapter(&request_options).await };

		// Please solve this problem cleanly
		trace!("Syncronously waiting for GPU adapter request to complete");
		let adapter = tokio::task::block_in_place(move || {
			tokio::runtime::Handle::current()
				.block_on(adapter)
				.wrap_err("Couldn't request an adapter")
		})?;
		{
			let adapter = adapter.get_info();
			info!(
				adapter.name,
				adapter.driver, adapter.driver_info, "Using this adapter (e.g. native GPU & library)"
			);
		}
		Ok(adapter)
	}

	pub fn device(adapter: &wgpu::Adapter) -> color_eyre::Result<(wgpu::Device, wgpu::Queue)> {
		let device_descriptor = {
			let mut features = adapter.features();
			if adapter.get_info().device_type == wgpu::DeviceType::DiscreteGpu {
				// See https://github.com/bevyengine/bevy/blob/1a346870288cb0f8b742e4a85fba0370842fc848/crates/bevy_render/src/renderer/mod.rs#L291
				features.remove(wgpu::Features::MAPPABLE_PRIMARY_BUFFERS);
			}
			let limits = adapter.limits();
			wgpu::DeviceDescriptor {
				label: Some("CUSTOM LABEL yeditor"),
				required_features: features,
				required_limits: limits,
				memory_hints: wgpu::MemoryHints::default(),
				trace: wgpu::Trace::Off,
			}
		};
		let device = async move { adapter.request_device(&device_descriptor).await };

		trace!("Syncronously waiting for GPU device and queue");
		tokio::task::block_in_place(move || {
			tokio::runtime::Handle::current()
				.block_on(device)
				.wrap_err("Couldn't request device <https://docs.rs/wgpu/latest/wgpu/struct.Adapter.html#method.request_device>")
		})
	}

	pub fn surface_format(
		surface: &wgpu::Surface,
		adapter: &wgpu::Adapter,
	) -> color_eyre::Result<wgpu::TextureFormat> {
		surface
			.get_capabilities(adapter)
			.formats
			.get(0)
			.cloned()
			.ok_or(eyre!("No available formats"))
	}
}

impl SetupApp {
	pub fn configure_surface(&self) {
		// let window_width = self.window.inner_size().width;
		// let window_height = self.window.inner_size().height;
		// let config = self.surface
		// 	.get_default_config(&self.adapter, window_width, window_height)
		// 	.ok_or(eyre!("Adapter surface mismatch for get_default_config https://docs.rs/wgpu/26.0.1/wgpu/struct.Surface.html#method.get_default_config"))?;
		// self.surface.configure(&self.device, &config);
		// Ok(())

		let surface_config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: self.surface_format,
			// Request compatibility with the sRGB-format texture view we‘re going to create later.
			view_formats: vec![self.surface_format.add_srgb_suffix()],
			alpha_mode: wgpu::CompositeAlphaMode::Auto,
			width: self.size.width,
			height: self.size.height,
			desired_maximum_frame_latency: 2,
			present_mode: wgpu::PresentMode::AutoVsync,
		};
		self.surface.configure(&self.device, &surface_config);
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		self.size = new_size;

		self.configure_surface()
	}

	fn render(&mut self) {
		// Create texture view
		let surface_texture = self
			.surface
			.get_current_texture()
			.expect("failed to acquire next swapchain texture");
		let texture_view = surface_texture
			.texture
			.create_view(&wgpu::TextureViewDescriptor {
				// Without add_srgb_suffix() the image we will be working with
				// might not be "gamma correct".
				format: Some(self.surface_format.add_srgb_suffix()),
				..Default::default()
			});

		// Renders a GREEN screen
		let mut encoder = self.device.create_command_encoder(&Default::default());
		// Create the renderpass which will clear the screen.
		let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: None,
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &texture_view,
				depth_slice: None,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			timestamp_writes: None,
			occlusion_query_set: None,
		});

		// If you wanted to call any drawing commands, they would go here.

		// End the renderpass.
		drop(renderpass);

		// Submit the command in the queue to execute
		self.queue.submit([encoder.finish()]);
		self.window.pre_present_notify();
		surface_texture.present();
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
