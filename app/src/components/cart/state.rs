use db::orders::ProductOrder;

use crate::prelude::*;

type InnerStorage = LocalStorageCartState;

// The WHOLE point of using local-storage is to read it at the start,
// and save it at mutate checkpoints
#[derive(Clone, Copy)]
pub struct GlobalCartState {
  inner: InnerStorage,
  reactive: RwSignal<ReactiveCart>,
  has_initially_synced: RwSignal<bool>,
}

use internal_local_storage::*;
pub use reactive_cart::*;

mod internal_local_storage;
mod reactive_cart;

impl GlobalCartState {
  pub(crate) fn provide_context() {
    let inner = InnerStorage::load();
    let state = GlobalCartState {
      inner,
      reactive: RwSignal::new(ReactiveCart::default()),
      has_initially_synced: RwSignal::new(false),
    };

    provide_context(state);

    // loads from inner storage from previous sessions
    // will return default before this runs
    #[cfg(feature = "hydrate")]
    request_animation_frame(move || {
      let initial_value = state.inner.get_untracked();
      info!(
        ?initial_value,
        "Loaded cart initial value! Yay, it was worth it lol"
      );
      state.reactive.set(ReactiveCart::from_plain(initial_value));
      *state.has_initially_synced.write_untracked() = true;
    });

    state.debugging_effect();
  }

  #[track_caller]
  pub fn from_context() -> GlobalCartState {
    expect_context::<GlobalCartState>()
  }

  pub(crate) fn debugging_effect(&self) {
    // debugging effect
    let local_storage_signal = *self;
    Effect::new(move || {
      let new_stored = local_storage_signal.inner.get();
      debug!(?new_stored, "A new value was stored in local storage, yay");
    });
  }

  fn reactive(&self) -> RwSignal<ReactiveCart> {
    if !self.has_initially_synced.get_untracked() {
      debug!("Getting reactive before initial sync");
    }
    self.reactive
  }

  pub fn read_sig(&self) -> ReadSignal<ReactiveCart> {
    self.reactive().read_only()
  }

  pub fn add(&self, product: ProductOrder) {
    debug!(message = "Adding product to cart", ?product);
    self.reactive().write().add(product.clone());
    self.inner.set(self.reactive().get().into_plain());
  }

  pub fn decrement(&self, product: ProductOrder) {
    debug!(message = "Decrementing product in cart", ?product);
    self.reactive().write().decrement(product.clone());
    self.inner.set(self.reactive().get().into_plain());
  }

  pub fn remove(&self, id: db::cartridges::CartridgeId) {
    debug!(message = "Removing product from cart", ?id);
    self.reactive().write().remove(id.clone());
    self.inner.set(self.reactive().get().into_plain());
  }
}
