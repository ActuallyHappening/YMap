pub mod prelude {
	pub(crate) use bevy::prelude::*;

	pub use crate::plugin::IosLogPlugin;
}

mod plugin {
	use tracing::Level;
	use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

	use crate::prelude::*;

	pub struct IosLogPlugin {
		filter: String,
		level: Level,
		ansi: bool,
	}

	impl Default for IosLogPlugin {
		fn default() -> Self {
			IosLogPlugin {
				level: Level::INFO,
				filter: format!(
					"{},wgpu=error,naga=warn,{}",
					Level::INFO,
					Self::IGNORE_DIRECTIVES
				),
				ansi: false,
			}
		}
	}

	impl IosLogPlugin {
		const IGNORE_DIRECTIVES: &'static str = r#"[{message = "processing non `RedrawRequested` event after the main event loop: AboutToWait"}]=trace,[{message = "processing `RedrawRequested` during the main event loop"}]=trace"#;
	}

	impl Plugin for IosLogPlugin {
		fn build(&self, _app: &mut App) {
			let env_filter = EnvFilter::try_new(format!("{},{}", self.level, self.filter))
				.or_else(|_| EnvFilter::try_new(IosLogPlugin::default().filter))
				.expect("Invalid filter");
			let mut fmt_layer = tracing_subscriber::fmt::layer().with_writer(std::io::stderr);
			fmt_layer.set_ansi(self.ansi);
			let init = Registry::default()
				.with(env_filter)
				.with(fmt_layer)
				.try_init();
			if let Err(e) = init {
				eprintln!("Failed to initialize logger: {}", e);
			}
		}
	}
}
