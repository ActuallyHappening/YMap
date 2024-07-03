use ysurreal::config::DBConnectRemoteConfig;

	use crate::config::DBAuthConfig;

	#[derive(Debug)]
	pub struct TestingAuthConfig<InnerConfig> {
		testing_connection: InnerConfig,
	}

	impl<InnerConfig> TestingAuthConfig<InnerConfig> {
		pub fn new(testing_connection: InnerConfig) -> Self {
			TestingAuthConfig { testing_connection }
		}
	}

	impl<InnerConfig> DBAuthConfig for TestingAuthConfig<InnerConfig>
	where
		InnerConfig: DBConnectRemoteConfig,
	{
		fn users_table(&self) -> String {
			"user".into()
		}

		fn users_scope(&self) -> String {
			"end_user".into()
		}
	}

	impl<InnerConfig> DBConnectRemoteConfig for TestingAuthConfig<InnerConfig>
	where
		InnerConfig: DBConnectRemoteConfig,
	{
		fn primary_database(&self) -> String {
			self.testing_connection.primary_database()
		}

		fn primary_namespace(&self) -> String {
			self.testing_connection.primary_namespace()
		}

		fn connect_host(&self) -> String {
			self.testing_connection.connect_host()
		}

		fn connect_port(&self) -> u16 {
			self.testing_connection.connect_port()
		}
	}