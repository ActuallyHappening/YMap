use leptos::prelude::*;
use mathquill_leptos::components::MathQuillStatic;

// please note, you will still need to load the library,
// this example won't work as-is (in your own code)
fn main() {
  leptos::mount::mount_to_body(move || {
    let current = RwSignal::new(String::from("1 + 1 = 2"));

    view! {
      <label for="latex-input">"Type latex here:"</label>
      <input id="latex-input" type="text" bind:value=current />

      <p>"And your latex will appear rendered here:"</p>
      <MathQuillStatic latex=current />
    }
  });
}
