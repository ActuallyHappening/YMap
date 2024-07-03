#[path = "../init.surql.rs"]
mod init;
use init::INIT_SURQL;

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use color_eyre::eyre::Report;
	use ysurreal::{config::start_in_memory, configs::TestingMem};

	use super::INIT_SURQL;
	use yauth::{prelude::*, signup::Signup};

	#[test_log::test(tokio::test)]
	async fn sign_up_works() -> Result<(), Report> {
		let conn_config = TestingMem::rand(INIT_SURQL.to_string());
		let db = start_in_memory(&conn_config).unwrap().await?;
		let auth_config = yauth::configs::TestingAuthConfig::new(&conn_config);

		let debug_info = db.query("INFO FOR db").await?;
		trace!(?debug_info);

		auth_config
			.sign_up(
				&db,
				&Signup {
					username: "my username 123".parse().unwrap(),
					password: "my password 123".parse().unwrap(),
					email: "me@mydomain.com".parse().unwrap(),
				},
			)
			.await?;

		Ok(())
	}
}
