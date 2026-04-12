use super::Style;

#[derive(Debug, Copy, Clone)]
pub enum SpecialMode {
	All,
	Leaves,
}

#[derive(Debug, Default)]
pub struct Params {
	pub special_mode: Option<SpecialMode>,
	pub style: Style,
}

impl Params {
	pub fn new(all: bool, leaves: bool, style: Style) -> Self {
		let special_mode = match (all, leaves) {
			(true, false) => Some(SpecialMode::All),
			(false, true) => Some(SpecialMode::Leaves),
			(false, false) => None,
			(true, true) => unreachable!("Invalid state"),
		};
		
		Self { special_mode, style }
	}
}
