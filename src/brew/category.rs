pub(super) enum Category {
	Formulae,
	Casks,
}

impl Category {
	pub(super) fn all() -> [Self; 2] {
		[Self::Formulae, Self::Casks]
	}

	pub(super) fn name(&self) -> &'static str {
		match self {
			Self::Formulae => "Formulae",
			Self::Casks    => "Casks",
		}
	}

	pub(super) fn option(&self) -> &'static str {
		match self {
			Self::Formulae => "--formulae",
			Self::Casks    => "--casks",
		}
	}
	
	pub(super) fn title(&self) -> &'static str {
		match self {
			Self::Formulae => "New Formulae\n",
			Self::Casks    => "New Casks\n",
		}
	}
}
