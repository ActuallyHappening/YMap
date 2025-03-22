use crate::{
  db::DbState,
  errors::{AppError, AppRes},
  prelude::*,
};

stylance::import_crate_style!(review_styles, "src/components/reviews/reviews.module.scss");

use db::cartridges::CartridgeId;

use stars::*;
#[path = "stars/stars.rs"]
mod stars;

#[path = "place/place.rs"]
mod place;

pub use routes::{Router, *};
mod routes {
  use crate::prelude::*;

  use super::place::PlaceReview;

  #[derive(Debug, Clone)]
  pub enum ReviewRoutes {
    Place,
    // View,
  }

  impl NestedRoute for ReviewRoutes {
    fn nested_base(&self) -> impl Route {
      TopLevelRoutes::Review
    }

    fn raw_path_suffix(&self) -> String {
      "/place".into()
    }
  }

  pub fn Router() -> impl MatchNestedRoutes + Clone {
    view! {
      <Route path=path!("/place") view=PlaceReview />
    }
    .into_inner()
  }
}

#[component]
pub fn ReviewsPreview(#[prop(into)] product: Signal<CartridgeId>) -> impl IntoView {
  let db = DbState::from_context();
  let rating = move || {
    db.read()
      .conn_old()
      .map(|conn| conn.reviews_downgraded().rating_for(product.get()))
      .err_generic_ref()
  };
  move || -> AppRes<_> {
    rating()
      .map(|rating| match rating {
        Some(rating) => {
          // rounds
          // todo feature: make stars continuous like JB HIFI
          let num = rating.average as u8;
          Either::Left(view! {
            <Stars num />
            <p> { format!("{:.2}", rating.average) } </p>
            <p> { format!("({})", num) } </p>
            // TODO: add review
          })
        }
        None => Either::Right(view! {}),
      })
      .map_err(AppError::from)
  }
}
