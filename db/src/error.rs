use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Couldn't connect to the DB (url)")]
  CouldntConnectToUrl {
    url: Url,
    #[source]
    err: surrealdb::Error,
  },

  #[error("Couldn't use namespace {ns} and database {db}")]
  CouldntUseNsDb {
    ns: String,
    db: String,
    #[source]
    err: surrealdb::Error,
  },

  #[error("Couldn't select data: {0}")]
  CouldntSelect(#[source] surrealdb::Error),

  #[error("Couldn't find a known record {0}")]
  KnownRecordNotFound(surrealdb::RecordId),

  #[error("Couldn't sing in")]
  CouldntSignIn(surrealdb::Error),

  #[error("Couldn't sign up")]
  CouldntSignUp(surrealdb::Error),

  #[error("{0}")]
  Inner(#[from] surrealdb_layers::Error),

  #[error("Couldn't start a live query to the backend")]
  LiveQueryStart(#[source] surrealdb::Error),

  #[error("Couldn't begin streaming a live query from the backend")]
  LiveQueryStream(#[source] surrealdb::Error),

  #[error("Couldn't deserialize item in a live query")]
  LiveQueryItem(#[source] surrealdb::Error),

  #[error("Item deleted")]
  LiveQueryItemDeleted(surrealdb::RecordId),

  #[error("Couldn't get root things")]
  CouldntListRootThings(#[source] surrealdb::Error),
  
  #[error("Couldn't list parents")]
  CouldntListParents(#[source] surrealdb::Error),

  #[error("Couldn't list children")]
  CouldntListChildren(#[source] surrealdb::Error),
}
