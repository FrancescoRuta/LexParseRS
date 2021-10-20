use std::{cell::RefCell, fmt::Display, rc::Rc, sync::{Arc, Mutex, RwLock}};

use shared::span::{Span, Spanned};

#[derive(Clone)]
pub struct Parser<Token> {
	tokens: Rc<Vec<Token>>,
	offset: usize,
}

pub struct Error {
	message: String,
	span: Option<Span>,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.message)?;
		if let Some(span) = &self.span {
			f.write_str(&span.to_string_start())?;
		}
		Ok(())
	}
}

impl Error {
	pub fn new<T>(message: String, span: Option<Span>) -> Result<T> {
		Err(Error {
			message,
			span,
		})
	}
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Parse<Token> where Self: Sized {
	fn parse(ts: &mut Parser<Token>) -> Result<Self>;
}

impl<Token: Spanned> Parser<Token> {
	
	pub fn new(tokens: Rc<Vec<Token>>) -> Self {
		Parser {
			tokens,
			offset: 0,
		}
	}
	
	pub fn parse_to_eof<P: Parse<Token>>(&mut self) -> Result<P> {
		let offset = self.offset;
		let r = P::parse(self);
		let r = if self.offset < self.tokens.len() {
			Error::new("Expected EOF".to_string(), Some(self.tokens[self.offset].span().clone()))
		} else {
			r
		};
		if r.is_err() {
			self.offset = offset;
		}
		r
	}
	
	pub fn parse<P: Parse<Token>>(&mut self) -> Result<P> {
		let offset = self.offset;
		let r = P::parse(self);
		if r.is_err() {
			self.offset = offset;
		}
		r
	}
	
	pub fn peek<P: Parse<Token>>(&mut self) -> bool {
		let offset = self.offset;
		let r = P::parse(self);
		self.offset = offset;
		r.is_ok()
	}
	
	pub fn get_next_token(&mut self) -> Option<&Token> {
		let r = self.tokens.get(self.offset)?;
		self.offset += 1;
		Some(r)
	}
	
}


impl<Token: Spanned, T> Parse<Token> for Vec<T> where T: Parse<Token> {
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		let mut v = Vec::new();
		while let Ok(r) = ts.parse() {
			v.push(r);
		}
		Ok(v)
	}
}

impl<Token: Spanned, T> Parse<Token> for Option<T> where T: Parse<Token> {
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok(ts.parse().ok())
	}
}

impl<Token: Spanned, T> Parse<Token> for Rc<T> where T: Parse<Token> {
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok(Rc::new(ts.parse()?))
	}
}

impl<Token: Spanned, T> Parse<Token> for RefCell<T> where T: Parse<Token> {
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok(RefCell::new(ts.parse()?))
	}
}

impl<Token: Spanned, T> Parse<Token> for Arc<T> where T: Parse<Token> {
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok(Arc::new(ts.parse()?))
	}
}

impl<Token: Spanned, T> Parse<Token> for Mutex<T> where T: Parse<Token> {
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok(Mutex::new(ts.parse()?))
	}
}

impl<Token: Spanned, T> Parse<Token> for RwLock<T> where T: Parse<Token> {
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok(RwLock::new(ts.parse()?))
	}
}

impl<Token: Spanned, T0> Parse<Token> for (T0,)
where
	T0: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1> Parse<Token> for (T0, T1)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1, T2> Parse<Token> for (T0, T1, T2)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
	T2: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1, T2, T3> Parse<Token> for (T0, T1, T2, T3)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
	T2: Parse<Token>,
	T3: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1, T2, T3, T4> Parse<Token> for (T0, T1, T2, T3, T4)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
	T2: Parse<Token>,
	T3: Parse<Token>,
	T4: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1, T2, T3, T4, T5> Parse<Token> for (T0, T1, T2, T3, T4, T5)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
	T2: Parse<Token>,
	T3: Parse<Token>,
	T4: Parse<Token>,
	T5: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1, T2, T3, T4, T5, T6> Parse<Token> for (T0, T1, T2, T3, T4, T5, T6)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
	T2: Parse<Token>,
	T3: Parse<Token>,
	T4: Parse<Token>,
	T5: Parse<Token>,
	T6: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1, T2, T3, T4, T5, T6, T7> Parse<Token> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
	T2: Parse<Token>,
	T3: Parse<Token>,
	T4: Parse<Token>,
	T5: Parse<Token>,
	T6: Parse<Token>,
	T7: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1, T2, T3, T4, T5, T6, T7, T8> Parse<Token> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
	T2: Parse<Token>,
	T3: Parse<Token>,
	T4: Parse<Token>,
	T5: Parse<Token>,
	T6: Parse<Token>,
	T7: Parse<Token>,
	T8: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Parse<Token> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
	T2: Parse<Token>,
	T3: Parse<Token>,
	T4: Parse<Token>,
	T5: Parse<Token>,
	T6: Parse<Token>,
	T7: Parse<Token>,
	T8: Parse<Token>,
	T9: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
		))
	}
}

impl<Token: Spanned, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Parse<Token> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
	T0: Parse<Token>,
	T1: Parse<Token>,
	T2: Parse<Token>,
	T3: Parse<Token>,
	T4: Parse<Token>,
	T5: Parse<Token>,
	T6: Parse<Token>,
	T7: Parse<Token>,
	T8: Parse<Token>,
	T9: Parse<Token>,
	T10: Parse<Token>,
{
	fn parse(ts: &mut Parser<Token>) -> Result<Self> {
		Ok((
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
			ts.parse()?,
		))
	}
}