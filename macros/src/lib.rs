use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::{__private::Span, quote};
use syn::Ident;

use crate::{ast::{LexerTokenMacroInput, LexingDirectiveStatement}};

mod ast;

#[proc_macro]
pub fn lexer_token(ts: TokenStream) -> TokenStream {
	let input: LexerTokenMacroInput = syn::parse(ts).unwrap();
	let this_crate: Ident = {
		let found_crate = crate_name("macros_wrapper").unwrap();
		match found_crate {
			FoundCrate::Itself => Ident::new("crate", Span::call_site()),
			FoundCrate::Name(name) => Ident::new(&name, Span::call_site())
		}
	};
	
	let LexerTokenMacroInput {
		vis,
		mod_name,
		double_colons: _,
		struct_name,
		lexing_directives,
	} = input;
	
	let lexing_directive_names: Vec<_> = lexing_directives.iter().filter(|d| !d.ignore).map(|d| d.ident.clone()).collect();
	
	let structs_declaration = lexing_directives.iter().filter(|d| !d.ignore).map(|
		LexingDirectiveStatement {
			ident,
			ignore: _,
			colon: _,
			lexing_directive,
		}
	| {
		let _ = lexing_directive;
		quote! {
			pub struct #ident {
				raw: std::string::String,
				span: #this_crate::span::Span,
			}
			impl #ident {
				pub fn raw(&self) -> &std::string::String {
					&self.raw
				}
				pub fn span(&self) -> &#this_crate::span::Span {
					&self.span
				}
			}
			impl #this_crate::parser::Parse<#struct_name> for #ident {
				fn parse(ts: &mut #this_crate::parser::Parser<Token>) -> #this_crate::parser::Result<Self> {
					todo!()
				}
			}
		}
	});
	
	let result = quote! {
		#vis mod #mod_name {
			pub enum #struct_name {
				#(#lexing_directive_names(#lexing_directive_names)),*
			}
			impl #struct_name {
				pub fn raw(&self) -> &std::string::String {
					match self {
						#(Self::#lexing_directive_names(v) => v.raw()),*
					}
				}
				pub fn span(&self) -> &#this_crate::span::Span {
					match self {
						#(Self::#lexing_directive_names(v) => v.span()),*
					}
				}
			}
			#(#structs_declaration)*
		}
	};
	
	println!("CODE: \n-----------------\n{}\n-----------------\n", result.to_string());
	
	result.into()
}
