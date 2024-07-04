use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Login() -> impl IntoView {
	let on_submit = |ev: leptos::ev::SubmitEvent| {
		ev.prevent_default();

		info!(message = "Form submitted",);
	};

	let (email, set_email) = create_signal(Err(ValidationError::empty(
		"password",
		"Please enter a password",
	)));

	view! {
		<div class="flex items-center justify-center">
			<h1>"Login"</h1>
			<form on:submit=on_submit>
				// <yauth::leptos_ui::UsernameInput />
				<div class="mb-4 md:mb-6">
					<yauth::leptos_ui::EmailInput set_email=set_email></yauth::leptos_ui::EmailInput>
				</div>
				<yauth::leptos_ui::PasswordInput></yauth::leptos_ui::PasswordInput>

				<button></button>
			</form>
		</div>
	}
}
