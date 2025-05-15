use crate::prelude::*;

#[component]
pub fn NavBar() -> Element {
	static CSS: Asset = asset!("/src/components/navbar.css");

	rsx! {
		document::Stylesheet { href: CSS }
		nav {
			id: "navbar-6465feb77a19f9ff0cf92f0936729085"
		}
	}
}
