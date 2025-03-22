use db::users::SignInUser;

use crate::{
  components::accounts::AccountRoutes,
  db::{ConnectErr, DbConn, DbState},
  prelude::*,
};

stylance::import_crate_style!(
  login_styles,
  "src/components/accounts/login/login.module.scss"
);

pub fn LoginButton() -> impl IntoView {
  let href = AccountRoutes::Login.abs_path();
  view! { <A href=href>"Login"</A> }
}

#[component]
fn LoginForm<F: Fn(SignInUser) + 'static>(handler: F) -> impl IntoView {
  let email = RwSignal::new(String::default());
  let plaintext_password = RwSignal::new(String::default());

  let handler = move |ev: web_sys::SubmitEvent| {
    // if this doesn't work, you will see query params in the url
    // also, this required the delegation feature on leptos
    // https://github.com/leptos-rs/leptos/issues/3457
    ev.prevent_default();

    handler(SignInUser {
      email: email.get(),
      plaintext_password: plaintext_password.get(),
    });
  };

  view! {
    <h1>"Login"</h1>
    <form on:submit=handler>
      <label for="email">"Email"</label>
      <input type="email" bind:value=email id="email" name="email" required />

      <label for="password">"Password"</label>
      <input type="password" bind:value=plaintext_password id="password" name="password" required />

      <button type="submit">"Login"</button>
    </form>
  }
}

pub fn Login() -> impl IntoView {
  let db = DbState::from_context();

  move || {
    let state = db.read();
    let state = state.conn_old();

    let login = move |creds| {
      db.write().login(creds);
    };

    match state {
      Ok(DbConn::User(_)) => view! {
        <h1>"Your already logged in"</h1>
        <p>"Redirecting to account homepage"</p>
        <Redirect path=AccountRoutes::Home.abs_path() />
      }
      .into_any(),
      Ok(DbConn::Guest(_)) => view! {
        <LoginForm handler=login />
      }
      .into_any(),
      Err(ConnectErr::WaitingForLogin(SignInUser { email, .. })) => view! {
        <h1>"Logging in ..."</h1>
        <p>{ format!("Logging you in as {}", email) }</p>
      }
      .into_any(),
      Err(err) => err.into_render().into_any(),
    }
  }
}
