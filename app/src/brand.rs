use crate::prelude::*;

pub fn Logo() -> impl IntoView {
  view! {
    <A href="/">
      <img src="/static/images/logo.svg" alt="JYD" height="48" width="96" style="width: 96px" />
    </A>
  }
}

pub const MISSION_STATEMENT: &str = "Print Smarter, Save Bigger";

pub fn InstagramLink() -> impl IntoView {
  let icon = icondata::FaInstagramBrands;
  let link = "https://www.instagram.com/jordan_yates_direct";
  view! {
    <A href=link>
      <crate::components::icons::IconSvg icon=icon style:width="2rem" />
    </A>
  }
}

pub fn TiktokLink() -> impl IntoView {
  let icon = icondata::FaTiktokBrands;
  let link = "https://www.tiktok.com/@jordanyatesdirect";
  view! {
    <A href=link>
      <crate::components::icons::IconSvg icon=icon style:width="2rem" />
    </A>
  }
}

pub fn FacebookLink() -> impl IntoView {
  let icon = icondata::FaFacebookBrands;
  let link = "https://www.facebook.com/share/12EK5PRKcBu/?mibextid=wwXIfr";
  view! {
    <A href=link>
      <crate::components::icons::IconSvg icon=icon style:width="2rem" />
    </A>
  }
}
