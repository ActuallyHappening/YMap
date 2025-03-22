use crate::{errors::components::Pre, prelude::*};
use db::support::{SupportTicket, SupportTicketBuilder, TicketType};

stylance::import_crate_style!(support_styles, "src/components/support/support.module.scss");

const SUPPORT_EMAIL: &str = "support@jordanyatesdirect.com";

pub fn SupportEmailLink() -> impl IntoView {
  view! { <A href=format!("mailto:{}", SUPPORT_EMAIL)>{SUPPORT_EMAIL}</A> }
}

pub fn CustomerSupportLinks() -> impl IntoView {
  let href = TopLevelRoutes::Support;
  view! {
    <A href=href.abs_path()>"Customer support"</A>
    <SupportEmailLink />
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, thiserror::Error)]
pub enum ComponentError {
  #[error("An error occurred talking to the server: {0}")]
  ServerFnError(ServerFnError),

  #[error("Couldn't add your support ticket")]
  InsertTicketErr(#[from] GenericError<db::support::InsertSupportErr>),
}

impl IntoRender for ComponentError {
  type Output = AnyView;

  fn into_render(self) -> Self::Output {
    view! {
      <h2>"There was an error submitting your ticket"</h2>
      <p>
        "Please try again later or email us at "
        <A href=format!("mailto:{}", SUPPORT_EMAIL)>{SUPPORT_EMAIL}</A>
      </p>
      <pre> { self.to_string() } </pre>
      <Pre err=self />
    }
    .into_any()
  }
}

impl ComponentError {
  fn from_nested(
    err: Result<Result<SupportTicket, ComponentError>, ServerFnError>,
  ) -> Result<SupportTicket, Self> {
    match err {
      Ok(Ok(ret)) => Ok(ret),
      Ok(Err(e)) => Err(e),
      Err(e) => Err(Self::ServerFnError(e)),
    }
  }
}

#[server(
  prefix = "/api/support",
  endpoint = "/add-ticket",
  // input = server_fn::codec::Json,
  output = server_fn::codec::Json
)]
async fn add_support_ticket(
  ticket: SupportTicketBuilder,
) -> Result<Result<SupportTicket, ComponentError>, ServerFnError> {
  Ok(thunk_add_support_ticket(ticket).await)
}

#[cfg(feature = "ssr")]
async fn thunk_add_support_ticket(
  ticket: SupportTicketBuilder,
) -> Result<SupportTicket, ComponentError> {
  use crate::server_state::ServerAxumState;

  info!(builder = ?ticket);

  // the magic
  let db = ServerAxumState::from_context().db;
  let ticket = db.support().insert_ticket(ticket).await.err_generic()?;

  info!(?ticket, "Ticket inserted successfully");

  Ok(ticket)
}

pub fn Support() -> impl IntoView {
  let add_support_ticket = ServerAction::<AddSupportTicket>::new();
  let value = add_support_ticket.value();

  #[derive(Clone)]
  enum State {
    Form,
    Success(SupportTicket),
    Error(ComponentError),
  }
  let state = move || match value.get() {
    None => State::Form,
    Some(v) => match ComponentError::from_nested(v) {
      Ok(ret) => State::Success(ret),
      Err(e) => State::Error(e),
    },
  };

  let form = move || {
    let type_options = TicketType::iter()
      .map(|t| {
        view! { <option value=t.kebab_name()>{t.to_string()}</option> }
      })
      .collect_view();
    view! {
      <ActionForm action={add_support_ticket} {..} class=support_styles::form>
        <h2>"Submit a support request"</h2>
        <select name="ticket[ticket_type]" required>
          {type_options}
        </select>
        <label for="name">"What should we call you in our support messages?"</label>
        <input type="text" name="ticket[name]" placeholder="Douglas Adams" id="name" required />
        <label for="email">"How should we contact you through email?"</label>
        <input
          type="email"
          name="ticket[email]"
          placeholder="youremail@gmail.com"
          id="email"
          required
        />
        <label for="content">"What can we help you with?"</label>
        <textarea
          name="ticket[content]"
          placeholder="What can we help you with?"
          minlength="3"
          required
        />
        <input type="submit" value="Submit Ticket" />
      </ActionForm>
    }
  };

  let success = |ret: SupportTicket| {
    view! {
      <h2>"Your ticket has been submitted!"</h2>
      <p>
        "We'll generally respond to your request within 3-5 days during our operating hours from 9am-6pm Monday to Friday"
      </p>
      <pre>{format!("{:?}", ret)}</pre>
    }
  };
  let error = move |err: ComponentError| err.into_render();

  view! {
    <div class=support_styles::support>
      <h1>"Support"</h1>
      <p>"For frequently asked questions, see our about page " <A href="/about">"here"</A></p>
      <p>
        "For more information, you can email us at us at "
        <A href=format!("mailto:{}", SUPPORT_EMAIL)>{SUPPORT_EMAIL}</A>
      </p>
      {move || match state() {
        State::Form => EitherOf3::A(form()),
        State::Success(ret) => EitherOf3::B(success(ret)),
        State::Error(err) => EitherOf3::C(error(err)),
      }}
    </div>
  }
}
