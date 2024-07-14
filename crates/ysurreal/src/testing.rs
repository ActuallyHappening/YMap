use crate::prelude::*;

pub struct TestingMemoryDB<C: Connection> {
	db: Surreal<C>,
	/// Drops this when out of scope, which is useful since this wraps a `surreal start` command
	cmd_handle: bossy::Handle,
}

impl<C: Connection> Drop for TestingMemoryDB<C> {
	fn drop(&mut self) {
		let cleanup = self.cmd_handle.kill();
		info!(
			message = "Cleaning up testing database...",
			?cleanup,
			note = "Ignore the next error from bossy, we definitely intend to leak this process"
		);
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
/// **Does not sign in for you**
pub async fn start_testing_db<Config>(config: &Config) -> Result<TestingMemoryDB<Any>, Report>
where
	Config: DBStartConfig + DBConnectRemoteConfig + DBRootCredentials,
{
	let cmd_args = config.get_unescaped_cli_args();
	let surreal_bin_path = which("surreal").expect("Couldn't find surreal binary");
	trace!(
		message = "Starting local surreal client with `surreal start`",
		?cmd_args
	);
	let cmd_handle = bossy::Command::pure(&surreal_bin_path)
		.with_arg("start")
		.with_args(&cmd_args)
		.with_args(["--no-banner", "--log=warn"])
		.run()?;

	debug!("Waiting for database to start up...");
	std::thread::sleep(std::time::Duration::from_secs(2));

	let db = config
		.connect_ws()
		.await
		.wrap_err("Couldn't connect to just-started CLI database")?;
	debug!("Connected to testing database.");

	config.use_primary_ns_db(&db).await?;

	Ok(TestingMemoryDB { db, cmd_handle })
}

use crate::config::{DBConnectRemoteConfig, DBRootCredentials, DBStartConfig};

/// Constructs an in-memory database for testing purposes.
pub struct TestingConfig {
	pub port: u16,
	pub username: String,
	pub password: String,
	pub init_surql: String,
}

impl TestingConfig {
	pub fn new(port: u16, init_surql: String) -> Self {
		TestingConfig {
			port,
			username: String::from("testing-username"),
			password: String::from("testing-password"),
			init_surql,
		}
	}

	fn random_port() -> u16 {
		let addr = std::net::TcpListener::bind((std::net::Ipv4Addr::from([127, 0, 0, 1]), 0))
			.expect("Couldn't bind to random port");
		let port = addr.local_addr().unwrap().port();
		trace!(?port, "Random port chosen for testing");
		port
	}

	/// Generates a [TestingMem] with a random port chosen by the OS to avoid conflicts
	pub fn rand(init_surql: String) -> Self {
		// let mut rand = rand::thread_rng();
		// TestingConfig::new(rand.gen_range(10000..20000), init_surql)
		let port_num = TestingConfig::random_port();
		TestingConfig::new(port_num, init_surql)
	}
}

impl DBStartConfig for TestingConfig {
	fn bind_port(&self) -> u16 {
		self.port
	}

	fn bind_host(&self) -> String {
		"127.0.0.1".into()
	}

	fn db_type(&self) -> crate::config::StartDBType {
		crate::config::StartDBType::Mem
	}

	fn init_surql(&self) -> String {
		self.init_surql.clone()
	}
}

impl DBRootCredentials for TestingConfig {
	fn root_username(&self) -> String {
		"root-testing".into()
	}

	fn root_password(&self) -> String {
		"testing password".into()
	}
}

impl DBConnectRemoteConfig for TestingConfig {
	fn primary_namespace(&self) -> String {
		// so that init.surql matches production
		"production".into()
	}

	fn primary_database(&self) -> String {
		// so that init.surql matches production
		"production".into()
	}

	fn connect_host(&self) -> String {
		"localhost".into()
	}

	fn connect_port(&self) -> u16 {
		self.port
	}
}
