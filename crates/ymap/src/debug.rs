use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_replicon::{prelude::*, RepliconPlugins};
use bevy_replicon_renet::RepliconRenetPlugins;

/// Enables much very useful debugging, that is NOT part of the normal UI
pub struct DebugPlugin;

const DEBUG_PORT: u16 = 42069;

pub type DebugMarker = Replicated;
pub fn debug_marker() -> DebugMarker {
	Replicated
}

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugins(NetworkDebuggingPlugin)
			.add_systems(Update, touch_system)
			.add_plugins(bevy_editor_pls::EditorPlugin::default())
			.insert_resource(editor_controls());
	}
}

struct NetworkDebuggingPlugin;

impl Plugin for NetworkDebuggingPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(bevy::winit::WinitSettings {
				focused_mode: bevy::winit::UpdateMode::Continuous,
				unfocused_mode: bevy::winit::UpdateMode::Continuous,
			})
			.add_plugins((RepliconPlugins, RepliconRenetPlugins))
			.replicate::<Transform>()
			.replicate::<Name>();
		if !app.is_plugin_added::<EguiPlugin>() {
			info!("Adding EGui plugin");
			app.add_plugins(EguiPlugin);
		}

		// init netcode depending on platform
		if cfg!(feature = "ios") {
			app.insert_resource(NetcodeConfig::new_hosting_public());
		} else {
			app.insert_resource(NetcodeConfig::new_client_machine_local());
		}
		app.add_systems(Startup, NetcodeConfig::add_netcode.map(bevy::utils::error));
	}
}

fn debug_window(mut contexts: EguiContexts) {
	egui::SidePanel::right("Network Debugging").show(contexts.ctx_mut(), |ui| {
		if ui.button("Server").clicked() {
			info!(message = "Server Clicked");
		}
		if ui.button("Client").clicked() {
			info!(message = "Client Clicked");
		}
	});
}

/// Changes the button from E to backslash \
fn editor_controls() -> bevy_editor_pls::controls::EditorControls {
	use bevy_editor_pls::controls;
	use bevy_editor_pls::controls::EditorControls;

	let mut editor_controls = EditorControls::default_bindings();
	editor_controls.unbind(controls::Action::PlayPauseEditor);

	editor_controls.insert(
		controls::Action::PlayPauseEditor,
		controls::Binding {
			input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Backslash)),
			conditions: vec![controls::BindingCondition::ListeningForText(false)],
		},
	);

	editor_controls
}

fn touch_system(touches: Res<Touches>) {
	// for touch in touches.iter_just_pressed() {
	// 	debug!(
	// 		"just pressed touch with id: {:?}, at: {:?}",
	// 		touch.id(),
	// 		touch.position()
	// 	);
	// }

	// for touch in touches.iter_just_released() {
	// 	debug!(
	// 		"just released touch with id: {:?}, at: {:?}",
	// 		touch.id(),
	// 		touch.position()
	// 	);
	// }

	// for touch in touches.iter_just_canceled() {
	// 	debug!("canceled touch with id: {:?}", touch.id());
	// }

	// you can also iterate all current touches and retrieve their state like this:
	for touch in touches.iter() {
		// debug!("active touch: {:?}", touch);
		// debug!("  just_pressed: {}", touches.just_pressed(touch.id()));
		let pos = touch.position();
		debug!(message = "Touch", ?pos);
	}
}

use netcode::NetcodeConfig;
mod netcode {
	use crate::prelude::*;
	use bevy_replicon::prelude::*;
	use bevy_replicon_renet::renet::{
		transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
		ConnectionConfig, RenetServer,
	};
	use std::{net::*, time::SystemTime};

	use super::DEBUG_PORT as DEFAULT_PORT;

	/// Holds information about what ip and port to connect to, or host on.
	#[derive(Resource, Debug, clap::Parser)]
	pub enum NetcodeConfig {
		Server {
			#[arg(long, default_value_t = Ipv4Addr::LOCALHOST.into())]
			ip: IpAddr,

			#[arg(short, long, default_value_t = DEFAULT_PORT)]
			port: u16,
		},
		Client {
			#[arg(short, long, default_value_t = Ipv4Addr::LOCALHOST.into())]
			ip: IpAddr,

			#[arg(short, long, default_value_t = DEFAULT_PORT)]
			port: u16,
		},
	}

	const PROTOCOL_ID: u64 = 0;

	impl NetcodeConfig {
		pub fn add_netcode(
			config: Res<NetcodeConfig>,
			channels: Res<RepliconChannels>,
			mut commands: Commands,
		) -> Result<(), color_eyre::Report> {
			match config.into_inner() {
				NetcodeConfig::Server { ip, port } => {
					use bevy_replicon_renet::RenetChannelsExt;
					let server_channels_config = channels.get_server_configs();
					let client_channels_config = channels.get_client_configs();

					let server = RenetServer::new(ConnectionConfig {
						server_channels_config,
						client_channels_config,
						..Default::default()
					});

					let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
					let public_addr = SocketAddr::new(*ip, *port);
					let socket = UdpSocket::bind(public_addr)
						.wrap_err("Could not construct UdpSocket")?;
					let server_config = ServerConfig {
						current_time,
						max_clients: 10,
						protocol_id: PROTOCOL_ID,
						authentication: ServerAuthentication::Unsecure,
						public_addresses: Default::default(),
					};
					let transport = NetcodeServerTransport::new(server_config, socket)?;

					commands.insert_resource(server);
					commands.insert_resource(transport);

					Ok(())
				}
				NetcodeConfig::Client { ip, port } => {
					todo!();

					Ok(())
				}
			}
		}
	}

	/// Only necessary for one-time setup
	impl Plugin for NetcodeConfig {
		fn build(&self, app: &mut App) {}
	}

	impl NetcodeConfig {
		pub const fn new_hosting_public() -> Self {
			NetcodeConfig::Server {
				ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
				port: DEFAULT_PORT,
			}
		}

		pub const fn new_hosting_machine_local() -> Self {
			NetcodeConfig::Server {
				ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
				port: DEFAULT_PORT,
			}
		}

		pub const fn new_client_machine_local() -> Self {
			NetcodeConfig::Client {
				ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
				port: DEFAULT_PORT,
			}
		}

		pub fn is_authoritative(&self) -> bool {
			match self {
				NetcodeConfig::Server { .. } => true,
				NetcodeConfig::Client { .. } => false,
			}
		}

		/// Used in a `.run_if` to signify a system that should only run if
		/// the current instance is the authoritative server
		pub fn has_authority() -> impl Fn(Res<NetcodeConfig>) -> bool {
			|config| config.is_authoritative()
		}
	}
}
