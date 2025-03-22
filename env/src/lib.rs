#![allow(unused_imports)]

mod prelude {
  pub(crate) use include_toml_key::include_toml_key;
}

pub use main::*;
mod main {
  #[cfg(feature = "db")]
  pub mod db {
    use crate::prelude::*;

    /// TODO: make server auth less extreme
    #[cfg(feature = "db-root-creds")]
    pub const ROOT_PASS: &str = include_toml_key!("JYD_DB_ROOT_PASS");
  }

  // pub mod server {
  //   use crate::prelude::*;

  //   #[cfg(feature = "server")]
  //   pub const SSH_NAME: &str = include_toml_key!("JYD_SSH_NAME");
  //   #[cfg(feature = "server")]
  //   pub const ROOT_DIR: &str = include_toml_key!("JYD_REMOTE_DIR");
  // }
}
