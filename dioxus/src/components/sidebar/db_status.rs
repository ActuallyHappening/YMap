use crate::{
	components::db::{DbConn, Waiting},
	prelude::*,
};

#[component]
pub fn DbConnectionStatus() -> Element {
	let db = DbConn::use_context();

	match db.cloned() {
		DbConn::Initial => rsx! { p { "Waiting" } },
		DbConn::Waiting(Waiting::Guest) => rsx! { p { "Connecting to db as guest ..." } },
		DbConn::Connected(_) => rsx! { p { "Connected to database" } },
	}
}
