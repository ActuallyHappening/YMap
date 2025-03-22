use crate::{DbInner, auth, prelude::*};

use convert_case::{Case, Casing as _};
use strum::EnumIter;
use surrealdb::RecordId;

const TABLE_NAME: &str = "support";

pub struct DbSupport {
  db: Db<auth::Root>,
}

impl Db<auth::Root> {
  pub fn support(self) -> DbSupport {
    DbSupport { db: self }
  }
}

impl DbSupport {
  fn db(&self) -> DbInner {
    self.db.db()
  }
}

impl DbSupport {
  pub async fn insert_ticket(
    &self,
    ticket_builder: SupportTicketBuilder,
  ) -> Result<SupportTicket, InsertSupportErr> {
    let tickets = self
      .db()
      .insert::<Vec<SupportTicket>>(TABLE_NAME)
      .content(ticket_builder)
      .await?;

    if tickets.len() > 1 {
      return Err(InsertSupportErr::MultipleTicketsReturned { len: tickets.len() });
    }

    tickets
      .into_iter()
      .next()
      .ok_or(InsertSupportErr::NoTicketReturned)
  }
}

#[derive(Debug, thiserror::Error)]
pub enum InsertSupportErr {
  #[error("Couldn't insert support ticket")]
  Underlying(#[from] surrealdb::Error),

  #[error("No support ticket returned?")]
  NoTicketReturned,

  #[error("Multiple tickets returned")]
  MultipleTicketsReturned { len: usize },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupportTicket {
  id: RecordId,
  ticket_type: TicketType,
  name: String,
  email: String,
  content: String,
  /// Created automatically
  created_at: DateTime,
}

impl SupportTicket {
  pub fn get_type(&self) -> TicketType {
    self.ticket_type
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn email(&self) -> &str {
    &self.email
  }

  pub fn content(&self) -> &str {
    &self.content
  }

  pub fn created_at(&self) -> time::OffsetDateTime {
    self.created_at.0
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupportTicketBuilder {
  pub ticket_type: TicketType,
  pub name: String,
  pub email: String,
  pub content: String,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum TicketType {
  RefundsAndCancellation,
  ProductEnquiry,
  Payments,
  Account,
  Feedback,
  Policies,
  Other,
}

impl<'de> serde::Deserialize<'de> for TicketType {
  fn deserialize<D>(deserializer: D) -> Result<TicketType, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    TicketType::iter()
      .find(|t| t.kebab_name() == s)
      .ok_or_else(|| serde::de::Error::custom(format!("invalid ticket type {}", s)))
  }
}

impl serde::Serialize for TicketType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.kebab_name().serialize(serializer)
  }
}

impl std::fmt::Display for TicketType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      Self::RefundsAndCancellation => "Refunds and Cancellations",
      Self::ProductEnquiry => "Product Enquiry",
      Self::Payments => "Payments",
      Self::Account => "My JYD Account",
      Self::Feedback => "Feedback",
      Self::Policies => "Policies",
      Self::Other => "Other",
    };
    write!(f, "{}", str)
  }
}

impl TicketType {
  pub fn kebab_name(self) -> String {
    self.to_string().to_case(Case::Kebab)
  }

  pub fn iter() -> impl Iterator<Item = TicketType> {
    <TicketType as strum::IntoEnumIterator>::iter()
  }
}
