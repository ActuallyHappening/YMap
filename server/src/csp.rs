use axum::{middleware::Next, response::Response};
use axum_csp::{CspDirectiveType, CspHeaderBuilder, CspValue};
use http::StatusCode;

/// https://docs.stripe.com/security/guide?csp=csp-connect
fn csp_policy() -> CspHeaderBuilder {
  CspHeaderBuilder::new()
    .add(
      CspDirectiveType::DefaultSrc,
      vec![
        CspValue::SelfSite,
        // TODO REMOVE ASAP
        csp!("https://checkout.stripe.com"),
        csp!("https://*.js.stripe.com"),
        csp!("https://js.stripe.com"),
        csp!("https://hooks.stripe.com"),
      ],
    )
    .add(
      CspDirectiveType::ConnectSrc,
      vec![
        CspValue::SelfSite,
        CspValue::Host {
          value: routes::db_wss().to_string(),
        },
        // CspValue::SchemeHttps,
        csp!("https://checkout.stripe.com"),
        csp!("https://api.stripe.com"),
        // // TODO undocumented? pls remove asap
        // csp!("https://m.stripe.network"),
        // csp!("https://q.stripe.com"),
      ],
    )
    .add(
      CspDirectiveType::FrameSrc,
      vec![
        csp!("https://checkout.stripe.com"),
        csp!("https://*.js.stripe.com"),
        csp!("https://js.stripe.com"),
        csp!("https://hooks.stripe.com"),
        // connect embedded components
        csp!("https://connect-js.stripe.com"),
        // // TODO undocumented? pls remove asap
        // csp!("https://m.stripe.network"),
        // csp!("https://q.stripe.com"),
      ],
    )
    .add(
      CspDirectiveType::ScriptSource,
      vec![
        CspValue::SelfSite,
        // CspValue::SchemeHttps,
        csp!("https://checkout.stripe.com"),
        csp!("https://js.stripe.com"),
        csp!("https://*.js.stripe.com"),
        // connect embedded components
        csp!("https://connect-js.stripe.com"),
        // // TODO undocumented? pls remove asap
        // csp!("https://m.stripe.network"),
        // csp!("https://q.stripe.com"),
      ],
    )
    // .add(CspDirectiveType::ScriptSourceElem, vec![
    //   CspValue::SelfSite,
    //   // CspValue::SchemeHttps,
    //   csp!("https://checkout.stripe.com"),
    //   csp!("https://js.stripe.com"),
    //   csp!("https://*.js.stripe.com"),
    //   // // TODO undocumented? pls remove asap
    //   // csp!("https://m.stripe.network"),
    //   // csp!("https://q.stripe.com"),
    // ])
    .add(
      CspDirectiveType::ImgSrc,
      vec![
        CspValue::SelfSite,
        // CspValue::SchemeHttps,
        csp!("https://*.stripe.com"),
        // connect embedded components
        csp!("https://*.stripe.com"),
        // // TODO undocumented? pls remove asap
        // csp!("https://m.stripe.network"),
        // csp!("https://q.stripe.com"),
      ],
    )
    .add(
      CspDirectiveType::StyleSource,
      vec![
        CspValue::SelfSite,
        csp!("sha256-0hAheEzaMe6uXIKV4EehS9pu1am1lj/KnnzrOYqckXk="),
      ],
    )
  // .add(CspDirectiveType::UpgradeInsecureRequests, vec![])
}

pub(crate) async fn csp_headers(
  req: axum::extract::Request,
  next: Next,
) -> core::result::Result<Response, StatusCode> {
  // wait for the middleware to come back
  let mut response = next.run(req).await;

  let policy = csp_policy();

  // add the header
  let headers = response.headers_mut();
  // headers.insert(axum::http::header::CONTENT_SECURITY_POLICY, policy.finish());
  headers.insert(
    axum::http::header::CONTENT_SECURITY_POLICY_REPORT_ONLY,
    policy.finish(),
  );

  Ok(response)
}

macro_rules! csp {
  ($host:literal) => {
    CspValue::Host {
      value: $host.into(),
    }
  };
}
use csp;
