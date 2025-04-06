use crate::prelude::*;

type AnyValue = surrealdb::Value;

#[derive(Deserialize, Debug)]
pub struct Thing<Payload> {
  id: ThingId,
  _debug_name: Option<String>,
  parents: Vec<ThingId>,
  payload: Payload,
}

impl<P> surrealdb_layers::Table for Thing<P> {
  const TABLE: &str = "thing";
}

impl<P> surrealdb_layers::GetId for Thing<P> {
  type Table = Self;
  type Id = ThingId;

  fn get_id(&self) -> &Self::Id {
    &self.id
  }
}

impl<P> Thing<P> {
  pub fn _debug_name(&self) -> Option<String> {
    self._debug_name.clone()
  }

  pub fn parents(&self) -> Vec<ThingId> {
    self.parents.clone()
  }

  pub fn payload(&self) -> &P {
    &self.payload
  }
}

/// A type that represents a known record
pub trait ThingRecord: surrealdb_layers::GetId<Table = Thing<Self::Payload>> + Sized {
  type Payload: serde::de::DeserializeOwned;

  fn known_id() -> Self::Id;
}

pub mod well_known;

pub mod db {
  use serde::de::DeserializeOwned;

  use crate::{
    db::auth::NoAuth,
    error::Error,
    layers::{GetDb, GetId},
    prelude::*,
  };

  use super::{
    Thing, ThingId,
    payload::{Payload, TryFromPayload},
  };

  pub struct TableThing<Auth, P> {
    db: Db<Auth>,
    payload: PhantomData<P>,
  }

  impl<Auth, P> GetDb for TableThing<Auth, P> {
    fn get_db(&self) -> &Surreal<Any> {
      self.db.get_db()
    }
  }

  impl<Auth, P> surrealdb_layers::DbTable for TableThing<Auth, P> {
    type Table = Thing<P>;
  }

  impl<Auth> Db<Auth> {
    pub fn thing<P>(self) -> TableThing<Auth, P> {
      TableThing {
        db: self,
        payload: PhantomData,
      }
    }
  }

  impl<P> TableThing<NoAuth, P> {
    pub fn select(&self) -> ThingSelector<NoAuth, P> {
      ThingSelector {
        db: self.db.clone(),
        where_clause: WhereClause::default(),
      }
    }
  }

  pub struct ThingSelector<Auth, P> {
    db: Db<Auth>,
    where_clause: WhereClause,
    payload: PhantomData<P>,
  }

  impl<Auth, P> GetDb for ThingSelector<Auth, P> {
    fn get_db(&self) -> &Surreal<Any> {
      self.db.get_db()
    }
  }

  impl<Auth, P> std::ops::Deref for ThingSelector<Auth, P> {
    type Target = WhereClause;

    fn deref(&self) -> &Self::Target {
      &self.where_clause
    }
  }

  impl<Auth, P> std::ops::DerefMut for ThingSelector<Auth, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.where_clause
    }
  }

  #[derive(Default)]
  pub struct WhereClause {
    parents: Vec<ThingId>,
  }

  impl WhereClause {
    fn into_query(self) -> String {
      let mut base: String = "SELECT * FROM things WHERE true".into();
      if !self.parents.is_empty() {
        base.push_str("(");
        let clauses = self
          .parents
          .into_iter()
          .map(|id| format!("{id} in parents"))
          .collect::<Vec<_>>()
          .join(" OR ");
        base.push_str(&clauses);
        base.push_str(")")
      };
      base
    }
  }

  impl<P> ThingSelector<NoAuth, P>
  where
    P: DeserializeOwned,
  {
    pub async fn get_known<T>(&self) -> Result<T, Error>
    where
      T: ThingRecord,
    {
      let id = T::known_id().surreal_id();
      debug!("About to deserialize");
      let thing = self
        .get_db()
        .select(id.clone())
        .await
        .map_err(Error::CouldntSelect);

      debug!(?thing);
      let thing: Option<Thing> = thing?;

      let thing = thing.ok_or(Error::KnownRecordNotFound(id))?;

      let t: T = T::try_from_table_value(thing)?;

      Ok(t)
    }
  }

  #[derive(Debug)]
  pub struct WebsiteRoot(Thing<WebsiteRootPayload>);

  impl GetId for WebsiteRoot {
    type Table = Thing;
    type Id = ThingId;

    fn get_id(&self) -> &Self::Id {
      &self.inner.id
    }
  }

  impl ThingRecord for WebsiteRoot {
    type Payload = WebsiteRootPayload;

    fn known_id() -> Self::Id {
      ThingId::new_known("websiteroot".into())
    }

    fn try_from_table_value(value: Thing) -> Result<Self, Error> {
      Ok(Self {
        websiteroot: WebsiteRootPayload::try_from_payload(value.payload())?,
        inner: value,
      })
    }
  }

  #[derive(Deserialize, Debug)]
  pub struct WebsiteRootPayload {
    root: Vec<ThingId>,
  }

  impl TryFromPayload for WebsiteRootPayload {
    fn try_from_payload(payload: Payload) -> Result<Self, Error> {
      let key = WebsiteRoot::known_id();
      let websitedata: WebsiteRootPayload = payload
        .get(key.clone())
        .ok_or(Error::MissingPayload { key })??;
      Ok(websitedata)
    }
  }
}

mod payload;

pub use id::ThingId;
pub mod id {
  use crate::prelude::*;

  use super::Thing;

  #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct ThingId(
    #[serde(deserialize_with = "surrealdb_layers::serde::string_or_struct")] surrealdb::RecordId,
  );

  impl surrealdb_layers::Id for ThingId {
    type Table = Thing;

    fn new_known(key: surrealdb::RecordIdKey) -> Self {
      Self((Thing::TABLE, key).into())
    }
    fn surreal_id(&self) -> surrealdb::RecordId {
      self.0.clone()
    }
  }

  /// Forwards Display impl
  impl Display for ThingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      Display::fmt(&self.0, f)
    }
  }

  impl FromStr for ThingId {
    type Err = surrealdb::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      Ok(ThingId(surrealdb::RecordId::from_str(s)?))
    }
  }
}
