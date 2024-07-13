use crate::prelude::*;
use yauth::cmds::session_info::SessionInfo;

pub async fn is_logged_in() -> Result<SessionInfo, AppError> {
	let state = crate::state::app_state();
	let config = state.config().await;
	let db = state.db().await?;
	let session_info = config.control_db(&db).session_info().await?;
	Ok(session_info)
}

#[component]
pub fn LoggedIn() -> impl IntoView {
	let session_info = create_local_resource(|| (), |_| async move { is_logged_in().await });
	let navigate = leptos_router::use_navigate();

	let main_view = move || match session_info.get() {
		Some(session_info) => match session_info {
			Ok(SessionInfo::UserSignedIn | SessionInfo::RootSignedIn) => {
				view! { <H1>"Signed In"</H1> }.into_view()
			}
			Ok(SessionInfo::SignedOutCompletely) => {
				info!("Loading the LoggedIn page while actually logged out");
				navigate("/login", Default::default());
				view! { <H1>"Logged out, redirecting to /login ..."</H1> }.into_view()
			}
			Err(err) => {
				error!("Error while checking if logged in: {:#?}", err);
				view! { <H1>"Error while checking if logged in"</H1> }.into_view()
			}
		},
		None => view! {<H1> "Loading ..."</H1>},
	};

	main_view
}
