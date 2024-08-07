use crate::prelude::*;
use yauth::prelude::*;
use ymap::auth::config::ProductionControllerConfig;
use ysurreal::prelude::*;

#[derive(Subcommand, Debug, Clone)]
pub enum AuthCommand {
	#[clap(alias = "signup")]
	SignUp {
		#[clap(flatten)]
		signup_options: yauth::cmds::signup::SignUp,
	},
	/// Only available with production credentials
	#[cfg(not(feature = "production"))]
	List,
}

pub async fn handle(
	config: &ProductionControllerConfig,
	command: &AuthCommand,
) -> Result<(), Report> {
	match command {
		AuthCommand::SignUp { signup_options } => {
			let db = config.connect_ws().await?;
			// db.use_ns(config.primary_namespace())
			// 	.use_db(config.primary_database())
			// 	.await?;

			config.control_db(&db).sign_up(signup_options).await?;

			Ok(())
		}
		#[cfg(not(feature = "production"))]
		AuthCommand::List => {
			let db = config.connect_ws().await?;
			// logs in as root to list all of them, else IAM error
			config.root_sign_in(&db).await?;
			info!("Listing users");
			let users = config.control_db(&db).list_users().await?;

			println!("Found {} users:", users.len());
			for user in users {
				println!("- {:?}", user);
			}

			Ok(())
		}
	}
}
