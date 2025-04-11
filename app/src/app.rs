use std::ops::Deref as _;

use crate::{db::DbConn, prelude::*, things::WebsiteRoot};
use leptos_router::{
  components::{Outlet, ParentRoute, Route, Router, Routes},
  path,
};
use thing::well_known::KnownRecord;

pub mod description;
pub mod latex_demo;

pub fn App() -> impl IntoView {
  leptos_meta::provide_meta_context();
  crate::db::DbConn::provide();

  provide_context(RootOwner(Owner::current().unwrap()));

  view! {
    <Router>
      <main>
        <Routes fallback=|| "404 Not Found">
          <Route path=path!("/") view=|| view! { <Redirect path=format!("/thing/{}", WebsiteRoot::known()) /> } />
          <ParentRoute path=path!("/thing") view=Outlet>
            <Route path=path!("") view=|| view! { <Redirect path=format!("/thing/{}", WebsiteRoot::known()) /> } />
            <Route path=path!(":id") view=|| view! { <Main /> } />
          </ParentRoute>
        </Routes>
      </main>
    </Router>

    <footer>
      <crate::db::Connect />
    </footer>
  }
}

pub fn Main() -> impl IntoView {
  let id = Signal::derive(move || {
    ThingId::new_known(
      leptos_router::hooks::use_params_map()
        .get()
        .get("id")
        .expect("Only render main with :id path param")
        .into(),
    )
  });
  view! {
    <ThingView id=id />
  }
}

#[component]
pub fn ThingView(id: Signal<ThingId>) -> impl IntoView {
  view! {
    // <ErrorBoundary name="Latex Demo">
      <description::DescriptionView id=id />
      <latex_demo::LatexDemo id=id />
    // </ErrorBoundary>
  }
}

#[derive(Clone)]
pub(crate) struct RootOwner(pub Owner);
