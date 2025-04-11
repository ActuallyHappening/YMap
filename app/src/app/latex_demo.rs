use mathquill_leptos::components::MathQuillField;
use thing::{payload::KnownPayloadEntry, well_known::KnownRecord};

use crate::prelude::*;

#[derive(Deserialize, Clone, Debug)]
pub struct LatexDemoPage(Thing<LatexDemoPayload>);

impl KnownRecord for LatexDemoPage {
  fn known() -> &'static str {
    "6uwvf0js9234j0tnvp92"
  }
}

#[derive(PSerialize, PDeserialize, Clone, Debug)]
pub struct LatexDemoPayload {
  #[serde(rename(expr = "LatexDemoEntry::known()"))]
  demo: LatexDemoEntry,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LatexDemoEntry {
  example_latex: String,
}

impl KnownPayloadEntry for LatexDemoEntry {
  fn known() -> &'static str {
    LatexDemoPage::known()
  }
}

#[component]
pub fn LatexDemo(id: Signal<ThingId>) -> Option<impl IntoView> {
  if id.get() != LatexDemoPage::known_id() {
    return None;
  }

  let initial_latex = Signal::derive(move || {
    super::known_id::<LatexDemoPage>().map(|page| page.0.payload().demo.example_latex.clone())
  });
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
