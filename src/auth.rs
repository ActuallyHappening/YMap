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
	async fn sign_up_works() -> Result<(), Report> {
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

		// signs in as a scoped user
		auth_config.sign_up(&db, &SignUp::testing_rand()).await?;

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn sign_in_works() -> Result<(), Report> {
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

		let credentials = SignUp::testing_rand();

		// signs in as a scoped user
		auth_config.sign_up(&db, &credentials).await?;
		auth_config.invalidate(&db).await?;

		// signs into already signed up user
		auth_config.sign_in(&db, &credentials.into()).await?;

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn sign_up_twice_fails() -> Result<(), Report> {
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

		let credentials = SignUp::testing_rand();

		// signs in as a scoped user
		auth_config.sign_up(&db, &credentials).await?;
		auth_config.invalidate(&db).await?;

		let result = auth_config.sign_up(&db, &credentials).await;
		assert!(result.is_err());

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn user_table_appends() -> Result<(), Report> {
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

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
		setup!(
			db = db,
			conn_config = conn_config,
			auth_config = auth_config
		);

		// doesn't require authorization by default which is sad
		// let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
		// assert_eq!(users.len(), 0, "Users table should not be readable until you have signed in");

		for i in 1..10 {
			let mut credentials = SignUp::testing_rand();
			credentials.email =
				yauth::types::Email::from_str(&format!("testgenerated{i}@me.com")).unwrap();

			// sign into scope account to create new user, then sign into root user
			auth_config.sign_up(&db, &credentials).await?;
			auth_config.invalidate(&db).await?;
			conn_config.root_sign_in(&db).await?;

			let users: Vec<serde_json::Value> = db.select(auth_config.users_table()).await?;
			assert_eq!(
				users.len(),
				i,
				"Users table should have {i} entry after signing {i} people in",
			);

			auth_config.invalidate(&db).await?
		}

		Ok(())
	}
}
