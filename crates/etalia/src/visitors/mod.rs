use crate::lir::LirTokenStream;
use crate::mir::MirTokenStream;

mod int_lit_visitor;
mod bidi_delim_strings_visitor;
mod punct_terminal_visitor;
mod quote_string_visitor;
pub use int_lit_visitor::IntLitVisitor;
pub use bidi_delim_strings_visitor::BidiDelimStringsVisitor;
pub use punct_terminal_visitor::PunctTerminalVisitor;
pub use quote_string_visitor::QuoteStringVisitor;

/// Visitor pass to convert a [LirTokenStream] to a [MirTokenStream]
pub trait LirRaiseVisitor {
	fn visit(&self, tokens: LirTokenStream) -> MirTokenStream;
}

/// Visitor for MIR optimization passes
pub trait MirOptVisitor {
	fn visit(&self, tokens: &mut MirTokenStream);
}
