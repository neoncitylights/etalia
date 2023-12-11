use crate::lir::LirToken;
use crate::mir::{MirToken, MirTokenStream};
use crate::visitors::MirOptVisitor;

/// MIR optimization pass to find instances of [MirToken::Uint] and [MirToken::Sint]
#[derive(Debug, Clone, Copy)]
pub struct IntLitVisitor;

impl MirOptVisitor for IntLitVisitor {
	fn visit(&self, stream: &mut MirTokenStream) {
		let mut int_buffer = String::new();
		let mut int_is_signed = false;
		let mut last_seen: Option<&LirToken> = None;

		for token in &stream.tokens {
			let temp_lir_stream = token.lower();
			let lir_stream_vec = temp_lir_stream.tokens;

			match token {
				MirToken::String(_) => {
					for ir_token in lir_stream_vec {
						match ir_token {
							LirToken::Hyphen => {
								if last_seen
									== Some(&LirToken::Hyphen)
								{
									int_is_signed =
										!int_is_signed;
								} else {
									int_buffer
										.push(ir_token
											.as_char());
								}
							}
							LirToken::Digit(c) => {
								int_buffer.push(c);
							}
							_ => {
								if !int_buffer.is_empty() {
									let int =
										if int_is_signed {
											MirToken::Sint(int_buffer.parse::<i32>().unwrap())
										} else {
											MirToken::Uint(int_buffer.parse::<u32>().unwrap())
										};

									int_buffer.clear();
									int_is_signed = false;
								}
							}
						}
					}
				}
				_ => {
					continue;
				}
			}
		}
	}
}
