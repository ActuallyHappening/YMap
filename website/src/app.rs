use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::*;

use crate::{
	error_template::{AppError, ErrorTemplate},
	pages::logged_in::LoggedIn,
	pages::login::Login,
};

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Meta name="charset" content="UTF-8"/>
		<Meta name="description" content="Leptonic CSR template"/>
		<Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
		<Meta name="theme-color" content="#e66956"/>

		<Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
		<Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

		<Title text="YMap Website"/>

		<Root default_theme=LeptonicTheme::default()>
			<Router fallback=|| {
				let mut outside_errors = Errors::default();
				outside_errors.insert_with_default_key(AppError::NotFound);
				view! { <ErrorTemplate outside_errors/> }
			}>
				<nav>
					<Box style="position: relative; border: 4px solid gray; width: 100%; height: 20em; overflow: auto;">
						<AppBar
						style="z-index: 1; background: var(--brand-color); color: white;">
							<H3 style="margin-left: 1em; color: white;">"YMap"</H3>
							<Stack
								orientation=StackOrientation::Horizontal
								spacing=Size::Em(1.0)
								style="margin-right: 1em"
							>
								<Icon icon=icondata::FaGithubBrands/>

							</Stack>
						</AppBar>

						<Box style="padding: 0.5em;">
							<P>"Scroll ↓"</P>
							<Stack spacing=Size::Em(
								0.5,
							)>
								{(0..10)
									.map(|_| view! { <Skeleton height=Size::Em(3.0)/> })
									.collect_view()}
							</Stack>
						</Box>
					</Box>
				</nav>
				<Routes>
					<Route path="/login" view=|| view! { <Login/> }/>
					<Route path="" view=|| view! { <LoggedIn/> }/>
				</Routes>
			</Router>
		</Root>
	}
}
