use std::ops::Deref;

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

pub fn load_payload<P>(id: Signal<ThingId>) -> Signal<Result<Thing<P>, AppError>>
where
  P: IsPayload + std::fmt::Debug + Clone + Unpin,
{
  Signal::derive(move || raw_load_payload(id.get()))
}

fn raw_load_payload<P>(id: ThingId) -> Result<Thing<P>, AppError>
where
  P: IsPayload + Clone + std::fmt::Debug + Unpin,
{
  if let Some(s) = use_context::<RwSignal<ContextCache<P>>>() {
    return ContextCache::get(&s.read());
  }

  // now we are initializing global state
  let root_owner = expect_context::<RootOwner>().0;
  root_owner.with(|| {
    provide_context(RwSignal::new(ContextCache::<P>::FirstTick));

    let stream = LocalResource::new(move || {
      let id = id.clone();
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

    fn set_context<T>(new_state: ContextCache<T>)
    where
      T: Send + Sync + 'static + std::fmt::Debug,
    {
      let rw_sig = expect_context::<Context<T>>();
      debug!("Updating cached signal: {:?}", new_state);
      rw_sig.set(new_state);
    }

    // using Effect is easy to understand
    // but technically inefficient,
    // TODO: think of a cleaner way of doing this
    Effect::new(move || match stream.get() {
      None => {
        set_context(ContextCache::<P>::WaitingForRootLocalResource);
      }
      Some(stream) => {
        let stream = stream.take();
        match stream {
          Err(err) => set_context(ContextCache::<P>::CouldntStart(err)),
          Ok(actual_data) => set_context(ContextCache::<P>::Done(actual_data)),
        }
      }
    });
  });

  // subscribes
  ContextCache::get(use_context::<Context<P>>().unwrap().read().deref())
}

type Context<T> = RwSignal<ContextCache<T>>;

/// Stored as `RwSignal<Cached<T>>`
#[derive(Debug)]
enum ContextCache<P: Send + Sync + 'static> {
  FirstTick,
  WaitingForRootLocalResource,
  CouldntStart(AppError),
  Done(Signal<Result<Thing<P>, AppError>>),
}

impl<P> ContextCache<P>
where
  P: IsPayload + Clone,
{
  fn get(&self) -> Result<Thing<P>, AppError> {
    match self {
      ContextCache::FirstTick => Err(AppError::FirstTimeGlobalState),
      ContextCache::WaitingForRootLocalResource => Err(AppError::FirstTimeGlobalState),
      ContextCache::CouldntStart(err) => Err(err.clone()),
      ContextCache::Done(sig) => sig.get(),
    }
  }
}

/// Loads info, subscribes to the relevant signals
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
