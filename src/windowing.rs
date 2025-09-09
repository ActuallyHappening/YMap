use crate::{prelude::*, App, SetupApp};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

impl SetupApp {
	pub fn window(event_loop: &ActiveEventLoop) -> color_eyre::Result<Window> {
		let attributes = WindowAttributes::default()
			.with_title("yeditor - winit")
			.with_theme(Some(winit::window::Theme::Dark));
		event_loop.create_window(attributes).wrap_err(
			"Couldn't create window <https://docs.rs/winit/latest/winit/event_loop/struct.ActiveEventLoop.html#method.create_window>"
		)
	}
}

impl ApplicationHandler for crate::App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		if let Err(err) = self.window(event_loop) {
			error!(%err, ?err, "Window couldn't create?");
			self.fatal_error = Some(err)
		}
		if let Err(err) = self.surface() {
			error!(%err, ?err, "Couldn't initialize surface");
			self.fatal_error = Some(err)
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
