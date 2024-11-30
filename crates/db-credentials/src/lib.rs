static_toml::static_toml!(static DB_TOML = include_toml!("../../db.toml"););

pub const DB_USERNAME: &str = DB_TOML.username;
pub const DB_PASSWORD: &str = DB_TOML.password;
pub const DB_NAMESPACE: &str = DB_TOML.namespace;
pub const DB_DATABASE: &str = DB_TOML.database;
pub const START_COMMAND: &str = DB_TOML.start_command;
/// Returns a number of running instances of surreal
pub const SEARCH_COMMAND: &str = DB_TOML.search_command;
/// Returns the number of killed processes / instances of surreal
pub const KILL_COMMAND: &str = DB_TOML.kill_command;
pub const CLEAN_COMMAND: &str = DB_TOML.clean_command;
/// Ipv4 e.g. 192.168.0.63:8000 or domain e.g. example.com:8000
pub const CONNECT_ENDPOINT: &str = DB_TOML.external_connect_endpoint;
