// #[proc_macro_derive(Serialize, attributes(serde, bar))]
// pub fn payload(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let input = parse_macro_input!(tokens as DeriveInput);
//   payload_impl(input)
//     .unwrap_or_else(syn::Error::into_compile_error)
//     .into()
// }

use syn::token::{Enum, Token, Union};
use syn::{Data, DataEnum, DataUnion, Error};

use syn::{DeriveInput, parse_macro_input};

#[allow(unreachable_code)]
// https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs
fn payload_impl(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let name = input.ident;

  Ok(match input.data {
    syn::Data::Struct(data) => {
      let fields = data.fields;

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
