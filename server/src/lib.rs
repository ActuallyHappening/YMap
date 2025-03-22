mod prelude {
  #![allow(unused_imports)]

  pub(crate) use crate::errors::*;
  pub(crate) use tracing::*;
}

mod errors {
  pub type Error = color_eyre::Report;
  pub type Result<T> = core::result::Result<T, Error>;

  pub use color_eyre::eyre::Context as _;
}

// mod api;
mod csp;
mod https;

pub use main::main;
mod main {
  use crate::prelude::*;

  use app::server_state::ServerAxumState;
  use axum::middleware::from_fn;
  use db::{Db, auth};
  use leptos::prelude::provide_context;
  use utils::prelude::bail;

  async fn connect_to_db() -> Result<Db<auth::Root>> {
    db::Db::connect_wss()
      .root(db::creds::Root::new())
      .finish()
      .await
      .inspect_err(|err| error!("Error connecting to db (debug error impl): {err:?}"))
      .wrap_err("Couldn't connect to db")
  }

  fn check_site_dir(site_dir: &camino::Utf8PathBuf) -> Result<()> {
    debug!(?site_dir);
    if !site_dir.exists() {
      bail!("Site directory does not exist");
    }
    if !site_dir.is_dir() {
      bail!("Site directory is not a directory");
    }
    Ok(())
  }

  pub async fn main() -> Result<()> {
    use axum::Router;
    use leptos_axum::{LeptosRoutes, generate_route_list};

    rustls::crypto::CryptoProvider::install_default(rustls::crypto::aws_lc_rs::default_provider())
      .expect("Crypto to work pls");

    let conf = leptos::config::get_configuration(None).wrap_err("Couldn't get leptos config")?;
    debug!(?conf, "Leptos config loaded");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(app::App);
    let site_dir = camino::Utf8PathBuf::from(leptos_options.site_root.as_ref());
    check_site_dir(&site_dir)?;

    let db = connect_to_db().await?;
    let state = {
      let stripe_api_key = env::stripe::AMPIDEXTEROUS_API_KEY;
      if stripe_api_key.contains("prod") {
        info!("Server is using a live Stripe API key for server fns");
        #[cfg(not(feature = "prod"))]
        color_eyre::eyre::bail!("Shouldn't be using a live Stripe API key if not in production!!");
      } else {
        info!(
          ?stripe_api_key,
          "Server is using a test Stripe API key for server fns"
        );
      }
      app::server_state::ServerAxumState {
        leptos_options: leptos_options.clone(),
        db: db.clone(),
        stripe: payments::server::ServerStripeController::new(stripe_api_key.into(), db),
      }
    };
    let state2 = state.clone();

    let dev_serve = std::env::var("JYD_DEV_SERVE").ok().is_some();
    let serve_dir = match dev_serve {
      true => {
        info!("Serving dev serve dir only");
        tower_http::services::ServeDir::new(site_dir)
      }
      false => {
        info!("Serving pre-compressed dir");
        tower_http::services::ServeDir::new(site_dir)
          .precompressed_br()
          .precompressed_gzip()
      }
    };

    let app = Router::<ServerAxumState>::new()
      // .nest(TopLevelRoutes::Api.get_path().as_ref(), api::router())
      // .with_state(api::ApiState::init().await?)
      .leptos_routes_with_context(&state, routes, move || provide_context(state2.clone()), {
        let leptos_options = leptos_options.clone();
        move || app::shell(leptos_options.clone())
      })
      .with_state(state)
      .fallback_service(serve_dir)
      .layer(from_fn(crate::csp::csp_headers));

    let certs = crate::https::Certs::get().wrap_err("Error getting certs")?;
    match certs {
      None => {
        info!("Website listenning on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
          .await
          .unwrap();
      }
      Some(certs) => {
        info!("Website listening on https://{}", &addr);
        if !certs.fullchain().exists() || !certs.private_key().exists() {
          let fullchain = certs.fullchain();
          let private_key = certs.private_key();
          warn!(
            ?fullchain,
            ?private_key,
            "Missing certs {} {}",
            fullchain.exists(),
            private_key.exists()
          );
        }

        axum_server::bind_rustls(addr, certs.rustls_config().await)
          .serve(app.into_make_service())
          .await
          .unwrap();
      }
    }

    Ok(())
  }
}
