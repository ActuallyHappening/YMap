#[proc_macro_derive(Serialize, attributes(serde))]
pub fn ser(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(tokens as DeriveInput);
	payload_impl(input, Do::Serialize)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn de(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(tokens as DeriveInput);
	payload_impl(input, Do::Deserialize)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}

enum Do {
	Serialize,
	Deserialize,
}

use darling::FromMeta;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::token::{Enum, Union};
use syn::{Data, DataEnum, DataUnion, Error, Fields, Ident};

use syn::{DeriveInput, parse_macro_input};

// https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs
fn payload_impl(input: syn::DeriveInput, emit: Do) -> syn::Result<proc_macro2::TokenStream> {
	let name = input.ident;
	let name_str = name.to_string();

	Ok(match input.data {
		syn::Data::Struct(data) => {
			let fields = extract_field_info(data.fields)?;
			let num_fields = fields.len();

			match emit {
				Do::Serialize => {
					let field_serializers = fields.iter().map(
						|MyField {
						   renamed_link,
						   written_name,
						   ..
						 }| {
							quote! {
								::serde::ser::SerializeStruct::serialize_field(
									&mut state,
									#renamed_link,
									&self.#written_name,
								)?;
							}
						},
					);

					quote! {
						impl ::serde::Serialize for #name {
							fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
							where
								S: ::serde::Serializer,
							{
								// PARAM len + name
								let mut state =
									::serde::Serializer::serialize_struct(serializer, #name_str, #num_fields)?;

								#( #field_serializers )*

								::serde::ser::SerializeStruct::end(state)
							}
						}
					}
					// quote! {}
				}
				Do::Deserialize => {
					// Field0, Field1 e.t.c.
					let field_enum_idents = fields
						.iter()
						.enumerate()
						.map(|(i, _)| format_ident!("Field{}", i.to_string()))
						.collect::<Vec<_>>();

					let everything = field_enum_idents.iter().zip(fields.iter());

					let fields_const_strs = fields
						.iter()
						.map(|MyField { written_name, .. }| written_name.to_string());

					let fields_visit_u64 = field_enum_idents.iter().enumerate().map(|(i, ident)| {
						let i: u64 = i.try_into().unwrap();
						quote! {
							#i => ::core::result::Result::Ok(Field::#ident)
						}
					});

					let fields_visit_str = field_enum_idents.iter().zip(fields.iter()).map(
						|(ident, MyField { renamed_link, .. })| {
							quote! {
								if v == #renamed_link {
									return ::core::result::Result::Ok(Field::#ident);
								}
							}
						},
					);
					let fields_visit_bytes = field_enum_idents.iter().zip(fields.iter()).map(
						|(ident, MyField { renamed_link, .. })| {
							quote! {
								if v == #renamed_link.as_bytes() {
									return ::core::result::Result::Ok(Field::#ident);
								}
							}
						},
					);
					let fields_visit_seq =
						fields
							.iter()
							.zip(field_enum_idents.iter())
							.map(|(MyField { ty, .. }, ident)| {
								quote! {
									let #ident = match ::serde::de::SeqAccess::next_element::<#ty>(&mut seq)? {
										::core::option::Option::Some(val) => val,
										::core::option::Option::None => {
											// PARAM
											return ::core::result::Result::Err(::serde::de::Error::invalid_length(
												0usize,
												&concat!("struct ", #name_str, " with ", #num_fields, " elements"),
											));
										}
									};
								}
							});
					let fields_visit_seq_final = fields.iter().zip(field_enum_idents.iter()).map(
						|(MyField { written_name, .. }, field_enum_ident)| {
							quote! {
								#written_name: #field_enum_ident
							}
						},
					);

					let visit_map_defs = field_enum_idents.iter().map(|ident| {
						quote! {
							let mut #ident = ::core::option::Option::None;
						}
					});
					let visit_map_matches = field_enum_idents.iter().zip(fields.iter()).map(
						|(ident, MyField { renamed_link, .. })| {
							quote! {
								Field::#ident => {
									if #ident.is_some() {
										// PARAM
										return ::core::result::Result::Err(::serde::de::Error::duplicate_field(
											// "thing:websiteroot (aka info)",
											// param_err_msg
											// concat!(stringify!(#written_name), " (but is is dynamically renamed)")
											#renamed_link
										));
									}
									#ident =
										::core::option::Option::Some(::serde::de::MapAccess::next_value(&mut map)?);
								}
							}
						},
					);
					let visit_map_final = everything.clone().map(
						|(
							ident,
							MyField {
								written_name,
								renamed_link,
								..
							},
						)| {
							quote! {
								#written_name: #ident.ok_or_else(||
									::serde::de::Error::missing_field(
										// concat!(stringify!(#written_name), " (but is is dynamically renamed)")
										#renamed_link
									)
								)?
							}
						},
					);

					quote! {
						const _: () = {
							impl<'de> ::serde::Deserialize<'de> for #name {
								fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
								where
									D: ::serde::Deserializer<'de>,
								{
									enum Field {
										#( #field_enum_idents, )*
										Ignore,
									}
									struct FieldVisitor;
									const FIELDS: &[&str] = &[ #( #fields_const_strs, )* ];

									impl<'de> ::serde::de::Visitor<'de> for FieldVisitor {
										type Value = Field;

										fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
											write!(f, "field identifier")
										}

										fn visit_u64<E>(self, v: u64) -> ::core::result::Result<Self::Value, E>
										where
											E: ::serde::de::Error,
										{
											match v {
												// PARAM
												// 1 => ::core::result::Result::Ok(Field::Field0),
												// 2 => ::core::result::Result::Ok(Field::Field1),

												#( #fields_visit_u64 , )*

												// PARAM deny_unknown_fields
												// _ => Err(E::invalid_value(
												//   serde::de::Unexpected::Unsigned(v),
												//   &"field index 0 <= i < 2",
												// )),
												_ => ::core::result::Result::Ok(Field::Ignore),
											}
										}

										fn visit_str<E>(self, v: &str) -> ::core::result::Result<Self::Value, E>
										where
											E: ::serde::de::Error,
										{
											// if v == <WebsiteInfo as IsPayloadEntry>::known() {
											//   return ::core::result::Result::Ok(Field::Field0);
											// }
											// if v == <NameEn as IsPayloadEntry>::known() {
											//   return ::core::result::Result::Ok(Field::Field1);
											// }
											#( #fields_visit_str )*

											::core::result::Result::Ok(Field::Ignore)
											// Err(de::Error::unknown_field(v, FIELDS))
										}

										fn visit_bytes<E>(self, v: &[u8]) -> ::core::result::Result<Self::Value, E>
										where
											E: ::serde::de::Error,
										{
											// if v == <WebsiteInfo as IsPayloadEntry>::known().as_bytes() {
											//   return ::core::result::Result::Ok(Field::Field0);
											// }
											// if v == <WebsiteInfo as IsPayloadEntry>::known().as_bytes() {
											//   return ::core::result::Result::Ok(Field::Field1);
											// }

											#( #fields_visit_bytes )*


											::core::result::Result::Ok(Field::Ignore)
											// Err(de::Error::unknown_field(
											//   &std::string::String::from_utf8_lossy(v),
											//   FIELDS,
											// ))
										}
									}

									impl<'de> serde::Deserialize<'de> for Field {
										fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
										where
											D: ::serde::de::Deserializer<'de>,
										{
											serde::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
										}
									}

									struct MyVisitor<'de> {
										// PARAM
										marker: ::core::marker::PhantomData<#name>,
										lifetime: ::core::marker::PhantomData<&'de ()>,
									}

									impl<'de> ::serde::de::Visitor<'de> for MyVisitor<'de> {
										// PARAM
										type Value = #name;

										fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
											// PARAM
											::std::write!(f, concat!("struct ", #name_str))
										}

										fn visit_seq<A>(self, mut seq: A) -> ::core::result::Result<Self::Value, A::Error>
										where
											A: ::serde::de::SeqAccess<'de>,
										{
											#( #fields_visit_seq ; )*

											::core::result::Result::Ok(#name {
												#( #fields_visit_seq_final, )*
											})
										}

										fn visit_map<A>(self, mut map: A) -> ::core::result::Result<Self::Value, A::Error>
										where
											A: ::serde::de::MapAccess<'de>,
										{
											// PARAM
											#( #visit_map_defs )*

											while let ::core::option::Option::Some(key) =
												::serde::de::MapAccess::next_key::<Field>(&mut map)?
											{
												match key {
													#( #visit_map_matches )*
													Field::Ignore => {
														_ = ::serde::de::MapAccess::next_value::<::serde::de::IgnoredAny>(&mut map);
													}
												}
											}
											// PARAM
											::core::result::Result::Ok(#name {
												#(#visit_map_final,)*
											})
										}
									}

									// PARAM
									::serde::Deserializer::deserialize_struct(
										deserializer,
										#name_str,
										FIELDS,
										MyVisitor {
											marker: ::std::marker::PhantomData,
											lifetime: ::std::marker::PhantomData,
										},
									)
								}
							}
						};
					}
				}
			}
		}
		Data::Enum(DataEnum {
			enum_token: Enum { span },
			..
		})
		| Data::Union(DataUnion {
			union_token: Union { span },
			..
		}) => return Err(Error::new(span, "Can only be used on structs")),
	})
}

