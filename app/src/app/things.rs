use crate::{
  app::{description, latex_demo},
  db::DbConn,
  prelude::*,
};

pub fn ThingView() -> impl IntoView {
  let id = Signal::derive(move || {
    ThingId::new_known(
      leptos_router::hooks::use_params_map()
        .get()
        .get("id")
        .expect("Only render main with :id path param")
        .into(),
    )
  });
  view! {
    <FullView id=id />
  }
}

#[component]
fn FullView(id: Signal<ThingId>) -> impl IntoView {
  view! {
    // <ErrorBoundary name="Latex Demo">
      <description::DescriptionView id=id />
      <latex_demo::LatexDemo id=id />
      <p> { move || id.get().to_string() } </p>
      <ManualAddParent id=id />
    // </ErrorBoundary>
  }
}

#[component]
fn ManualAddParent(id: Signal<ThingId>) -> impl IntoView {
  let add_parent = Action::new_local(move |info: &(ThingId, ThingId)| {
    let (child, parent) = info.clone();
    let db = DbConn::from_context();
    async move {
      db.read()
        .guest()?
        .relate_parents(child, vec![parent])
        .await?;
      AppResult::Ok(())
    }
  });
  let parent_id = RwSignal::new(String::new());
  let on_click = move |_| {
    let res = parent_id.get().parse::<ThingId>();
    match res {
      Err(err) => add_parent
        .value()
        .set(Some(Err(AppError::CouldntParseRecordId {
          err: GenericError::from(err),
          str: std::sync::Arc::from(parent_id.get()),
        }))),
      Ok(parent) => {
        add_parent.dispatch((id.get(), parent));
      }
    }
  };
  view! {
    <div>
    <label for="parent-id">"Manually add a parent:"</label>
      <input type="text" name="parent-id" placeholder="Parent ID" bind:value=parent_id />
      <button on:click=on_click>
        "Add parent"
      </button>
    </div>
  }
}
