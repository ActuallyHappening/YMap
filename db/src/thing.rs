use crate::prelude::*;

type AnyValue = surrealdb::Value;

#[derive(Deserialize, Debug)]
pub struct Thing {
  id: ThingId,
  _debug_name: Option<String>,
  parents: Vec<ThingId>,
  payload: payload::Payload,
}

impl surrealdb_layers::Table for Thing {
  const TABLE: &str = "thing";
}

impl surrealdb_layers::GetId for Thing {
  type Table = Self;
  type Id = ThingId;

  fn get_id(&self) -> &Self::Id {
    &self.id
  }
}

impl Thing {
  pub fn _debug_name(&self) -> Option<String> {
    self._debug_name.clone()
  }

  pub fn parents(&self) -> Vec<ThingId> {
    self.parents.clone()
  }

  pub fn payload(&self) -> payload::Payload {
    self.payload.clone()
  }
}

pub mod db {
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

  pub struct TableThing<Auth>(Db<Auth>);

  impl<Auth> GetDb for TableThing<Auth> {
    fn get_db(&self) -> &Surreal<Any> {
      self.0.get_db()
    }
  }

  impl<Auth> surrealdb_layers::DbTable for TableThing<Auth> {
    type Table = Thing;
  }

  impl<Auth> Db<Auth> {
    pub fn thing(self) -> TableThing<Auth> {
      TableThing(self)
    }
  }

  impl TableThing<NoAuth> {
    pub fn select(&self) -> ThingSelector<NoAuth> {
      ThingSelector {
        db: self.0.clone(),
        where_clause: WhereClause::default(),
      }
    }
  }

  pub struct ThingSelector<Auth> {
    db: Db<Auth>,
    where_clause: WhereClause,
  }

  impl<Auth> GetDb for ThingSelector<Auth> {
    fn get_db(&self) -> &Surreal<Any> {
      self.db.get_db()
    }
  }

  impl<Auth> std::ops::Deref for ThingSelector<Auth> {
    type Target = WhereClause;

    fn deref(&self) -> &Self::Target {
      &self.where_clause
    }
  }

  impl<Auth> std::ops::DerefMut for ThingSelector<Auth> {
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

  impl ThingSelector<NoAuth> {
    pub async fn get_known<T>(&self) -> Result<T, Error>
    where
      T: DbRecord,
    {
      let id = T::known_id().surreal_id();
      let thing: Option<Thing> = self
        .get_db()
        .select(id.clone())
        .await
        .map_err(Error::CouldntSelect)?;
      let thing = thing.ok_or(Error::KnownRecordNotFound(id))?;

      let t: T = T::try_from_table_value(thing)?;

      Ok(t)
    }
  }

  #[derive(Debug)]
  pub struct WebsiteRoot {
    inner: Thing,
    websiteroot: WebsiteData,
  }

  impl GetId for WebsiteRoot {
    type Table = Thing;
    type Id = ThingId;

    fn get_id(&self) -> &Self::Id {
      &self.inner.id
    }
  }

  /// A type that represents a known record
  pub trait DbRecord: GetId<Table = Thing> + Sized {
    fn known_id() -> Self::Id;

    fn try_from_table_value(value: Thing) -> Result<Self, Error>;
  }

  impl DbRecord for WebsiteRoot {
    fn known_id() -> Self::Id {
      ThingId::new_known("websiteroot".into())
    }

    fn try_from_table_value(value: Thing) -> Result<Self, Error> {
      Ok(Self {
        websiteroot: WebsiteData::try_from_payload(value.payload())?,
        inner: value,
      })
    }
  }

  #[derive(Deserialize, Debug)]
  pub struct WebsiteData {
    show_children: Vec<ThingId>,
  }

  impl TryFromPayload for WebsiteData {
    fn try_from_payload(payload: Payload) -> Result<Self, Error> {
      let key = WebsiteRoot::known_id();
      let websitedata = payload
        .get(key.clone())
        .ok_or(Error::MissingPayload { key })??;
      Ok(websitedata)
    }
  }
}

mod payload {
  use serde::de::DeserializeOwned;

  use crate::{error::Error, prelude::*};

  use super::{AnyValue, ThingId};

  /// A newtype to handle serialization and deserialization of payloads
  /// since the keys are stored only as strings in the db
  #[derive(Serialize, Deserialize, Clone, Debug)]
  #[serde(try_from = "PayloadSerde", into = "PayloadSerde")]
  pub struct Payload(HashMap<ThingId, AnyValue>);

  impl FromIterator<(ThingId, AnyValue)> for Payload {
    fn from_iter<T: IntoIterator<Item = (ThingId, AnyValue)>>(iter: T) -> Self {
      Payload(iter.into_iter().collect())
    }
  }
  impl IntoIterator for Payload {
    type Item = (ThingId, AnyValue);
    type IntoIter = std::collections::hash_map::IntoIter<ThingId, AnyValue>;

    fn into_iter(self) -> Self::IntoIter {
      self.0.into_iter()
    }
  }

  impl Payload {
    pub fn get<T>(&self, key: ThingId) -> Option<Result<T, Error>>
    where
      T: DeserializeOwned + 'static,
    {
      self.0.get(&key).map(|value| {
        surrealdb::value::from_value(value.clone()).map_err(|err| Error::DeserializePayloadValue {
          key,
          ty: std::any::TypeId::of::<T>(),
          err,
        })
      })
    }
  }

  pub trait TryFromPayload: Sized {
    fn try_from_payload(payload: Payload) -> Result<Self, Error>;
  }

  #[derive(Serialize, Deserialize)]
  struct PayloadSerde(HashMap<String, AnyValue>);

  impl From<Payload> for PayloadSerde {
    fn from(value: Payload) -> Self {
      PayloadSerde(value.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
  }

  impl TryFrom<PayloadSerde> for Payload {
    type Error = surrealdb::Error;

    fn try_from(value: PayloadSerde) -> Result<Self, Self::Error> {
      Ok(Payload(
        value
          .0
          .into_iter()
          .map(|(k, v)| Result::<_, surrealdb::Error>::Ok((ThingId::from_str(&k)?, v)))
          .collect::<Result<_, _>>()?,
      ))
    }
  }
}

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
