use url::Url;

pub mod prelude {
  pub(crate) use tracing::*;

  pub use crate::{NestedRoute, Route, RouteHelpers, TopLevelRoutes};
}

use crate::prelude::*;

pub fn base() -> Url {
  Url::parse("https://jordanyatesdirect.com").unwrap()
}

pub fn db_wss() -> Url {
  let mut url = base();
  url.set_scheme("wss").unwrap();
  url.set_port(Some(42069)).unwrap();

  // if let Ok(db_endpoint) = std::env::var("DB_ENDPOINT") {
  //   url = Url::parse(&db_endpoint).expect("Couldn't parse the DB_ENDPOINT env variable");
  //   debug!(messsage = "Updated the db connection endpoint", ?url, %db_endpoint);
  // }

  url
}

pub fn db_https() -> Url {
  let mut url = db_wss();
  url.set_scheme("https").unwrap();
  url
}

pub trait NestedRoute: Route {
  fn nested_base(&self) -> impl Route;

  /// May contain trailing and leading slashes
  fn raw_path_suffix(&self) -> String;
}

pub trait NestedRouteHelpers {
  fn path_suffix(&self) -> String;
}

impl<T> NestedRouteHelpers for T
where
  T: NestedRoute,
{
  fn path_suffix(&self) -> String {
    self.raw_path_suffix().trim_matches('/').to_owned()
  }
}

impl<T> Route for T
where
  T: NestedRoute,
{
  /// Todo: work out a way of propogating the lifetime
  /// rather than cloning with `.to_string()`
  fn raw_base(&self) -> String {
    self.nested_base().base()
  }

  fn raw_path(&self) -> String {
    let base_path = self.nested_base().path();
    format!("{}/{}", base_path, self.path_suffix())
  }
}

/// refactor to rely more interanlly on [`::url::Url`]
pub trait Route {
  /// May contain trailing slashes
  fn raw_base(&self) -> String {
    crate::base().to_string()
  }

  /// May contain leading and trailing slashes
  fn raw_path(&self) -> String;

  fn query(&self) -> std::collections::HashMap<Box<str>, Box<str>> {
    unimplemented!()
  }
}

impl<T> RouteHelpers for T where T: Route {}

pub trait RouteHelpers: Route {
  /// Removes trailing and leading slashes for easier formatting
  fn path(&self) -> String {
    self.raw_path().trim_matches('/').to_owned()
  }

  /// Adds a prefix '/'
  fn abs_path(&self) -> String {
    format!("/{}", self.path())
  }

  /// Removes trailing slashes for easier formatting
  fn base(&self) -> String {
    self.raw_base().trim_end_matches('/').to_owned()
  }

  fn full_url(&self) -> Url {
    Url::parse(&format!("{}/{}", self.base(), self.path()))
      .expect("Url parsing failed for an implementor of `Route`")
  }
}

#[derive(strum::Display, Clone, Copy)]
pub enum TopLevelRoutes {
  /// /
  Home,
  Store,
  Review,
  Cart,
  Account,
  About,
  Policies,
  Support,
  Api,
  Static,
  Order,
}

impl TopLevelRoutes {
  pub fn iter_footer() -> impl Iterator<Item = Self> {
    vec![
      Self::Home,
      Self::Store,
      Self::Cart,
      Self::Account,
      Self::About,
      Self::Policies,
      Self::Support,
    ]
    .into_iter()
  }

  pub fn name(self) -> String {
    self.to_string()
  }
}

impl Route for TopLevelRoutes {
  fn raw_path(&self) -> String {
    match self {
      Self::Home => "/",
      Self::Store => "/store",
      Self::Review => "/review",
      Self::Account => "/account",
      Self::Cart => "/cart",
      Self::Order => "/order",
      Self::About => "/about",
      Self::Policies => "/policies",
      Self::Support => "/support",
      Self::Api => "/api",
      Self::Static => "/static",
    }
    .into()
  }
}

pub mod checkout;
pub mod orders;

pub mod product_listing {
  use crate::prelude::*;

  pub struct ProductListingUrl {
    id: String,
  }

  pub struct ProductUrl {
    id: String,
  }

  impl Route for ProductListingUrl {
    fn raw_path(&self) -> String {
      format!("/store/{}", self.id)
    }
  }

  impl Route for ProductUrl {
    fn raw_path(&self) -> String {
      format!("/static/images/products_listing/{}.png", self.id)
    }
  }

  impl ProductListingUrl {
    pub fn from_key(key: surrealdb::RecordIdKey) -> Self {
      Self {
        id: key.to_string(),
      }
    }
  }

  impl ProductUrl {
    pub fn from_key(key: surrealdb::RecordIdKey) -> Self {
      Self {
        id: key.to_string(),
      }
    }
  }
}
