use utils::prelude::info;

fn main() {
  utils::tracing::install_tracing("debug").unwrap();

  leptos_rsx::rsx!(div { class: 69 });
}
