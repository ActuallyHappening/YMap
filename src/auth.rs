#[path = "../init.surql.rs"]
mod init;
use init::INIT_SURQL;

pub mod config {
	use openssh::Session;

	use crate::prelude::*;

	#[derive(Args, Debug, Clone)]
	pub struct ProductionConfig {
		#[arg(long, default_value_t = { Secrets::ssh_name() })]
		pub ssh_name: String,

		#[arg(long, default_value_t = Utf8PathBuf::from("/root/home/YMap/surreal.db"))]
		pub surreal_data_path: Utf8PathBuf,

		#[arg(long, default_value_t = Utf8PathBuf::from("/usr/local/bin/surreal"))]
		pub surreal_binary_path: Utf8PathBuf,

		#[arg(long, default_value_t = Utf8PathBuf::from("/root/.cargo/bin/nu"))]
		pub nu_binary_path: Utf8PathBuf,
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

	impl DBRootCredentials for ProductionConfig {
		fn root_password(&self) -> String {
			Secrets::production_password()
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

	impl ProductionConfig {
		pub async fn ssh(&self) -> Result<Session, openssh::Error> {
			let ssh_name = self.ssh_name.as_str();
			info!(message = "Connecting to server host", ?ssh_name);
			Session::connect_mux(ssh_name, openssh::KnownHosts::Strict).await
		}
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use color_eyre::eyre::Report;
	use ysurreal::{config::start_in_memory, configs::TestingMem};

	use super::INIT_SURQL;
	use yauth::{prelude::*, signup::SignUp};

	#[test_log::test(tokio::test)]
	async fn sign_up_works() -> Result<(), Report> {
		let conn_config = TestingMem::rand(INIT_SURQL.to_string());
		let db = start_in_memory(&conn_config).unwrap().await?;
		let auth_config = yauth::configs::TestingAuthConfig::new(&conn_config);

		let debug_info = db.query("INFO FOR db").await?;
		trace!("{:#?}", debug_info);

		auth_config
			.sign_up(
				&db,
				&SignUp {
					username: "my username 123".parse().unwrap(),
					password: "my password 123".parse().unwrap(),
					email: "me@mydomain.com".parse().unwrap(),
				},
			)
			.await?;

		Ok(())
	}
}
