static_toml::static_toml!(static DB_TOML = include_toml!("../../db.toml"););

pub const DB_USERNAME: &str = DB_TOML.username;
pub const DB_PASSWORD: &str = DB_TOML.password;
pub const DB_NAMESPACE: &str = DB_TOML.namespace;
pub const DB_DATABASE: &str = DB_TOML.database;
