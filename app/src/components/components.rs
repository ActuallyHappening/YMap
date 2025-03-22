stylance::import_crate_style!(article_styles, "src/components/article.scss");

pub mod icons;

#[path = "about/about.rs"]
pub mod about;

#[path = "footer/footer.rs"]
pub mod footer;

#[path = "navbar/navbar.rs"]
pub mod navbar;

#[path = "store/store.rs"]
pub mod store;

#[path = "home/home.rs"]
pub mod home;

#[path = "search_bar/search_bar.rs"]
pub mod search_bar;

#[path = "db_conn/db_conn.rs"]
pub mod db_conn;

#[path = "policies/policies.rs"]
pub mod policies;

#[path = "errors/errors.rs"]
pub mod errors;

#[path = "cart/cart.rs"]
pub mod cart;

#[path = "support/support.rs"]
pub mod support;

#[path = "accounts/accounts.rs"]
pub mod accounts;

#[path = "orders/orders.rs"]
pub mod orders;

#[path = "reviews/reviews.rs"]
pub mod reviews;
