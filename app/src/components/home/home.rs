use crate::prelude::*;
stylance::import_crate_style!(home_styles, "src/components/home/home.module.scss");

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    // <div class=home_styles::hscroller>
    // </div>

    <div class=home_styles::splash>
      <div class=home_styles::inner>
        <img src="/static/images/homepage/1.png" height="60vh" />
        <h1>"Print Smarter, Save Bigger"</h1>
      </div>
    </div>

    <components::store::AllListings />
  }
}
