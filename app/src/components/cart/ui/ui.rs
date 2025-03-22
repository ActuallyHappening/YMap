use std::collections::HashSet;

use crate::{
  components::{
    cart::{call_to_action::CheckoutButton, state::GlobalCartState},
    icons,
    store::{Price, StoreProductExt as _},
  },
  db::DbState,
  errors::components::Pre,
  prelude::*,
};
use db::{
  cartridges::{self, CartridgeId},
  orders::ProductOrder,
  users,
};

stylance::import_crate_style!(ui_styles, "src/components/cart/ui/ui.module.scss");

pub fn CartReview() -> impl IntoView {
  let cart = GlobalCartState::from_context();

  let not_empty = move || !cart.read_sig().read().is_empty();
  let line_items = move || {
    let fallback =
      move || view! { <p>"Your cart is currently empty! Get shopping ;)"</p> }.into_any();
    let ui = move || view! { <LineItems cart=cart /> };
    view! {
      <Show when=not_empty fallback>
        {ui}
      </Show>
    }
  };

  let subtotal = move || {
    let ui = move || view! { <Subtotal cart=cart /> };
    view! { <Show when=not_empty>{ui}</Show> }
  };

  view! {
    <div class=ui_styles::split>
      <div class=ui_styles::left>
        <h1>"Shopping Cart"</h1>
        <hr />
        {line_items}
      </div>
      <div class=ui_styles::right>{subtotal}</div>
    </div>
  }
}

#[component]
fn Subtotal(cart: GlobalCartState) -> impl IntoView {
  move || {
    let cart = cart.read_sig().read().deref();
  }
}

#[component]
fn LineItems(cart: GlobalCartState) -> impl IntoView {
  #[derive(Debug, Clone, thiserror::Error)]
  enum ComponentError {
    #[error("Not connected to DB")]
    DbDisconnected(#[from] GenericError<crate::db::ConnectErr>),

    #[error("Loading cartridges")]
    LoadingCartridges,
  }

  impl IntoRender for ComponentError {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
      view! {
        <Pre err=self.clone() />
        <p>{ self.to_string() }</p>
      }
      .into_any()
    }
  }

  let ids = Memo::new(move |_| {
    let cart = cart.read_sig().read();
    cart.keys().cloned().collect::<HashSet<_>>()
  });
  let db = DbState::from_context();
  let data = move || {
    let ids = ids.get();
    let data = db
      .read()
      .conn_old()
      .err_generic_ref()?
      .cartridges_downgraded()
      .select()
      .get();
    let mut data = data.ok_or(ComponentError::LoadingCartridges)?;
    data.retain(|cartridge| ids.contains(&cartridge.id()));
    // sorting keeps consistent order
    data.sort_by(|a, b| a.id().cmp(&b.id()));
    Result::<_, ComponentError>::Ok(data)
  };

  move || {
    data().map_view(|data| {
      data
        .into_iter()
        .map(|cartridge| view! { <LineItem cartridge /> })
        .collect_view()
    })
  }
}

#[component]
fn LineItem(cartridge: db::cartridges::Cartridge) -> impl IntoView {
  let image_url = cartridge.image_url().path();
  let name = cartridge.name();
  let price_aud_dollars = cartridge.price_aud_dollars();

  view! {
    <div class=ui_styles::line_item>
      <img src=image_url />
      <div>
        <h3>{name}</h3>
        <p>"Model compat TODO"</p>
        <p>"IN STOCK TODO"</p>
        <p>"Delivery time + cost TODO"</p>
        <div>
          <Quantity cartridge=cartridge.id() />
          <hr />
          <Delete cartridge=cartridge.id() />
        </div>
      </div>
      <hr />
      <Price price_aud_dollars />
    </div>
  }
}

#[component]
fn Delete(#[prop(into)] cartridge: Signal<CartridgeId>) -> impl IntoView {
  let cart = GlobalCartState::from_context();
  let delete = move |_| cart.remove(cartridge.get());
  let icon = icondata::FaTrashCanSolid;
  view! {
    <button on:click=delete>
      <icons::IconSvg icon=icon />
    </button>
  }
}

/// This has caused me so much pain and suffering.
/// Why do my signals keep ending up disposed?
/// Why is this even rendered if my signal is disposed?
/// Is there anything really wrong with using GlobalCartState
/// like I am?
/// All questions that I don't know the answer to. Yet.
/// - Actually Happening, 2025
#[component]
fn Quantity(cartridge: CartridgeId) -> impl IntoView {
  debug!("Rendering Quantity");
  let cart = GlobalCartState::from_context();
  let cartridge = Signal::stored(cartridge);

  let quantity = move || {
    cartridge
      .try_get()
      .map(|id| {
        cart
          .read_sig()
          .read()
          .get(&id)
          .map(|p| p.read().quantity().to_string())
          .unwrap_or("0?".to_string())
      })
      .unwrap_or("0??".to_string())
  };

  let increment = {
    let increment = move |_| cart.add(ProductOrder::new(cartridge.get(), u8!(1)));
    view! { <button on:click=increment>"+"</button> }
  };

  let quantity = move || {
    view! { <span>{quantity}</span> }
  };

  let decrement = move || {
    let decrement = move |_| cart.decrement(ProductOrder::new(cartridge.get(), u8!(1)));
    let enabled = cartridge
      .try_get()
      .map(|id| {
        cart
          .read_sig()
          .read()
          .get(&id)
          .is_some_and(|p| p.read().quantity() > u8!(1))
      })
      .unwrap_or(false);
    view! {
      <button on:click=decrement disabled=!enabled>
        "-"
      </button>
    }
  };

  view! {
    {decrement}
    {quantity}
    {increment}
  }
}
