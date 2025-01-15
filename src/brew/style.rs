pub enum Style {
	Formulae,
	Casks,
}

impl Style {
	pub fn iter() -> impl Iterator<Item = Self> {
		[
			Self::Formulae,
			Self::Casks,
		].into_iter()
	}

	pub fn name(&self) -> &'static str {
		match self {
			Self::Formulae => "Formulae",
			Self::Casks    => "Casks",
		}
	}

	pub fn option(&self) -> &'static str {
		match self {
			Self::Formulae => "--formulae",
			Self::Casks    => "--casks",
		}
	}
	
	pub fn title(&self) -> &'static str {
		match self {
			Self::Formulae => "New Formulae\n",
			Self::Casks    => "New Casks\n",
		}
	}
}
