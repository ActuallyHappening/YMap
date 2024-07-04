//! This module provides only the building blocks for the UI
//! The actual web UI used in the `ymap` project lives in the `ymap` crate itself

use crate::prelude::*;
use leptos::*;

#[component]
fn UsernameInput() -> impl IntoView {
	let (raw_username, set_raw_username) = create_signal("Your Username".to_string());

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
fn PasswordInput() -> impl IntoView {
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

#[component]
fn EmailInput() -> impl IntoView {
	let (raw_email, set_raw_email) = create_signal("".to_string());

	view! {
		<div class="mb-4 md:mb-6">
			<input
				class="w-full p-2"
				type="email"
				placeholder="Email"
				on:input=move |ev| {
					set_raw_email.set(event_target_value(&ev));
				}
			/>
		</div>
	}
}
