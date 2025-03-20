#![expect(dead_code)]

use utils::{cmds::ssh, paths::RemoteFile, tools::cp};

use crate::prelude::*;

/// /etc/letsencrypt
pub(crate) struct LetsEncryptCertsDir(RemoteDir);

impl ServerPath for LetsEncryptCertsDir {
  fn get_dir(&self) -> &RemoteDir {
    &self.0
  }
}

/// .../certs
pub struct Certs(RemoteDir);

impl ServerPath for Certs {
  fn get_dir(&self) -> &RemoteDir {
    &self.0
  }
}

impl LetsEncryptCertsDir {
  pub(crate) fn new() -> LetsEncryptCertsDir {
    LetsEncryptCertsDir(RemoteDir::new_unchecked(
      "/etc/letsencrypt/live/jordanyatesdirect.com".into(),
    ))
  }

  fn privkey(&self) -> RemoteFile {
    self.file("privkey.pem")
  }

  fn fullchain(&self) -> RemoteFile {
    self.file("fullchain.pem")
  }
}

#[extension(pub(crate) trait ServerCertsPathExt)]
impl project_paths::server::Root {
  fn certs(&self) -> Certs {
    Certs(self.dir("certs"))
  }
}

impl Certs {
  fn privkey(&self) -> RemoteFile {
    self.file("privkey.pem")
  }

  fn fullchain(&self) -> RemoteFile {
    self.file("fullchain.pem")
  }
}

pub async fn copy_certs(server: &ssh::Session) -> Result<()> {
  let letsencrypt_certs_dir = LetsEncryptCertsDir::new();
  let certs_dir = server.paths().certs();

  server
    .command(cp::ServerFileArgs {
      from: letsencrypt_certs_dir.privkey(),
      to: certs_dir.privkey(),
    })?
    .run_and_wait()
    .await?;
  server
    .command(cp::ServerFileArgs {
      from: letsencrypt_certs_dir.fullchain(),
      to: certs_dir.fullchain(),
    })?
    .run_and_wait()
    .await?;

  Ok(())
}
