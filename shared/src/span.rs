#[derive(Clone, Debug)]
pub struct Span {
	file: String,
	start: LineColumn,
	end: LineColumn,
}

#[derive(PartialEq, Clone, Debug)]
pub struct LineColumn {
	line: u32,
	column: u32,
}

impl PartialOrd for LineColumn {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.line > other.line {
			Some(std::cmp::Ordering::Greater)
		} else if self.line < other.line {
			Some(std::cmp::Ordering::Less)
		} else if self.column > other.column {
			Some(std::cmp::Ordering::Greater)
		} else if self.column < other.column {
			Some(std::cmp::Ordering::Less)
		} else {
			Some(std::cmp::Ordering::Equal)
		}
	}
}

impl LineColumn {
	pub fn new(line: u32, column: u32) -> LineColumn {
		LineColumn {
			line,
			column,
		}
	}
	pub fn line(&self) -> u32 {
		self.line
	}
	pub fn column(&self) -> u32 {
		self.column
	}
}

impl Span {
	pub fn default() -> Span {
		Span {
			file: String::new(),
			start: LineColumn {
				column: 0,
				line: 0,
			},
			end: LineColumn {
				column: 0,
				line: 0,
			},
		}
	}
	pub fn new(file: String, start: LineColumn, end: LineColumn) -> Span {
		Span {
			file,
			start,
			end,
		}
	}
	pub fn to_string_start(&self) -> String {
		format!("{}:{}:{}", self.file, self.start.line, self.start.column)
	}
	pub fn to_string_end(&self) -> String {
		format!("{}:{}:{}", self.file, self.end.line, self.end.column)
	}
	pub fn join(s1: &Span, s2: &Span) -> Span {
		if s1.file != s2.file {
			panic!("Join between different files' spans is illegal.");
		}
		let start = if s1.start < s2.start {
			s1.start.clone()
		} else {
			s2.start.clone()
		};
		let end = if s1.end > s2.end {
			s1.end.clone()
		} else {
			s2.end.clone()
		};
		Span {
			file: s1.file.clone(),
			start,
			end,
		}
	}
	pub fn start(&self) -> &LineColumn {
		&self.start
	}
	pub fn end(&self) -> &LineColumn {
		&self.end
	}
	pub fn file(&self) -> &str {
		&self.file
	}
}

pub trait Spanned {
	fn span(&self) -> &Span;
}