use leptos::prelude::*;
use mathquill_leptos::components::MathQuillField;

// please note, you will still need to load the library,
// this example won't work as-is
fn main() {
  let current = RwSignal::new(String::new());
  let on_edit = Callback::new(move |latex: String| {
    // Handle the edit event here,
    // e.g. syncing to a local signal
    current.set(latex);
  });

  leptos::mount::mount_to_body(move || {
    view! {
      <MathQuillField on_edit />
      <p> { move || format!("You have typed in Latex: {}", current.read()) } </p>
    }
  });
}
