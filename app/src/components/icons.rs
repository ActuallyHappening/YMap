use crate::prelude::*;

#[component]
pub fn IconSvg(icon: icondata::Icon) -> impl IntoView {
  let style = icon.style;
  assert!(icon.x.is_none());
  assert!(icon.y.is_none());
  assert!(icon.width.is_none());
  assert!(icon.height.is_none());
  let view_box = icon.view_box;
  assert!(icon.stroke_linecap.is_none());
  assert!(icon.stroke_linejoin.is_none());
  assert!(icon.stroke_width.is_none());
  assert!(icon.fill.is_none());
  let data = icon.data;
  view! { <svg style=style viewBox=view_box inner_html=data></svg> }
}
