use syn::{ExprClosure, Ident, LitStr, Token, Visibility, braced, parse::{Parse, ParseStream}, punctuated::Punctuated};

#[derive(Debug)]
pub struct LexerTokenMacroInput {
	pub vis: Visibility,
	pub mod_name: Ident,
	pub double_colons: Token![::],
	pub struct_name: Ident,
	pub lexing_directives: Punctuated<LexingDirectiveStatement, Token![,]>,
}

#[derive(Debug)]
pub struct LexingDirectiveStatement {
	pub ident: Ident,
	pub ignore: bool,
	pub colon: Token![:],
	pub lexing_directive: LexingDirective,
}

#[derive(Debug)]
pub enum LexingDirective {
	RegEx(LexingDirectiveRegEx),
	Closure(LexingDirectiveClosure),
}

#[derive(Debug)]
pub struct LexingDirectiveRegEx {
	pub value: LitStr,
}

#[derive(Debug)]
pub struct LexingDirectiveClosure {
	pub closure: ExprClosure,
}

impl Parse for LexerTokenMacroInput {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let content;
		let vis = input.parse()?;
		let mod_name = input.parse()?;
		let double_colons = input.parse()?;
		let struct_name = input.parse()?;
		let _ = braced!(content in input);
		let lexing_directives = content.parse_terminated(LexingDirectiveStatement::parse)?;
		Ok(LexerTokenMacroInput {
			vis,
			mod_name,
			double_colons,
			struct_name,
			lexing_directives,
		})
	}
}

impl Parse for LexingDirectiveStatement {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(LexingDirectiveStatement {
			ident: input.parse()?,
			ignore: input.parse::<Option<Ident>>()?.map_or_else(|| false, |i| i.to_string() == "ignore"),
			colon: input.parse()?,
			lexing_directive: input.parse()?,
		})
	}
}

impl Parse for LexingDirective {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(if input.peek(LitStr) {
			LexingDirective::RegEx(input.parse()?)
		} else {
			LexingDirective::Closure(input.parse()?)
		})
	}
}

impl Parse for LexingDirectiveRegEx {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(LexingDirectiveRegEx {
			value: input.parse()?,
		})
	}
}

impl Parse for LexingDirectiveClosure {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(LexingDirectiveClosure {
			closure: input.parse()?,
		})
	}
}