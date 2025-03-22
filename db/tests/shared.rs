use std::str::FromStr as _;

use db::users::UserId;

pub fn fake_user_id() -> UserId {
  db::users::UserId::new_unchecked(
    surrealdb::RecordId::from_str("user:hqs4i07p52u5uje67wx7").unwrap(),
  )
}
