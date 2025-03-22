use crate::{components::accounts::AccountRoutes, prelude::*};

stylance::import_crate_style!(footer_styles, "src/components/footer/footer.module.scss");

pub fn Footer() -> impl IntoView {
  view! { <footer class=footer_styles::footer>{upper_footer()} {lower_footer()}</footer> }
}

fn upper_footer() -> impl IntoView {
  view! {
    <div class=footer_styles::upper>
      <FooterSection title=None>
        <crate::brand::Logo />
        <p>{crate::brand::MISSION_STATEMENT}</p>
        <div class=footer_styles::socials>
          <crate::brand::InstagramLink />
          <crate::brand::TiktokLink />
          <crate::brand::FacebookLink />
        </div>
      </FooterSection>
      <FooterSection title=Some("About Us")>
        <A href="/about#About%20Us">"About Us"</A>
        <A href="/about#Owners Overview">"Owners Overview"</A>
        <A href="/about#Support">"Support"</A>
        <A href="/about#FAQs">"FAQs"</A>
      </FooterSection>
      <FooterSection title=Some(
        "Links",
      )>
        {TopLevelRoutes::iter_footer()
          .map(|route: TopLevelRoutes| {
            let href = route.raw_path().to_string();
            view! { <A href=href>{route.name()}</A> }
          })
          .collect_view()}
      </FooterSection>
      <FooterSection title=Some(
        "Accounts",
      )>
        {AccountRoutes::iter_footer()
          .map(|route| {
            let href = route.path().to_string();
            view! { <A href=href>{route.name()}</A> }
          })
          .collect_view()}
      </FooterSection>
      <FooterSection title=Some("Account")>
        <crate::components::db_conn::DbConnectionStatus />
      </FooterSection>
    </div>
  }
}

// Print Smarter, Save Bigger

#[component]
fn FooterSection(children: Children, title: Option<&'static str>) -> impl IntoView {
  view! {
    <div class=footer_styles::section>
      {title
        .map(|t| {
          view! { <h3>{t}</h3> }
        })} {children()}
    </div>
  }
}

fn lower_footer() -> impl IntoView {
  view! {
    <div class=footer_styles::lower>
      <p>"Copyright © 2025 JYD. All rights reserved."</p>
      <LegalBtn />
    </div>
  }
}

fn LegalBtn() -> impl IntoView {
  let href = TopLevelRoutes::Policies.raw_path();
  view! { <A href=href>"Legal"</A> }
}
