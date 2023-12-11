use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::RangeInclusive;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Citation {
	pub authors: Vec<Individual>,
	pub number: SequencedNumber,
	pub publisher: String,
	pub publication_datetime: CitationDate,
	pub contributors: Vec<Individual>,
	pub title_source: String,
	pub title_container: String,
	pub location: Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequencedNumber {
	pub parent: u32,
	pub child: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CitationDate {
	Y(i32),
	YM(i32, u32),
	YMD(i32, u32, u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Individual {
	pub kind: IndividualKind,
	pub name: String,
}

impl Individual {
	pub const fn new(kind: IndividualKind, name: String) -> Self {
		Self { kind, name }
	}

	pub const fn author(name: String) -> Self {
		Self::new(IndividualKind::Author, name)
	}

	pub const fn editor(name: String) -> Self {
		Self::new(IndividualKind::Editor, name)
	}

	pub const fn contributor(name: String) -> Self {
		Self::new(IndividualKind::Contributor, name)
	}

	pub const fn translator(name: String) -> Self {
		Self::new(IndividualKind::Translator, name)
	}

	pub const fn is_author(&self) -> bool {
		matches!(self.kind, IndividualKind::Author)
	}

	pub const fn is_editor(&self) -> bool {
		matches!(self.kind, IndividualKind::Editor)
	}

	pub const fn is_contributor(&self) -> bool {
		matches!(self.kind, IndividualKind::Contributor)
	}

	pub const fn is_translator(&self) -> bool {
		matches!(self.kind, IndividualKind::Translator)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndividualKind {
	Author,
	Editor,
	Contributor,
	Translator,
	Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
	Url(Url),
	Page(Page),
	Place(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
	FromTo(RangeInclusive<u32>),
	At(u32),
}

impl Page {
	pub const fn is_range(&self) -> bool {
		matches!(self, Self::FromTo(_))
	}
}

impl Display for Page {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::FromTo(r) => write!(f, "{}-{}", r.start(), r.end()),
			Self::At(i) => write!(f, "{}", i),
		}
	}
}
