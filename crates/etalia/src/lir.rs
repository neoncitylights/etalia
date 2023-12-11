use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LirToken {
	Comma,
	Colon,
	Semicolon,
	Apostrophe,
	Question,
	Exclamation,
	Period,
	Quote,
	OpenParen,
	OpenBrace,
	OpenBracket,
	CloseParen,
	CloseBrace,
	CloseBracket,
	Slash,
	Hyphen,
	Wsp(char),
	Alpha(char),
	Digit(char),
}

impl LirToken {
	pub fn try_new(c: char) -> Option<Self> {
		match c {
			',' => Some(Self::Comma),
			':' => Some(Self::Colon),
			';' => Some(Self::Semicolon),
			'\'' => Some(Self::Apostrophe),
			'?' => Some(Self::Question),
			'!' => Some(Self::Exclamation),
			'.' => Some(Self::Period),
			'"' => Some(Self::Quote),
			'(' => Some(Self::OpenParen),
			'{' => Some(Self::OpenBrace),
			'[' => Some(Self::OpenBracket),
			')' => Some(Self::CloseParen),
			'}' => Some(Self::CloseBrace),
			']' => Some(Self::CloseBracket),
			'/' => Some(Self::Slash),
			'-' => Some(Self::Hyphen),
			c if c.is_whitespace() => Some(Self::Wsp(c)),
			c if c.is_alphabetic() => Some(Self::Alpha(c)),
			c if c.is_numeric() => Some(Self::Digit(c)),
			_ => None,
		}
	}

	pub fn as_char(&self) -> char {
		match self {
			LirToken::Comma => ',',
			LirToken::Colon => ':',
			LirToken::Semicolon => ';',
			LirToken::Apostrophe => '\'',
			LirToken::Question => '?',
			LirToken::Exclamation => '!',
			LirToken::Period => '.',
			LirToken::Quote => '"',
			LirToken::OpenParen => '(',
			LirToken::OpenBrace => '{',
			LirToken::OpenBracket => '[',
			LirToken::CloseParen => ')',
			LirToken::CloseBrace => '}',
			LirToken::CloseBracket => ']',
			LirToken::Slash => '/',
			LirToken::Hyphen => '-',
			LirToken::Wsp(c) | LirToken::Alpha(c) | LirToken::Digit(c) => *c,
		}
	}

	pub const fn is_term_punct(&self) -> bool {
		matches!(self, Self::Question | Self::Exclamation | Self::Period)
	}

	pub const fn is_nonterm_punct(&self) -> bool {
		matches!(
			self,
			Self::Comma
				| Self::Colon | Self::Semicolon | Self::Apostrophe
				| Self::Hyphen
		)
	}

	pub const fn is_open_delim(&self) -> bool {
		matches!(self, Self::OpenParen | Self::OpenBrace | Self::OpenBracket)
	}

	pub const fn is_close_delim(&self) -> bool {
		matches!(
			self,
			Self::CloseParen | Self::CloseBrace | Self::CloseBracket
		)
	}

	pub const fn is_matching_delims(&self, closing: &LirToken) -> bool {
		match (self, closing) {
			(Self::Quote, Self::Quote)
			| (Self::OpenParen, Self::CloseParen)
			| (Self::OpenBrace, Self::CloseBrace)
			| (Self::OpenBracket, Self::CloseBracket) => true,
			_ => false,
		}
	}
}

impl From<LirToken> for char {
	fn from(t: LirToken) -> char {
		t.as_char()
	}
}

impl TryFrom<char> for LirToken {
	type Error = ();

	fn try_from(c: char) -> Result<Self, Self::Error> {
		Self::try_new(c).ok_or(())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LirTokenStream {
	pub tokens: Vec<LirToken>,
}

impl LirTokenStream {
	pub fn new(tokens: Vec<LirToken>) -> Self {
		Self { tokens }
	}

	pub fn try_new(s: &str) -> Result<Self, (usize, char)> {
		let mut tokens = Vec::new();
		for (idx, c) in s.chars().enumerate() {
			if let Some(token) = LirToken::try_new(c) {
				tokens.push(token);
			} else {
				return Err((idx, c));
			}
		}
		Ok(Self::new(tokens))
	}

	pub fn as_slice(&self) -> &[LirToken] {
		self.tokens.as_slice()
	}
}

impl From<Vec<LirToken>> for LirTokenStream {
	fn from(v: Vec<LirToken>) -> Self {
		Self::new(v)
	}
}

impl Display for LirTokenStream {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let mut buffer = String::with_capacity(self.tokens.len());
		for t in &self.tokens {
			buffer.push(t.as_char());
		}

		write!(f, "{}", buffer)
	}
}

#[cfg(test)]
mod tests_lir_token {
	use super::*;

