use crate::prelude::*;
use leptos_router::MatchNestedRoutes;

stylance::import_crate_style!(cart_style, "src/components/cart/cart.module.scss");

type Codee = codee::string::Base64<codee::binary::MsgpackSerdeCodec>;

pub mod state;

#[path = "ui/ui.rs"]
pub mod ui;

#[path = "call_to_action/call_to_action.rs"]
pub mod call_to_action;

#[path = "checkout/checkout.rs"]
pub mod checkout;

pub enum CartRoutes {
  /// .../
  Review,
  /// .../checkout
  Checkout,
}

impl CartRoutes {
  pub fn final_suffix(&self) -> &'static str {
    match self {
      Self::Review => "/",
      Self::Checkout => "/checkout",
    }
  }
}

impl NestedRoute for CartRoutes {
  fn nested_base(&self) -> impl Route {
    TopLevelRoutes::Cart
  }

  fn raw_path_suffix(&self) -> String {
    self.final_suffix().into()
  }
}

pub fn Router() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route path=path!("") view=ui::CartReview />
    <Route path=path!("/checkout") view=checkout::CheckoutPage />
  }
  .into_inner()
}
