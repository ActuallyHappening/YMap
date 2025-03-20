use tracing::*;

/// Certs using `letsencrypt`
#[derive(Debug)]
pub struct Certs {
  fullchain: camino::Utf8PathBuf,
  privkey: camino::Utf8PathBuf,
}

impl Certs {
  pub fn get() -> color_eyre::Result<Option<Self>> {
    let env = std::env::var("JYD_CERTS").ok();
    match env {
      Some(str) => {
        let base_dir = camino::Utf8PathBuf::from(str);
        if !base_dir.is_dir() {
          return Err(color_eyre::eyre::eyre!(
            "Couldn't find certs directory at {}",
            base_dir.as_str()
          ));
        }
        let privkey_meta = std::fs::metadata(base_dir.join("privkey.pem"))?;
        info!(?privkey_meta, "Loaded certs");
        Ok(Some(Certs {
          fullchain: base_dir.join("fullchain.pem").canonicalize_utf8()?,
          privkey: base_dir.join("privkey.pem").canonicalize_utf8()?,
        }))
      }
      None => {
        info!("Since the 'JYD_CERTS' env variable was not present, not encrypting using SSL");
        Ok(None)
      }
    }
  }

  pub async fn rustls_config(&self) -> axum_server::tls_rustls::RustlsConfig {
    axum_server::tls_rustls::RustlsConfig::from_pem_file(self.fullchain(), self.private_key())
      .await
      .unwrap()
  }

  pub fn fullchain(&self) -> &camino::Utf8Path {
    &self.fullchain
  }

  pub fn private_key(&self) -> &camino::Utf8Path {
    &self.privkey
  }
}
