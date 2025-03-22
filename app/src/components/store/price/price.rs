use crate::prelude::*;

stylance::import_crate_style! {
  price_styles,
  "src/components/store/price/price.module.scss"
}

#[component]
pub fn Price(#[prop(into)] price_aud_dollars: Signal<String>) -> impl IntoView {
  view! {
    <p class=price_styles::price>
      <span class=price_styles::number>{price_aud_dollars}</span>
    </p>
  }
}
