use clap::ValueEnum;
use tabled::settings::{
	themes::Theme,
	Style as TableStyle,
};

#[derive(Debug, Default, Copy, Clone, ValueEnum)]
pub enum Style {
	#[default]
	Blank,
	Ascii,
	Psql,
	Markdown,
	Modern,
	Sharp,
	Rounded,
	ModernRounded,
	Extended,
	Rst,
	AsciiRounded
}

impl Style {
	pub(super) fn theme(&self) -> Theme {
		match self {
			Self::Blank 		=> Theme::from(TableStyle::blank()),
			Self::Ascii 		=> Theme::from(TableStyle::ascii()),
			Self::Psql 			=> Theme::from(TableStyle::psql()),
			Self::Markdown 		=> Theme::from(TableStyle::markdown()),
			Self::Modern 		=> Theme::from(TableStyle::modern()),
			Self::Sharp 		=> Theme::from(TableStyle::sharp()),
			Self::Rounded 		=> Theme::from(TableStyle::rounded()),
			Self::ModernRounded => Theme::from(TableStyle::modern_rounded()),
			Self::Extended 		=> Theme::from(TableStyle::extended()),
			Self::Rst 			=> Theme::from(TableStyle::re_structured_text()),
			Self::AsciiRounded 	=> Theme::from(TableStyle::ascii_rounded()),
		}
	}
}