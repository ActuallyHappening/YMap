#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../loading.md"))]
#![allow(non_snake_case)]

pub(crate) mod prelude {
  pub(crate) use leptos::prelude::*;
  #[allow(unused_imports)]
  pub(crate) use tracing::{debug, error, info, trace, warn};
}

pub mod components;
