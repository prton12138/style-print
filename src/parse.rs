use proc_macro2::TokenStream as TokenStream2;
use syn::Token;
use crate::style::styles_find;
//use syn::Ident;
use syn::spanned::Spanned;
use quote::{quote,ToTokens};


pub struct StyleListParser {
	styles_str:String
} 

impl syn::parse::Parse for StyleListParser {
	fn parse(input:syn::parse::ParseStream) -> syn::Result<Self> {
		let expr_closeure = input.parse::<syn::ExprClosure>()?;
		//println!("{expr_closeure:#?}");
		let mut style_punctuated_str = Vec::new();
		for ident in expr_closeure.inputs.iter() {
			if let syn::Pat::Ident(ref i) = ident {
				let one_style = styles_find(i)?;
				style_punctuated_str.push(one_style);
			}
		}
		if let syn::Expr::Lit(syn::ExprLit {
            		lit:syn::Lit::Str(
            			ref lit_str,..
            		),..
		}) = expr_closeure.body.as_ref() {
			Ok(Self{
				styles_str:format!("\x1b[{}m{}\x1b[0m",style_punctuated_str.join(";"),lit_str.token()).to_string()
			})
		} else {
			Err(syn::Error::new(expr_closeure.span(),"not a literal".to_string()))
		}

	}
}

pub struct StyleSingleParser {
	style_list:StyleListParser,
	expr_list:Vec<syn::Expr>
}

impl syn::parse::Parse for StyleSingleParser {
	fn parse(input:syn::parse::ParseStream) -> syn::Result<Self> {
		let mut style_list = input.parse::<StyleListParser>()?;
		let mut expr_list = Vec::new();
		loop {
			if let Err(_) = input.parse::<Token!(,)>() {
				break;
			}
			let expr =  input.parse::<syn::Expr>()?;
			expr_list.push(expr);
		};
		if let Ok(_) = input.parse::<Token!(;)>() {
			style_list.styles_str += "\n";
		}
		Ok(Self{
			style_list:style_list,
			expr_list:expr_list
		})
	}
}

pub struct StyleMultiParser {
	style_lists:Box<Vec<StyleListParser>>,
	expr_list:Box<Vec<syn::Expr>>
}



impl syn::parse::Parse for StyleMultiParser {
	fn parse(input:syn::parse::ParseStream) -> syn::Result<Self> {
		let mut style_lists = Vec::new();
		let mut expr_list = Vec::new();
		loop {
			let single = input.parse::<StyleSingleParser>();
			match single {
				Ok(mut s) => {
					style_lists.push(s.style_list);
					expr_list.append(&mut s.expr_list);
				}
				Err(_) => {
					break;
				}
			}
		}
		Ok(Self{
			style_lists:Box::new(style_lists),
			expr_list:Box::new(expr_list)
		})
	}
}

impl ToTokens for StyleMultiParser {
	fn to_tokens(&self, ts: &mut TokenStream2) {
		let literal_vec:Vec<_> = self.style_lists.iter().map(|l|l.styles_str.clone()).collect();
		let literal = literal_vec.join("");
		let args = self.expr_list.as_ref();
		ts.extend(quote!(
			#literal #(,#args)*
		));
	}
}
