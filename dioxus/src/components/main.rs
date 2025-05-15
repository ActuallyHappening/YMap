use crate::{prelude::*, Route};

/// Outlet
#[component]
pub fn Main() -> Element {
	static CSS: Asset = asset!("/src/components/main.css");
	rsx! {
		document::Stylesheet { href: CSS }
		div {
			id: "main-2e9f4c7a71b86fd843771c927a6be20e",
			components::navbar::NavBar {}
			components::sidebar::SideBar {}
			main {
				id: "main-content-8d5361af431d152c99c7c7e39814ff2b",
				Outlet::<Route> {}
			}
		}
	}
}
