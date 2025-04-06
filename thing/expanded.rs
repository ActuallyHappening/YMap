#![feature(prelude_import)]
#![allow(unused_imports, async_fn_in_trait)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod prelude {
    pub(crate) use extension_traits::extension;
    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use tracing::{debug, error, info, trace, warn};
    pub use crate::db::ThingExt as _;
    pub(crate) use db::prelude::*;
}
pub mod errors {
    use crate::prelude::*;
    pub enum Error {
        #[error("Couldn't select data: {0}")]
        CouldntSelect(#[source] surrealdb::Error),
        #[error("Couldn't find a known record {0}")]
        KnownRecordNotFound(surrealdb::RecordId),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::CouldntSelect(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "CouldntSelect",
                        &__self_0,
                    )
                }
                Error::KnownRecordNotFound(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "KnownRecordNotFound",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::thiserror::__private::Error for Error {
        fn source(
            &self,
        ) -> ::core::option::Option<&(dyn ::thiserror::__private::Error + 'static)> {
            use ::thiserror::__private::AsDynError as _;
            #[allow(deprecated)]
            match self {
                Error::CouldntSelect { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::KnownRecordNotFound { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for Error {
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            use ::thiserror::__private::AsDisplay as _;
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                Error::CouldntSelect(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!("Couldn\'t select data: {0}", __display0),
                                )
                        }
                    }
                }
                Error::KnownRecordNotFound(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Couldn\'t find a known record {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
            }
        }
    }
}
use crate::prelude::*;
pub struct Thing<Payload> {
    id: ThingId,
    _debug_name: Option<String>,
    parents: Vec<ThingId>,
    payload: Payload,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, Payload> _serde::Deserialize<'de> for Thing<Payload>
    where
        Payload: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
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
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private::Ok(__Field::__field0),
                        "_debug_name" => _serde::__private::Ok(__Field::__field1),
                        "parents" => _serde::__private::Ok(__Field::__field2),
                        "payload" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private::Ok(__Field::__field0),
                        b"_debug_name" => _serde::__private::Ok(__Field::__field1),
                        b"parents" => _serde::__private::Ok(__Field::__field2),
                        b"payload" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, Payload>
            where
                Payload: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<Thing<Payload>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, Payload> _serde::de::Visitor<'de> for __Visitor<'de, Payload>
            where
                Payload: _serde::Deserialize<'de>,
            {
                type Value = Thing<Payload>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Thing")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        ThingId,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Thing with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Thing with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Vec<ThingId>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Thing with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        Payload,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct Thing with 4 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Thing {
                        id: __field0,
                        _debug_name: __field1,
                        parents: __field2,
                        payload: __field3,
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
                    let mut __field0: _serde::__private::Option<ThingId> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Vec<ThingId>> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<Payload> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<ThingId>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "_debug_name",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "parents",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<ThingId>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "payload",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<Payload>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("id")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("_debug_name")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("parents")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("payload")?
                        }
                    };
                    _serde::__private::Ok(Thing {
                        id: __field0,
                        _debug_name: __field1,
                        parents: __field2,
                        payload: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "id",
                "_debug_name",
                "parents",
                "payload",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Thing",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Thing<Payload>>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl<Payload: ::core::fmt::Debug> ::core::fmt::Debug for Thing<Payload> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Thing",
            "id",
            &self.id,
            "_debug_name",
            &self._debug_name,
            "parents",
            &self.parents,
            "payload",
            &&self.payload,
        )
    }
}
impl<P> surrealdb_layers::Table for Thing<P> {
    const TABLE: &str = "thing";
}
impl<P> surrealdb_layers::GetId for Thing<P> {
    type Table = Self;
    type Id = ThingId;
    fn get_id(&self) -> &Self::Id {
        &self.id
    }
}
impl<P> Thing<P> {
    pub fn _debug_name(&self) -> Option<String> {
        self._debug_name.clone()
    }
    pub fn parents(&self) -> Vec<ThingId> {
        self.parents.clone()
    }
    pub fn payload(&self) -> &P {
        &self.payload
    }
}
pub mod well_known {
    use crate::prelude::*;
    use super::{ThingId, payload::IsPayloadEntry};
    pub trait KnownRecord {
        fn known() -> &'static str;
        fn known_id() -> ThingId {
            ThingId::new_known(Self::known().into())
        }
    }
    pub struct NameEn(String);
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for NameEn {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<NameEn>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = NameEn;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct NameEn",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: String = <String as _serde::Deserialize>::deserialize(
                            __e,
                        )?;
                        _serde::__private::Ok(NameEn(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct NameEn with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(NameEn(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "NameEn",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<NameEn>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for NameEn {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "NameEn",
                    &self.0,
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for NameEn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "NameEn", &&self.0)
        }
    }
    impl IsPayloadEntry for NameEn {
        fn known() -> &'static str {
            "name-en"
        }
    }
    pub struct DescriptionEn(String);
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for DescriptionEn {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<DescriptionEn>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = DescriptionEn;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct DescriptionEn",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: String = <String as _serde::Deserialize>::deserialize(
                            __e,
                        )?;
                        _serde::__private::Ok(DescriptionEn(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct DescriptionEn with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(DescriptionEn(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "DescriptionEn",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<DescriptionEn>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for DescriptionEn {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "DescriptionEn",
                    &self.0,
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for DescriptionEn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "DescriptionEn",
                &&self.0,
            )
        }
    }
    impl IsPayloadEntry for DescriptionEn {
        fn known() -> &'static str {
            "description-en"
        }
    }
    pub mod website {
        use serde::de::{self, Visitor};
        use crate::{
            Thing, ThingId, payload::{IsPayload, IsPayloadEntry},
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
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for WebsiteRootPayload {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "WebsiteRootPayload",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "info",
                        &self.info,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DynamicNames {
            pairs: Vec<(String,)>,
        }
        impl<'de> Deserialize<'de> for WebsiteRootPayload {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                enum Field {
                    Field0,
                    Field1,
                }
                struct FieldVisitor;
                const FIELDS: &[&str] = &["thing:websiteroot", "thing:name-en"];
                impl Field {
                    pub fn name(self) -> &'static str {
                        match self {
                            Field::Field0 => <WebsiteInfo as IsPayloadEntry>::known(),
                            Field::Field1 => <NameEn as IsPayloadEntry>::known(),
                        }
                    }
                }
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;
                    fn expecting(
                        &self,
                        f: &mut std::fmt::Formatter,
                    ) -> std::fmt::Result {
                        f.write_fmt(format_args!("field identifier"))
                    }
                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            1 => Ok(Field::Field0),
                            2 => Ok(Field::Field1),
                            _ => {
                                Err(
                                    E::invalid_value(
                                        serde::de::Unexpected::Unsigned(v),
                                        &"field index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        if v == Field::Field0.name() {
                            return Ok(Field::Field0);
                        }
                        if v == Field::Field1.name() {
                            return Ok(Field::Field1);
                        }
                        Err(de::Error::unknown_field(v, FIELDS))
                    }
                    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        if v == Field::Field0.name().as_bytes() {
                            return Ok(Field::Field0);
                        }
                        if v == Field::Field1.name().as_bytes() {
                            return Ok(Field::Field1);
                        }
                        Err(
                            de::Error::unknown_field(
                                &std::string::String::from_utf8_lossy(v),
                                FIELDS,
                            ),
                        )
                    }
                }
                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: de::Deserializer<'de>,
                    {
                        serde::Deserializer::deserialize_identifier(
                            deserializer,
                            FieldVisitor,
                        )
                    }
                }
                struct MyVisitor<'de> {
                    marker: std::marker::PhantomData<WebsiteRootPayload>,
                    lifetime: std::marker::PhantomData<&'de ()>,
                }
                impl<'de> Visitor<'de> for MyVisitor<'de> {
                    type Value = WebsiteRootPayload;
                    fn expecting(
                        &self,
                        f: &mut std::fmt::Formatter,
                    ) -> std::fmt::Result {
                        f.write_fmt(format_args!("struct WebsiteRootPayload"))
                    }
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: de::SeqAccess<'de>,
                    {
                        let field0 = match de::SeqAccess::next_element::<
                            WebsiteInfo,
                        >(&mut seq)? {
                            Some(val) => val,
                            None => {
                                return Err(
                                    de::Error::invalid_length(
                                        0usize,
                                        &"struct WebsiteRootPayload with 2 elements",
                                    ),
                                );
                            }
                        };
                        let field1 = match de::SeqAccess::next_element::<
                            NameEn,
                        >(&mut seq)? {
                            Some(val) => val,
                            None => {
                                return Err(
                                    de::Error::invalid_length(
                                        1usize,
                                        &"struct WebsiteRootPayload with 2 elements",
                                    ),
                                );
                            }
                        };
                        Ok(WebsiteRootPayload {
                            info: field0,
                            name: field1,
                        })
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: de::MapAccess<'de>,
                    {
                        let mut field0 = None;
                        let mut field1 = None;
                        while let Some(key) = de::MapAccess::next_key::<
                            Field,
                        >(&mut map)? {
                            match key {
                                Field::Field0 => {
                                    if field0.is_some() {
                                        return Err(
                                            de::Error::duplicate_field("thing:websiteroot (aka info)"),
                                        );
                                    }
                                    field0 = Some(de::MapAccess::next_value(&mut map)?);
                                }
                                Field::Field1 => {
                                    if field1.is_some() {
                                        return Err(
                                            de::Error::duplicate_field("thing:name-en (aka name)"),
                                        );
                                    }
                                    field1 = Some(de::MapAccess::next_value(&mut map)?);
                                }
                            }
                        }
                        Ok(WebsiteRootPayload {
                            info: field0
                                .ok_or_else(|| de::Error::missing_field(
                                    "thing:websiteroot (aka info)",
                                ))?,
                            name: field1
                                .ok_or_else(|| de::Error::missing_field(
                                    "thing:name-en (aka name)",
                                ))?,
                        })
                    }
                }
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
        pub struct WebsiteInfo {
            show_children: Vec<ThingId>,
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for WebsiteInfo {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
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
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "show_children" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"show_children" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<WebsiteInfo>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = WebsiteInfo;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct WebsiteInfo",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Vec<ThingId>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct WebsiteInfo with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(WebsiteInfo {
                                show_children: __field0,
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
                            let mut __field0: _serde::__private::Option<Vec<ThingId>> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "show_children",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<ThingId>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("show_children")?
                                }
                            };
                            _serde::__private::Ok(WebsiteInfo {
                                show_children: __field0,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["show_children"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "WebsiteInfo",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<WebsiteInfo>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for WebsiteInfo {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "WebsiteInfo",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "show_children",
                        &self.show_children,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for WebsiteInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "WebsiteInfo",
                    "show_children",
                    &&self.show_children,
                )
            }
        }
        impl IsPayload for WebsiteRootPayload {}
        impl IsPayloadEntry for WebsiteInfo {
            fn known() -> &'static str {
                WebsiteRoot::known()
            }
        }
    }
}
mod db {
    use crate::{errors::Error, prelude::*};
    use db::auth::NoAuth;
    use serde::de::DeserializeOwned;
    use super::{Thing, well_known::KnownRecord};
    /**

This is an extension trait for the following impl:
```rust ,ignore
#[extension(pub trait ThingExt)]
impl for Db < NoAuth >

```*/
    #[allow(nonstandard_style)]
    pub trait ThingExt {
        async fn thing<P>(&self) -> Result<Thing<P>, Error>
        where
            Thing<P>: DeserializeOwned + KnownRecord;
    }
    impl ThingExt for Db<NoAuth> {
        async fn thing<P>(&self) -> Result<Thing<P>, Error>
        where
            Thing<P>: DeserializeOwned + KnownRecord,
        {
            let id = <Thing<P>>::known_id();
            let thing: Option<Thing<P>> = self
                .db()
                .select(id.clone())
                .await
                .map_err(|err| Error::CouldntSelect(err))?;
            let thing = thing.ok_or(Error::KnownRecordNotFound(id.into_inner()))?;
            Ok(thing)
        }
    }
}
mod payload {
    use serde::{Deserializer, de::DeserializeOwned};
    use crate::layers::Id;
    use super::ThingId;
    pub trait IsPayload {}
    /// Todo: write a trait to deserialize
    /// using this dynamic key
    pub trait IsPayloadEntry: DeserializeOwned {
        fn known() -> &'static str;
        fn known_id() -> ThingId {
            ThingId::new_known(Self::known().into())
        }
    }
}
pub use id::ThingId;
pub mod id {
    use std::{fmt::Display, str::FromStr};
    use surrealdb::opt::IntoResource;
    use crate::prelude::*;
    use super::Thing;
    pub struct ThingId(
        #[serde(deserialize_with = "surrealdb_layers::serde::string_or_struct")]
        surrealdb::RecordId,
    );
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ThingId {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ThingId>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ThingId;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct ThingId",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: surrealdb::RecordId = surrealdb_layers::serde::string_or_struct(
                            __e,
                        )?;
                        _serde::__private::Ok(ThingId(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: surrealdb::RecordId,
                                phantom: _serde::__private::PhantomData<ThingId>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de>
                            for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: surrealdb_layers::serde::string_or_struct(
                                            __deserializer,
                                        )?,
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                _serde::de::SeqAccess::next_element::<
                                    __DeserializeWith<'de>,
                                >(&mut __seq)?,
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct ThingId with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ThingId(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "ThingId",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ThingId>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ThingId {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "ThingId",
                    &self.0,
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for ThingId {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ThingId", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ThingId {
        #[inline]
        fn clone(&self) -> ThingId {
            ThingId(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ThingId {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ThingId {
        #[inline]
        fn eq(&self, other: &ThingId) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ThingId {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<surrealdb::RecordId>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ThingId {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ThingId,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ThingId {
        #[inline]
        fn cmp(&self, other: &ThingId) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ThingId {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    impl ThingId {
        pub fn into_inner(self) -> surrealdb::RecordId {
            self.0
        }
    }
    impl surrealdb_layers::Id for ThingId {
        type Table = Thing<()>;
        fn new_known(key: surrealdb::RecordIdKey) -> Self {
            Self((Thing::<()>::TABLE, key).into())
        }
    }
    impl<P> IntoResource<Option<Thing<P>>> for ThingId {
        fn into_resource(self) -> surrealdb::Result<surrealdb::opt::Resource> {
            IntoResource::<Option<Thing<P>>>::into_resource(self.0)
        }
    }
    /// Forwards Display impl
    impl Display for ThingId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Display::fmt(&self.0, f)
        }
    }
    impl FromStr for ThingId {
        type Err = surrealdb::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(ThingId(surrealdb::RecordId::from_str(s)?))
        }
    }
}
