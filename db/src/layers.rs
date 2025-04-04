#![allow(async_fn_in_trait)]

pub mod prelude {
  pub use super::{ConnBuilderUrl as _, DbConnBuilder as _, GetDb as _, Id as _, Table as _};
  pub use surrealdb::Surreal;
  pub use surrealdb::engine::any::Any;
}

use surrealdb::{
  Surreal,
  engine::any::{self, Any},
};

use crate::prelude::*;

/// A surrealdb table
pub trait Table {
  const TABLE: &str;
}

/// A surrealdb table, with a unique ID type
pub trait TableWithId: Table {
  type Id: Id<Table = Self>;

  fn get_id(&self) -> &Self::Id;
  fn id(&self) -> Self::Id {
    self.get_id().clone()
  }
}

/// A newtype around a database connection associated
/// only with a single table
pub trait DbTable: GetDb {
  type Table: Table;
  
  
}

/// A transparent wrapper around an ID that has the
/// invariant of pointing to a specific table,
/// with the table being pointed to being completely
/// known just from the type, e.g. `UserId` would implement
/// `Id` for the `User` table.
///
/// This type SHOULD deserialize as either a string or the
/// [`surrealdb::RecordId`] type, but should always serialize as
/// the [`surrealdb::RecordId`] type.
///
/// This type SHOULD [`Display`] as tablename:recordidkey, which is the
/// default for [`surrealdb::RecordId`].
pub trait Id: Debug + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Display {
  type Table: Table;

  /// Usually derived from associated [`Self::Table`]
  const TABLE: &str = Self::Table::TABLE;
}

/// For your db wrapper types, allows you to access the underlying db,
/// usually only in your wrapping code rather than your consuming code
/// so your abstractions don't leak too much
pub trait GetDb {
  fn get_db(&self) -> &Surreal<Any>;
  fn db(&self) -> Surreal<Any> {
    self.get_db().clone()
  }
}

/// A struct holding the information used to initially authenticate a query,
/// e.g. email + plaintext_password.
pub trait Creds {
  type Auth: Auth;

  async fn signin(&self, db: &Surreal<Any>) -> Result<Self::Auth, surrealdb::Error>;
}

/// A struct holding the information necessary to keep
/// authenticating the query, usually a [surrealdb::opt::auth::Jwt]
pub trait Auth: Sized {
  async fn authenticate(&self, db: &Surreal<Any>) -> Result<Self, surrealdb::Error>;
}

fn url_with_scheme(mut url: Url, scheme: &'static str) -> Result<Url, Error> {
  url
    .set_scheme(scheme)
    .map_err(|_| Error::CouldntSetScheme {
      url: url.clone(),
      scheme: scheme.to_owned(),
    })?;
  Ok(url)
}

/// Methods to chose the url to connect to,
/// then move onto the next stage
pub trait ConnBuilderUrl: Sized {
  type Next;

  fn url(self, url: Url) -> Self::Next;
  fn default_url(&self) -> Result<Url, Error>;

  fn default(self) -> Result<Self::Next, Error> {
    let url = self.default_url()?;
    Ok(self.url(url))
  }

  fn wss_url(&self) -> Result<Url, Error> {
    url_with_scheme(self.default_url()?, "wss")
  }
  fn wss(self) -> Result<Self::Next, Error> {
    let url = self.wss_url()?;
    Ok(self.url(url))
  }
  fn ws_url(&self) -> Result<Url, Error> {
    url_with_scheme(self.default_url()?, "ws")
  }
  fn ws(self) -> Result<Self::Next, Error> {
    let url = self.ws_url()?;
    Ok(self.url(url))
  }
  fn https_url(&self) -> Result<Url, Error> {
    url_with_scheme(self.default_url()?, "ws")
  }
  fn https(self) -> Result<Self::Next, Error> {
    let url = self.https_url()?;
    Ok(self.url(url))
  }
  fn http_url(&self) -> Result<Url, Error> {
    url_with_scheme(self.default_url()?, "ws")
  }
  fn http(self) -> Result<Self::Next, Error> {
    let url = self.http_url()?;
    Ok(self.url(url))
  }
}

pub trait DbConnBuilder {
  type Next;

  fn get_ns(&self) -> impl Into<String>;
  fn ns(&self) -> String {
    self.get_ns().into()
  }

  fn get_db(&self) -> impl Into<String>;
  fn db(&self) -> String {
    self.get_db().into()
  }

  fn get_url(&self) -> &Url;
  fn url(&self) -> Url {
    self.get_url().clone()
  }

  /// Connects to the url.
  /// Sets the correct NS and DB.
  /// Doesn't handle authentication/credentials.
  async fn db_connect(&self) -> Result<Surreal<Any>, Error> {
    let db = any::connect(self.get_url().to_string())
      .await
      .map_err(|source| Error::CouldntConnect {
        url: self.url(),
        source,
      })?;
    db.use_ns(self.get_ns())
      .use_db(self.get_db())
      .await
      .map_err(Error::CouldntSetNsDb)?;
    Ok(db)
  }

  async fn db_authenticate(&self, conn: Surreal<Any>) -> Result<Self::Next, Error>;

  /// Connects, sets ns and db, and handles authentication all at once!!
  async fn connect(&self) -> Result<Self::Next, Error> {
    let conn = self.db_connect().await?;
    self.db_authenticate(conn).await
  }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{0}")]
  ParseError(#[from] url::ParseError),

  #[error("Couln't set the scheme to {scheme}")]
  CouldntSetScheme { url: Url, scheme: String },

  #[error("Couln't connect to {url}: {source}")]
  CouldntConnect {
    url: Url,
    #[source]
    source: surrealdb::Error,
  },

  #[error("Couldn't set ns/db: {0}")]
  CouldntSetNsDb(#[source] surrealdb::Error),

  #[error("Couldn't authenticate: {0}")]
  CouldntAuthenticate(#[source] surrealdb::Error),
}

pub mod serde {
  use core::fmt;

  use serde::{
    Deserializer,
    de::{self, Visitor},
  };

  use crate::prelude::*;

  pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
  where
    T: Deserialize<'de> + FromStr,
    <T as FromStr>::Err: Display,
    D: Deserializer<'de>,
  {
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
      T: Deserialize<'de> + FromStr,
      <T as FromStr>::Err: Display,
    {
      type Value = T;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or map")
      }

      fn visit_str<E>(self, value: &str) -> Result<T, E>
      where
        E: de::Error,
      {
        FromStr::from_str(value).map_err(|err| E::custom(err))
      }

      fn visit_map<M>(self, map: M) -> Result<T, M::Error>
      where
        M: de::MapAccess<'de>,
      {
        // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
        // into a `Deserializer`, allowing it to be used as the input to T's
        // `Deserialize` implementation. T then deserializes itself using
        // the entries from the map visitor.
        Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
      }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
  }
}
