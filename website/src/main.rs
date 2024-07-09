#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	use axum::Router;
	use leptonic_template_ssr::app::*;
	use leptonic_template_ssr::fileserv::file_and_error_handler;
	use leptos::*;
	use leptos_axum::{generate_route_list, LeptosRoutes};

	use tracing_subscriber::{prelude::*, util::SubscriberInitExt, Layer};

	let log_filter = tracing_subscriber::filter::Targets::new()
		.with_default(tracing::Level::INFO)
		.with_target("tokio", tracing::Level::WARN)
		.with_target("runtime", tracing::Level::WARN);

	let fmt_layer = tracing_subscriber::fmt::layer()
		.pretty()
		.with_file(true)
		.with_line_number(true)
		.with_ansi(true)
		.with_thread_names(false)
		.with_thread_ids(false);

	let fmt_layer_filtered = fmt_layer.with_filter(log_filter);

	tracing_subscriber::Registry::default()
		.with(fmt_layer_filtered)
		.init();

	// Setting get_configuration(None) means we'll be using cargo-leptos's env values
	// For deployment these variables are:
	// <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
	// Alternately a file can be specified such as Some("Cargo.toml")
	// The file would need to be included with the executable when moved to deployment
	let conf = get_configuration(None).await.unwrap();
	let leptos_options = conf.leptos_options;
	let addr = leptos_options.site_addr;
	let routes = generate_route_list(App);

	// build our application with a route
	let app = Router::new()
		.leptos_routes(&leptos_options, routes, App)
		.fallback(file_and_error_handler)
		.with_state(leptos_options);

	tracing::info!("listening on http://{}", &addr);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
