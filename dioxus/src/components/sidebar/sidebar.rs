use crate::prelude::*;

mod db_status;

#[component]
pub fn SideBar() -> Element {
	static CSS: Asset = asset!("/src/components/sidebar/sidebar.css");
	rsx! {
		document::Stylesheet { href: CSS }
		AppErrorBoundary {
			div {
				class: "sidebar-905d91c3b13f1d6d5124584221b162dc",
				"Sidebar!"
				components::db::DbConnector { }
				components::sidebar::db_status::DbConnectionStatus { }
			}
		}
	}
}
