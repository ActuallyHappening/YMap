use mathquill_leptos::components::MathQuillField;
use thing::{payload::IsPayloadEntry, well_known::KnownRecord};

use crate::prelude::*;

pub struct LatexDemoPage(Thing<LatexDemoPayload>);

impl KnownRecord for LatexDemoPage {
  fn known() -> &'static str {
    "6uwvf0js9234j0tnvp92"
  }
}

#[derive(PSerialize, PDeserialize)]
pub struct LatexDemoPayload {
  #[serde(rename(expr = "LatexDemoEntry::known()"))]
  demo: LatexDemoEntry,
}

#[derive(Serialize, Deserialize)]
pub struct LatexDemoEntry {
  esxample_latex: String,
}

impl IsPayloadEntry for LatexDemoEntry {
  fn known() -> &'static str {
    LatexDemoPage::known()
  }
}

#[component]
pub fn LatexDemo(id: Signal<ThingId>) -> impl IntoView {
  if id.get() != LatexDemoPage::known_id() {
    return None;
  }

  let initial_latex = todo!();

  let latex = RwSignal::new(String::new());
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

  Some(view! {
    <h1> "YMap" </h1>
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
}
