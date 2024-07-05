use yauth::{
	signin::SignIn,
	types::{Email, Password},
};
use ymap::auth::config::ProductionConfig;

use crate::prelude::*;

async fn login(credentials: &SignIn) -> Result<Jwt, AuthError> {
	info!("Logging in ..");
	let config = ProductionConfig::new();
	let db = config.connect_ws().await?;
	let (jwt, user_record) = config.control_db(&db).sign_in(credentials).await?;

	debug!("Logged in user {}", user_record.id());

	Ok(jwt)
}

#[component]
pub fn Login() -> impl IntoView {
	// email
	let (raw_email, set_raw_email) = create_signal(String::new());
	let email = Signal::derive(move || Email::from_str(&raw_email.get()));

	// password
	let (raw_password, set_raw_password) = create_signal(String::new());
	let password = Signal::derive(move || Password::from_str(&raw_password.get()));

	let (error, set_error) = create_signal(None);

	let submit_action = create_action(|credentials: &SignIn| {
		let credentials = credentials.clone();
		async move {
			let jwt = login(&credentials).await;
			trace!(?jwt);
		}
	});

	let on_submit = move |ev: leptos::ev::SubmitEvent| {
		ev.prevent_default();

		let email = match email.get() {
			Ok(email) => email,
			Err(err) => {
				set_error.set(Some(err));
				return;
			}
		};
		let password = match password.get() {
			Ok(password) => password,
			Err(err) => {
				set_error.set(Some(err));
				return;
			}
		};
		set_error.set(None);

		submit_action.dispatch(SignIn { email, password });
	};

	view! {
		<div style="display: flex; flex-direction: column;">
			<H1>"Login"</H1>
			<form on:submit=on_submit style="display: flex; flex-direction: column; align-items: center;">

				<TextInput
					placeholder="Email"
					on:input=move |ev| {
						let value = event_target_value(&ev);
						set_raw_email.set(value.clone());
					}

					get=raw_email
				/>

				<PasswordInput
					placeholder="Password"
					on:input=move |ev| {
						let value = event_target_value(&ev);
						set_raw_password.set(value.clone());
					}

					get=raw_password
				/>

				<Button on_click=|_| {}>"Login"</Button>
			</form>
			{move || error.with(|err| err.as_ref().map(|err| format!("Error: {}", err)))}
		</div>
	}
}
