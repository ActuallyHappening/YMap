use crate::{
  components::accounts::AccountRoutes,
  db::{DbConn, DbState},
  errors::components::Pre,
  prelude::*,
};

stylance::import_crate_style!(
  login_styles,
  "src/components/accounts/login/login.module.scss"
);

#[component]
fn ChangeEmailForm<F: Fn(String) + 'static>(handler: F) -> impl IntoView {
  let new_email = RwSignal::new(String::default());

  let handler = move |ev: web_sys::SubmitEvent| {
    // if this doesn't work, you will see query params in the url
    // also, this required the delegation feature on leptos
    // https://github.com/leptos-rs/leptos/issues/3457
    ev.prevent_default();

    handler(new_email.get());
  };

  view! {
    <form on:submit=handler>
      <label for="email">"New email"</label>
      <input type="email" bind:value=new_email id="email" name="email" required />

      <button type="submit">"Update email"</button>
    </form>
  }
}

pub fn ChangeEmail() -> impl IntoView {
  let db = DbState::from_context();

  move || {
    let state = db.read();
    let state = state.conn_old();

    state.map_view(|state| match state {
      DbConn::User(_) => {
        #[derive(Debug, thiserror::Error)]
        enum UpdateEmailErr {
          #[error("Lost connection to database")]
          Conn(#[from] GenericError<crate::db::ConnectErr>),

          #[error("Not logged in")]
          NotLoggedIn,

          #[error("{0}")]
          Underlying(#[from] GenericError<crate::db::UpdateEmailErr>),
        }

        impl IntoRender for &UpdateEmailErr {
          type Output = AnyView;

          fn into_render(self) -> Self::Output {
            view! {
              <p> { self.to_string() }</p>
              <Pre err=GenericError::from_ref(self) />
            }
            .into_any()
          }
        }

        let update_email = Action::new_local(move |new_email: &String| {
          let new_email = new_email.clone();
          async {
            let user = {
              let db = DbState::from_context().read();
              let db = db.conn_old().err_generic_ref()?;
              let DbConn::User(user) = db else {
                return Err(UpdateEmailErr::NotLoggedIn);
              };
              user.users().clone()
            };
            user.update_email(new_email).await.err_generic()?;

            Result::<(), UpdateEmailErr>::Ok(())
          }
        });
        let handler = move |new_email| {
          update_email.dispatch(new_email);
        };

        match (
          update_email.input().read().deref(),
          update_email.value().read().deref(),
        ) {
          (None, None) => view! {
            <h1>"Change your email"</h1>
            <ChangeEmailForm handler />
          }
          .into_any(),
          (Some(new_email), _) => view! {
            <h1>"Chaning email ..."</h1>
            <p> { format!("Changing email to {}", new_email) }</p>
          }
          .into_any(),
          (None, Some(Ok(()))) => view! {
            <h1>"Email changed successfully"</h1>
          }
          .into_any(),
          (None, Some(Err(err))) => view! {
            <h1>"Error changing email"</h1>
            { err }
          }
          .into_any(),
        }
      }

      DbConn::Guest(_) => view! {
        <h1>"You must be logged in to change your email"</h1>
        <p>"Redirecting to login page..."</p>
        <Redirect path=AccountRoutes::Login.abs_path() />
      }
      .into_any(),
    })
  }
}
