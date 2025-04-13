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

pub use error::Error;
use thing::ThingId;
pub mod error;

pub mod user;

mod things {
  use futures_core::Stream;
  use surrealdb::{Action, Notification};
  use thing::{Thing, ThingId, payload::IsPayload, well_known::KnownRecord};

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
        .map_err(|err| Error::CouldntSelect(err))?;
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

impl<Auth> Db<Auth> {
  pub async fn root_things(&self) -> Result<Vec<ThingId>, Error> {
    Ok(
      self
        .get_db()
        .query("fn::root_things()")
        .await
        .map_err(Error::CouldntListRootThings)?
        .take(0)
        .map_err(Error::CouldntListRootThings)?,
    )
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
