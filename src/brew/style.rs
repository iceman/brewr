pub enum Style {
	Formulae,
	Casks,
}

impl Style {
	pub fn iter() -> impl Iterator<Item = Style> {
		[
			Style::Formulae,
			Style::Casks,
		]
		.into_iter()
	}
	
	pub fn name(&self) -> &'static str {
		match self {
			Style::Formulae => "Formulae",
			Style::Casks    => "Casks",
		}
	}
	
	pub fn option(&self) -> &'static str {
		match self {
			Style::Formulae => "--formulae",
			Style::Casks    => "--casks",
		}
	}
}
