use crate::prelude::*;

pub struct TestingMemoryDB<C: Connection> {
	db: Surreal<C>,
	/// Drops this when out of scope, which is useful since this wraps a `surreal start` command
	cmd_handle: bossy::Handle,
}

impl<C: Connection> Drop for TestingMemoryDB<C> {
	fn drop(&mut self) {
		let cleanup = self.cmd_handle.kill();
		info!(message = "Cleaning up testing database...", ?cleanup);
	}
}

impl<C: Connection> std::ops::Deref for TestingMemoryDB<C> {
	type Target = Surreal<C>;

	fn deref(&self) -> &Self::Target {
		&self.db
	}
}

impl<C: Connection> std::ops::DerefMut for TestingMemoryDB<C> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.db
	}
}

/// Start a new in-memory database for **testing only**.
/// Switches to primary database and namespace implicitly.
pub async fn start_testing_db<Config>(config: &Config) -> Result<TestingMemoryDB<Any>, Report>
where
	Config: DBStartConfig + DBConnectRemoteConfig + DBRootCredentials,
{
	let cmd_args = config.get_cli_args();
	let surreal_bin_path = which("surreal").expect("Couldn't find surreal binary");
	let cmd_handle = bossy::Command::pure(&surreal_bin_path)
		.with_arg("start")
		.with_args(&cmd_args)
		.run()?;

	info!("Waiting for database to start up...");
	std::thread::sleep(std::time::Duration::from_secs(1));

	let db = config
		.connect_ws()
		.await
		.wrap_err("Couldn't connect to just-started CLI database")?;

	Ok(TestingMemoryDB { db, cmd_handle })
}
