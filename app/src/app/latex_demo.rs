use mathquill_leptos::components::MathQuillField;
use thing::{
  payload::{IsPayload, KnownPayloadEntry},
  well_known::KnownRecord,
};

use crate::{db::known_thing, prelude::*};

#[derive(Deserialize, Clone, Debug)]
pub struct LatexDemoThing(Thing<LatexDemoPayload>);

impl KnownRecord for LatexDemoThing {
  type Payload = LatexDemoPayload;

  fn known() -> &'static str {
    "6uwvf0js9234j0tnvp92"
  }
  fn from_inner(inner: Thing<Self::Payload>) -> Self {
    Self(inner)
  }
}

#[derive(PSerialize, PDeserialize, Clone, Debug)]
pub struct LatexDemoPayload {
  #[serde(rename(expr = "LatexDemoEntry::known_full()"))]
  demo: LatexDemoEntry,
}

impl IsPayload for LatexDemoPayload {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LatexDemoEntry {
  example_latex: String,
}

impl KnownPayloadEntry for LatexDemoEntry {
  fn known() -> &'static str {
    LatexDemoThing::known()
  }
  fn known_full() -> &'static str {
    "thing:6uwvf0js9234j0tnvp92"
  }
}

#[component]
pub fn LatexDemo(id: Signal<ThingId>) -> impl IntoView {
  let initial_latex = Signal::derive(move || {
    known_thing::<LatexDemoThing>()
      .get()
      .map(|page| page.0.payload().demo.example_latex.clone())
  });
  let ui = move || -> AppResult<_> {
    let latex = RwSignal::new(initial_latex.get()?);
    let on_edit = Callback::new(move |new_latex: String| latex.set(new_latex));
    let latex_ast = move || latex_parser::LatexTokens::parse_from_latex(&latex.read());
    let ir_1 = move || -> Result<_, cas::contexts::scalar::real::Error> {
      let latex = latex_ast()?;
      cas::contexts::scalar::real::IR1Expr::from_latex_tokens(latex)
    };
    let ir2 = move || -> Result<_, cas::contexts::scalar::real::Error> {
      let res = ir_1()?;
      res
        .into_iter()
        .map(|tokens| cas::contexts::scalar::real::IR2Exprs::from_ir1(tokens))
        .collect::<Result<Vec<_>, _>>()
    };
    let ir3 = move || -> Result<_, cas::contexts::scalar::real::Error> {
      let res = ir2()?;
      Ok(
        res
          .into_iter()
          .map(|tokens| cas::contexts::scalar::real::IR3Expr::from_ir2(tokens))
          .collect::<Vec<_>>(),
      )
    };

    Ok(view! {
      <MathQuillField on_edit=on_edit />
      <p> { move || format!("Raw latex: {}", latex.get()) } </p>
      <p> { move || match latex_ast() {
        Ok(ast) => format!("Successfully parsed: {:?}", ast),
        Err(err) => format!("Couldn't parse what you have typed: {}", err),
      } } </p>
      <p> { move || match ir_1() {
        Ok(ir) => format!("Successfully converted to IR1: {:?}", ir),
        Err(err) => format!("Couldn't convert to IR1: {}", err),
      } } </p>
      <p> { move || match ir2() {
        Ok(ir) => format!("Successfully converted to IR2: {:?}", ir),
        Err(err) => format!("Couldn't convert to IR2: {}", err),
      } } </p>
      <p> { move || match ir3() {
        Ok(ir) => format!("Successfully converted to IR3: {:?}", ir),
        Err(err) => format!("Couldn't convert to IR3: {}", err),
      } } </p>
    })
  };
  move || {
    if id.get() == LatexDemoThing::known_id() {
      debug!("Latex demo id!");
      Some(ui.handle_error())
    } else {
      debug!(
        "Not same id: {} != {}",
        id.get(),
        LatexDemoThing::known_id()
      );
      None
    }
  }
}
