use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use crate::parse::StyleMultiParser;
pub fn do_expand(input:TokenStream) -> syn::Result<TokenStream2> {
	let style_multi_parse:StyleMultiParser = syn::parse(input)?; 
	Ok(quote!(
		#style_multi_parse
	))
}
