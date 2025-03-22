use crate::prelude::*;

stylance::import_crate_style!(
  stars_styles,
  "src/components/reviews/stars/stars.module.scss"
);

#[component]
pub fn Stars(#[prop(into)] num: Signal<u8>) -> impl IntoView {
  let states = Memo::new(move |_| {
    let mut offs = Vec::with_capacity(5);
    offs.extend(vec![false; 5]);
    for i in 0..num.get() {
      offs[i as usize] = true;
    }
    offs
  });
  move || {
    states
      .get()
      .into_iter()
      .map(|on| view! { <Star on /> })
      .collect_view()
  }
}

#[component]
fn Star(#[prop(into)] on: Signal<bool>) -> impl IntoView {
  const ON: &str = "/static/images/star-on.svg";
  const OFF: &str = "/static/images/star-off.svg";

  let src = move || match on.get() {
    true => ON,
    false => OFF,
  };
  view! { <img src=src /> }
}
