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

  pub fn MathQuillField() -> impl IntoView {
    let node_ref = NodeRef::new();
    node_ref.on_load(|el: web_sys::HtmlSpanElement| {
      let mathquill = crate::js::MathQuill::get_global_interface();
      let field = mathquill.mount_field(&el, crate::js::Config::default());

      let current = field.latex();
      info!(?current, "MathQuillField mounted");
    });
    view! {
        <span node_ref=node_ref />
    }
  }
}

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