	#[test]
	fn test_try_new() {
		assert_eq!(LirToken::try_new(','), Some(LirToken::Comma));
		assert_eq!(LirToken::try_new(':'), Some(LirToken::Colon));
		assert_eq!(LirToken::try_new(';'), Some(LirToken::Semicolon));
		assert_eq!(LirToken::try_new('\''), Some(LirToken::Apostrophe));
		assert_eq!(LirToken::try_new('?'), Some(LirToken::Question));
		assert_eq!(LirToken::try_new('!'), Some(LirToken::Exclamation));
		assert_eq!(LirToken::try_new('.'), Some(LirToken::Period));
		assert_eq!(LirToken::try_new('"'), Some(LirToken::Quote));
		assert_eq!(LirToken::try_new('('), Some(LirToken::OpenParen));
		assert_eq!(LirToken::try_new(')'), Some(LirToken::CloseParen));
		assert_eq!(LirToken::try_new('['), Some(LirToken::OpenBracket));
		assert_eq!(LirToken::try_new(']'), Some(LirToken::CloseBracket));
		assert_eq!(LirToken::try_new('{'), Some(LirToken::OpenBrace));
		assert_eq!(LirToken::try_new('}'), Some(LirToken::CloseBrace));
		assert_eq!(LirToken::try_new('/'), Some(LirToken::Slash));
		assert_eq!(LirToken::try_new('-'), Some(LirToken::Hyphen));
		assert_eq!(LirToken::try_new('a'), Some(LirToken::Alpha('a')));
		assert_eq!(LirToken::try_new('A'), Some(LirToken::Alpha('A')));
		assert_eq!(LirToken::try_new('0'), Some(LirToken::Digit('0')));
		assert_eq!(LirToken::try_new(' '), Some(LirToken::Wsp(' ')));
	}

	#[test]
	fn test_as_char() {
		assert_eq!(LirToken::Comma.as_char(), ',');
		assert_eq!(LirToken::Colon.as_char(), ':');
		assert_eq!(LirToken::Semicolon.as_char(), ';');
		assert_eq!(LirToken::Apostrophe.as_char(), '\'');
		assert_eq!(LirToken::Question.as_char(), '?');
		assert_eq!(LirToken::Exclamation.as_char(), '!');
		assert_eq!(LirToken::Period.as_char(), '.');
		assert_eq!(LirToken::Quote.as_char(), '"');
		assert_eq!(LirToken::OpenParen.as_char(), '(');
		assert_eq!(LirToken::CloseParen.as_char(), ')');
		assert_eq!(LirToken::OpenBracket.as_char(), '[');
		assert_eq!(LirToken::CloseBracket.as_char(), ']');
		assert_eq!(LirToken::OpenBrace.as_char(), '{');
		assert_eq!(LirToken::CloseBrace.as_char(), '}');
		assert_eq!(LirToken::Slash.as_char(), '/');
		assert_eq!(LirToken::Hyphen.as_char(), '-');
		assert_eq!(LirToken::Alpha('a').as_char(), 'a');
		assert_eq!(LirToken::Alpha('A').as_char(), 'A');
		assert_eq!(LirToken::Digit('0').as_char(), '0');
		assert_eq!(LirToken::Wsp(' ').as_char(), ' ');
	}

	#[test]
	fn test_is_term_punct() {
		assert!(LirToken::Question.is_term_punct());
		assert!(LirToken::Exclamation.is_term_punct());
		assert!(LirToken::Period.is_term_punct());
	}

