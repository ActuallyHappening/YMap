use crate::prelude::*;
use mathquill_leptos::components::*;

pub fn App() -> impl IntoView {
  leptos_meta::provide_meta_context();

  let latex = RwSignal::new(String::new());
  let on_edit = Callback::new(move |new_latex: String| latex.set(new_latex));
  let latex_ast = move || latex_parser::LatexTokens::parse_from_latex(&latex.read());
  let ir_1 = move || -> Result<_, cas::contexts::scalar::real::Error> {
    let latex = latex_ast()?;
    cas::contexts::scalar::real::IR1Expr::from_latex_tokens(latex)
  };
  let ir2 = move || -> Result<_, cas::contexts::scalar::real::Error> {
    let res = ir_1()?;
    let lhs = res
      .into_iter()
      .next()
      .ok_or(cas::contexts::scalar::real::Error::NoTokens)?;
    cas::contexts::scalar::real::IR2Exprs::from_ir1(lhs)
  };

  view! {
    <h1> "YMap" </h1>
    <MathQuillField on_edit=on_edit />
    <p> { move || latex.get() } </p>
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
  }
}
