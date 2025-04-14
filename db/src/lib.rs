//! Specific to YMap abstractions

#![allow(async_fn_in_trait)]

pub mod prelude {
  #![allow(unused_imports)]

  pub(crate) use std::collections::HashMap;
  pub(crate) use std::fmt::{Debug, Display};
  pub(crate) use std::hash::Hash;
  pub(crate) use std::marker::PhantomData;
  pub(crate) use std::str::FromStr;

  pub(crate) use extension_traits::extension;
  pub(crate) use serde::{Deserialize, Serialize};
  pub(crate) use url::Url;

  pub(crate) use crate::Error;
  pub(crate) use utils::prelude::*;

  pub use surrealdb_layers;
  pub use surrealdb_layers::prelude::*;
  pub use thing::prelude::*;

  pub use crate::Db;
  pub use crate::things::ThingExt as _;
}

use std::collections::HashSet;

pub use error::Error;
use serde::de::DeserializeOwned;
use surrealdb_layers::Table;
use thing::{Thing, ThingId, builder::ThingBuilder, parent::ParentId, payload::IsPayload};
pub mod error;

pub mod user;

mod things {
  use futures_core::Stream;
  use surrealdb::{Action, Notification};
  use thing::{Thing, ThingId, payload::IsPayload};

  use crate::{auth, prelude::*};

  #[derive(Serialize, Deserialize, Debug)]
  enum Mutation<T> {
    Created(T),
    Updated(T),
    Deleted(T),
  }

  impl<T> From<Notification<T>> for Mutation<T> {
    fn from(notification: Notification<T>) -> Self {
      match notification.action {
        Action::Create => Mutation::Created(notification.data),
        Action::Update => Mutation::Updated(notification.data),
        Action::Delete => Mutation::Deleted(notification.data),
        _ => unreachable!(),
      }
    }
  }

  #[extension(pub trait ThingExt)]
  impl Db<auth::NoAuth> {
    async fn load_thing<P>(&self, id: ThingId) -> Result<Thing<P>, Error>
    where
      P: IsPayload,
    {
      let thing: Option<Thing<P>> = self
        .db()
        .select(id.clone().into_inner())
        .await
        .map_err(Error::CouldntSelect)?;
      let thing = thing.ok_or(Error::KnownRecordNotFound(id.into_inner()))?;
      Ok(thing)
    }

    async fn load_thing_stream<P>(
      &self,
      id: ThingId,
    ) -> Result<impl Stream<Item = Result<Thing<P>, Error>> + 'static, Error>
    where
      P: IsPayload + Unpin + Debug,
    {
      let initial: Thing<P> = self.load_thing(id.clone()).await?;
      let deltas = self
        .get_db()
        .query(format!("LIVE SELECT * FROM thing WHERE id = {}", id))
        // .bind(("id", id.clone()))
        .await
        .map_err(Error::LiveQueryStart)?
        .stream::<Notification<Thing<P>>>(0)
        .map_err(Error::LiveQueryStream)?;

      Ok(async_stream::stream! {
        yield Ok(initial);

        for await delta in deltas {
          let delta: Mutation<Thing<P>> = delta.map_err(Error::LiveQueryItem)?.into();
          match delta {
            Mutation::Updated(item) => yield Ok(item),
            Mutation::Created(item) => {
              warn!(?id, "Why is a thing being created in a live query?");
              yield Ok(item)
            },
            Mutation::Deleted(item) => {
              error!(?item, ?id, "Why is a thing being deleted in a live query? oh oooh");
              yield Err(Error::LiveQueryItemDeleted(id.clone().into_inner()))
            },
          }
        }
      })
    }
  }
}

use crate::prelude::*;

#[derive(Clone)]
pub struct Db<Auth> {
  db: Surreal<Any>,
  auth: Auth,
}

impl<Auth> Db<Auth> {
  pub fn auth(&self) -> &Auth {
    &self.auth
  }
}

