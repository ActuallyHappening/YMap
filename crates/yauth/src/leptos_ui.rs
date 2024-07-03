//! This module provides only the building blocks for the UI
//! The actual web UI used in the `ymap` project lives in the `ymap` crate itself

use crate::prelude::*;
use leptos::*;

#[component]
fn UsernameInput() -> impl IntoView {
	view! {
		<div class="flex items-center justify-center h-[90vh]">
		<Card class="sm:w-[80vw] xl:w-[40vw] p-4">
			<template #title>Login</template>
			<template #content>
				<form @submit.prevent="handleLogin">
					<div class="mb-4 md:mb-6">
						<InputText class="w-full p-2" type="email" placeholder="Email" v-model="email" />
					</div>
					<div class="mb-4 md:mb-6">
						<InputText
							type="password"
							class="w-full p-2"
							placeholder="Password"
							v-model="password"
						/>
					</div>
					<Button class="w-full p-2" label="Log In" type="submit" />
				</form>
			</template>
		</Card>
	</div>
	}
}

#[component]
fn PasswordInput() -> impl IntoView {
	view! {
		<div class="mb-4 md:mb-6">
			<InputText
				type="password"
				class="w-full p-2"
				placeholder="Password"
				v-model="password"
			/>
		</div>
	}
}

#[component]
fn EmailInput() -> impl IntoView {
	view! {
		<div class="mb-4 md:mb-6">
			<InputText class="w-full p-2" type="email" placeholder="Email" v-model="email"/>
		</div>
	}
}
