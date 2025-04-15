use db::auth;

use crate::{
  app::{description, latex_demo, params_id},
  db::DbConn,
  prelude::*,
};

pub fn ThingView() -> impl IntoView {
  let id = params_id();
  let ui = move || {
    let id = id.get()?;
    AppResult::Ok(view! {
      <FullView id=id />
    })
  };
  ui.handle_error()
}

#[component]
pub fn FullView(#[prop(into)] id: Signal<ThingId>) -> impl IntoView {
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
  let db = DbConn::from_context();
  let add_parent = Action::new_local(
    move |info: &(ThingId, ThingId, AppResult<Db<auth::NoAuth>>)| {
      let (child, parent, db) = info.clone();
      info!(?child, ?parent, "Linking child to parent");
      async move {
        let parent = db?
          .relate_parents(child, [parent].into_iter().collect())
          .await?;
        AppResult::Ok(parent[0].clone())
        // AppResult::Ok(todo!())
      }
    },
  );
  let add_parent_computation = move || {
    let Some(res) = add_parent.value().get() else {
      return Err(AppError::None);
    };
    res.map(|parent_id| view! { <p> {format!("Successfully linked this record with a parent ({})", parent_id)} </p> } )
  };
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
        add_parent.dispatch((id.get(), parent, db.read().guest()));
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
      { add_parent_computation.handle_error() }
    </div>
  }
}
