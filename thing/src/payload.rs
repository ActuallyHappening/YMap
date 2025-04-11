use serde::{Deserializer, de::DeserializeOwned};

use surrealdb_layers::Id;

use super::ThingId;

pub trait IsPayload {}

/// Todo: write a trait to deserialize
/// using this dynamic key
pub trait KnownPayloadEntry: DeserializeOwned {
  fn known() -> &'static str;

  fn known_id() -> ThingId {
    ThingId::new_known(Self::known().into())
  }
}

// pub fn deserialize_with<'de, D, T>(deserializer: D) -> Result<T, D::Error>
// where
//   D: Deserializer<'de>,
// {
//   todo!()
// }

// use serde::de::DeserializeOwned;

// use crate::{error::Error, prelude::*};

// use super::{AnyValue, ThingId};

// /// A newtype to handle serialization and deserialization of payloads
// /// since the keys are stored only as strings in the db
// #[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(try_from = "PayloadSerde", into = "PayloadSerde")]
// pub struct Payload(HashMap<ThingId, AnyValue>);

// impl FromIterator<(ThingId, AnyValue)> for Payload {
//   fn from_iter<T: IntoIterator<Item = (ThingId, AnyValue)>>(iter: T) -> Self {
//     Payload(iter.into_iter().collect())
//   }
// }
// impl IntoIterator for Payload {
//   type Item = (ThingId, AnyValue);
//   type IntoIter = std::collections::hash_map::IntoIter<ThingId, AnyValue>;

//   fn into_iter(self) -> Self::IntoIter {
//     self.0.into_iter()
//   }
// }

// impl Payload {
//   pub fn get<T>(&self, key: ThingId) -> Option<Result<T, Error>>
//   where
//     T: DeserializeOwned + 'static,
//   {
//     self.0.get(&key).map(|value| {
//       surrealdb::value::from_value(value.clone()).map_err(|err| Error::DeserializePayloadValue {
//         key,
//         ty: std::any::TypeId::of::<T>(),
//         err,
//       })
//     })
//   }
// }

// pub trait TryFromPayload: Sized {
//   fn try_from_payload(payload: Payload) -> Result<Self, Error>;
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct PayloadSerde(HashMap<String, AnyValue>);

// impl From<Payload> for PayloadSerde {
//   fn from(value: Payload) -> Self {
//     PayloadSerde(
//       value
//         .into_iter()
//         .map(|(k, v)| {
//           (
//             k.to_string(),
//             v, // surrealdb::value::from_value(v)
//                //   .expect("serde_json::Value to be more permissive than surrealdb::Value"),
//           )
//         })
//         .collect(),
//     )
//   }
// }

// impl TryFrom<PayloadSerde> for Payload {
//   type Error = Error;

//   fn try_from(value: PayloadSerde) -> Result<Self, Self::Error> {
//     Ok(Payload(
//       value
//         .0
//         .into_iter()
//         .map(|(k, v)| Result::<_, surrealdb::Error>::Ok((ThingId::from_str(&k)?, v)))
//         .collect::<Result<_, _>>()
//         .map_err(Error::DeserializingPayload)?,
//     ))
//   }
// }

// #[test]
// #[ignore = "Ik it fails, https://github.com/surrealdb/surrealdb/issues/5754"]
// fn surreal_bug() -> color_eyre::Result<()> {
//   utils::tracing::install_tracing("debug")?;

//   let json: serde_json::Value = serde_json::json! {{
//     "thing:example": 123,
//   }};
//   debug!(?json);

//   let surreal: surrealdb::value::Value = surrealdb::value::to_value(json)?;
//   debug!(?surreal);

//   // works
//   let deserialized: HashMap<String, u32> = surrealdb::value::from_value(surreal.clone())?;
//   debug!(?deserialized);

//   // broken?
//   let bad: HashMap<String, surrealdb::Value> = surrealdb::value::from_value(surreal.clone())?;
//   debug!(?bad);

//   // broken?
//   let bad: HashMap<String, serde_json::Value> = surrealdb::value::from_value(surreal)?;
//   debug!(?bad);

//   Ok(())
// }

// #[test]
// fn surreal_bugs() -> color_eyre::Result<()> {
//   utils::tracing::install_tracing("debug")?;

//   {
//     let val = serde_json::json!(123);
//     let surreal: surrealdb::Value = surrealdb::value::to_value(val)?;
//     let num: u32 = surrealdb::value::from_value(surreal.clone())?;
//     debug!(?num);
//   }

//   {
//     let val = serde_json::json!({ "key": "value" });
//     let val = serde_json::json!(123);
//     let surreal: surrealdb::Value = surrealdb::value::to_value(val)?;
//     let res: surrealdb::Value = surrealdb::value::from_value(surreal.clone())?;
//     debug!(?res);
//   }

//   Ok(())
// }

// #[test]
// #[ignore]
// fn surreal_value() -> color_eyre::Result<()> {
//   utils::tracing::install_tracing("debug")?;

//   let json: serde_json::Value = serde_json::json! {{
//     "thing:example": "a string",
//   }};
//   debug!(?json);

//   let json_str = json.to_string();
//   debug!(?json_str);

//   let obj_from_str = surrealdb::Value::from_str(&json_str)?;
//   debug!(?obj_from_str);

//   let obj: surrealdb::Object = serde_json::from_value(json)?;
//   debug!(?obj);

//   Ok(())
// }
