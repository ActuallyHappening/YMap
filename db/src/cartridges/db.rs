use crate::{
  auth,
  prelude::*,
  select::{LiveSelect, LiveSelectTable},
};

use super::Cartridge;

#[derive(Clone)]
pub struct DbCartridges<Auth> {
  db: Db<Auth>,
}

impl<Auth> Db<Auth> {
  pub fn cartridges(self) -> DbCartridges<Auth> {
    DbCartridges { db: self }
  }
}

impl DbCartridges<auth::User> {
  pub fn downgrade(self) -> DbCartridges<auth::NoAuth> {
    DbCartridges {
      db: self.db.downgrade(),
    }
  }
}

impl<Auth> GetDb for DbCartridges<Auth> {
  fn db(&self) -> crate::DbInner {
    self.db.db()
  }
}

impl<Auth> LiveSelectTable<Cartridge> for LiveSelect<Cartridge, Auth>
where
  LiveSelect<Cartridge, Auth>: Clone,
{
  fn table_name() -> &'static str {
    Cartridge::TABLE
  }
}

pub type SelectCartridgeErr = crate::select::SelectTableErr<Cartridge>;
impl<Auth> DbCartridges<Auth> {
  pub fn select(self) -> LiveSelect<Cartridge, Auth> {
    LiveSelect::new(self.db)
  }

  // /// Gets all available products
  // pub async fn select_star(&self) -> Result<Vec<Cartridge>, SelectCartridgesErr> {
  //   Ok(self.db().select(Cartridge::TABLE).await?)
  // }

  // /// Errors if product doesn't exist
  // pub async fn select_one(
  //   &self,
  //   id: CartridgeId,
  // ) -> Result<Option<Cartridge>, SelectCartridgesErr> {
  //   trace!(?id, "Getting single catridge");
  //   Ok(self.db().db().select((Cartridge::TABLE, id.key())).await?)
  // }

  // /// Does not preserve order, see [`Self::select_one`]
  // pub async fn select_many(
  //   &self,
  //   ids: impl IntoIterator<Item = CartridgeId>,
  // ) -> Result<Vec<Cartridge>, SelectCartridgesErr> {
  //   // let mut set = tokio::task::JoinSet::new();

  //   // for id in ids.into_iter() {
  //   //   set.spawn(Self::select_one(db.clone(), id));
  //   // }

  //   // set.join_all().await.into_iter().collect()

  //   // TODO: Don't wait serially! Get tokio runtime to
  //   // exist in here somehow
  //   let mut vec = Vec::new();
  //   let mut errs = Vec::new();
  //   for id in ids.into_iter() {
  //     let res = self.select_one(id.clone()).await?;
  //     match res {
  //       Some(c) => vec.push(c),
  //       None => errs.push(id),
  //     }
  //   }
  //   if errs.is_empty() {
  //     Ok(vec)
  //   } else {
  //     Err(SelectCartridgesErr::CouldntFindIds(errs))
  //   }
  // }
}

// #[derive(Debug, thiserror::Error)]
// pub enum SelectCartridgesErr {
//   #[error("Couldn't find cartridges for ids: {0:?}")]
//   CouldntFindIds(Vec<CartridgeId>),

//   #[error("Couldn't select cartridges")]
//   Underlying(#[from] surrealdb::Error),
// }
