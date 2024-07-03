use crate::prelude::*;
use yauth::prelude::*;
use ymap::auth::config::ProductionConfig;

#[derive(Subcommand, Debug, Clone)]
pub enum AuthCommand {
	#[clap(alias = "signup")]
	SignUp {
		#[clap(flatten)]
		signup_options: yauth::signup::SignUp,
	},
	List,
}

pub async fn handle(config: &ProductionConfig, command: &AuthCommand) -> Result<(), Report> {
	match command {
		AuthCommand::SignUp { signup_options } => {
			let db = config.connect_ws().await?;
			// db.use_ns(config.primary_namespace())
			// 	.use_db(config.primary_database())
			// 	.await?;

			config.sign_up(&db, signup_options).await?;

			Ok(())
		}
		AuthCommand::List => {
			let db = config.connect_ws().await?;
			// logs in as root to list all of them, else IAM error
			config.root_sign_in(&db).await?;
			config.list_users(&db).await?;

			Ok(())
		}
	}
}
