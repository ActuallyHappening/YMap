use crate::{app_state, prelude::*};

#[component]
pub fn LoggedIn() -> impl IntoView {
	let state = app_state();
	create_effect(move |_| {
		info!("Loaded the Logged in page");
	});
	view! {
		<h1>"Checking auth status ..."</h1>
	}
}
