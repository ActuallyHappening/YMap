use super::Price;
use super::StoreProductExt as _;
use crate::components::cart::call_to_action::AddToCart;
use crate::prelude::*;

stylance::import_crate_style!(
  preview_styles,
  "src/components/store/listing_preview/preview.module.scss"
);

#[component]
pub fn ProductPreview(#[prop(into)] product: Signal<db::cartridges::Cartridge>) -> impl IntoView {
  let name = move || product.read().name();
  let image_url = move || product.read().image_url().path();
  let href = move || product.read().listing_url().path();
  let price = Signal::derive(move || product.read().price_aud_dollars());
  let payments_product = Signal::derive(move || product.get().into_order(1.try_into().unwrap()));

  view! {
    <A href={href} {..} class=preview_styles::preview>
      <img src=image_url />
      <h3>{name}</h3>
      <br style="margin-bottom: 1rem" />
      <Price price_aud_dollars=price />
      <AddToCart product=payments_product />
    </A>
  }
}
