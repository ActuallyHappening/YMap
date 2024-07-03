use crate::prelude::*;

#[derive(Args, Debug, Clone)]
pub struct ProductionConfig {
	#[arg(long, default_value_t = { Secrets::ssh_name() })]
	ssh_name: String,
}


#[derive(Subcommand, Debug, Clone)]
pub enum ProductionCommand {
	Kill,
	Clean,
	Start,
	Import,
	Connect,
}

pub async fn handle(config: &ProductionConfig, command: &ProductionCommand) -> Result<(), Report> {
	Ok(())
}