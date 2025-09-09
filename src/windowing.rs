use crate::{prelude::*, App, SetupApp};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

impl App {
	pub fn window(event_loop: &ActiveEventLoop) -> color_eyre::Result<Window> {
		let attributes = WindowAttributes::default()
			.with_title("yeditor - winit")
			.with_theme(Some(winit::window::Theme::Dark));
		event_loop.create_window(attributes).wrap_err(
			"Couldn't create window <https://docs.rs/winit/latest/winit/event_loop/struct.ActiveEventLoop.html#method.create_window>"
		)
	}
}

impl App {
	pub fn setup(&mut self) -> color_eyre::Result<&SetupApp> {
		match self {
			Self::Initial => bail!("App state is initial not setup"),
			Self::FatalError(err) => bail!("App state is fatally errored not setup: {}", err),
			Self::Setup(state) => Ok(state),
		}
	}
}

impl ApplicationHandler for crate::App {
	#[tracing::instrument(skip_all, name = "winit::ApplicationHandler::resumed")]
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		match self {
			Self::Setup(_) => {
				info!("Had resumed called on an already setup app");
			}
			Self::FatalError(err) => {
				error!(%err, ?err, "Error in current state in resumed");
			}
			Self::Initial => {
				// setup
				let err_boundary = (move || -> color_eyre::Result<SetupApp> {
					let window = App::window(&event_loop)?;
					let instance = App::instance()?;
					let surface = App::surface(&instance, &window)?;
					let adapter = App::adapter(&instance, &surface)?;
					let (device, queue) = App::device(&adapter)?;
					Ok(todo!())
				})();
			}
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
		let app = self.setup().unwrap();
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
				app.window.request_redraw();
			}
			_ => (),
		}
	}
}