#[derive(Debug)]
struct MyField {
	written_name: Ident,
	ty: syn::Type,
	renamed_link: syn::Expr,
}

#[derive(FromMeta)]
struct FieldAttrs {
	rename: FieldRename,
}

/// Attrs to parse
#[derive(Clone, FromMeta)]
struct FieldRename {
	#[darling(rename = "expr")]
	renamed_link: syn::LitStr,
}

fn extract_field_info(fields: Fields) -> Result<Vec<MyField>, Error> {
	match fields {
		Fields::Unit => Err(Error::new(Span::call_site(), "Only works on named fields")),
		Fields::Named(fields) => {
			let mut ret = Vec::new();
			for field in fields.named.iter() {
				let written_name = field.ident.as_ref().unwrap();

				let mut valid_attrs: Option<FieldAttrs> = None;
				for attr_line in field.attrs.iter() {
					let meta = &attr_line.meta;
					let my_attrs = FieldAttrs::from_meta(meta)?;
					match valid_attrs {
						Some(_prev) => {
							// todo: propagate prev span
							return Err(Error::new(Span::call_site(), "Cannot rename twice"));
						}
						None => {
							valid_attrs = Some(my_attrs);
						}
					}
				}

				ret.push(MyField {
					written_name: written_name.clone(),
					ty: field.ty.clone(),
					renamed_link: valid_attrs
						.ok_or(Error::new(
							written_name.span(),
							"Must provide a rename(fn) attr",
						))?
						.rename
						.renamed_link
						.parse()?,
				})
			}
			Ok(ret)
		}
		Fields::Unnamed(fields) => Err(Error::new(
			fields.paren_token.span.join(),
			"Only works on named fields",
		)),
	}
}
