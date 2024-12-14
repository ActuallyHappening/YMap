static_toml::static_toml!(static DB_TOML = include_toml!("../../db.toml"););

pub const DB_USERNAME: &str = DB_TOML.username;
pub const DB_PASSWORD: &str = DB_TOML.password;
pub const DB_NAMESPACE: &str = DB_TOML.namespace;
pub const DB_DATABASE: &str = DB_TOML.database;

pub fn start_command() -> impl Iterator<Item = &'static str> {
    DB_TOML.start_command.iter().cloned()
}

const SEARCH_COMMAND: [&str; 3] = DB_TOML.search_command;
/// Returns a number of running instances of surreal
pub fn search_command() -> impl Iterator<Item = &'static str> {
    SEARCH_COMMAND.iter().cloned()
}

const KILL_COMMAND: [&str; 3] = DB_TOML.kill_command;
/// Returns the number of killed processes / instances of surreal
pub fn kill_command() -> impl Iterator<Item = &'static str> {
    KILL_COMMAND.iter().cloned()
}

const CLEAN_COMMAND: [&str; 3] = DB_TOML.clean_command;
pub fn clean_command() -> impl Iterator<Item = &'static str> {
    CLEAN_COMMAND.iter().cloned()
}

/// Ipv4 e.g. 192.168.0.63:8000 or domain e.g. example.com:8000
pub const CONNECT_ENDPOINT: &str = DB_TOML.external_connect_endpoint;
