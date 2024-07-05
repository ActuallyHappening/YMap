#[path = "../init.surql.rs"]
mod init;

pub mod config {
	use std::marker::PhantomData;

	use crate::prelude::*;

	#[cfg(not(feature = "production"))]
	pub use production_controller::ProductionControllerConfig;
	#[cfg(not(feature = "production"))]
	mod production_controller {
		use crate::prelude::*;

		use crate::auth::config::ProductionConfig;

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
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/init.surql")).into()
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

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use color_eyre::eyre::Report;
	use ysurreal::{config::start_blank_memory_db, configs::TestingMem};

	use super::init::INIT_SURQL;
	use yauth::signup::SignUp;

	macro_rules! setup {
		(db = $db:ident, conn_config = $conn_config:ident, auth_config = $auth_config:ident) => {
			let $conn_config = TestingMem::rand(INIT_SURQL.to_string());
			let $db = start_blank_memory_db(&$conn_config).unwrap().await?;
			$conn_config.init_query(&$db).await?;
			$conn_config.use_primary_ns_db(&$db).await?;
			let $auth_config = yauth::configs::TestingAuthConfig::new(&$conn_config);
		};
	}

	#[test_log::test(tokio::test)]
	async fn db_sign_up() -> Result<(), Report> {
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

		// signs in as a scoped user
		auth_config.control_db(&db).sign_up(&SignUp::testing_rand()).await?;

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn db_sign_in() -> Result<(), Report> {
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

		let credentials = SignUp::testing_rand();

		// signs in as a scoped user
		auth_config.control_db(&db).sign_up(&credentials).await?;
		auth_config.control_db(&db).invalidate().await?;

		// signs into already signed up user
		auth_config.control_db(&db).sign_in(&credentials.into()).await?;

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn db_sign_up_twice_fails() -> Result<(), Report> {
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

		let credentials = SignUp::testing_rand();

		// signs in as a scoped user
		auth_config.control_db(&db).sign_up(&credentials).await?;
		auth_config.control_db(&db).invalidate().await?;

		let result = auth_config.control_db(&db).sign_up(&credentials).await;
		assert!(result.is_err());

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn db_user_table_appends() -> Result<(), Report> {
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

		// doesn't require authorization by default which is sad
		// let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
		// assert_eq!(users.len(), 0, "Users table should not be readable until you have signed in");

		let credentials = SignUp::testing_rand();
		auth_config.control_db(&db).sign_up(&credentials).await?;

		let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
		assert_eq!(
			users.len(),
			1,
			"Users table should have one entry after signing one person in"
		);

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn db_user_table_appends_multiple() -> Result<(), Report> {
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

		// doesn't require authorization by default which is sad
		// let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
		// assert_eq!(users.len(), 0, "Users table should not be readable until you have signed in");

		for i in 1..5 {
			let mut credentials = SignUp::testing_rand();
			credentials.email =
				yauth::types::Email::from_str(&format!("testgenerated{i}@me.com")).unwrap();

			// sign into scope account to create new user, then sign into root user
			auth_config.control_db(&db).sign_up(&credentials).await?;
			auth_config.control_db(&db).invalidate().await?;
			// basically signing into root here, since the default is no authentication
			// conn_config.root_sign_in(&db).await?;

			let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
			assert_eq!(
				users.len(),
				i,
				"Users table should have {i} entry after signing {i} people in",
			);

			auth_config.control_db(&db).invalidate().await?
		}

		Ok(())
	}
}
