use crate::prelude::*;
use leptos_router::{
  components::{Outlet, ParentRoute, Route, Router, Routes},
  path,
};

pub mod description;
pub mod explore;
pub mod latex_demo;
pub mod login;

pub fn App() -> impl IntoView {
  leptos_meta::provide_meta_context();
  crate::db::DbConn::provide();

  provide_context(RootOwner(Owner::current().unwrap()));

  view! {
    <Router>
      <main>
        <Routes fallback=|| "404 Not Found">
          <Route path=path!("/") view=|| view! { <Redirect path="/explore" /> } />
          // <Route path=path!("/explore") view=explore::ExploreRoot />
          <ParentRoute path=path!("/explore") view=explore::Explore>
            <Route path=path!("") view=explore::ExploreRoot />
            <Route path=path!(":id") view=explore::ExploreChild />
          </ParentRoute>
          <ParentRoute path=path!("/thing") view=Outlet>
            <Route path=path!("") view=|| view! { <Redirect path="/explore" /> } />
            <Route path=path!(":id") view=|| view! { <things::ThingView /> } />
          </ParentRoute>
        </Routes>
      </main>
    </Router>

    <footer>
      <crate::db::Connect />
    </footer>
  }
}

fn params_id() -> Signal<AppResult<ThingId>> {
  Signal::derive(move || {
    let str = leptos_router::hooks::use_params_map()
      .get()
      .get("id")
      .expect("Only render ExploreChild with :id path param");
    let id: ThingId =
      format!("thing:{}", str)
        .parse()
        .map_err(|err| AppError::CouldntParseRecordId {
          str: std::sync::Arc::from(str),
          err: GenericError::from(err),
        })?;
    AppResult::Ok(id)
  })
}

pub mod things;

#[derive(Clone)]
pub(crate) struct RootOwner(pub Owner);
