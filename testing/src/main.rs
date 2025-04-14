use utils::prelude::info;

fn main() {
  utils::tracing::install_tracing("debug").unwrap();

  let id = surrealdb::RecordId::from(("table", "id-with-hyphens"));
  let id_str = id.to_string();
  let id_parsed: surrealdb::RecordId = id_str.parse().unwrap();

  info!(?id, %id_str, ?id_parsed);

  assert_eq!(id, id_parsed);
}
