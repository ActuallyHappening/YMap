use leptos_router::hooks::use_params_map;
use surrealdb::RecordIdKey;

use crate::{
  components::store::{AllListings, IndividualProductPage},
  prelude::*,
};

/// Handler /store/id-here234jh/Any-abitraty-name-for-seo
pub fn Router() -> impl MatchNestedRoutes + Clone {
  let home_page = || {
    view! {
      <h1>"JYD Store"</h1>
      <AllListings />
    }
  };
  let individual_product_page = || {
    view! { <IndividualProductPage /> }
  };
  view! {
    <Route path=path!("") view=home_page />
    <ParentRoute path=path!(":id") view=individual_product_page>
      <Route path=path!("") view=|| () />
      <Route path=path!(":name") view=|| () />
    </ParentRoute>
  }
  .into_inner()
}

#[derive(PartialEq, Clone)]
pub(super) struct StoreRoutes {
  pub(super) product_id: db::cartridges::CartridgeId,
  pub(super) abitrary_name: Option<String>,
}

impl NestedRoute for StoreRoutes {
  fn nested_base(&self) -> impl Route {
    TopLevelRoutes::Store
  }

  fn raw_path_suffix(&self) -> String {
    format!(
      "/{}/{}",
      self.product_id.key(),
      self.abitrary_name.as_ref().unwrap_or(&"".to_string())
    )
  }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
  #[error("Couldn't find param :id")]
  CouldntFindIdParam,
}

impl StoreRoutes {
  pub fn use_from_reactive_system() -> Result<StoreRoutes, Error> {
    let params = use_params_map().read();
    let product_id: RecordIdKey = params.get("id").ok_or(Error::CouldntFindIdParam)?.into();
    let abitrary_name = params.get("name");
    Ok(Self {
      product_id: db::cartridges::CartridgeId::new_unchecked(product_id),
      abitrary_name,
    })
  }

  /// [`Err`] if changed, marks for [`StoreRoutes.navigate_to`]
  pub fn normalize(self, product: &db::cartridges::Cartridge) -> Result<Self, Self> {
    let actual_name = product.kebab_name();
    let actual_id = product.id();

    let expected = Self {
      product_id: actual_id,
      abitrary_name: Some(actual_name),
    };
    if self != expected {
      Err(expected)
    } else {
      Ok(self)
    }
  }
}
