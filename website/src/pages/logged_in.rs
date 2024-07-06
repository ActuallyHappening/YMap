use crate::prelude::*;

#[component]
pub fn LoggedIn() -> impl IntoView {
    create_effect(move |_| {
        info!("Loaded the Logged in page");
        let session_info = 
    });
	view! {
		<h1>"Logged In"</h1>
	}
}
