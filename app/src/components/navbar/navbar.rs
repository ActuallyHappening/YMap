use web_sys::MouseEvent;

use crate::prelude::*;

stylance::import_crate_style!(navbar_style, "src/components/navbar/navbar.module.scss");

fn SearchIcon() -> impl IntoView {
  let icon = icondata::AiSearchOutlined;
  view! { <crate::components::icons::IconSvg icon=icon style:width="2rem" /> }
}

fn SearchBar() -> impl IntoView {
  let on_click = |e: MouseEvent| {
    e.prevent_default();
    warn!("TODO: impl search");
  };
  view! {
    <form>
      <button type="submit" on:click=on_click>
        <SearchIcon />
      </button>
      <input type="text" name="q" placeholder="Search our products ..." />
    </form>
  }
}

fn TrackOrderBtn() -> impl IntoView {
  view! {
    <button>
      <span>"Track Order"</span>
    </button>
  }
}

/// todo: icon for logged in, + for logged out
fn AccountBtn() -> impl IntoView {
  view! {
    <button>
      <span>"Account"</span>
    </button>
  }
}

/// todo: icon + notifications support
fn CartBtn() -> impl IntoView {
  let icon = icondata::FaCartShoppingSolid;
  let route = TopLevelRoutes::Cart.path();
  view! {
    <A href=route>
      <crate::components::icons::IconSvg icon=icon style:width="2rem" />
    </A>
  }
}

fn StoreBtn() -> impl IntoView {
  let icon = icondata::BiStoreAltRegular;
  view! {
    <A href=TopLevelRoutes::Store.path()>
      <crate::components::icons::IconSvg icon=icon style:width="2rem" />
    </A>
  }
}

#[component]
pub fn NavBar() -> impl IntoView {
  view! {
    <nav id="globalnav-nav">
      <div>
        <crate::brand::Logo />
      </div>
      <div class=navbar_style::search_bar>
        <SearchBar />
      </div>
      <div class=navbar_style::socials>
        <crate::brand::InstagramLink />
        <crate::brand::TiktokLink />
        <crate::brand::FacebookLink />
      </div>
      <div class=navbar_style::btns>
        // <TrackOrderBtn />
        // <AccountBtn />
        <CartBtn />
      </div>
    </nav>
  }
}
