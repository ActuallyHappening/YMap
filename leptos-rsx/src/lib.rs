// #[proc_macro_derive(Serialize, attributes(serde))]
// pub fn ser(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let input = parse_macro_input!(tokens as DeriveInput);
//   payload_impl(input, Do::Serialize)
//     .unwrap_or_else(syn::Error::into_compile_error)
//     .into()
// }

use syn::parse_macro_input;

#[proc_macro]
pub fn rsx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as RsxData);
	std::fs::write(
		"/home/ah/Desktop/YMap/leptos-rsx/output.rsn",
		format!("{:#?}", input),
	)
	.unwrap();

	panic!("{:?}", input)
}

// fn rsx(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
//   let input: RsxData = syn::parse::Parse::parse(input)?;
//   todo!()
// }

#[derive(Debug)]
struct RsxData(dioxus_rsx::CallBody);

impl syn::parse::Parse for RsxData {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let body = input.parse()?;
		Ok(RsxData(body))
	}
}
