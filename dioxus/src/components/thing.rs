use thing::well_known::DocumentedPayload;

use crate::{components::db::DbConn, prelude::*};

#[component]
pub fn ThingPreviewString(id_key: String) -> Element {
	let id = ThingId::parse_key(&id_key)
		.make_generic()
		.map_err(AppError::ParseRouteKey)?;
	rsx! {
		ThingPreview { id: id }
	}
}

#[component]
pub fn ThingPreview(id: ThingId) -> Element {
	let key = id.key();
	// let title = "French (Language)";
	// let description = "French something";
	rsx! {
		div {
			class: "thing-a22e93084e3bef59733a6ba8f99c7e63",
			AppSuspenseBoundary {
				AppErrorBoundary {
					Description { id: id }
				}
			}
			Link {
				to: "/thing/{key}",
				"Go to"
			}
			Link {
				to: "/explore/{key}",
				"Explore"
			}
		}
	}
}

type Res<T> = dioxus::hooks::Resource<Result<T, AppError>>;

#[component]
pub fn Description(id: ThingId) -> Element {
	info!("Description ran");

	let db = DbConn::use_context();
	let documentation: Res<Thing<DocumentedPayload>> = use_resource(move || {
		let status = db.cloned();
		info!("Resource ran: {:?}", status);

		let id = id.clone();
		async move {
			let thing: Thing<DocumentedPayload> = db
				.cloned()
				.guest()?
				.select_thing::<DocumentedPayload>(id.clone())
				.await?
				.ok_or(AppError::ThingDoesntExist(id))?;
			info!("Resource returning {:?}", thing);
			AppResult::Ok(thing)
		}
	});

	use_effect(move || {
		let documentation = documentation.cloned();
		trace!(
			"Description-effect documentation updated: {:?}",
			documentation
		);
	});

	#[component]
	fn Inner(documentation: Res<Thing<DocumentedPayload>>) -> Element {
		debug!("Inner ran: {:?}", documentation.value().cloned());

		use_effect(move || {
			let documentation = documentation.cloned();
			trace!("Inner-effect documentation updated: {:?}", documentation);
		});

		// let documentation = documentation.suspend()?.cloned()?;
		let documentation = match documentation.suspend()?.cloned() {
			Ok(d) => d,
			Err(err) => {
				return rsx! {
					p { "Error found: {err:?}" }
				}
			}
		};
		let name = documentation.payload().name.to_string();
		let description = documentation.payload().description.to_string();

		rsx! {
			// div {
				h1 { "{name}" },
				p { "{description}" }
			// }
		}
	}

	rsx! {
		p { "Description"}
		AppErrorBoundary {
			AppSuspenseBoundary {
				Inner { documentation: documentation }
			}
		}
	}
}
