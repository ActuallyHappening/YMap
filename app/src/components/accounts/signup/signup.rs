use db::users::SignUpUser;

use crate::{
  db::{ConnectErr, DbConn, DbState},
  prelude::*,
};

stylance::import_crate_style!(
  signup_styles,
  "src/components/accounts/signup/signup.module.scss"
);

#[component]
fn SignUpForm<F: Fn(SignUpUser) + 'static>(handler: F) -> impl IntoView {
  let email = RwSignal::new(String::default());
  let name = RwSignal::new(String::default());
  let plaintext_password = RwSignal::new(String::default());

  let handler = move |ev: web_sys::SubmitEvent| {
    // if this doesn't work, you will see query params in the url
    // also, this required the delegation feature on leptos
    // https://github.com/leptos-rs/leptos/issues/3457
    ev.prevent_default();

    handler(SignUpUser {
      email: email.get(),
      name: name.get(),
      plaintext_password: plaintext_password.get(),
    });
  };

  view! {
    <h1>"Sign Up"</h1>
    <form on:submit=handler>
      <label for="email">"Email"</label>
      <input type="email" bind:value=email id="email" name="email" required />

      <label for="name">"Name"</label>
      <input type="text" bind:value=name id="name" name="name" required />

      <label for="password">"Password"</label>
      <input type="password" bind:value=plaintext_password id="password" name="password" required />

      <button type="submit">"Login"</button>
    </form>
  }
}

pub fn SignUp() -> impl IntoView {
  let db = DbState::from_context();
  move || {
    let state = db.read();
    let state = state.conn_old();

    let signup = move |creds| {
      db.write().signup(creds);
    };

    match state {
      Ok(DbConn::User(_)) => view! {
        <SignUpForm handler=signup />
        <p>"Note: This action will automatically sign out of your existing account, and into your new account."</p>
      }.into_any(),
      Ok(DbConn::Guest(_)) => view! {
        <SignUpForm handler=signup />
      }.into_any(),
      Err(ConnectErr::WaitingForSignup(user)) => view! {
        <h1>"Signing you up ..."</h1>
        <p>{ format!("Waiting to sign you up as {}", user.email)} </p>
      }.into_any(),
      Err(other) => other.into_render().into_any(),
    }
  }
}
