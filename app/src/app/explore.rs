use thing::well_known::DocumentedPayload;

use crate::{
	db::{DbConn, load_payload, root_things},
	prelude::*,
};

use super::params_id;

/// Used to wrap the /explore route
pub fn Explore() -> impl IntoView {
	view! {
		<div class="explore-root-erwouifnjk7fh">
			<div class="expore-content-fdsh7432589FHDbdf">
				<h1> "Explore the YMap knowledge database"</h1>
				<Outlet />
			</div>
		</div>
	}
}

/// See a things children
pub fn ExploreRoot() -> impl IntoView {
	let root_things = root_things();
	let ui = move || {
		let ids = root_things.get()?;
		AppResult::Ok(view! { <ExploreThings ids=ids /> })
	};
	ui.handle_error()
}

pub fn ExploreChild() -> impl IntoView {
	let id = params_id();
	let ids = LocalResource::new(move || {
		let child = id.get();
		async move {
			let children = DbConn::from_context()
				.read()
				.guest()?
				.children_of_thing(child?)
				.await?;
			AppResult::Ok(children)
		}
	});
	let ui = move || match ids.get() {
		None => Err(AppError::DataLoading),
		Some(ids) => {
			let ids = ids.take()?;
			Ok(view! { <ExploreThings ids=ids /> })
		}
	};
	ui.handle_error()
}

/// Preview of thigns
#[component]
fn ExploreThings(#[prop(into)] ids: Signal<Vec<ThingId>>) -> impl IntoView {
	let thing_previews = move || {
		ids
			.get()
			.into_iter()
			.map(|id| view! { <ThingPreview id=id /> })
			.collect_view()
	};
	view! {
		<div class="explore-things-hbn273869dchk374">
			{ thing_previews }
		</div>
	}
}

#[component]
fn ThingPreview(#[prop(into)] id: Signal<ThingId>) -> impl IntoView {
	let description = load_payload::<DocumentedPayload>(id);
	let ui = move || -> AppResult<_> {
		let desc = description.get()?;
		let title = desc.payload().name.to_string();
		// let description = desc.payload().description.to_string();
		Ok(view! {
			<h2>{title}</h2>
			// <p>{description}</p>
		})
	};
	view! {
		<div class="thing-preview-oui2397840CH729384CH432h">
			{ ui.handle_error() }
			<A href=format!("/thing/{}", id.get().key())>"Go to"</A>
			<A href=format!("/explore/{}", id.get().key())>"Explore"</A>
			<p>{move || id.get().to_string()}</p>
		</div>
	}
}
