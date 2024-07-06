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
	let session_info = create_resource(|| (), |_| async move { is_logged_in.await });

	view! {
		<h1>"Checking auth status ..."</h1>
	}
}
