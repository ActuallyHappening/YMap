use db::cartridges::CartridgeId;

use crate::{
  components::{
    cart::call_to_action::AddToCart,
    store::{Price, StoreProductExt as _, StoreRoutes},
  },
  db::DbState,
  errors::components::Pre,
  prelude::*,
};

stylance::import_crate_style!(
  full_styles,
  "src/components/store/listing_full/full.module.scss"
);

#[derive(Debug, Clone, thiserror::Error)]
pub enum ComponentError {
  #[error("Error connecting to db: {0}")]
  DBConnError(#[from] GenericError<crate::db::ConnectErr>),

  #[error("Loading product ...")]
  ProductNotLoadedYet,

  #[error("Error loading product from db: {0}")]
  ErrorLoadingCartridge(#[from] GenericError<db::cartridges::SelectCartridgeErr>),

  #[error("Couldn't find specified order")]
  CartridgeNotFound { id: CartridgeId },

  #[error("Invalid URL: {0}")]
  InvalidURL(components::store::routes::Error),
}

impl IntoRender for ComponentError {
  type Output = AnyView;

  fn into_render(self) -> Self::Output {
    view! {
      <p> { self.to_string() }</p>
      <Pre err=self />
    }
    .into_any()
  }
}

pub fn IndividualProductPage() -> impl IntoView {
  let route =
    Signal::derive(|| StoreRoutes::use_from_reactive_system().map_err(ComponentError::InvalidURL));
  let db = DbState::from_context();

  // let product = LocalResource::new(move || async move {
  //   let conn = db.read();
  //   let db = conn.conn().err_generic_ref()?;
  //   let id = route.get().map(|p| p.product_id)?;
  //   db.clone()
  //     .cartridges_downgraded()
  //     .clone()
  //     .select()
  //     .initial_one(id.clone())
  //     .await
  //     .err_generic()?
  //     .ok_or(ComponentError::CartridgeNotFound { id })
  // });

  let product = Signal::derive(move || {
    let products = db
      .read()
      .conn_old()
      .err_generic_ref()?
      .clone()
      .cartridges_downgraded()
      .select()
      .read();
    let id = route.get()?.product_id;
    let product = products
      .as_ref()
      .ok_or(ComponentError::ProductNotLoadedYet)?
      .into_iter()
      .find(|product| product.id() == id)
      .ok_or(ComponentError::CartridgeNotFound { id })?
      .clone();

    Result::<_, ComponentError>::Ok(product)
  });

  // manages abitrary name
  Effect::new(move || {
    let product = product.read();
    let Ok(product) = product.deref() else {
      return;
    };
    let Ok(route) = route.get() else {
      error!("Invalid Route");
      return;
    };
    if let Err(route) = route.normalize(&product) {
      navigate(route)
    }
  });

  move || {
    product
      .get()
      .map_view(|product| view! { <ProductFullPage product /> })
  }
}

#[component]
pub fn ProductFullPage(#[prop(into)] product: Signal<db::cartridges::Cartridge>) -> impl IntoView {
  let name = move || product.with(|p| p.get_name().to_owned());
  let img_url = move || format!("/{}", product.read().image_url().path());
  let price = Signal::derive(move || product.read().price_aud_dollars());
  let description = Signal::derive(move || product.read().description());
  let payments_product = Signal::derive(move || product.get().into_order(u8!(1)));
  let brand_url = move || product.read().brand_url().abs_path();

  let db = DbState::from_context();
  let rating = Signal::derive(move || {
    db.read()
      .conn_old()
      .ok()
      .and_then(|conn| conn.reviews_downgraded().rating_for(product.read().id()))
  });

  view! {
    <div class=full_styles::div>
      <div class=full_styles::split>
        <div class=full_styles::left>
          <img src=img_url />
        </div>
        <div class=full_styles::right>
          <div class=full_styles::brand>
            <img src=brand_url class=full_styles::brand />
          </div>
          <h1>{name}</h1>
          <p>{description}</p>
          <Price price_aud_dollars=price />
          <AddToCart product=payments_product />
        </div>
      </div>
    </div>
  }
}
