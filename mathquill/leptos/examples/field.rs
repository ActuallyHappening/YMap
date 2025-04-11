use leptos::prelude::*;
use mathquill_leptos::components::MathQuillField;

// please note, you will still need to load the library,
// this example won't work as-is (in your own code)
fn main() {
  let current = RwSignal::new(String::from("initial=i^3*n*a*l"));

  leptos::mount::mount_to_body(move || {
    view! {
      <MathQuillField latex=current />
      <p> { move || format!("You have typed in Latex: {}", current.read()) } </p>
    }
  });
}
