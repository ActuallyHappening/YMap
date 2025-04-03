#![allow(non_snake_case)]

pub mod prelude {
  pub(crate) use leptos::prelude::*;
  pub(crate) use url::Url;

  pub(crate) use crate::errors::Error;
}

pub mod errors {
  use crate::prelude::*;

  #[derive(Debug, thiserror::Error)]
  pub enum Error {
    #[error("Url must end with a trailing slash")]
    UrlNotTrailingSlash(Url),

    #[error("Couldn't join URL")]
    CouldntJoinUrl {
      orig: Url,
      path: String,
      #[source]
      source: url::ParseError,
    },
  }
}

pub mod assets {
  use crate::prelude::*;

  /// The absolute base path for assets to be loaded from the frontend,
  /// see [`AssetsBasePath::new`]
  pub struct AssetsBasePath(Url);

  impl AssetsBasePath {
    /// Must be absolute, e.g. https://your-domain.com/static/
    pub fn new(url: Url) -> Result<Self, Error> {
      if !url[url::Position::BeforePath..url::Position::AfterPath].ends_with('/') {
        Err(Error::UrlNotTrailingSlash(url))
      } else {
        Ok(Self(url))
      }
    }

    pub fn into_inner(self) -> Url {
      self.0
    }

    pub fn jquery_path() -> &'static str {
      "jquery.min.js"
    }

    pub fn jquery_js(&self) -> Result<Url, Error> {
      self
        .0
        .join(AssetsBasePath::jquery_path())
        .map_err(|err| Error::CouldntJoinUrl {
          orig: self.0.clone(),
          path: AssetsBasePath::jquery_path().to_string(),
          source: err,
        })
    }

    pub fn mathquill_css_path() -> &'static str {
      "mathquill.css"
    }

    pub fn mathquill_css(&self) -> Result<Url, Error> {
      self
        .0
        .join(AssetsBasePath::mathquill_css_path())
        .map_err(|err| Error::CouldntJoinUrl {
          orig: self.0.clone(),
          path: AssetsBasePath::mathquill_css_path().to_string(),
          source: err,
        })
    }

    pub fn mathquill_js_path() -> &'static str {
      "mathquill.min.js"
    }

    pub fn mathquill_js(&self) -> Result<Url, Error> {
      self
        .0
        .join(AssetsBasePath::mathquill_js_path())
        .map_err(|err| Error::CouldntJoinUrl {
          orig: self.0.clone(),
          path: AssetsBasePath::mathquill_js_path().to_string(),
          source: err,
        })
    }
  }
}

pub mod components {
  use leptos_meta::{Link, Script};

  use crate::{assets::AssetsBasePath, prelude::*};

  /// Mathquill requires JQuery (idk why)
  pub fn JQueryScript(assets_dir: AssetsBasePath) -> impl IntoView {
    let src = assets_dir
      .jquery_js()
      .expect("Couldn't get jquery_js")
      .to_string();
    view! {
        <Script src />
    }
  }

  pub fn MathquillScript(assets_dir: AssetsBasePath) -> impl IntoView {
    let src = assets_dir
      .mathquill_js()
      .expect("Couldn't get mathquill js")
      .to_string();
    view! {
        <Script src />
    }
  }

  pub fn MathquillCss(assets_dir: AssetsBasePath) -> impl IntoView {
    let href = assets_dir
      .mathquill_css()
      .expect("Couldn't get mathquill css")
      .to_string();
    view! {
        <Link rel="stylesheet" href />
    }
  }
}
