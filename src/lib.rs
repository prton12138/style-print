use proc_macro::TokenStream;
mod style;
mod parse;
mod expand;
use quote::quote;
use expand::do_expand;

#[proc_macro]
pub fn style_print(input:TokenStream) -> TokenStream {
	match do_expand(input) {
		Ok(c) => quote!(print!(#c)).into(),
		Err(e) => e.to_compile_error().into()
	}
}



