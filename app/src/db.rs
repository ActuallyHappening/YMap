use db::{
  auth,
  users::{self},
};

use crate::{errors::components::Pre, prelude::*};

/// Lives at the (reactive) top level of the app,
/// so all of APIs that set new signals on this must use the same
/// reactive owner as the root
pub struct DbState {
  /// All signal stored in this should have this as the owner
  root_owner: Owner,
  conn: Result<DbConn, ConnectErr>,
}

impl DbState {
  /// Should be put on the root reactive, above everything else that
  /// consumes it.
  /// Does not connect
  // techniquelly doesn't need to take this as an argument,
  // but for clarity this seems better
  pub(crate) fn new(this_owner: Owner) -> Self {
    Self {
      root_owner: this_owner,
      conn: Err(ConnectErr::default()),
    }
  }

  pub fn root_owner(&self) -> Owner {
    self.root_owner.clone()
  }
}

/// I would really like avoid this being `Clone`,
/// but I must because when connecting this lives in an `Action` `Signal`
#[derive(Debug, Clone, thiserror::Error, Default)]
pub enum ConnectErr {
  /// Marks for reconnection as a guest when first starting.
  #[error("Waiting to connect to db")]
  #[default]
  WaitingInitial,

  /// Used to logout
  #[error("Waiting to connect after logging out")]
  WaitingForGuestConn,

  #[error("Waiting for login")]
  WaitingForLogin(users::SignInUser),

  #[error("Waiting for signup")]
  WaitingForSignup(users::SignUpUser),

  /// Usually network
  #[error("{0}")]
  Underlying(GenericError<db::connect::ConnectErr>),

  #[error("{0}")]
  Initialization(GenericError<InitializationErr>),
}

impl IntoRender for &ConnectErr {
  type Output = AnyView;

  fn into_render(self) -> Self::Output {
    let generic = GenericError::from_ref(self);
    trace!(?self, "Rendering &app::db::ConnectErr");
    view! {
      <p> { generic.to_string() }</p>
      <Pre err=generic />
    }
    .into_any()
  }
}

#[derive(Debug, thiserror::Error)]
pub enum InitializationErr {
  #[error("Error initializing user database: {0}")]
  UserSelect(#[from] db::users::SelectUserErr),

  #[error("Error initializing order database: {0}")]
  OrderSelect(#[from] db::orders::SelectOrderErr),

  #[error("Error initializing cartridge database: {0}")]
  CartridgeSelect(#[from] db::cartridges::SelectCartridgeErr),

  #[error("Error initializing review database: {0}")]
  ReviewsSelect(#[from] db::reviews::SelectReviewErr),
}

impl From<InitializationErr> for ConnectErr {
  fn from(value: InitializationErr) -> Self {
    ConnectErr::Initialization(GenericError::from(value))
  }
}

impl From<db::connect::ConnectErr> for ConnectErr {
  fn from(value: db::connect::ConnectErr) -> Self {
    ConnectErr::Underlying(GenericError::from(value))
  }
}

pub use _connecting::*;
mod _connecting {
  use db::{connect::DbConnectBuilder, creds};

  use super::*;

  impl DbState {
    /// This non-reactively mutates the database connection state.
    /// To notify children of this change, you should therefore
    /// store [DbState] in some top-level [`Signal`].
    pub fn reconnect(&mut self, conn: impl std::borrow::Borrow<Result<DbConn, ConnectErr>>) {
      info!("Updating db connection state");
      self.conn = conn.borrow().clone();
    }
  }

  pub struct Reconnect {
    pub root_owner: Owner,
    pub db: DbConnectBuilder<creds::Guest>,
  }

  impl Reconnect {
    pub fn start(root_owner: Owner) -> Self {
      Reconnect {
        root_owner,
        db: db::Db::connect_wss(),
      }
    }
  }
}

pub use _connected::*;
mod _connected {
  use super::*;

  /// getters
  impl DbState {
    pub fn conn_old(&self) -> Result<&DbConn, &ConnectErr> {
      self.conn.as_ref()
    }

    pub fn conn(&self) -> Result<&DbConn, AppError> {
      self.conn.as_ref().err_generic_ref().map_err(AppError::from)
    }
  }

  /// setters
  impl DbState {
    pub fn login(&mut self, creds: users::SignInUser) {
      self.conn = Err(ConnectErr::WaitingForLogin(creds))
    }

    pub fn signup(&mut self, creds: users::SignUpUser) {
      self.conn = Err(ConnectErr::WaitingForSignup(creds))
    }

    pub fn logout(&mut self) {
      self.conn = Err(ConnectErr::WaitingForGuestConn)
    }
  }

  #[derive(Clone)]
  pub enum DbConn {
    Guest(_guest::DbGuest),
    User(dbusers::DbUser),
  }

  impl DbConn {
    pub fn downgrade(self) -> _guest::DbGuest {
      match self {
        DbConn::Guest(guest) => guest,
        DbConn::User(user) => user.into(),
      }
    }
  }
}

pub use _guest::*;
mod _guest {
  use super::*;

  #[derive(Clone)]
  pub struct DbGuest {
    root_owner: Owner,
    db: Db<auth::NoAuth>,
    cartridges: ReactiveCartridges<auth::NoAuth>,
    orders: ReactiveOrders<auth::NoAuth>,
    reviews: ReactiveReviews<auth::NoAuth>,
  }

  impl From<DbUser> for DbGuest {
    fn from(user: DbUser) -> Self {
      Self {
        cartridges: user.cartridges.downgrade(),
        orders: user.orders.downgrade(),
        reviews: user.reviews.downgrade(),
        root_owner: user.root_owner,
        db: user.db.downgrade(),
      }
    }
  }

  /// getters
  impl DbConn {
    pub fn guest(&self) -> Option<&DbGuest> {
      match self {
        DbConn::Guest(guest) => Some(guest),
        _ => None,
      }
    }
  }

  /// constructors
  impl DbGuest {
    pub async fn new(root_owner: Owner, db: Db<auth::NoAuth>) -> Result<Self, InitializationErr> {
      Ok(Self {
        cartridges: ReactiveCartridges::new(&root_owner, &db).await?,
        orders: ReactiveOrders::new(&root_owner, &db).await?,
        reviews: ReactiveReviews::new(&root_owner, &db).await?,
        root_owner,
        db,
      })
    }
  }

  /// getters
  impl DbGuest {
    pub fn cartridges(&self) -> &ReactiveCartridges<auth::NoAuth> {
      &self.cartridges
    }

    pub fn orders(&self) -> &ReactiveOrders<auth::NoAuth> {
      &self.orders
    }

    pub fn reviews(&self) -> &ReactiveReviews<auth::NoAuth> {
      &self.reviews
    }
  }
}

pub use dbusers::*;
pub mod dbusers;

pub use cartridges::*;
mod cartridges;

pub use orders::*;
pub mod orders;

pub use reviews::*;
pub mod reviews;
