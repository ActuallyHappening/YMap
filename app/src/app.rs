use crate::{db::DbConn, prelude::*, things::WebsiteRoot};
use leptos_router::{
  components::{Outlet, ParentRoute, Route, Router, Routes},
  path,
};
use mathquill_leptos::components::*;
use thing::well_known::KnownRecord;

pub fn App() -> impl IntoView {
  leptos_meta::provide_meta_context();
  crate::db::DbConn::provide();

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
  view! {}
}

/// Loads info, will reactively update its value
pub async fn known_id<T>(current: ThingId) -> Signal<Result<T, AppError>>
where
  T: KnownRecord,
{
  Signal::derive(move || {
    let db = DbConn::from_context();
    let db = db.read().guest()?;

    db.known_thing::<T>();

    todo
  })
}

pub mod latex_demo;
