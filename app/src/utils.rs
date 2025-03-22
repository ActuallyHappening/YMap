use crate::prelude::*;

pub fn navigate(path: impl Route) {
  use leptos_router::NavigateOptions;

  let navigate = leptos_router::hooks::use_navigate();

  // e.g. localhost:3000
  let current_url = leptos_router::hooks::use_url().get();
  let href = format!("{}/{}", current_url.origin(), path.path());

  #[cfg(feature = "hydrate")]
  {
    info!(?href, "Navigating ...");

    navigate(
      &href,
      NavigateOptions {
        resolve: false,
        ..Default::default()
      },
    )
  }
  #[cfg(not(feature = "hydrate"))]
  {
    debug!(?href, "Pretend navigating because we are on the server");
  }
}
