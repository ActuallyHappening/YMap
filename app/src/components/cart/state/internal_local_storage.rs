use db::orders::cart::Cart;
use leptos_use::UseCookieOptions;

use crate::{components::cart::Codee, prelude::*};

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) struct LocalStorageCartState {
  read: Signal<Cart>,
  write: WriteSignal<Cart>,
}

#[allow(dead_code)]
impl LocalStorageCartState {
  pub(super) fn load() -> Self {
    let options = leptos_use::storage::UseStorageOptions::default().on_error(|err| {
      error!(?err, "Error with local storage");
    });
    // .filter(leptos_use::utils::FilterOptions::debounce(100.0))
    // .delay_during_hydration(true);
    let (read, write, _) = leptos_use::storage::use_local_storage_with_options::<Cart, Codee>(
      "cart-local-storage",
      options,
    );

    info!(val = ?read.get_untracked(), "Just initialized local_storage");

    Self { read, write }
  }

  pub fn get(self) -> Cart {
    self.read.get()
  }

  pub fn get_untracked(self) -> Cart {
    self.read.get_untracked()
  }

  pub fn set(self, cart: Cart) {
    debug!(?cart, "Writing to local storage");
    self.write.set(cart);
  }

  pub fn update<F>(self, f: F)
  where
    F: FnOnce(&mut Cart),
  {
    self.write.update(move |cart| {
      f(cart);
      debug!(?cart, "Updating local storage");
    });
  }
}

/// Should be cheap to clone, to [`GlobalCartState::expect_from_context`]
///
/// Currently, some random BorrowMutErr panic happens and I'm not bothered to debug it
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) struct CookieCartState {
  read: Signal<Option<Cart>>,
  write: WriteSignal<Option<Cart>>,
}

#[allow(dead_code)]
impl CookieCartState {
  pub fn get(self) -> Cart {
    self.read.get().unwrap_or_default()
  }

  pub fn get_untracked(self) -> Cart {
    self.read.get_untracked().unwrap_or_default()
  }

  // pub fn set(self, cart: Cart) {
  //   debug!(?cart, "Writing to cookies");
  //   self.write.set(Some(cart));
  // }

  pub fn update<F>(self, f: F)
  where
    F: FnOnce(&mut Cart),
  {
    self.write.update(move |cart| match cart {
      Some(cart) => {
        f(cart);
        debug!(?cart, "Updating cookies");
      }
      None => {
        let mut cart = Cart::default();
        f(&mut cart);
        debug!(?cart, "Updating cookies (from default)");
        self.write.set(Some(cart));
      }
    });
  }

  pub(super) fn load() -> CookieCartState {
    let (read, write) = leptos_use::use_cookie_with_options::<Cart, Codee>(
      "cart",
      UseCookieOptions::<
        Cart,
        <Codee as codee::Encoder<Cart>>::Error,
        <Codee as codee::Decoder<Cart>>::Error,
      >::default()
      .on_error(std::sync::Arc::new(|err| {
        error!(?err, "Error with cookie storage");
      }))
      .max_age(3600_000)
      .path("/"),
    );

    // write.set(Some(Cart::default()));
    info!(initial = ?read.get_untracked(), "Just setup cookies signals");

    CookieCartState { read, write }
  }
}
