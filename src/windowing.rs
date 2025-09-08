use crate::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub struct App {
	window: Option<Window>,
	fatal_error: Option<color_eyre::Report>,
}

impl App {
	pub fn new()
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		match event_loop.create_window(Window::default_attributes()) {
			Ok(window) => self.window = Some(window),
			Err(err) => self.fatal_error = Some(eyre!("Couldn't create window ({}): {:?}", err, err)),
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
		match event {
			WindowEvent::CloseRequested => {
				info!("The close button was pressed; stopping");
				event_loop.exit();
			}
			WindowEvent::RedrawRequested => {
				// Redraw the application.
				//
				// It's preferable for applications that do not render continuously to render in
				// this event rather than in AboutToWait, since rendering in here allows
				// the program to gracefully handle redraws requested by the OS.

				// Draw.

				// Queue a RedrawRequested event.
				//
				// You only need to call this if you've determined that you need to redraw in
				// applications which do not always need to. Applications that redraw continuously
				// can render here instead.
				self.window.as_ref().unwrap().request_redraw();
			}
			_ => (),
		}
	}
}
