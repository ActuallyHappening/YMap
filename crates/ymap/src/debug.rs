use crate::prelude::*;
use std::marker::PhantomData;

use bevy::ecs::entity::MapEntities;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_replicon::{prelude::*, RepliconPlugins};
use bevy_replicon_renet::RepliconRenetPlugins;

/// Enables much very useful debugging, that is NOT part of the normal UI
pub struct DebugPlugin;

const DEBUG_PORT: u16 = 42069;

pub type DebugMarker = Replicated;
pub fn debug_marker() -> DebugMarker {
	bevy_replicon::prelude::Replicated
}

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugins(bevy_editor_pls::EditorPlugin::default())
			.add_plugins(NetworkDebuggingPlugin)
			.add_systems(Update, touch_system)
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
			.replicate::<Name>();
		if !app.is_plugin_added::<EguiPlugin>() {
			info!("Adding EGui plugin");
			app.add_plugins(EguiPlugin);
		}

		// init netcode depending on platform
		if cfg!(feature = "ios") {
			app.insert_resource(NetcodeConfig::new_hosting_public());
		} else {
			app.insert_resource(NetcodeConfig::new_client_public(
				std::env::var("LOCAL_IP")
					.expect("Pass LOCAL_IP env var")
					.parse()
					.expect("Couldn't parse LOCAL_IP"),
			));
		}
		app.add_systems(Startup, NetcodeConfig::add_netcode.map(bevy::utils::error));

		// control server
		app.debug_control::<Transform>();
	}
}

#[extension(trait AppExt)]
impl App {
	fn debug_control<T: Component + std::fmt::Debug + Clone + Serialize + DeserializeOwned>(
		&mut self,
	) -> &mut Self {
		self
			.replicate::<T>()
			.add_mapped_client_event::<ComponentChanged<T>>(ChannelKind::Ordered)
			.add_systems(
				Update,
				sync_transforms_ios::<T>.run_if(|| cfg!(feature = "ios")),
			)
			.add_systems(
				Update,
				sync_transforms_non_ios::<T>.run_if(|| !cfg!(feature = "ios")),
			)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
struct ComponentChanged<T: Component> {
	entity: Entity,
	new_component: T,
	_phantom: PhantomData<T>,
}

impl<T: Component> MapEntities for ComponentChanged<T> {
	fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
		self.entity = entity_mapper.map_entity(self.entity);
	}
}

/// Update local state
// #[cfg(feature = "ios")]
fn sync_transforms_ios<T: Component + std::fmt::Debug + Clone>(
	mut receive_events: EventReader<FromClient<ComponentChanged<T>>>,
	mut query: Query<&mut T, With<Replicated>>,
) {
	for event in receive_events.read() {
		if let Ok(mut transform) = query.get_mut(event.event.entity) {
			*transform = event.event.new_component.clone();
		} else {
			error_once!(message = "Couldn't find entity that was sent from client", event = ?event.event, query = ?query.iter().collect::<Vec<_>>());
		}
	}
}

/// Send from client
// #[cfg(not(feature = "ios"))]
fn sync_transforms_non_ios<T: Component + Clone>(
	mut send_events: EventWriter<ComponentChanged<T>>,
	change: Query<(Entity, &T), (With<Replicated>, Changed<T>)>,
) {
	for (entity, new_component) in change.iter() {
		send_events.send(ComponentChanged {
			entity,
			new_component: new_component.clone(),
			_phantom: PhantomData,
		});
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

fn touch_system(
	touches: Res<Touches>,
	window: Query<&Window>,
	time: Res<Time>,
	mut window_timer: Local<Timer>,
) {
	once!({
		*window_timer = Timer::from_seconds(10.0, TimerMode::Repeating);
		debug!("Touch System Running");
	});
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

	if let Ok(window) = window.get_single() {
		let window_resolution = &window.resolution;
		let logical_dimensions = Vec2::new(window_resolution.width(), window_resolution.height());
		if window_timer.tick(time.delta()).just_finished() {
			debug!(message = "Window Resolution", ?window_resolution, ?logical_dimensions);
			// WindowResolution { physical_width: 2778, physical_height: 1940, scale_factor_override: None, scale_factor: 1.7195877 }
		}

		// you can also iterate all current touches and retrieve their state like this:
		for touch in touches.iter() {
			// debug!("active touch: {:?}", touch);
			// debug!("  just_pressed: {}", touches.just_pressed(touch.id()));
			let pos = touch.position();
			let normalized_pos_window = Vec2::new(
				pos.x / window_resolution.width(),
				pos.y / window_resolution.height(),
			);
			debug!(message = "Touch", ?pos, ?normalized_pos_window);
		}
	} else {
		warn!("No window found");
	}
}

use netcode::NetcodeConfig;
use serde::de::DeserializeOwned;
mod netcode {
	use crate::prelude::*;
	use bevy_replicon::prelude::*;
	use bevy_replicon_renet::renet::{
		transport::{
			ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
			ServerConfig,
		},
		ConnectionConfig, RenetClient, RenetServer,
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
			use bevy_replicon_renet::RenetChannelsExt;
			match config.into_inner() {
				NetcodeConfig::Server { ip, port } => {
					info!(message = "Setting up as server, hosting", ?ip, ?port);
					let server_channels_config = channels.get_server_configs();
					let client_channels_config = channels.get_client_configs();

					let server = RenetServer::new(ConnectionConfig {
						server_channels_config,
						client_channels_config,
						..Default::default()
					});

					let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
					let public_addr = SocketAddr::new(*ip, *port);
					let socket = UdpSocket::bind(public_addr).wrap_err("Could not construct UdpSocket")?;
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
					info!(message = "Setting up as client, connecting", ?ip, ?port);
					let server_channels_config = channels.get_server_configs();
					let client_channels_config = channels.get_client_configs();

					let client = RenetClient::new(ConnectionConfig {
						server_channels_config,
						client_channels_config,
						..Default::default()
					});

					let current_time = SystemTime::now()
						.duration_since(SystemTime::UNIX_EPOCH)
						.unwrap();
					let client_id = current_time.as_millis() as u64;
					let server_addr = SocketAddr::new(*ip, *port);
					let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))
						.expect("Couldn't bind to (unspecified) socket");
					let authentication = ClientAuthentication::Unsecure {
						client_id,
						protocol_id: PROTOCOL_ID,
						server_addr,
						user_data: None,
					};
					let transport = NetcodeClientTransport::new(current_time, authentication, socket)
						.wrap_err("Couldn't join to server")?;

					commands.insert_resource(client);
					commands.insert_resource(transport);

					Ok(())
				}
			}
		}
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

		pub const fn new_client_public(ip: Ipv4Addr) -> Self {
			NetcodeConfig::Client {
				ip: IpAddr::V4(ip),
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
