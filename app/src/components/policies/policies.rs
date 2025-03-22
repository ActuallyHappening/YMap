use crate::components::article_styles;
use crate::prelude::*;

use leptos_router::{MatchNestedRoutes, StaticSegment, path};
use routes::Route;
use std::{borrow::Cow, fmt::Display};
use strum::IntoEnumIterator;

stylance::import_crate_style!(
  policies_style,
  "src/components/policies/policies.module.scss"
);

#[derive(strum::EnumIter)]
pub enum Policies {
  TermsAndConditions,
  PrivacyPolicy,
  ReturnsPolicy,
}

impl Policies {
  fn suffix(&self) -> &'static str {
    match self {
      Self::TermsAndConditions => "terms-and-conditions",
      Self::PrivacyPolicy => "privacy-policy",
      Self::ReturnsPolicy => "returns-policy",
    }
  }
}

impl Policies {
  pub fn as_path(&self) -> Cow<'static, str> {
    format!("/policies/{}", self.suffix()).into()
  }
}

impl Display for Policies {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::TermsAndConditions => write!(f, "Terms and Conditions"),
      Self::PrivacyPolicy => write!(f, "Privacy Policy"),
      Self::ReturnsPolicy => write!(f, "Returns Policy"),
    }
  }
}

#[component]
pub fn PoliciesWrapper(children: Children) -> impl IntoView {
  let a_attrs = view! { <{..} id="policies-back-button" /> };
  view! {
    <div class=policies_style::parent>
      <A href=TopLevelRoutes::Policies.raw_path() {..a_attrs}>
        "⏴ See all policies"
      </A>
      {children()}
    </div>
  }
}

pub fn PoliciesRoute() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route
      path=StaticSegment(Policies::TermsAndConditions.suffix())
      view=|| {
        view! {
          <PoliciesWrapper>
            <TermsAndConditions />
          </PoliciesWrapper>
        }
      }
    />
    <Route
      path=StaticSegment(Policies::PrivacyPolicy.suffix())
      view=|| {
        view! {
          <PoliciesWrapper>
            <PrivacyPolicy />
          </PoliciesWrapper>
        }
      }
    />
    <Route
      path=StaticSegment(Policies::ReturnsPolicy.suffix())
      view=|| {
        view! {
          <PoliciesWrapper>
            <ReturnsPolicy />
          </PoliciesWrapper>
        }
      }
    />
    <Route path=path!("/*") view=PoliciesHomePage />
  }
  .into_inner()
}

fn PoliciesHomePage() -> impl IntoView {
  let policy_links = Policies::iter()
    .map(|policy| {
      view! {
        <li>
          <A href=policy.as_path()>{policy.to_string()}</A>
        </li>
      }
    })
    .collect_view();

  view! {
    <div class=policies_style::div>
      <h1>"Policies"</h1>
      <ul class=policies_style::policy_list>{policy_links}</ul>
    </div>
  }
}

fn TermsAndConditions() -> impl IntoView {
  let inner_html = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../public/static/html/policies/terms-and-conditions.html"
  ));
  view! { <article class=article_styles::article inner_html=inner_html /> }
}

fn PrivacyPolicy() -> impl IntoView {
  let inner_html = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../public/static/html/policies/privacy-policy.html"
  ));
  view! { <article class=article_styles::article inner_html=inner_html /> }
}

fn ReturnsPolicy() -> impl IntoView {
  let inner_html = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../public/static/html/policies/returns-policy.html"
  ));
  view! { <article class=article_styles::article inner_html=inner_html /> }
}
