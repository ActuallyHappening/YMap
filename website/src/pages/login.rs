use yauth::types::{Email, Password, ValidationError};

use crate::prelude::*;

#[component]
pub fn Login() -> impl IntoView {
	// email
	let (email, set_email) = create_signal(Err(ValidationError::empty(
		"email",
		"Please enter an email",
	)));
	let (raw_email, set_raw_email) = create_signal(String::new());

	// password
	let (password, set_password) = create_signal(Err(ValidationError::empty(
		"password",
		"Please enter a password",
	)));
	let (raw_password, set_raw_password) = create_signal(String::new());

	let on_submit = move |ev: leptos::ev::SubmitEvent| {
		ev.prevent_default();

		let email = email.get();
		let password = password.get();

		info!(message = "Logging in ...", ?email);
	};

	view! {
		<div class="flex items-center justify-center">
			<h1>"Login"</h1>
			<form on:submit=on_submit>

				<TextInput
					placeholder="Email"
					on:input=move |ev| {
						let value = event_target_value(&ev);
						set_raw_email.set(value.clone());
						set_email.set(Email::from_str(&value));
					}

					get=raw_email
				/>

				<PasswordInput
					placeholder="Password"
					on:input=move |ev| {
						let value = event_target_value(&ev);
						set_raw_password.set(value.clone());
						set_password.set(Password::from_str(&value));
					}

					get=raw_email
				/>

				<Button type="submit">
					"Login"
				</Button>
			</form>
		</div>
	}
}
