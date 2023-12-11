use crate::lir::LirToken;
use crate::mir::MirTokenStream;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
	pub kind: ParseErrorKind,
	pub idx: usize,
}

impl ParseError {
	pub fn new(kind: ParseErrorKind, idx: usize) -> Self {
		Self { kind, idx }
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
	Date(chrono::ParseError),
	UnbalancedDelim(MirTokenStream, LirToken),
	UnrecognizedSymbol(char),
}
