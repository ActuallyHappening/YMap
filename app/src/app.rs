use crate::prelude::*;
use leptos_router::{
  components::{Outlet, ParentRoute, Route, Router, Routes},
  path,
};

pub mod description;
pub mod explore;
pub mod latex_demo;

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

pub mod things {
  use crate::{
    app::{description, latex_demo},
    prelude::*,
  };

  pub fn ThingView() -> impl IntoView {
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
      <FullView id=id />
    }
  }

  #[component]
  fn FullView(id: Signal<ThingId>) -> impl IntoView {
    view! {
      // <ErrorBoundary name="Latex Demo">
        <description::DescriptionView id=id />
        <latex_demo::LatexDemo id=id />
      // </ErrorBoundary>
    }
  }
}

#[derive(Clone)]
pub(crate) struct RootOwner(pub Owner);