	#[test]
	fn test_is_nonterm_punct() {
		assert!(LirToken::Comma.is_nonterm_punct());
		assert!(LirToken::Colon.is_nonterm_punct());
		assert!(LirToken::Semicolon.is_nonterm_punct());
		assert!(LirToken::Apostrophe.is_nonterm_punct());
		assert!(LirToken::Hyphen.is_nonterm_punct());
	}

	#[test]
	fn test_is_open_delim() {
		assert!(LirToken::OpenParen.is_open_delim());
		assert!(LirToken::OpenBrace.is_open_delim());
		assert!(LirToken::OpenBracket.is_open_delim());
	}

	#[test]
	fn test_is_close_delim() {
		assert!(LirToken::CloseParen.is_close_delim());
		assert!(LirToken::CloseBrace.is_close_delim());
		assert!(LirToken::CloseBracket.is_close_delim());
	}

	#[test]
	fn test_is_matching_delims() {
		assert!(LirToken::Quote.is_matching_delims(&LirToken::Quote));
		assert!(LirToken::OpenParen.is_matching_delims(&LirToken::CloseParen));
		assert!(LirToken::OpenBrace.is_matching_delims(&LirToken::CloseBrace));
		assert!(LirToken::OpenBracket.is_matching_delims(&LirToken::CloseBracket));
	}

	#[test]
	fn test_impl_from_lir_token_for_char() {
		assert_eq!(char::from(LirToken::Comma), ',');
		assert_eq!(char::from(LirToken::Colon), ':');
		assert_eq!(char::from(LirToken::Semicolon), ';');
		assert_eq!(char::from(LirToken::Apostrophe), '\'');
		assert_eq!(char::from(LirToken::Question), '?');
		assert_eq!(char::from(LirToken::Exclamation), '!');
		assert_eq!(char::from(LirToken::Period), '.');
		assert_eq!(char::from(LirToken::Quote), '"');
		assert_eq!(char::from(LirToken::OpenParen), '(');
		assert_eq!(char::from(LirToken::CloseParen), ')');
		assert_eq!(char::from(LirToken::OpenBracket), '[');
		assert_eq!(char::from(LirToken::CloseBracket), ']');
		assert_eq!(char::from(LirToken::OpenBrace), '{');
		assert_eq!(char::from(LirToken::CloseBrace), '}');
		assert_eq!(char::from(LirToken::Slash), '/');
		assert_eq!(char::from(LirToken::Hyphen), '-');
		assert_eq!(char::from(LirToken::Alpha('a')), 'a');
		assert_eq!(char::from(LirToken::Alpha('A')), 'A');
		assert_eq!(char::from(LirToken::Digit('0')), '0');
		assert_eq!(char::from(LirToken::Wsp(' ')), ' ');
	}
}

#[cfg(test)]
mod tests_lir_token_stream {
	use super::*;

	#[test]
	fn test_try_new_empty() {
		let stream = LirTokenStream::try_new("");
		assert!(stream.is_ok_and(|s| s.tokens.is_empty()));
	}

	#[test]
	fn test_try_new_1() {
		let stream = LirTokenStream::try_new("abc");
		assert!(stream.is_ok_and(|s| s.tokens
			== vec![
				LirToken::Alpha('a'),
				LirToken::Alpha('b'),
				LirToken::Alpha('c')
			]));
	}

	#[test]
	fn test_as_slice() {
		let stream = LirTokenStream::try_new("abc");
		assert!(stream.is_ok());

		let stream = stream.unwrap();
		assert_eq!(
			stream.as_slice(),
			&[
				LirToken::Alpha('a'),
				LirToken::Alpha('b'),
				LirToken::Alpha('c')
			]
		);
	}

	#[test]
	fn test_impl_display() {
		let stream = LirTokenStream::try_new("abc");
		assert!(stream.is_ok());

		let stream = stream.unwrap();
		assert_eq!(format!("{}", stream), "abc");
	}

	#[test]
	fn test_impl_from_vec() {
		let tokens = vec![
			LirToken::Alpha('a'),
			LirToken::Alpha('b'),
			LirToken::Alpha('c'),
		];

		assert_eq!(LirTokenStream::from(tokens.clone()).tokens, tokens.clone());
	}
}
