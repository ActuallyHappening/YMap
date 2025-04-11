use crate::prelude::*;

#[derive(Deserialize, Clone, Debug)]
pub struct LatexDemoPage {
  doesnt_exist: String,
}

#[component]
pub fn LatexDemo() -> impl IntoView {
  let initial_latex = Signal::derive(move || super::known_id().map(|page| page.doesnt_exist));
  let ui = move || -> AppResult<_> {
    let latex = RwSignal::new(initial_latex.get()?);

    Ok(view! {
      <h1> "YMap" </h1>
      <p> {latex} </p>
    })
  };
  let ui = move || {
    let ui = ui();
    if let Err(err) = &ui {
      debug!(?err, "Error the UI is rendering");
    } else {
      debug!("The ui is rendering a normal view");
    }
    ui
  };

  Some(ui)
}
