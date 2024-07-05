use crate::{
		config::{DBConnectRemoteConfig, DBRootCredentials, DBStartConfig},
		prelude::*,
	};

	/// Constructs an in-memory database for testing purposes.
	pub struct TestingMem {
		pub port: u16,
		pub username: String,
		pub password: String,
		pub init_surql: String,
	}

	impl TestingMem {
		pub fn new(port: u16, init_surql: String) -> Self {
			TestingMem {
				port,
				username: String::from("testing-username"),
				password: String::from("testing-password"),
				init_surql,
			}
		}

		/// Generates a [TestingMem] with a random port between 10000 and 20000.
		pub fn rand(init_surql: String) -> Self {
			let mut rand = rand::thread_rng();
			TestingMem::new(rand.gen_range(10000..20000), init_surql)
		}
	}

	impl DBStartConfig for TestingMem {
		fn bind_port(&self) -> u16 {
			self.port
		}

		fn db_type(&self) -> crate::config::StartDBType {
			crate::config::StartDBType::Mem
		}

		fn init_surql(&self) -> String {
			self.init_surql.clone()
		}
	}

	impl DBRootCredentials for TestingMem {
		fn root_username(&self) -> String {
			"root-testing".into()
		}

		fn root_password(&self) -> String {
			"testing password".into()
		}
	}

	impl DBConnectRemoteConfig for TestingMem {
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