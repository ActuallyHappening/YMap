use std::cell::OnceCell;

use crate::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use ymap::auth::config::ProductionConfig;

use crate::{error_template::ErrorTemplate, pages::logged_in::LoggedIn, pages::login::Login};



#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	let config = ProductionConfig::new();
	provide_context(AppState {
		config,
		db: OnceCell::default(),
	});

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
				// <Box style="position: relative; border: 4px solid gray; width: 100%; height: 20em; overflow: auto;">
				<nav>// <AppBar style="z-index: 1; background: var(--brand-color); color: white; height: 100%;">
				// <H3 style="margin-left: 1em; color: white;">"YMap"</H3>
				// <Stack
				// orientation=StackOrientation::Horizontal
				// spacing=Size::Em(1.0)
				// style="margin-right: 1em"
				// >
				// <LinkExt
				// href="https://github.com/ActuallyHappening/YMap"
				// target=LinkExtTarget::Blank
				// >
				// <Icon icon=icondata::FaGithubBrands/>
				// </LinkExt>

				// <Link href="/login">"Login" <Icon icon=icondata::LuDoorOpen/></Link>
				// </Stack>
				// </AppBar>
				// </Box>
				</nav>
				<main style="width: 100vw; height: 100vh; display: flex; justify-content: center;">
					<Routes>
						<Route path="/login" view=|| view! { <Login/> }/>
						<Route path="" view=|| view! { <LoggedIn/> }/>
						<Route path="/logged-in" view=|| view! { <LoggedIn/> }/>
					</Routes>
				</main>
			</Router>
		</Root>
	}
}
