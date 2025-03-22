use stripe_js::embedded_checkout::InitOptions;

use crate::prelude::*;

pub(crate) fn StripeScript() -> impl leptos::IntoView {
  // maybe need to remove async
  leptos::view! { <script src="https://js.stripe.com/v3/" /> }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StripeSession {
  pub client_secret: String,
  pub order_id: db::orders::OrderId,
}

#[component]
pub(crate) fn EmbeddedCheckout(session: StripeSession) -> impl IntoView {
  Effect::new(move || {
    let client_secret = session.clone().client_secret;
    let publish_key = env::stripe::AMPIDEXTEROUS_PUBLISH_KEY.to_string();
    if publish_key.contains("live") {
      debug!(
        prod = cfg!(feature = "prod"),
        "Starting live embedded checkout ..."
      );
    } else {
      debug!(?publish_key, "Starting dev embedded checkout ...");
    }
    leptos::task::spawn_local(async {
      let stripe = stripe_js::stripe::Stripe::new(publish_key);
      let checkout = stripe
        .init_embedded_checkout(InitOptions::new(client_secret))
        .await;
      checkout.mount("#stripe-checkout-div");
    });
  });

  view! { <div id="stripe-checkout-div" style:width="100%"></div> }
}
