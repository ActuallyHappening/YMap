//! This module provides only the building blocks for the UI
//! The actual web UI used in the `ymap` project lives in the `ymap` crate itself

use crate::{
	prelude::*,
	types::{Password, Username},
};
use leptos::*;

#[component]
pub fn UsernameInput(username: WriteSignal<Result<Username, ValidationError>>) -> impl IntoView {
	let (raw_username, set_raw_username) = create_signal("Your Username".to_string());

	create_effect(move |_| {
		let raw_username = raw_username.get();
		username.set(Username::from_str(&raw_username));
	});

	view! {
		<div class="mb-4 md:mb-6">
			<input
				type="text"
				class="w-full p-2"
				placeholder="Username"
				on:input=move |ev| {
					set_raw_username.set(event_target_value(&ev));
				}

				prop:value=raw_username
			/>
		</div>
	}
}

#[component]
pub fn PasswordInput() -> impl IntoView {
	let (raw_password, set_raw_password) = create_signal("".to_string());

	view! {
		<div class="mb-4 md:mb-6">
			<input
				type="password"
				class="w-full p-2"
				placeholder="Password"
				on:input=move |ev| {
					set_raw_password.set(event_target_value(&ev));
				}

				prop:value=raw_password
			/>
		</div>
	}
}

/// Directly a <input>
#[component]
pub fn EmailInput(set_email: WriteSignal<Result<Password, ValidationError>>) -> impl IntoView {
	let (raw_email, set_raw_email) = create_signal("".to_string());

	// updates the parent's email signal
	// with the result of parsing the email string
	create_effect(move |_| {
		let raw_email = raw_email.get();
		set_email.set(Password::from_str(&raw_email));
	});

	view! {
		
			<input
				class="w-full p-2"
				type="email"
				placeholder="Email"
				on:input=move |ev| {
					set_raw_email.set(event_target_value(&ev));
				}

				prop:value=raw_email
			/>
	}
}
