use crate::prelude::*;

use super::{ThingId, payload::IsPayloadEntry};

pub trait KnownRecord {
  fn known() -> &'static str;
  fn known_id() -> ThingId {
    ThingId::new_known(Self::known().into())
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NameEn(String);

impl IsPayloadEntry for NameEn {
  fn known() -> &'static str {
    "name-en"
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DescriptionEn(String);

impl IsPayloadEntry for DescriptionEn {
  fn known() -> &'static str {
    "description-en"
  }
}

pub mod website {
  use serde::de::{self, Visitor};

  use crate::{
    Thing, ThingId,
    payload::{IsPayload, IsPayloadEntry},
    prelude::*,
  };

  use super::{KnownRecord, NameEn};

  pub type WebsiteRoot = Thing<WebsiteRootPayload>;

  impl KnownRecord for WebsiteRoot {
    fn known() -> &'static str {
      "websiteroot"
    }
    fn known_id() -> ThingId {
      ThingId::new_known("websiteroot".into())
    }
  }

  #[derive(Debug)]
  // #[serde(deny_unknown_fields)]
  pub struct WebsiteRootPayload {
    // #[serde(rename = "thing:websiteroot")]
    info: WebsiteInfo,

    // #[serde(rename = "thing:name-en")]
    name: NameEn,
  }

  impl<'de> Deserialize<'de> for WebsiteRootPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'de>,
    {
      enum Field {
        Field0,
        Field1,
        Ignore,
      }
      struct FieldVisitor;
      const FIELDS: &[&str] = &["thing:websiteroot", "thing:name-en"];

      impl<'de> Visitor<'de> for FieldVisitor {
        type Value = Field;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
          write!(f, "field identifier")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
          E: serde::de::Error,
        {
          match v {
            // PARAM
            1 => Ok(Field::Field0),
            2 => Ok(Field::Field1),
            // PARAM
            // _ => Err(E::invalid_value(
            //   serde::de::Unexpected::Unsigned(v),
            //   &"field index 0 <= i < 2",
            // )),
            _ => Ok(Field::Ignore),
          }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
          E: serde::de::Error,
        {
          if v == <WebsiteInfo as IsPayloadEntry>::known() {
            return Ok(Field::Field0);
          }
          if v == <NameEn as IsPayloadEntry>::known() {
            return Ok(Field::Field1);
          }
          Ok(Field::Ignore)
          // Err(de::Error::unknown_field(v, FIELDS))
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
          E: de::Error,
        {
          if v == <WebsiteInfo as IsPayloadEntry>::known().as_bytes() {
            return Ok(Field::Field0);
          }
          if v == <WebsiteInfo as IsPayloadEntry>::known().as_bytes() {
            return Ok(Field::Field1);
          }
          Ok(Field::Ignore)
          // Err(de::Error::unknown_field(
          //   &std::string::String::from_utf8_lossy(v),
          //   FIELDS,
          // ))
        }
      }

      impl<'de> Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
          D: de::Deserializer<'de>,
        {
          serde::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
        }
      }

      struct MyVisitor<'de> {
        // PARAM
        marker: std::marker::PhantomData<WebsiteRootPayload>,
        lifetime: std::marker::PhantomData<&'de ()>,
      }

      impl<'de> Visitor<'de> for MyVisitor<'de> {
        // PARAM
        type Value = WebsiteRootPayload;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
          // PARAM
          write!(f, "struct WebsiteRootPayload")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
          A: de::SeqAccess<'de>,
        {
          // PARAM by field
          let field0 = match de::SeqAccess::next_element::<WebsiteInfo>(&mut seq)? {
            Some(val) => val,
            None => {
              // PARAM
              return Err(de::Error::invalid_length(
                0usize,
                &"struct WebsiteRootPayload with 2 elements",
              ));
            }
          };
          // PARAM by field
          let field1 = match de::SeqAccess::next_element::<NameEn>(&mut seq)? {
            Some(val) => val,
            None => {
              // PARAM
              return Err(de::Error::invalid_length(
                1usize,
                &"struct WebsiteRootPayload with 2 elements",
              ));
            }
          };
          // PARAM
          Ok(WebsiteRootPayload {
            info: field0,
            name: field1,
          })
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
          A: de::MapAccess<'de>,
        {
          // PARAM
          let mut field0 = None;
          let mut field1 = None;
          while let Some(key) = de::MapAccess::next_key::<Field>(&mut map)? {
            match key {
              Field::Field0 => {
                if field0.is_some() {
                  // PARAM
                  return Err(de::Error::duplicate_field("thing:websiteroot (aka info)"));
                }
                field0 = Some(de::MapAccess::next_value(&mut map)?);
              }
              Field::Field1 => {
                if field1.is_some() {
                  // PARAM
                  return Err(de::Error::duplicate_field("thing:name-en (aka name)"));
                }
                field1 = Some(de::MapAccess::next_value(&mut map)?);
              }
              Field::Ignore => {
                _ = de::MapAccess::next_value::<de::IgnoredAny>(&mut map);
              }
            }
          }
          // PARAM
          Ok(WebsiteRootPayload {
            info: field0.ok_or_else(|| de::Error::missing_field("thing:websiteroot (aka info)"))?,
            name: field1.ok_or_else(|| de::Error::missing_field("thing:name-en (aka name)"))?,
          })
        }
      }

      // PARAM
      serde::Deserializer::deserialize_struct(
        deserializer,
        "WebsiteRootPayload",
        FIELDS,
        MyVisitor {
          marker: std::marker::PhantomData,
          lifetime: std::marker::PhantomData,
        },
      )
    }
  }

  impl Serialize for WebsiteRootPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      // PARAM len + name
      let mut state =
        serde::Serializer::serialize_struct(serializer, "WebsiteRootPayload", 0 + 1 + 1)?;
      // PARAM repeat
      serde::ser::SerializeStruct::serialize_field(
        &mut state,
        <WebsiteInfo as IsPayloadEntry>::known(),
        &self.info,
      )?;
      serde::ser::SerializeStruct::serialize_field(
        &mut state,
        <NameEn as IsPayloadEntry>::known(),
        &self.name,
      )?;
      serde::ser::SerializeStruct::end(state)
    }
  }

  #[derive(Deserialize, Serialize, Debug)]
  pub struct WebsiteInfo {
    show_children: Vec<ThingId>,
  }

  impl IsPayload for WebsiteRootPayload {}

  impl IsPayloadEntry for WebsiteInfo {
    fn known() -> &'static str {
      WebsiteRoot::known()
    }
  }
}
