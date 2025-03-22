use db::{creds, users};
use surrealdb::opt::auth::Jwt;

use crate::{
  db::{ConnectErr, DbConn, DbGuest, DbState, DbUser, Reconnect},
  prelude::*,
};

stylance::import_crate_style!(db_conn_style, "src/components/db_conn/db_conn.module.scss");

pub fn DbConnectionStatus() -> impl IntoView {
  debug!("Rendering DbConnectionStatus");

  let db_sig = DbState::from_context();
  // nothing should reactively depend on these signals
  // because we are treating them as links to the outside world,
  // so just .get_untracked everywhere
  let (read_jwt, write_jwt, _) = leptos_use::storage::use_local_storage_with_options::<
    Option<String>,
    codee::string::JsonSerdeCodec,
  >("auth", leptos_use::storage::UseStorageOptions::default());

  #[derive(Clone)]
  enum ConnectAs {
    Default,
    Guest,
    SignIn(users::SignInUser),
    SignUp(users::SignUpUser),
  }

  impl ConnectAs {
    fn from(state: &DbState) -> Option<Self> {
      match state.conn_old() {
        Ok(_) => None,
        Err(ConnectErr::WaitingInitial) => Some(ConnectAs::Default),
        Err(ConnectErr::WaitingForGuestConn) => Some(ConnectAs::Guest),
        Err(ConnectErr::WaitingForLogin(login)) => Some(ConnectAs::SignIn(login.clone())),
        Err(ConnectErr::WaitingForSignup(signup)) => Some(ConnectAs::SignUp(signup.clone())),
        Err(_) => Some(ConnectAs::Default),
      }
    }
  }

  // gets jwt on connect
  let connect = async move |Reconnect { root_owner, db },
                            connect_as: ConnectAs|
              -> Result<DbConn, ConnectErr> {
    Ok(match connect_as {
      ConnectAs::Default => {
        // read from jwt
        if let Some(jwt) = read_jwt.get_untracked() {
          debug!("Found a JWT, authenticating as user");
          let db = db
            .user()
            .authenticate(creds::AuthenticateUser(Jwt::from(jwt)))
            .finish()
            .await;

          match db {
            Ok(db) => DbConn::User(DbUser::new(root_owner, db).await?),
            Err(err) => {
              if let db::connect::ConnectErr::CouldntAuthenticate {
                err: surrealdb::Error::Api(surrealdb::error::Api::Query(err)),
                ..
              } = &err
              {
                // a heuristic, idk any good ways of knowing this atm
                if format!("{:?}", err).contains("token has expired") {
                  return Err(ConnectErr::WaitingForGuestConn);
                }
              }
              return Err(err.into());
            }
          }
        } else {
          debug!("Didn't find a JWT, authenticating as guest");
          let db = db.finish().await?;
          DbConn::Guest(DbGuest::new(root_owner, db).await?)
        }
      }
      ConnectAs::Guest => {
        // delete jwt
        debug!("Deleting JWT");
        if write_jwt.try_set(None).is_some() {
          warn!(
            message = "Couldn't mutate local JWT",
            note = "This will occur if you change pages / re-render DbConnectionStatus while in an auth change"
          );
        }

        let db = db.finish().await?;

        DbConn::Guest(DbGuest::new(root_owner, db).await?)
      }
      ConnectAs::SignIn(login) => {
        let db = db.user().signin(login).finish().await?;
        DbConn::User(DbUser::new(root_owner, db).await?)
      }
      ConnectAs::SignUp(signup) => {
        let db = db.user().signup(signup).finish().await?;
        DbConn::User(DbUser::new(root_owner, db).await?)
      }
    })
  };
  let new_conn = LocalResource::new(move || {
    // depends on current state
    let state = db_sig.read();
    let connect_as = ConnectAs::from(state.deref());
    let root_owner = state.root_owner();
    let reconnect = Reconnect::start(root_owner);
    async move {
      match connect_as {
        None => None,
        Some(connect_as) => Some(connect(reconnect, connect_as).await),
      }
    }
  });
  // set jwt on auth as appropriate
  Effect::new(move || {
    let db = db_sig.read();
    let db = db.conn_old();
    match db {
      Ok(DbConn::User(auth)) => {
        info!("Detected user auth, setting user auth jwt");
        write_jwt.set(Some(auth.auth().jwt().into_insecure_token()));
      }
      Ok(DbConn::Guest(_)) => {
        // is done when explicitely connecting as guest
      }
      Err(_err) => {
        // nothing
      }
    }
  });

  let ok = move |msg: &'static str| view! { <p> { msg } </p> };
  let current = move || {
    let state = db_sig.read();
    let state = state.conn_old();

    state.map_view(|db_conn| match db_conn {
      DbConn::User(_) => ok("Connected (user)"),
      DbConn::Guest(_) => ok("Connected (guest)"),
    })
  };
  let suspend = move || {
    trace!("Suspence rendering");
    Suspend::new(async move {
      trace!("Suspend started");
      new_conn.await.map(|new_conn| {
        trace!("Suspend finished");
        // reactively update database connection
        info!("New db conn loaded!");
        db_sig.write().reconnect(new_conn);
      })
    })
  };

  let fallback = move || view! { <pre> "Establishing new connection ..." </pre> };
  view! {
    { current }
    <Suspense fallback>
      { suspend }
    </Suspense>
  }
}
