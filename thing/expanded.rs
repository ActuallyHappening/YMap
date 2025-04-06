
pub struct WebsiteRootPayload {
  info: WebsiteInfo,
  name: NameEn,
}
#[automatically_derived]
impl ::core::fmt::Debug for WebsiteRootPayload {
  #[inline]
  fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
    ::core::fmt::Formatter::debug_struct_field2_finish(
      f,
      "WebsiteRootPayload",
      "info",
      &self.info,
      "name",
      &&self.name,
    )
  }
}
#[doc(hidden)]
#[allow(
  non_upper_case_globals,
  unused_attributes,
  unused_qualifications,
  clippy::absolute_paths
)]
const _: () = {
  #[allow(unused_extern_crates, clippy::useless_attribute)]
  extern crate serde as _serde;
  #[automatically_derived]
  impl<'de> _serde::Deserialize<'de> for WebsiteRootPayload {
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
    where
      __D: _serde::Deserializer<'de>,
    {
      #[allow(non_camel_case_types)]
      #[doc(hidden)]
      enum __Field {
        __field0,
        __field1,
        __ignore,
      }
      #[doc(hidden)]
      struct __FieldVisitor;
      #[automatically_derived]
      impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
        type Value = __Field;
        fn expecting(
          &self,
          __formatter: &mut _serde::__private::Formatter,
        ) -> _serde::__private::fmt::Result {
          _serde::__private::Formatter::write_str(__formatter, "field identifier")
        }
        fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
        where
          __E: _serde::de::Error,
        {
          match __value {
            0u64 => _serde::__private::Ok(__Field::__field0),
            1u64 => _serde::__private::Ok(__Field::__field1),
            _ => _serde::__private::Ok(__Field::__ignore),
          }
        }
        fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
        where
          __E: _serde::de::Error,
        {
          match __value {
            "info" => _serde::__private::Ok(__Field::__field0),
            "name" => _serde::__private::Ok(__Field::__field1),
            _ => _serde::__private::Ok(__Field::__ignore),
          }
        }
        fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
        where
          __E: _serde::de::Error,
        {
          match __value {
            b"info" => _serde::__private::Ok(__Field::__field0),
            b"name" => _serde::__private::Ok(__Field::__field1),
            _ => _serde::__private::Ok(__Field::__ignore),
          }
        }
      }
      #[automatically_derived]
      impl<'de> _serde::Deserialize<'de> for __Field {
        #[inline]
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
          __D: _serde::Deserializer<'de>,
        {
          _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
        }
      }
      #[doc(hidden)]
      struct __Visitor<'de> {
        marker: _serde::__private::PhantomData<WebsiteRootPayload>,
        lifetime: _serde::__private::PhantomData<&'de ()>,
      }
      #[automatically_derived]
      impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
        type Value = WebsiteRootPayload;
        fn expecting(
          &self,
          __formatter: &mut _serde::__private::Formatter,
        ) -> _serde::__private::fmt::Result {
          _serde::__private::Formatter::write_str(__formatter, "struct WebsiteRootPayload")
        }
        #[inline]
        fn visit_seq<__A>(
          self,
          mut __seq: __A,
        ) -> _serde::__private::Result<Self::Value, __A::Error>
        where
          __A: _serde::de::SeqAccess<'de>,
        {
          let __field0 = match _serde::de::SeqAccess::next_element::<WebsiteInfo>(&mut __seq)? {
            _serde::__private::Some(__value) => __value,
            _serde::__private::None => {
              return _serde::__private::Err(_serde::de::Error::invalid_length(
                0usize,
                &"struct WebsiteRootPayload with 2 elements",
              ));
            }
          };
          let __field1 = match _serde::de::SeqAccess::next_element::<NameEn>(&mut __seq)? {
            _serde::__private::Some(__value) => __value,
            _serde::__private::None => {
              return _serde::__private::Err(_serde::de::Error::invalid_length(
                1usize,
                &"struct WebsiteRootPayload with 2 elements",
              ));
            }
          };
          _serde::__private::Ok(WebsiteRootPayload {
            info: __field0,
            name: __field1,
          })
        }
        #[inline]
        fn visit_map<__A>(
          self,
          mut __map: __A,
        ) -> _serde::__private::Result<Self::Value, __A::Error>
        where
          __A: _serde::de::MapAccess<'de>,
        {
          let mut __field0: _serde::__private::Option<WebsiteInfo> = _serde::__private::None;
          let mut __field1: _serde::__private::Option<NameEn> = _serde::__private::None;
          while let _serde::__private::Some(__key) =
            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
          {
            match __key {
              __Field::__field0 => {
                if _serde::__private::Option::is_some(&__field0) {
                  return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::duplicate_field("info"),
                  );
                }
                __field0 = _serde::__private::Some(
                  _serde::de::MapAccess::next_value::<WebsiteInfo>(&mut __map)?,
                );
              }
              __Field::__field1 => {
                if _serde::__private::Option::is_some(&__field1) {
                  return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::duplicate_field("name"),
                  );
                }
                __field1 =
                  _serde::__private::Some(_serde::de::MapAccess::next_value::<NameEn>(&mut __map)?);
              }
              _ => {
                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
              }
            }
          }
          let __field0 = match __field0 {
            _serde::__private::Some(__field0) => __field0,
            _serde::__private::None => _serde::__private::de::missing_field("info")?,
          };
          let __field1 = match __field1 {
            _serde::__private::Some(__field1) => __field1,
            _serde::__private::None => _serde::__private::de::missing_field("name")?,
          };
          _serde::__private::Ok(WebsiteRootPayload {
            info: __field0,
            name: __field1,
          })
        }
      }
      #[doc(hidden)]
      const FIELDS: &'static [&'static str] = &["info", "name"];
      _serde::Deserializer::deserialize_struct(
        __deserializer,
        "WebsiteRootPayload",
        FIELDS,
        __Visitor {
          marker: _serde::__private::PhantomData::<WebsiteRootPayload>,
          lifetime: _serde::__private::PhantomData,
        },
      )
    }
  }
};
impl Serialize for WebsiteRootPayload {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state =
      serde::Serializer::serialize_struct(serializer, "WebsiteRootPayload", 0 + 1 + 1)?;
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