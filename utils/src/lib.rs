pub mod prelude {
  pub use color_eyre::eyre::{WrapErr, bail, eyre};
  pub use tracing::{debug, error, info, trace, warn};

  pub use color_eyre;
  pub use thiserror;
  pub use tokio;
  pub use tracing_subscriber;

  #[cfg(feature = "cli")]
  pub use clap::{self, Parser};
}

pub mod tracing;

pub fn install_crypto() -> color_eyre::Result<()> {
  use crate::prelude::*;

  rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .map_err(|_| eyre!("Couldn't install aws-lc-rs default crypto provider"))
}
