pub mod prelude;

#[cfg(feature = "cli-support")]
pub fn install_crypto() -> color_eyre::Result<()> {
  use crate::prelude::*;

  rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .map_err(|_| eyre!("Couldn't install aws-lc-rs default crypto provider"))
}

pub mod background;
pub mod cmds;
pub mod paths;
pub mod project_paths;
pub mod tools;
pub mod tracing;
