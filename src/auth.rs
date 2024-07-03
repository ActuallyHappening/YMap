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
	use ysurreal::{config::start_blank_memory_db, configs::TestingMem};

	use super::INIT_SURQL;
	use yauth::{prelude::*, signup::SignUp};

	#[test_log::test(tokio::test)]
	async fn sign_up_works() -> Result<(), Report> {
		let conn_config = TestingMem::rand(INIT_SURQL.to_string());
		let db = start_blank_memory_db(&conn_config).unwrap().await?;
		let auth_config = yauth::configs::TestingAuthConfig::new(&conn_config);

		let debug_info = db.query("INFO FOR db").await?;
		trace!("{:#?}", debug_info);

		// signs in as a scoped user
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

	#[test_log::test(tokio::test)]
	async fn user_table_appends() -> Result<(), Report> {
		let conn_config = TestingMem::rand(INIT_SURQL.to_string());
		let db = start_blank_memory_db(&conn_config).unwrap().await?;
		conn_config.use_primary_ns_db(&db).await?;
		// doesn't require authorization by default
		conn_config.init_query(&db).await?;
		let auth_config = yauth::configs::TestingAuthConfig::new(&conn_config);

		// doesn't require authorization by default which is sad
		// let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
		// assert_eq!(users.len(), 0, "Users table should not be readable until you have signed in");

		let credentials = SignUp::testing_rand();
		auth_config.sign_up(&db, &credentials).await?;

		let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
		assert_eq!(
			users.len(),
			1,
			"Users table should have one entry after signing one person in"
		);

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn user_table_appends_multiple() -> Result<(), Report> {
		let conn_config = TestingMem::rand(INIT_SURQL.to_string());
		let db = start_blank_memory_db(&conn_config).unwrap().await?;
		conn_config.use_primary_ns_db(&db).await?;
		// doesn't require authorization by default
		conn_config.init_query(&db).await?;
		let auth_config = yauth::configs::TestingAuthConfig::new(&conn_config);

		// doesn't require authorization by default which is sad
		// let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
		// assert_eq!(users.len(), 0, "Users table should not be readable until you have signed in");

		for i in 1..10 {
			let mut credentials = SignUp::testing_rand();
			credentials.email = yauth::types::Email::from_str(&format!("testgenerated{i}@me.com")).unwrap();
			auth_config.sign_up(&db, &credentials).await?;

			let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
			assert_eq!(
				users.len(),
				i,
				"Users table should have {i} entry after signing {i} people in",
			);

			auth_config.invalidate(&db).await?;
		}

		Ok(())
	}
}
