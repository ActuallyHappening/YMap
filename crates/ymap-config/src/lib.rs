pub mod prelude {
	pub(crate) use camino::Utf8PathBuf;
	pub(crate) use clap::Args;
	pub(crate) use std::marker::PhantomData;
	pub(crate) use tracing::*;

	pub(crate) use crate::secrets::*;
	pub(crate) use yauth::prelude::*;
	pub(crate) use ysurreal::prelude::*;

	pub use crate::production_client::ProductionConfig;
	#[cfg(not(feature = "production"))]
	pub use crate::production_controller::ProductionControllerConfig;
}

pub mod secrets;

#[cfg(not(feature = "production"))]
pub use production_controller::ProductionControllerConfig;
#[cfg(not(feature = "production"))]
mod production_controller {
	use crate::prelude::*;

	#[derive(Args, Debug, Clone)]
	pub struct ProductionControllerConfig {
		#[clap(flatten)]
		pub production_config: ProductionConfig,

		#[arg(long)]
		#[cfg_attr(not(feature = "production"), arg(default_value_t = { Secrets::ssh_name() }))]
		pub ssh_name: String,

		#[arg(long, default_value_t = Utf8PathBuf::from("/root/home/YMap/surreal.db"))]
		pub surreal_data_path: Utf8PathBuf,

		#[arg(long, default_value_t = Utf8PathBuf::from("/usr/local/bin/surreal"))]
		pub surreal_binary_path: Utf8PathBuf,

		#[arg(long, default_value_t = Utf8PathBuf::from("/root/.cargo/bin/nu"))]
		pub nu_binary_path: Utf8PathBuf,
	}

	impl DBStartConfig for ProductionControllerConfig {
		fn init_surql(&self) -> String {
			self.production_config.init_surql()
		}

		fn bind_port(&self) -> u16 {
			self.production_config.bind_port()
		}

		fn db_type(&self) -> ysurreal::config::StartDBType {
			self.production_config.db_type()
		}
	}

	impl DBRootCredentials for ProductionControllerConfig {
		/// The magic of [ProductionControllerConfig] versus just plain
		/// [ProductionConfig].
		fn root_password(&self) -> String {
			Secrets::production_password()
		}
	}

	impl DBConnectRemoteConfig for ProductionControllerConfig {
		fn primary_namespace(&self) -> String {
			self.production_config.primary_namespace()
		}

		fn primary_database(&self) -> String {
			self.production_config.primary_database()
		}

		fn connect_host(&self) -> String {
			self.production_config.connect_host()
		}

		fn connect_port(&self) -> u16 {
			self.production_config.connect_port()
		}
	}

	impl DBAuthConfig for ProductionControllerConfig {
		fn users_scope(&self) -> String {
			"end_user".into()
		}

		fn users_table(&self) -> String {
			"user".into()
		}
	}

	impl ProductionControllerConfig {
		#[cfg(not(target_arch = "wasm32"))]
		pub async fn ssh(&self) -> Result<openssh::Session, openssh::Error> {
			let ssh_name = self.ssh_name.as_str();
			info!(message = "Connecting to server host", ?ssh_name);
			openssh::Session::connect_mux(ssh_name, openssh::KnownHosts::Strict).await
		}
	}
}

mod production_client {
	//! Available always, esspecially when the 'production' feature flag is enabled

	use crate::prelude::*;

	/// The specific configuration used by `ymap` in production
	#[derive(Default, Debug, Args, Clone)]
	pub struct ProductionConfig {
		#[clap(skip = PhantomData)]
		_marker: PhantomData<()>,
	}

	impl ProductionConfig {
		/// Trait implementations provide the necessary data
		pub fn new() -> Self {
			Self::default()
		}
	}

	impl DBStartConfig for ProductionConfig {
		fn init_surql(&self) -> String {
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../init.surql")).into()
		}

		fn bind_port(&self) -> u16 {
			42069
		}

		fn db_type(&self) -> ysurreal::config::StartDBType {
			ysurreal::config::StartDBType::File {
				data_path: Utf8PathBuf::from("/root/home/YMap/surreal.db"),
			}
		}
	}

	impl DBConnectRemoteConfig for ProductionConfig {
		fn primary_namespace(&self) -> String {
			"production".into()
		}

		fn primary_database(&self) -> String {
			"production".into()
		}

		fn connect_host(&self) -> String {
			"actually-happening.foundation".into()
		}

		fn connect_port(&self) -> u16 {
			42069
		}
	}

	impl DBAuthConfig for ProductionConfig {
		fn users_scope(&self) -> String {
			"end_user".into()
		}

		fn users_table(&self) -> String {
			"user".into()
		}
	}
}