impl Db<auth::User> {
  pub fn downgrade(self) -> Db<auth::NoAuth> {
    Db {
      db: self.db,
      auth: auth::NoAuth,
    }
  }
}

/// Select primitives
impl<Auth> Db<Auth> {
  /// ```surql
  /// DEFINE FUNCTION OVERWRITE fn::root_things() {
  ///   let $mentioned_as_children = <set>(SELECT in FROM parent).map(|$val| $val.in);
  ///   RETURN (SELECT id FROM thing WHERE !($mentioned_as_children.matches(id).any())).map(|$id| $id.id);
  /// };
  /// ```
  pub async fn root_things(&self) -> Result<Vec<ThingId>, Error> {
    self
      .get_db()
      .query("fn::root_things()")
      .await
      .map_err(Error::CouldntListRootThings)?
      .take(0)
      .map_err(Error::CouldntListRootThings)
  }

  /// ```surql
  /// DEFINE FUNCTION OVERWRITE fn::parents_of_thing($id: record<thing>) {
  ///   RETURN (SELECT -> parent -> thing AS parents FROM $id).map(|$val| $val.parents)[0];
  /// };
  /// ```
  pub async fn parents_of_thing(&self, id: ThingId) -> Result<Vec<ThingId>, Error> {
    self
      .get_db()
      .query("fn::parents_of_thing($id)")
      .bind(("id", id))
      .await
      .map_err(Error::CouldntListParents)?
      .take(0)
      .map_err(Error::CouldntListParents)
  }

  /// ```surql
  /// DEFINE FUNCTION OVERWRITE fn::children_of_thing($id: record<thing>) {
  ///   RETURN (SELECT <- parent <- thing AS children FROM $id).map(|$val| $val.children)[0];
  /// };
  /// ```
  pub async fn children_of_thing(&self, id: ThingId) -> Result<Vec<ThingId>, Error> {
    self
      .get_db()
      .query("fn::children_of_thing($id)")
      .bind(("id", id))
      .await
      .map_err(Error::CouldntListChildren)?
      .take(0)
      .map_err(Error::CouldntListChildren)
  }
}

/// Create primitives
impl<Auth> Db<Auth> {
  pub async fn create_thing<P>(
    &self,
    thing: ThingBuilder<P>,
    parents: HashSet<ThingId>,
  ) -> Result<(Thing<P>, Vec<ParentId>), Error>
  where
    P: Serialize + DeserializeOwned + IsPayload,
  {
    let thing: Thing<P> = self
      .get_db()
      .create(Thing::<()>::TABLE)
      .content(thing)
      .await
      .map_err(Error::CouldntCreateThing)?
      .ok_or(Error::CreatedThingNone)?;
    let parents = self.relate_parents(thing.id(), parents).await?;
    Ok((thing, parents))
  }

  /// ```surql
  /// DEFINE FUNCTION OVERWRITE fn::relate_parents($child: record<thing>, $parents: set<record<thing>>) {
  ///   RETURN (RELATE $child -> parent -> $parents);
  /// };
  /// ```
  pub async fn relate_parents(
    &self,
    child: ThingId,
    parents: HashSet<ThingId>,
  ) -> Result<Vec<ParentId>, Error> {
    let parents_len = parents.len();
    debug!(
      "Calling fn::relate_parents({}, [{}])",
      child,
      parents
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    );
    let ret: Vec<ParentId> = self
      .get_db()
      .query("fn::relate_parents($child, $parents)")
      .bind(("child", child))
      .bind(("parents", parents))
      .await
      .map_err(Error::CouldntRelateParents)?
      .take(0)
      .map_err(Error::CouldntRelateParents)?;
    #[cfg(debug_assertions)]
    if parents_len != ret.len() {
      error!(?ret);
      panic!("Expected {} parents, got {}", parents_len, ret.len());
    }
    Ok(ret)
  }
}

impl<Auth> surrealdb_layers::GetDb for Db<Auth> {
  fn get_db(&self) -> &Surreal<Any> {
    &self.db
  }
}

pub mod auth;
pub mod conn;
pub mod creds;
