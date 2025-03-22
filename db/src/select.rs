use std::{collections::HashSet, convert::Infallible, hash::Hash, marker::PhantomData};

use tokio_stream::Stream;

use crate::{
  DbInner, auth,
  orders::{Order, OrderId},
  prelude::*,
};

#[derive(Clone)]
pub struct LiveSelect<T, Auth> {
  db: Db<Auth>,
  t: PhantomData<T>,
}

impl<T, Auth> LiveSelect<T, Auth> {
  pub fn new(db: Db<Auth>) -> Self {
    Self { db, t: PhantomData }
  }
}

pub use mutation::Mutation;
pub mod mutation {
  use std::collections::HashMap;

  use crate::prelude::*;

  #[derive(Debug)]
  pub enum Mutation<T> {
    Created(T),
    Updated(T),
    Deleted(T),
  }

  impl<T> Mutation<T>
  where
    T: TableDescriptor,
  {
    pub fn apply(self, state: &mut HashMap<<T as TableDescriptor>::Id, T>) {
      match self {
        Mutation::Created(data) => {
          let id = data.id();
          if state.contains_key(&id) {
            warn!("Overwriting existing {} data", T::debug_name());
          }
          state.insert(id, data);
        }
        Mutation::Updated(data) => {
          let id = data.id();
          if !state.contains_key(&id) {
            warn!("Updating a non-existant {}", T::debug_name());
          }
          state.insert(id, data);
        }
        Mutation::Deleted(data) => {
          let id = data.id();
          if !state.contains_key(&id) {
            warn!("Deleting a non-existant {}", T::debug_name());
          }
          state.remove(&id);
        }
      }
    }
  }
}

impl<T, Auth> GetDb for LiveSelect<T, Auth> {
  fn db(&self) -> DbInner {
    self.db.db()
  }
}

#[derive(Debug, thiserror::Error)]
pub enum SelectTableErr<T> {
  #[error("Couldn't get live stream of orders")]
  LiveSelect(#[source] surrealdb::Error),

  #[error("Failed to get an individual order from live stream")]
  LiveItem(#[source] surrealdb::Error),

  #[error("{0}")]
  InitialSelect(#[source] surrealdb::Error),

  #[error("")]
  Phantom { inf: Infallible, ph: PhantomData<T> },
}

#[allow(private_bounds)]
pub trait LiveSelectTable<T>: GetDb
where
  T: serde::de::DeserializeOwned + Debug + Clone + TableDescriptor + Send + Sync + 'static + Unpin,
  Self: Clone + Sized,
{
  fn table_name() -> &'static str;

  async fn initial(self) -> Result<Vec<T>, SelectTableErr<T>> {
    Ok(
      self
        .db()
        .select(Self::table_name())
        .await
        .map_err(SelectTableErr::InitialSelect)?,
    )
  }

  async fn stream_delta(
    self,
  ) -> Result<impl Stream<Item = Result<Mutation<T>, SelectTableErr<T>>>, SelectTableErr<T>> {
    let stream = self
      .db()
      .select::<Vec<T>>(Self::table_name())
      .live()
      .await
      .map_err(SelectTableErr::LiveSelect)?;
    let stream = stream.map(|res| {
      res
        .map_err(SelectTableErr::LiveItem)
        .map(|notification| match notification.action {
          surrealdb::Action::Create => Mutation::Created(notification.data),
          surrealdb::Action::Update => Mutation::Updated(notification.data),
          surrealdb::Action::Delete => Mutation::Deleted(notification.data),
          _ => unreachable!("unimplemented variant of action"),
        })
    });
    Ok(stream)
  }

  /// Stores the initial, and computes changes.
  /// Only logs hard errors
  async fn stream(self) -> Result<impl Stream<Item = Vec<T>>, SelectTableErr<T>> {
    let initial = self.clone().initial().await?;
    let delta_stream = self.stream_delta().await?;

    Ok(async_stream::stream! {
      yield initial.clone();

      let mut state = std::collections::HashMap::new();
      for order in initial {
        let id = order.id();
        state.insert(id, order);
      }

      for await delta in delta_stream {
        if let Ok(delta) = delta {
          delta.apply(&mut state);
          yield state.values().cloned().collect();
        } else {
          warn!(?delta, "Failed to get {} delta", Order::debug_name());
        }
      }
    })
  }

  async fn initial_one(
    self,
    id: <T as TableDescriptor>::Id,
  ) -> Result<Option<T>, SelectTableErr<T>> {
    Ok(self.initial().await?.into_iter().find(|o| o.id() == id))
  }

  /// Todo: port over this API to use hash map on ID
  async fn initial_many(
    self,
    ids: HashSet<<T as TableDescriptor>::Id>,
  ) -> Result<HashSet<T>, SelectTableErr<T>>
  where
    T: Eq + Hash,
  {
    Ok(
      self
        .initial()
        .await?
        .into_iter()
        .filter(|o| ids.contains(&o.id()))
        .collect(),
    )
  }
}
