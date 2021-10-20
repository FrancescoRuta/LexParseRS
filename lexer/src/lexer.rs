use std::marker::PhantomData;

pub use once_cell::unsync::Lazy as Lazy;
pub use regex::Regex as Regex;

pub struct LexerError(pub String);

pub trait Tokenize where Self: Sized {
	fn tokenize(lexer: &Lexer<Self>) -> Result<Option<Self>, LexerError>;
}

pub struct Lexer<T: Tokenize> {
	t: PhantomData<T>,
}

impl<T: Tokenize> Lexer<T> {
	
	pub fn new() -> Self {
		Lexer {
			t: PhantomData::default(),
		}
	}
	pub fn read_next_token(&self) -> Result<T, LexerError> {
		todo!()
	}
	
}