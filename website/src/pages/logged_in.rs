use yauth::cmds::session_info::SessionInfo;

use crate::{app_state, prelude::*, AppError};

pub async fn is_logged_in() -> Result<SessionInfo, AppError> {
	let state = app_state();
	let config = state.config().await;
	let db = state.db().await?;
	let session_info = config.control_db(&db).session_info().await?;
	Ok(session_info)
}

#[component]
pub fn LoggedIn() -> impl IntoView {
	let state = app_state();
	let session_info = create_resource(|| (), |_| async move { is_logged_in().await });
	let navigate = leptos_router::use_navigate();

	let main_view = move || match session_info.get() {
		Some(session_info) => match session_info {
			Ok(SessionInfo::SignedIn) => view! { <h1>"Signed In "</h1> }.into_view(),
			Ok(SessionInfo::SignedOut) => {
				info!("Loading the LoggedIn page while actually logged out");
				navigate("/login", Default::default());
				view! { <h1>"Logged out, redirecting ..."</h1> }.into_view()
			}
			Err(err) => {
				error!("Error while checking if logged in: {:#?}", err);
				view! { <h1>"Error while checking if logged in"</h1> }.into_view()
			}
		},
	};

	view! { move || main_view() }
}
