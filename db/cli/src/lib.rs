#![allow(async_fn_in_trait)]

pub mod prelude {
  #![allow(unused_imports)]

  pub(crate) use db::prelude::*;
  pub(crate) use utils::prelude::*;

  pub use crate::pueue::PueueExt as _;
}

mod cli;

pub mod pueue {
  use crate::{args::ProdSurreal, prelude::*};

  impl pueue::ProcInfo for ProdSurreal {
    fn group() -> &'static str {
      pueue::PROD_GROUP
    }

    fn label() -> &'static str {
      "db"
    }
  }

  #[extension(pub trait PueueExt)]
  impl pueue::Session {
    async fn db(&mut self, args: ProdSurreal) -> Result<pueue::TaskHandle<'_, ProdSurreal>> {
      self.handle(args).await
    }
  }
}

pub mod args {
  use utils::cmds::IntoRemoteArgs;

  use crate::prelude::*;

  /// TODO: generalize this, and reference Rust symbols
  /// instead of hard coding
  pub struct ProdSurreal {}

  impl ProdSurreal {
    pub fn new() -> Self {
      Self {}
    }
  }

  impl IntoRemoteArgs for ProdSurreal {
    fn binary_name() -> String {
      "surreal".into()
    }

    fn binary_path(&self) -> Result<RemoteFile> {
      Ok(RemoteFile::new_unchecked(Utf8PathBuf::from(
        "/usr/local/bin/surreal",
      )))
    }

    fn into_args(self) -> Vec<String> {
      vec![
        "start".into(),
        "surrealkv:///home/ah/jyd/db".into(),
        "--bind=0.0.0.0:42069".into(),
        "--strict".into(),
        "--allow-guests".into(),
        "--log".into(),
        "debug".into(),
        "--web-crt=/etc/letsencrypt/live/jordanyatesdirect.com/fullchain.pem".into(),
        "--web-key=/etc/letsencrypt/live/jordanyatesdirect.com/privkey.pem".into(),
      ]
    }
  }
}

pub mod main {
  use crate::cli::*;
  use crate::prelude::*;

  pub async fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command() {
      Command::Export { path } => {
        let db = db::Db::connect_https()
          .root(db::creds::Root::new())
          .finish()
          .await?;

        // create empty file at path
        drop(
          std::fs::File::create(&path)
            .wrap_err("Couldn't create placeholder empty file")
            .with_note(|| path.to_string())?,
        );
        db.export(path).await?;

        info!("Successfully exported");

        Ok(())
      }
      Command::Import { path } => {
        let db = db::Db::connect_https()
          .root(db::creds::Root::new())
          .finish()
          .await?;

        if !path.is_file() {
          bail!("Path must be a file");
        }
        if std::fs::read_to_string(&path)?.is_empty() {
          bail!("Path shouldn't be empty obviously");
        }

        db.import(path).await?;

        info!("Successfully imported");

        Ok(())
      }
      Command::Auth(Auth::SignUp {
        email,
        name,
        password,
      }) => {
        let user = db::users::SignUpUser {
          email,
          name,
          plaintext_password: password,
        };

        let db = db::Db::connect_wss().user().signup(user).finish().await?;
        let user = db.users().select().initial().await?;

        info!("Signed up user: {:?}", user);

        Ok(())
      }
      Command::Auth(Auth::SignIn { email, password }) => {
        let user = db::users::SignInUser {
          email,
          plaintext_password: password,
        };

        let db = db::Db::connect_wss().user().signin(user).finish().await?;
        let user = db.users().select().initial().await?;

        info!("Signed in user: {:?}", user);

        Ok(())
      }
    }
  }
}
