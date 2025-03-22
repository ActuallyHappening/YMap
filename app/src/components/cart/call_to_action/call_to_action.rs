use db::orders::ProductOrder;

use crate::{
  components::cart::{CartRoutes, state::GlobalCartState},
  prelude::*,
};

stylance::import_crate_style!(
  call_to_action_styles,
  "src/components/cart/call_to_action/call_to_action.module.scss"
);

#[component]
pub fn AddToCart(#[prop(into)] product: Signal<ProductOrder>) -> impl IntoView {
  let cart = GlobalCartState::from_context();
  let on_click = move |_| {
    cart.add(product.get());
  };
  view! {
    <button on:click=on_click class=call_to_action_styles::btn>
      <p>"Add to Cart"</p>
    </button>
  }
}

pub fn CheckoutButton() -> impl IntoView {
  view! {
    <A href=CartRoutes::Checkout.abs_path() {..} class=call_to_action_styles::btn>
      "Checkout"
    </A>
  }
}
