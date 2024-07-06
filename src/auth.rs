#[path = "../init.surql.rs"]
mod init;

pub mod config;

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use color_eyre::eyre::Report;
	use ysurreal::prelude::*;

	use super::init::INIT_SURQL;
	use yauth::{cmds::signup::SignUp, configs::TestingAuthConfig};

	#[test_log::test(tokio::test)]
	async fn db_sign_up() -> Result<(), Report> {
		let conn_config = TestingConfig::rand(INIT_SURQL.into());
		let db = start_testing_db(&conn_config).await?;
		conn_config.use_primary_ns_db(&db).await?;

		// signs into root to initialize the server
		// then immidiately signs out
		conn_config.root_sign_in(&db).await?;
		conn_config.init_query(&db).await?;
		db.invalidate().await?;

		let auth_config = TestingAuthConfig::new(&conn_config);
		let auth_control = auth_config.control_db(&db);
		// end setup

		// signs in as a scoped user
		auth_control.sign_up(&SignUp::testing_rand()).await?;

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn db_sign_in() -> Result<(), Report> {
		let conn_config = TestingConfig::rand(INIT_SURQL.into());
		let db = start_testing_db(&conn_config).await?;
		conn_config.use_primary_ns_db(&db).await?;

		// signs into root to initialize the server
		// then immidiately signs out
		conn_config.root_sign_in(&db).await?;
		conn_config.init_query(&db).await?;
		db.invalidate().await?;

		let auth_config = TestingAuthConfig::new(&conn_config);
		let auth_control = auth_config.control_db(&db);
		// end setup

		let credentials = SignUp::testing_rand();

		// signs in as a scoped user
		auth_control.sign_up(&credentials).await?;
		auth_control.invalidate().await?;

		// signs into already signed up user
		auth_config
			.control_db(&db)
			.sign_in(&credentials.into())
			.await?;

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn db_sign_up_twice_fails() -> Result<(), Report> {
		let conn_config = TestingConfig::rand(INIT_SURQL.into());
		let db = start_testing_db(&conn_config).await?;
		conn_config.use_primary_ns_db(&db).await?;

		// signs into root to initialize the server
		// then immidiately signs out
		conn_config.root_sign_in(&db).await?;
		conn_config.init_query(&db).await?;
		db.invalidate().await?;

		let auth_config = TestingAuthConfig::new(&conn_config);
		let auth_control = auth_config.control_db(&db);
		// end setup

		let credentials = SignUp::testing_rand();

		// signs in as a scoped user
		auth_control.sign_up(&credentials).await?;
		auth_control.invalidate().await?;

		let result = auth_control.sign_up(&credentials).await;
		assert!(result.is_err());

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn db_user_table_appends() -> Result<(), Report> {
		let conn_config = TestingConfig::rand(INIT_SURQL.into());
		let db = start_testing_db(&conn_config).await?;
		conn_config.use_primary_ns_db(&db).await?;

		// signs into root to initialize the server
		// then immidiately signs out
		conn_config.root_sign_in(&db).await?;
		conn_config.init_query(&db).await?;
		db.invalidate().await?;

		let auth_config = TestingAuthConfig::new(&conn_config);
		let auth_control = auth_config.control_db(&db);
		// end setup

		let credentials = SignUp::testing_rand();
		auth_control.sign_up(&credentials).await?;

		let users = auth_control.list_users().await?;
		assert_eq!(
			users.len(),
			1,
			"Users table should be able to see themselves after signing in"
		);

		// this actually errors since signed out users can't even see that the table exists!
		// auth_control.invalidate().await?;
		// assert_eq!(
		// 	auth_control.list_users().await?.len(),
		// 	0,
		// 	"Users can't see other users when signed out"
		// );

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn db_user_table_appends_multiple() -> Result<(), Report> {
		let conn_config = TestingConfig::rand(INIT_SURQL.into());
		let db = start_testing_db(&conn_config).await?;
		conn_config.use_primary_ns_db(&db).await?;

		// signs into root to initialize the server
		// then immidiately signs out
		conn_config.root_sign_in(&db).await?;
		conn_config.init_query(&db).await?;
		db.invalidate().await?;

		let auth_config = TestingAuthConfig::new(&conn_config);
		let auth_control = auth_config.control_db(&db);
		// end setup

		for i in 1..5 {
			let mut credentials = SignUp::testing_rand();
			credentials.email =
				yauth::types::Email::from_str(&format!("testgenerated{i}@me.com")).unwrap();

			// sign into scope account to create new user, then sign into root user
			auth_control.sign_up(&credentials).await?;
			auth_control.invalidate().await?;
			// basically signing into root here, since the default is no authentication
			conn_config.root_sign_in(&db).await?;

			let users = auth_control.list_users().await?;
			assert_eq!(
				users.len(),
				i,
				"Users table should have {i} entry after signing {i} people in",
			);

			auth_control.invalidate().await?
		}

		Ok(())
	}
}
