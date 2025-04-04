#![allow(non_snake_case)]

pub mod prelude {
  pub(crate) use leptos::prelude::*;
  #[allow(unused_imports)]
  pub(crate) use tracing::{debug, error, info, trace, warn};
}

pub mod components;
