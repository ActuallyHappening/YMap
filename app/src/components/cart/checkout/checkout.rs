//! https://jordanyatesdirect.com/cart/checkout?successful-checkout-session=cs_test_a1UG4OVrc1223uyFxLFUpe0sZvVbCU9GdzPke4LD6dzMYb107jFAZwBtez

use db::{orders::OrderBuilder, users};
use leptos::server_fn::codec::Json;

use crate::{
  components::cart::state::GlobalCartState, db::DbState, prelude::*, stripe::StripeSession,
};

use err::ComponentError;
mod err {
  use db::users;

  use crate::{errors::components::Pre, prelude::*};

  #[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
  pub enum ComponentError {
    #[error("Loading stripe checkout page ...")]
    Loading,

    #[error("Waiting to connect to db: {0}")]
    DbDisconnected(#[from] GenericError<crate::db::ConnectErr>),

    #[error("You must be logged in")]
    NotLoggedIn,

    #[error("Loading user info from db")]
    UserInfoLoading,

    #[error("Couldn't load user info from db: {0}")]
    LoadingUserInfo(#[from] GenericError<users::SelectUserErr>),

    #[error("Server function error, probably a network issue, please reload and try again")]
    ServerFnError(ServerFnError),

    #[error("Unable to place order: {0}")]
    PlaceOrder(#[from] GenericError<db::orders::mutations::PlaceOrderError>),

    #[error("Unable to create checkout session using Stripe")]
    Payments(#[from] GenericError<payments::ServerError>),

    #[error("Unable to place payment order: {0}")]
    PromoteOrderCheckoutId(#[from] GenericError<db::orders::promotions::PromoteOrderError>),
  }

  impl IntoRender for ComponentError {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
      view! {
        <p>{format!("{}", self)}</p>
        <Pre err=self />
      }
      .into_any()
    }
  }

  impl From<ServerFnError> for ComponentError {
    fn from(value: ServerFnError) -> Self {
      Self::ServerFnError(value)
    }
  }
}

pub fn CheckoutPage() -> impl IntoView {
  debug!("Rendering CheckoutPage");
  let cart = GlobalCartState::from_context();
  let db_conn = DbState::from_context();

  // magic!
  let client_secret = LocalResource::new(move || {
    debug!("Running client_secret api call");
    let cart = cart.read_sig().get().into_plain();
    async move {
      // reactively subscribes to db changes
      // and user live updates
      let user: users::User = {
        let user = db_conn
          .read()
          .conn_old()
          .err_generic_ref()?
          .old_user()
          .ok_or(ComponentError::NotLoggedIn)?
          .users()
          .select()
          .read();
        user
          .as_ref()
          .ok_or(ComponentError::UserInfoLoading)?
          .as_ref()
          .err_generic_ref()?
          .clone()
      };
      let order = OrderBuilder {
        user: user.id(),
        cart,
      };
      create_session(order, user.email()).await
    }
  });
  let fallback = move || ComponentError::Loading.into_render();
  let ui = move || {
    Suspend::new(async move {
      match client_secret.await {
        Ok(session) => view! { <crate::stripe::EmbeddedCheckout session=session /> }.into_any(),
        Err(err) => err.into_render().into_any(),
      }
    })
  };
  view! { <Suspense fallback>{ui}</Suspense> }
}

pub async fn create_session(
  order: OrderBuilder,
  email: email_address::EmailAddress,
) -> Result<StripeSession, ComponentError> {
  _server_create_session(order, email).await?
}

/// TODO: Check user authentication, so that somebody can't make unpaid orders
/// on anybodies behalf?
#[server(prefix = "/api/checkout", endpoint = "/create-session", input = Json, output = Json)]
pub async fn _server_create_session(
  order: OrderBuilder,
  email: email_address::EmailAddress,
) -> Result<Result<StripeSession, ComponentError>, ServerFnError> {
  Ok(server_create_session(order, email).await)
}

#[cfg(feature = "ssr")]
async fn server_create_session(
  order: OrderBuilder,
  email: email_address::EmailAddress,
) -> Result<StripeSession, ComponentError> {
  let db = crate::server_state::ServerAxumState::from_context().db;
  let db = db.orders();
  let order = db.place_order(order.clone()).await.err_generic()?;

  // links to stripe
  let stripe = crate::server_state::ServerAxumState::from_context().stripe;
  let checkout_session = stripe
    .create_checkout_session(order.clone(), email.to_string())
    .await
    .err_generic()?;
  let client_secret = checkout_session.client_secret().err_generic()?;

  let order = db
    .update_checkout_id(order, checkout_session.id())
    .await
    .err_generic()?;

  Ok(StripeSession {
    client_secret,
    order_id: order.id(),
  })
}
