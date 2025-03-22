use db::cartridges::Cartridge;

use crate::{components::store::ProductPreview, db::DbState, errors::components::Pre};

use crate::prelude::*;

stylance::import_crate_style!(
  listings_style,
  "src/components/store/all_listings/listings.module.scss"
);

#[derive(Debug, Clone, thiserror::Error)]
enum ComponentError {
  #[error("Loading products ...")]
  LoadingProducts,

  #[error("{0}")]
  DbDisconnected(#[from] GenericError<crate::db::ConnectErr>),
}

impl IntoRender for ComponentError {
  type Output = AnyView;

  fn into_render(self) -> Self::Output {
    if matches!(self, ComponentError::LoadingProducts) {
      return view! { <p> "Loading ..." </p> }.into_any();
    }
    view! {
      <p> { self.to_string() } </p>
      <Pre err=self />
    }
    .into_any()
  }
}

#[component]
pub fn AllListings() -> impl IntoView {
  let db = DbState::from_context();
  // let cartridges = LocalResource::new(move || async move {
  //   let db = db.read();
  //   let db = db.conn().err_generic_ref()?;
  //   let cartridges = db
  //     .cartridges_downgraded()
  //     .select_star()
  //     .await
  //     .err_generic()?;
  //   Result::<_, ComponentError>::Ok(cartridges)
  // });
  let cartridges = move || {
    let cartridges = db
      .read()
      .conn_old()
      .err_generic_ref()?
      .cartridges_downgraded()
      .select()
      .get()
      .ok_or(ComponentError::LoadingProducts)?;
    Result::<Vec<Cartridge>, ComponentError>::Ok(cartridges)
  };
  let ui = move || {
    cartridges().map_view(|data| {
      data
        .iter()
        .map(|product| {
          let product = product.clone();
          view! { <ProductPreview product=product /> }
        })
        .collect_view()
    })
  };
  view! {
    <components::search_bar::SearchBar />
    <div class=listings_style::listing_parent>
      { ui }
    </div>
  }
}
