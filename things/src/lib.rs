pub mod prelude {
  pub(crate) use utils::prelude::*;
}

pub mod common {
  use crate::prelude::*;

  pub trait AThing {
    /// e.g. "thing:websiteroot"
    const ID: &str;
  }
}

pub mod website {
  use crate::prelude::*;

  ///
  pub struct WebsiteRoot;

  pub struct WebsiteChildren;
}
