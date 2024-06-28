use bevy::prelude::*;

#[bevy_main]
fn main() {
	std::env::set_var("NO_COLOR", "1");

	App::new()
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(bevy::window::Window {
						mode: bevy::window::WindowMode::Fullscreen,
						..default()
					}),
					..default()
				})
				.set(bevy::log::LogPlugin {
					level: tracing::Level::INFO,
					filter: "testing_winit=trace".into(),
					custom_layer: callback,
				})
				.disable::<bevy::audio::AudioPlugin>(),
		)
		.add_systems(Startup, || {
			trace!("Trace from Binary!");
			info!("Binary running");
			error!("Testing error");
		})
		.add_systems(Update, listen_to_pen_events)
		.run();
}

fn listen_to_pen_events(mut events: EventReader<bevy::input::touch::PenEvent>) {
	for e in events.read() {
		info!("Pen event: {:#?}", e);
	}
}

fn winit_noisy_filter() -> tracing_subscriber::filter::Targets {
	tracing_subscriber::filter::Targets::new()
		.with_target(
			"winit::platform_impl::platform::app_state",
			tracing_subscriber::filter::LevelFilter::ERROR,
		)
		.with_default(tracing_subscriber::filter::LevelFilter::TRACE)
}

// fn callback(subscriber: BoxedSubscriber) -> BoxedSubscriber {
// 	subscriber.with(tracing_subscriber::filter::Filter::from(
// 		winit_noisy_filter(),
// 	))
// }

fn callback(
	_: &mut App,
) -> Option<Box<dyn tracing_subscriber::Layer<tracing_subscriber::Registry> + Sync + Send>> {
	Some(Box::new(winit_noisy_filter()))
}
