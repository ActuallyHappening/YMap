mod prelude {
  #![allow(unused_imports)]

  pub(crate) use camino::{Utf8Path, Utf8PathBuf};
  pub(crate) use quote::TokenStreamExt as _;
  pub(crate) use quote::quote;
  pub(crate) use std::fs;

  pub(crate) use crate::error::*;
}

use crate::prelude::*;

mod error {
  use crate::prelude::*;

  #[allow(dead_code)]
  #[derive(Debug)]
  pub(crate) enum Error {
    Syn(syn::Error),
    // PathIsntUtf8(camino::FromPathBufError),
    CantFindToml {
      err: std::io::Error,
      path: Utf8PathBuf,
    },
    EnvVariableDoesntExist {
      key: String,
      err: std::env::VarError,
    },
    CantParseToml {
      err: toml::de::Error,
    },
    KeyDoesntExist {
      key: String,
    },
    KeyIsntString {
      key: String,
      err: toml::de::Error,
    },
  }

  impl Error {
    pub fn into_compile_error(self) -> proc_macro2::TokenStream {
      if let Error::Syn(err) = self {
        err.into_compile_error()
      } else {
        syn::Error::new(proc_macro2::Span::call_site(), format!("{:?}", self)).into_compile_error()
      }
    }
  }

  impl From<syn::Error> for Error {
    fn from(value: syn::Error) -> Error {
      Error::Syn(value)
    }
  }

  pub(crate) type Result<T> = core::result::Result<T, Error>;
}

struct Input {
  key: String,
}

impl Input {
  fn key(self) -> String {
    self.key
  }
}

impl syn::parse::Parse for Input {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let key = input.parse::<syn::LitStr>()?.value();
    Ok(Input { key })
  }
}

#[proc_macro]
pub fn include_toml_key(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as Input);

  let output = if cfg!(feature = "load-through-env") {
    input.process_env()
  } else {
    input.process_file()
  };

  output
    .map(|o| quote! { #o })
    .unwrap_or_else(Error::into_compile_error)
    .into()
}

struct Output {
  value: String,
}

impl quote::ToTokens for Output {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let value = syn::LitStr::new(&self.value, proc_macro2::Span::call_site());
    value.to_tokens(tokens);
  }
}

impl Input {
  pub fn process_file(self) -> Result<Output> {
    let env_path = env_path()?;
    let value = load_toml_key(&env_path, &self.key())?;
    Ok(Output { value })
  }

  pub fn process_env(self) -> Result<Output> {
    let key = self.key();
    let value = std::env::var(&key).map_err(|err| Error::EnvVariableDoesntExist { key, err })?;
    Ok(Output { value })
  }
}

fn env_path() -> Result<Utf8PathBuf> {
  let mut base = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  assert!(base.pop());
  assert!(base.pop());
  Ok(base.join(".env.toml"))
}

fn load_toml_key(path: &Utf8Path, key: &str) -> Result<String> {
  let file = fs::read_to_string(path).map_err(|err| Error::CantFindToml {
    err,
    path: path.to_owned(),
  })?;
  let file: toml::Table = file.parse().map_err(|err| Error::CantParseToml { err })?;
  let data = file.get(key).ok_or(Error::KeyDoesntExist {
    key: key.to_owned(),
  })?;
  let str = data
    .clone()
    .try_into()
    .map_err(|err| Error::KeyIsntString {
      key: key.to_owned(),
      err,
    })?;
  Ok(str)
}
