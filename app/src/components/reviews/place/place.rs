use db::{
  cartridges::CartridgeId,
  reviews::{PlaceReviewErr, Review, ReviewBuilder},
};
use leptos_router::{
  hooks::use_query,
  params::{IntoParam, ParamsError},
};

use crate::{
  db::DbState,
  errors::{AppError, AppRes},
  prelude::*,
};

stylance::import_crate_style!(
  place_styles,
  "src/components/reviews/place/place.module.scss"
);

#[derive(Params, Clone, PartialEq)]
struct PlaceParams {
  /// only key
  cartridge: CartridgeKey,
}

#[derive(Clone, PartialEq)]
struct CartridgeKey(CartridgeId);

impl IntoParam for CartridgeKey {
  fn into_param(
    value: Option<&str>,
    name: &str,
  ) -> std::result::Result<Self, leptos_router::params::ParamsError> {
    value
      .ok_or(ParamsError::MissingParam(name.to_string()))
      .map(|s| CartridgeKey(CartridgeId::from_key(s)))
  }
}

pub fn PlaceReview() -> impl IntoView {
  let query = use_query::<PlaceParams>();
  let db = DbState::from_context();

  let cartridge = move || {
    query
      .get()
      .map(|params| params.cartridge.0)
      .map_err(AppError::from)
  };
  let action = ServerAction::<PlaceReview>::new();
  let form = move || {
    view! {
      <ActionForm action {..} class=place_styles::form>
        <select name="review[rating]" required>
          <option value="0"> { "0" } </option>
          <option value="1"> { "1" } </option>
          <option value="2"> { "2" } </option>
          <option value="3"> { "3" } </option>
          <option value="4"> { "4" } </option>
          <option value="5"> { "5" } </option>
        </select>
        <label for="ticket[content]">"Any other comments?"</label>
        <textarea
          name="ticket[content]"
          placeholder="Your message here"
          minlength="3"
        />
        <input type="submit" value="Place Review" />
      </ActionForm>
    }
  };
  move || {
    leptos::html::div().class(place_styles::review).child((
      view! {
        <h1>"Place a review"</h1>
      },
      move || match cartridge() {
        Err(_err) => Either::Left(view! {
          <p> "You are missing the cartridge URL param" </p>
        }),
        Ok(cartridge) => Either::Right((
          // user auth
          db.read().conn().map(|conn| {
            conn
              .user()
              .map(|_| view! { <p> "Your review will be linked with your account" </p>})
          }),
          // cartridge preview
          db.read().conn().map(|conn| {
            conn
              .downgrade()
              .cartridges()
              .select()
              .read()
              .as_ref()
              .ok_or(AppError::LoadingContent)
              .map(|cartridges| cartridges.into_iter().find(|c| c.id() == cartridge).next().unwrap_or_)
          }),
        )),
      },
      form,
    ))
  }
}

async fn place_review(review: ReviewBuilder) -> AppRes<Review> {
  AppError::flatten_server_fn(_place_review(review).await)
}

#[server(
  prefix = "/api/reviews",
  endpoint = "/place",
  // input = server_fn::codec::Json,
  output = server_fn::codec::Json
)]
async fn _place_review(
  review: ReviewBuilder,
) -> Result<Result<Review, GenericError<PlaceReviewErr>>, ServerFnError> {
  Ok(thunk_place_review(review).await.err_generic())
}

#[cfg(feature = "ssr")]
async fn thunk_place_review(review: ReviewBuilder) -> Result<Review, PlaceReviewErr> {
  use crate::server_state::ServerAxumState;

  info!(builder = ?review);

  // the magic
  let db = ServerAxumState::from_context().db;
  let review = db.reviews().place_review(review).await?;

  info!(?review, "Review inserted successfully");

  Ok(review)
}
