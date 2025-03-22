use strum::EnumIter;

use crate::prelude::*;

stylance::import_crate_style!(
  accounts_styles,
  "src/components/accounts/accounts.module.scss"
);

#[path = "login/login.rs"]
pub mod login;

#[path = "signup/signup.rs"]
pub mod signup;

#[path = "home/home.rs"]
pub mod home;

#[path = "reset_password/reset_password.rs"]
pub mod reset_password;

#[path = "change_email/change_email.rs"]
pub mod change_email;

pub mod logout;

#[derive(Clone, Copy, EnumIter)]
pub enum AccountRoutes {
  Home,
  Login,
  Register,
  ChangeEmail,
  ResetPassword,
  Logout,
}

impl AccountRoutes {
  pub fn final_suffix(&self) -> &'static str {
    match self {
      Self::Home => "/",
      Self::Login => "/login",
      Self::Register => "/register",
      Self::ChangeEmail => "/change-email",
      Self::ResetPassword => "/reset-password",
      Self::Logout => "/logout",
    }
  }

  pub fn iter_footer() -> impl Iterator<Item = AccountRoutes> {
    [
      Self::Home,
      Self::Login,
      Self::Register,
      Self::ChangeEmail,
      Self::ResetPassword,
      Self::Logout,
    ]
    .into_iter()
  }

  pub fn name(self) -> &'static str {
    match self {
      Self::Home => "Account Home",
      Self::Login => "Login",
      Self::Register => "Sign Up",
      Self::ChangeEmail => "Change Email",
      Self::ResetPassword => "Reset Password",
      Self::Logout => "Logout",
    }
  }
}

impl NestedRoute for AccountRoutes {
  fn nested_base(&self) -> impl Route {
    TopLevelRoutes::Account
  }

  fn raw_path_suffix(&self) -> String {
    self.final_suffix().into()
  }
}

pub fn Router() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route path=path!("") view=home::Home />
    <Route path=path!("/login") view=login::Login />
    <Route path=path!("/register") view=signup::SignUp />
    <Route path=path!("/change-email") view=change_email::ChangeEmail />
    <Route path=path!("/reset-password") view=reset_password::ResetPassword />
    <Route path=path!("/logout") view=logout::Logout />
  }
  .into_inner()
}
