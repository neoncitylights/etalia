#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
	Terminal,
	NonTerminal,
}

impl SymbolKind {
	pub fn as_bool(&self) -> bool {
		bool::from(self)
	}
}

impl From<bool> for SymbolKind {
	fn from(b: bool) -> Self {
		match b {
			true => Self::Terminal,
			false => Self::NonTerminal,
		}
	}
}

impl From<&SymbolKind> for bool {
	fn from(s: &SymbolKind) -> Self {
		match s {
			SymbolKind::Terminal => true,
			SymbolKind::NonTerminal => false,
		}
	}
}

impl From<SymbolKind> for bool {
	fn from(s: SymbolKind) -> Self {
		match s {
			SymbolKind::Terminal => true,
			SymbolKind::NonTerminal => false,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn symbol_kind_as_bool() {
		assert_eq!(SymbolKind::Terminal.as_bool(), true);
		assert_eq!(SymbolKind::NonTerminal.as_bool(), false);
	}

	#[test]
	fn symbol_kind_from_bool() {
		assert_eq!(SymbolKind::from(true), SymbolKind::Terminal);
		assert_eq!(SymbolKind::from(false), SymbolKind::NonTerminal);
	}

	#[test]
	fn symbol_kind_from_ref_symbol_kind() {
		assert_eq!(bool::from(&SymbolKind::Terminal), true);
		assert_eq!(bool::from(&SymbolKind::NonTerminal), false);
	}

	#[test]
	fn symbol_kind_from_symbol_kind() {
		assert_eq!(bool::from(SymbolKind::Terminal), true);
		assert_eq!(bool::from(SymbolKind::NonTerminal), false);
	}
}
