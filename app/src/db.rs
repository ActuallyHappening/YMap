//! To select the parent ids of a record:
//! - Duplicated
//! SElECT -> parent -> thing AS parents FROM thing:fbrngbalrk14hows7u15;
//! To load the actual things that are the parents:
//! SElECT -> parent -> thing.* AS parents FROM thing:fbrngbalrk14hows7u15;
//!
//! RELATE thing:child -> parent -> thing:parent
//!         in                          out
//!
//! -- $non_root is any id mentioned as a child in the relational db
//! LET $non_root = <set>(SELECT in FROM parent).map(|$val| $val.in);
//! SELECT * FROM thing WHERE !$non_root.matches(id).any();

use std::{collections::HashMap, ops::Deref};

use db::{Db, auth, creds};
use thing::{payload::IsPayload, well_known::KnownRecord};

use crate::{app::RootOwner, prelude::*};

pub enum DbConn {
  WaitingForGuest {
    prev_err: Option<db::Error>,
  },
  Guest(Result<Db<auth::NoAuth>, db::Error>),
  WaitingForSignUp {
    creds: creds::SignUpUser,
    prev_err: Option<db::Error>,
  },
  WaitingForSignIn {
    creds: creds::SignInUser,
    prev_err: Option<db::Error>,
  },
  User(Result<Db<auth::User>, db::Error>),
}

impl DbConn {
  pub fn provide() {
    leptos::context::provide_context(RwSignal::new(DbConn::WaitingForGuest { prev_err: None }));
  }

  pub fn from_context() -> RwSignal<DbConn> {
    leptos::context::use_context().expect("Call DbConn::provide() above you first")
  }

  /// Will still magically be able to select correct records if signed in
  pub fn guest(&self) -> Result<Db<auth::NoAuth>, AppError> {
    match self {
      DbConn::WaitingForGuest { .. }
      | DbConn::WaitingForSignIn { .. }
      | DbConn::WaitingForSignUp { .. } => Err(AppError::DbWaiting),
      DbConn::Guest(res) => Ok(res.as_ref()?.clone()),
      DbConn::User(res) => Ok(res.as_ref()?.clone().downgrade()),
    }
  }
}

pub fn Connect() -> impl IntoView {
  let state = DbConn::from_context();
  move || match state.read().deref() {
    DbConn::WaitingForGuest { .. } => {
      let suspend = Suspend::new(async move {
        let res = (async || -> Result<Db<auth::NoAuth>, db::Error> {
          Ok(db::Db::build().wss()?.await?.prod().await?.public())
        })()
        .await;

        let msg = match &res {
          Ok(_) => "Connected as guest successfully!".into(),
          Err(err) => format!("Failed to connect as guest: {}", err),
        };

        DbConn::from_context().set(DbConn::Guest(res));

        view! { <pre> {msg} </pre>}
      });
      view! {
        <p> "Connecting as guest ..." </p>
        {suspend}
      }
      .into_any()
    }
    DbConn::Guest(Ok(_)) => view! {
      <p> "Connected (guest)" </p>
    }
    .into_any(),
    DbConn::Guest(Err(_)) => view! {
      <p> "Failed to connect (as guest)" </p>
      <p> "Reload to try again" </p>
    }
    .into_any(),
    _ => todo!(),
  }
}

/// Only reactively updates to DB changes,
/// not a LIVE updating signal (yet)
pub fn root_things() -> Signal<AppResult<Vec<ThingId>>> {
  Signal::derive(move || {
    // cache in context
    #[derive(Clone)]
    enum Cache {
      FirstTick,
      WaitingForRootLocalResource,
      Done(AppResult<Vec<ThingId>>),
    }
    impl Cache {
      fn resolve(self) -> AppResult<Vec<ThingId>> {
        match self {
          Cache::FirstTick => Err(AppError::FirstTimeGlobalState),
          Cache::WaitingForRootLocalResource => Err(AppError::DataLoading),
          Cache::Done(res) => res,
        }
      }
    }

    if let Some(c) = use_context::<RwSignal<Cache>>() {
      c.get().resolve()
    } else {
      let root_owner = expect_context::<RootOwner>().0;
      root_owner.with(|| {
        provide_context(RwSignal::new(Cache::FirstTick));

        let resource = LocalResource::new(|| {
          let db = DbConn::from_context();
          async move {
            let data = db.read().guest()?.root_things().await?;
            AppResult::Ok(data)
          }
        });

        Effect::new(move || match resource.get() {
          None => expect_context::<RwSignal<Cache>>().set(Cache::WaitingForRootLocalResource),
          Some(data) => expect_context::<RwSignal<Cache>>().set(Cache::Done(data.take())),
        });
      });
      expect_context::<RwSignal<Cache>>().get().resolve()
    }
  })
}

