use crate::lir::LirTokenStream;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirToken {
	Uint(u32),
	Sint(i32),
	String(String),
	QuoteString(String),
	ParenString(String),
	BraceString(String),
	BracketString(String),
}

impl MirToken {
	pub fn lower(&self) -> LirTokenStream {
		LirTokenStream::try_new(self.to_string().as_str()).unwrap()
	}

	#[inline]
	pub fn str(s: &str) -> Self {
		Self::String(String::from(s))
	}

	#[inline]
	pub fn quote(s: &str) -> Self {
		Self::QuoteString(String::from(s))
	}

	#[inline]
	pub fn paren(s: &str) -> Self {
		Self::ParenString(String::from(s))
	}

	#[inline]
	pub fn brace(s: &str) -> Self {
		Self::BraceString(String::from(s))
	}

	#[inline]
	pub fn bracket(s: &str) -> Self {
		Self::BracketString(String::from(s))
	}
}

impl Display for MirToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let mut buffer = String::new();
		buffer.push_str(
			match self {
				MirToken::Uint(u) => u.to_string(),
				MirToken::Sint(s) => s.to_string(),
				MirToken::String(s) => s.to_string(),
				MirToken::QuoteString(s) => format!("\"{}\"", s),
				MirToken::ParenString(s) => format!("({})", s),
				MirToken::BraceString(s) => format!("{{{}}}", s),
				MirToken::BracketString(s) => format!("[{}]", s),
			}
			.as_str(),
		);

		write!(f, "{}", buffer)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirTokenStream {
	pub tokens: Vec<MirToken>,
}

impl MirTokenStream {
	pub const fn new(tokens: Vec<MirToken>) -> Self {
		Self { tokens }
	}
}

impl Display for MirTokenStream {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let mut buffer = String::new();
		for t in &self.tokens {
			buffer.push_str(t.to_string().as_str())
		}

		write!(f, "{}", buffer)
	}
}

#[cfg(test)]
mod tests_mir_token {
	use super::*;
	use crate::lir::LirToken;

	#[test]
	fn test_lower_ints() {
		assert_eq!(
			MirToken::Uint(0u32).lower(),
			LirTokenStream::try_new("0").unwrap()
		);
		assert_eq!(
			MirToken::Sint(0i32).lower(),
			LirTokenStream::try_new("0").unwrap()
		);
	}

	#[test]
	fn test_lower_quotes() {
		assert_eq!(
			MirToken::quote("hello").lower(),
			LirTokenStream::from(vec![
				LirToken::Quote,
				LirToken::Alpha('h'),
				LirToken::Alpha('e'),
				LirToken::Alpha('l'),
				LirToken::Alpha('l'),
				LirToken::Alpha('o'),
				LirToken::Quote,
			])
		);
	}

	#[test]
	fn test_lower_parens() {
		assert_eq!(
			MirToken::paren("hello").lower(),
			LirTokenStream::from(vec![
				LirToken::OpenParen,
				LirToken::Alpha('h'),
				LirToken::Alpha('e'),
				LirToken::Alpha('l'),
				LirToken::Alpha('l'),
				LirToken::Alpha('o'),
				LirToken::CloseParen,
			])
		);
	}

	#[test]
	fn test_lower_brace() {
		assert_eq!(
			MirToken::brace("hello").lower(),
			LirTokenStream::from(vec![
				LirToken::OpenBrace,
				LirToken::Alpha('h'),
				LirToken::Alpha('e'),
				LirToken::Alpha('l'),
				LirToken::Alpha('l'),
				LirToken::Alpha('o'),
				LirToken::CloseBrace,
			])
		);
	}

	#[test]
	fn test_lower_brackets() {
		assert_eq!(
			MirToken::bracket("hello").lower(),
			LirTokenStream::from(vec![
				LirToken::OpenBracket,
				LirToken::Alpha('h'),
				LirToken::Alpha('e'),
				LirToken::Alpha('l'),
				LirToken::Alpha('l'),
				LirToken::Alpha('o'),
				LirToken::CloseBracket,
			])
		);
	}

	#[test]
	fn test_str_methods() {
		assert_eq!(MirToken::str(""), MirToken::String(String::new()));
		assert_eq!(
			MirToken::str("hello"),
			MirToken::String(String::from("hello"))
		);

		assert_eq!(MirToken::paren(""), MirToken::ParenString(String::new()));
		assert_eq!(
			MirToken::paren("hello"),
			MirToken::ParenString(String::from("hello"))
		);

		assert_eq!(MirToken::brace(""), MirToken::BraceString(String::new()));
		assert_eq!(
			MirToken::brace("hello"),
			MirToken::BraceString(String::from("hello"))
		);

		assert_eq!(
			MirToken::bracket(""),
			MirToken::BracketString(String::new())
		);
		assert_eq!(
			MirToken::bracket("hello"),
			MirToken::BracketString(String::from("hello"))
		);
	}
}
