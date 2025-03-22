use reactive_stores::Store;

use crate::{db::DbState, prelude::*};

/// Used while rendering, effectively client side only
#[derive(reactive_stores::Store)]
pub struct GlobalState {
  /// Currently using only on the client
  db: DbState,
}

pub type GlobalStateStore = Store<GlobalState>;

impl GlobalState {
  pub fn from_context() -> GlobalStateStore {
    expect_context::<reactive_stores::Store<GlobalState>>()
  }

  pub(crate) fn new(current_owner: Owner) -> Self {
    debug!(
      message = "Constructing GlobalState",
      note = "This should only happen once",
    );
    GlobalState {
      db: DbState::new(current_owner),
    }
  }
}

type DbStateSignal =
  reactive_stores::Subfield<reactive_stores::Store<GlobalState>, GlobalState, crate::db::DbState>;
impl DbState {
  pub fn from_context() -> DbStateSignal {
    expect_context::<reactive_stores::Store<GlobalState>>().db()
  }
}
