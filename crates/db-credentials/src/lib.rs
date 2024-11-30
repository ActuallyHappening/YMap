static_toml::static_toml!(static DB_TOML = include_toml!("../../db.toml"););

pub const DB_USERNAME: &str = DB_TOML.username;
pub const DB_PASSWORD: &str = DB_TOML.password;
pub const DB_NAMESPACE: &str = DB_TOML.namespace;
pub const DB_DATABASE: &str = DB_TOML.database;
pub const START_COMMAND: &str = DB_TOML.start_command;
pub const CLEAN_COMMAND: &str = DB_TOML.clean_command;
pub const CONNECT_ENDPOINT: &str = DB_TOML.external_connect_endpoint;
