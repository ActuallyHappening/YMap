use crate::{
  errors::{AppError, components::Pre},
  prelude::*,
};

use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::components::{Route, Routes};

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();
  leptoaster::provide_toaster();

  let root_owner = Owner::current().unwrap();
  provide_context(reactive_stores::Store::new(
    crate::rendering_state::GlobalState::new(root_owner),
  ));

  components::cart::state::GlobalCartState::provide_context();

  // let ssr = leptos_router::SsrMode::Static(leptos_router::static_routes::StaticRoute::new());
  let ssr = leptos_router::SsrMode::OutOfOrder;
  let fallback = |errors: ArcRwSignal<Errors>| {
    errors
      .read()
      .iter()
      .map(|(_id, err)| match err.downcast_ref::<AppError>() {
        None => Either::Left({
          error!(?err, "Handling an unknown error case!");
          view! {
            <pre> { "Unknown error when downcasting from" } </pre>
            <p> "An unknown error occurred" </p>
          }
        }),
        Some(err) => Either::Right({
          view! {
            <Pre err=GenericError::from_ref(err) />
            <p> { err.to_string() }</p>
          }
        }),
      })
      .collect_view()
  };

  view! {
    // injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href="/pkg/jyd-website.css" />

    // sets the document title
    <Title text="Jordan Yates Direct" />

    // maybe we don't need to load this every time
    // to avoid giving users cookies, but idk yet
    <crate::stripe::StripeScript />

    // toaster doesn't need to be anywhere specific
    // I don't think
    <leptoaster::Toaster stacked=true />

    // content for this welcome page
    <leptos_router::components::Router>
      <components::navbar::NavBar />

      <main>
        <ErrorBoundary fallback>
          <Routes fallback=|| "Page not found".into_view()>
            <Route path=path!("") view=components::home::HomePage ssr=ssr.clone() />
            <ParentRoute path=path!("/store") view=Outlet ssr=ssr.clone()>
              <components::store::Router />
            </ParentRoute>
            <ParentRoute path=path!("/review") view=Outlet ssr=ssr.clone()>
              <components::reviews::Router />
            </ParentRoute>
            <ParentRoute path=path!("/account") view=Outlet ssr=ssr.clone()>
              <components::accounts::Router />
            </ParentRoute>
            <ParentRoute path=path!("/order") view=Outlet ssr=ssr.clone()>
              <components::orders::Router />
            </ParentRoute>
            <Route path=path!("/about") view=components::about::About ssr=ssr.clone() />
            <ParentRoute path=path!("/policies") view=Outlet ssr=ssr.clone()>
              <components::policies::PoliciesRoute />
            </ParentRoute>
            <Route path=path!("/support") view=components::support::Support ssr=ssr.clone() />
            <ParentRoute path=path!("/cart") view=Outlet ssr=ssr.clone()>
              <components::cart::Router />
            </ParentRoute>
          </Routes>
        </ErrorBoundary>
      </main>

      <components::footer::Footer />
    </leptos_router::components::Router>
  }
}
