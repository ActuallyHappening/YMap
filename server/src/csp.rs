use axum::{middleware::Next, response::Response};
use axum_csp::{CspDirectiveType, CspHeaderBuilder, CspValue};
use http::StatusCode;

/// https://docs.stripe.com/security/guide?csp=csp-connect
fn csp_policy() -> CspHeaderBuilder {
  CspHeaderBuilder::new()
    .add(CspDirectiveType::DefaultSrc, vec![CspValue::SelfSite])
    .add(
      CspDirectiveType::ConnectSrc,
      vec![
        CspValue::SelfSite,
        // CspValue::Host {
        //   value: routes::db_wss().to_string(),
        // },
      ],
    )
    .add(CspDirectiveType::FrameSrc, vec![])
    .add(
      CspDirectiveType::ScriptSource,
      vec![CspValue::SelfSite, CspValue::SchemeHttps],
    )
    .add(CspDirectiveType::ImgSrc, vec![CspValue::SelfSite])
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

#[allow(unused_macros)]
macro_rules! csp {
  ($host:literal) => {
    CspValue::Host {
      value: $host.into(),
    }
  };
}
#[allow(unused_imports)]
use csp;
