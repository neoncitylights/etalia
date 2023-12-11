use crate::lir::{LirToken, LirTokenStream};
use crate::mir::{MirToken, MirTokenStream};
use crate::visitors::LirRaiseVisitor;

/// LIR to MIR pass to convert bidi-delimited strings to [MirToken::ParenString], [MirToken::BraceString], and [MirToken::BracketString]
#[derive(Debug, Clone)]
pub struct BidiDelimStringsVisitor;

impl BidiDelimStringsVisitor {
	pub(crate) fn on_close_delim(
		&self,
		delims: &mut Vec<LirToken>,
		mir: &mut Vec<MirToken>,
		is_open: &mut bool,
		lir_open: LirToken,
		mir_string: MirToken,
		mir_string_buf: &mut String,
		error_msg: &str,
	) {
		if delims.last() != Some(&lir_open) {
			println!("{}", error_msg);
		} else {
			delims.pop();
			mir.push(mir_string);
			mir_string_buf.clear();
			*is_open = false;
		}
	}
}

impl LirRaiseVisitor for BidiDelimStringsVisitor {
	fn visit(&self, stream: LirTokenStream) -> MirTokenStream {
		let lir_slice = stream.as_slice();

		let mut is_open = false;
		let mut mir: Vec<MirToken> = Vec::new();
		let mut delims: Vec<LirToken> = Vec::new();
		let mut delim_buffer = String::new();
		let mut other_buffer = String::new();

		// store indices for the delimited buffer and non-delimited buffer,
		// instead of using a String and constantly reallocate memory
		// by pushing and clearing the buffer
		let mut delim_buffer_idx_start = 0usize;
		let mut delim_buffer_idx_end = 0usize;
		let mut other_buffer_idx_start = 0usize;
		let mut other_buffer_idx_end = 0usize;

		for (idx, token) in lir_slice.iter().enumerate() {
			match token {
				LirToken::OpenParen
				| LirToken::OpenBrace
				| LirToken::OpenBracket => {
					delims.push(*token);
					is_open = true;
					mir.push(MirToken::String(other_buffer.clone()));
					other_buffer.clear();
				}
				LirToken::CloseParen => {
					if delims.last() != Some(&LirToken::OpenParen) {
						println!("1");
					} else {
						delims.pop();
						mir.push(MirToken::ParenString(
							delim_buffer.clone(),
						));
						delim_buffer.clear();
						is_open = false;
					}
				}
				LirToken::CloseBrace => {
					if delims.last() != Some(&LirToken::OpenBrace) {
						println!("2");
					} else {
						delims.pop();
						mir.push(MirToken::BraceString(
							delim_buffer.clone(),
						));
						delim_buffer.clear();
						is_open = false;
					}
				}
				LirToken::CloseBracket => {
					if delims.last() != Some(&LirToken::OpenBracket) {
						println!("3");
					} else {
						delims.pop();
						mir.push(MirToken::BracketString(
							delim_buffer.clone(),
						));
						delim_buffer.clear();
						is_open = false;
					}
				}
				_ => {
					if is_open {
						delim_buffer.push(token.as_char());
					} else {
						other_buffer.push(token.as_char());
					}
				}
			}
		}

		if !other_buffer.is_empty() {
			mir.push(MirToken::String(other_buffer.clone()));
		}

		MirTokenStream::new(mir)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "Asmelash, L. (2019, August 14). Social media use may harm teens' mental health by disrupting positive activities, study says. CNN.";
		// Asmelash, L.
		// (2019, August 14).
		// Social media use may harm teens' mental health by disrupting positive activities, study says.
		// CNN.
		let stream = LirTokenStream::try_new(s).unwrap();
		//println!("{:?}", stream);

		let v = BidiDelimStringsVisitor {};
		let tokens = v.visit(stream);

		assert_eq!(
			tokens,
			MirTokenStream::new(vec![
				MirToken::String("Asmelash, L. ".to_string()),
				MirToken::ParenString("2019, August 14".to_string()),
				MirToken::String(". Social media use may harm teens' mental health by disrupting positive activities, study says. CNN.".to_string()),
			])
		)
	}
}
