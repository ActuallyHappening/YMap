#![allow(non_snake_case)]

pub mod prelude {
  pub(crate) use leptos::prelude::*;
  #[allow(unused_imports)]
  pub(crate) use tracing::{debug, error, info, trace, warn};
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
  ///
  /// It is best you use this API, because the mathquill .css file
  /// imports from paths relative to itself
  #[derive(Debug, Clone)]
  pub struct AssetsBasePath {
    url: Url,
    pub mathquill_dir_path: &'static str,
  }

  #[test]
  fn assets_path_requires_trailing_slash() {
    let bad = "https://demo.com/page";
    let url = bad.parse().unwrap();
    let _assets_dir = AssetsBasePath::new(url, "any").expect_err("Didn't reject no trailing slash");
  }

  impl AssetsBasePath {
    /// Must be absolute with a trailing slash, e.g. https://your-domain.com/static/
    pub fn new(url: Url, mathquill_dir_path: &'static str) -> Result<Self, Error> {
      if !url[url::Position::BeforePath..url::Position::AfterPath].ends_with('/') {
        Err(Error::UrlNotTrailingSlash(url))
      } else {
        Ok(Self {
          url,
          mathquill_dir_path,
        })
      }
    }

    pub fn into_inner(self) -> Url {
      self.url
    }

    fn join(&self, path: &str) -> Result<AssetsBasePath, Error> {
      self
        .url
        .join(path)
        .map_err(|err| Error::CouldntJoinUrl {
          orig: self.url.clone(),
          path: path.to_owned(),
          source: err,
        })
        .map(|url| Self {
          url,
          mathquill_dir_path: self.mathquill_dir_path,
        })
    }

    pub fn jquery_path() -> &'static str {
      "jquery.min.js"
    }

    pub fn jquery_js(&self) -> Result<Url, Error> {
      self.join(Self::jquery_path()).map(Self::into_inner)
    }

    pub fn mathquill_dir_path(&self) -> &'static str {
      self.mathquill_dir_path
    }

    pub fn mathquill_css_path() -> &'static str {
      "mathquill.css"
    }

    pub fn mathquill_css(&self) -> Result<Url, Error> {
      self
        .join(self.mathquill_dir_path())?
        .join(Self::mathquill_css_path())
        .map(Self::into_inner)
    }

    pub fn mathquill_js_path() -> &'static str {
      "mathquill.min.js"
    }

    pub fn mathquill_js(&self) -> Result<Url, Error> {
      self
        .join(self.mathquill_dir_path())?
        .join(Self::mathquill_js_path())
        .map(Self::into_inner)
    }
  }
}

pub mod components;

mod js {
  use crate::prelude::*;

  use web_sys::wasm_bindgen::{JsCast as _, prelude::Closure};

  pub struct MathQuill(mathquill_js_sys::MathQuill);

  pub struct Config(mathquill_js_sys::Config);

  impl Default for Config {
    fn default() -> Self {
      let closure: Closure<dyn FnMut()> = Closure::new(|| info!("Default editted!"));
      let callback = Box::leak(Box::new(closure));
      Self(mathquill_js_sys::Config {
        space_behaves_like_tab: true,
        handlers: mathquill_js_sys::Handlers {
          edit: callback.as_ref().clone().unchecked_into(),
        },
      })
    }
  }

  impl MathQuill {
    pub fn get_global_interface() -> Self {
      Self(mathquill_js_sys::MathQuill::getInterface(2))
    }

    pub fn mount_field(&self, html_element: &web_sys::HtmlElement, config: Config) -> MathField {
      MathField(self.0.MathField(html_element, Some(config.0)))
    }

    // pub fn mount_static(&self, node_ref: &web_sys::HtmlEle)
  }

  pub struct MathField(mathquill_js_sys::MathField);

  impl MathField {
    pub(crate) fn latex(&self) -> String {
      self.0.latex()
    }
  }
}
