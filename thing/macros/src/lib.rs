// #[proc_macro_derive(Serialize, attributes(serde, bar))]
// pub fn payload(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let input = parse_macro_input!(tokens as DeriveInput);
//   payload_impl(input)
//     .unwrap_or_else(syn::Error::into_compile_error)
//     .into()
// }

use darling::{FromDeriveInput, FromMeta};
use proc_macro2::Span;
use syn::token::{Enum, Token, Union};
use syn::{Data, DataEnum, DataUnion, Error, Fields, Ident};

use syn::{DeriveInput, parse_macro_input};

#[allow(unreachable_code)]
// https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs
fn payload_impl(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let name = input.ident;

  Ok(match input.data {
    syn::Data::Struct(data) => {
      let fields = extract_field_info(data.fields)?;

      todo!();
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

struct MyField {
  written_name: Ident,
  renamed_link: syn::LitStr,
}

#[derive(FromMeta)]
struct ValidAttrs {
  rename: RenameAttr,
}

/// Attrs to parse
#[derive(Clone, FromMeta)]
struct RenameAttr {
  #[darling(rename = "fn")]
  renamed_link: syn::LitStr,
}

fn extract_field_info(fields: Fields) -> Result<Vec<MyField>, Error> {
  match fields {
    Fields::Unit => Err(Error::new(Span::call_site(), "Only works on named fields")),
    Fields::Named(fields) => {
      let mut ret = Vec::new();
      for field in fields.named.iter() {
        let written_name = field.ident.as_ref().unwrap();

        let mut valid_attrs: Option<ValidAttrs> = None;
        for attr_line in field.attrs.iter() {
          let meta = &attr_line.meta;
          let my_attrs = ValidAttrs::from_meta(meta)?;
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
          renamed_link: valid_attrs
            .ok_or(Error::new(
              written_name.span(),
              "Must provide a rename(fn) attr",
            ))?
            .rename
            .renamed_link,
        })
      }
      Ok(ret)
    }
    Fields::Unnamed(fields) => Err(Error::new(Span::call_site(), "Only works on named fields")),
  }
}
