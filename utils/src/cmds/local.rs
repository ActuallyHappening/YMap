use std::collections::binary_heap;

use crate::prelude::*;

use super::{BinaryPath, IntoArgs};

pub struct Command {
  inner: bossy::Command,
}

impl Command {
  pub fn pure<A: IntoArgs>(args: A) -> Result<Self> {
    let binary_path = args.binary_path()?;
    if let BinaryPath::Remote(path) = binary_path {
      bail!(
        "Remote binary path not supported using `cmds::local::Command`: {}",
        path
      );
    }
    let inner = bossy::Command::pure(binary_path.as_str());
    Self::new(inner, args)
  }

  pub fn impure<A: IntoArgs>(args: A) -> Result<Self> {
    let binary_path = args.binary_path()?;
    if let BinaryPath::Remote(path) = binary_path {
      bail!(
        "Remote binary path not supported using `cmds::local::Command`: {}",
        path
      );
    }
    let inner = bossy::Command::impure(binary_path.as_str());
    Self::new(inner, args)
  }

  fn new<A: IntoArgs>(mut inner: bossy::Command, args: A) -> Result<Self> {
    for (key, value) in args.env_vars() {
      let key = key.as_ref();
      match value {
        Some(value) => {
          inner.add_env_var(key, value);
        }
        None => {
          let var = std::env::var(key).wrap_err(format!("Couldn't find env variable {}", key))?;
          inner.add_env_var(key, var);
        }
      }
    }
    inner.add_args(args.into_args());
    Ok(Command { inner })
  }

  pub fn run_and_wait(mut self) -> Result<()> {
    let exit = self
      .inner
      .run_and_wait()
      .wrap_err("Failed to run local Command")?;
    if !exit.success() {
      bail!("local Command failed");
    }

    Ok(())
  }
}
