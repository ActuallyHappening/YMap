use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::*;

use crate::error::AppError;
use crate::error_template::ErrorTemplate;
use crate::pages::logged_in::LoggedIn;
use crate::pages::login::Login;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	crate::state::provide_app_context();

	view! {
			<Meta name="charset" content="UTF-8"/>
			<Meta name="description" content="Leptonic SSR template"/>
			<Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
			<Meta name="theme-color" content="#8856e6"/>

			<Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
			<Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

			<Title text="YMap Application Website"/>

			<Root default_theme=LeptonicTheme::default()>
					<Router fallback=|| {
							let mut outside_errors = Errors::default();
							outside_errors.insert_with_default_key(AppError::NotFound);
							view! {
									<ErrorTemplate outside_errors/>
							}
					}>
							<Routes>
									<Route path="" view=|| view! { <LoggedIn/> }/>
									<Route path="/login" view=|| view! { <Login/> }/>
									<Route path="/loggedin" view=|| view! { <LoggedIn/> }/>
							</Routes>
					</Router>
			</Root>
	}
}