/// LIVE updates signal
pub fn load_payload<P>(id: Signal<ThingId>) -> Signal<Result<Thing<P>, AppError>>
where
  P: IsPayload + std::fmt::Debug + Clone + Unpin,
{
  Signal::derive(move || raw_load_payload(id.get()))
}

/// LIVE updates signal
pub fn known_thing<T>() -> Signal<Result<T, AppError>>
where
  T: KnownRecord,
  <T as KnownRecord>::Payload: Unpin + std::fmt::Debug + Clone,
{
  Signal::derive(move || {
    let payload = raw_load_payload(T::known_id())?;
    Ok(T::from_inner(payload))
  })
}

struct Context<P: Send + Sync + 'static>(RwSignal<HashMap<ThingId, PayloadCache<P>>>);

impl<P: IsPayload> Clone for Context<P> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

fn root_owner() -> Owner {
  expect_context::<RootOwner>().0
}

impl<P: IsPayload> Context<P> {
  fn get_or_init() -> Self {
    root_owner().with(|| {
      if use_context::<Self>().is_none() {
        provide_context::<Self>(Self(RwSignal::new(HashMap::default())));
      }
      expect_context::<Self>()
    })
  }

  pub fn use_context(id: ThingId) -> Option<PayloadCache<P>> {
    Self::get_or_init().0.get().get(&id).map(|p| p.clone())
  }
  pub fn expect_context(id: ThingId) -> PayloadCache<P> {
    Self::use_context(id).expect(
      "You called Context::expect_context, but an entry with the specified ThingId wasn't found",
    )
  }
  pub fn set_context(id: ThingId, payload: PayloadCache<P>) {
    Self::get_or_init().0.update(|map| {
      map.insert(id, payload);
    });
  }
}

/// Stored as `RwSignal<Cached<T>>`
#[derive(Debug)]
enum PayloadCache<P: Send + Sync + 'static> {
  FirstTick,
  WaitingForRootLocalResource,
  CouldntStart(AppError),
  Done(ArcSignal<Result<Thing<P>, AppError>>),
}

impl<P: IsPayload> Clone for PayloadCache<P> {
  fn clone(&self) -> Self {
    match self {
      PayloadCache::FirstTick => PayloadCache::FirstTick,
      PayloadCache::WaitingForRootLocalResource => PayloadCache::WaitingForRootLocalResource,
      PayloadCache::CouldntStart(err) => PayloadCache::CouldntStart(err.clone()),
      PayloadCache::Done(sig) => PayloadCache::Done(ArcSignal::clone(sig)),
    }
  }
}

impl<P> PayloadCache<P>
where
  P: IsPayload + Clone,
{
  fn resolve(&self) -> Result<Thing<P>, AppError> {
    match self {
      PayloadCache::FirstTick => Err(AppError::FirstTimeGlobalState),
      PayloadCache::WaitingForRootLocalResource => Err(AppError::FirstTimeGlobalState),
      PayloadCache::CouldntStart(err) => Err(err.clone()),
      PayloadCache::Done(sig) => sig.get(),
    }
  }
}

/// Subscribes
fn raw_load_payload<P>(id: ThingId) -> Result<Thing<P>, AppError>
where
  P: IsPayload + Clone + std::fmt::Debug + Unpin,
{
  if let Some(s) = Context::<P>::use_context(id.clone()) {
    return s.resolve();
  }

  // now we are initializing global state
  root_owner().with(|| {
    Context::<P>::set_context(id.clone(), PayloadCache::<P>::FirstTick);

    let id2 = id.clone();
    let stream = LocalResource::new(move || {
      let id = id2.clone();
      let db = DbConn::from_context();
      async move {
        let stream = db.read().guest()?.load_thing_stream::<P>(id).await?;
        let sig = ReadSignal::from_stream(stream);

        // maps from db::Error to AppError
        let mapped = Signal::derive(move || {
          let sig = sig.read();
          let res = sig
            .deref()
            .as_ref()
            .ok_or(AppError::LiveQueryStreamWaiting)?
            .as_ref()
            .map(Clone::clone)?;
          AppResult::Ok(res)
        });
        AppResult::Ok(mapped)
      }
    });

    // using Effect is easy to understand
    // but technically inefficient,
    // TODO: think of a cleaner way of doing this
    let id2 = id.clone();
    Effect::new(move || match stream.get() {
      None => {
        Context::<P>::set_context(id2.clone(), PayloadCache::<P>::WaitingForRootLocalResource);
      }
      Some(stream) => {
        let stream = stream.take();
        match stream {
          Err(err) => Context::<P>::set_context(id2.clone(), PayloadCache::<P>::CouldntStart(err)),
          Ok(actual_data) => Context::<P>::set_context(
            id2.clone(),
            PayloadCache::<P>::Done(ArcSignal::from(actual_data)),
          ),
        }
      }
    });
  });

  // subscribes
  Context::<P>::expect_context(id.clone()).resolve()
}
