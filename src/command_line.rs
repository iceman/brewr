use clap::Parser;

use super::{SpecialMode, Style};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(override_usage = "brewr [OPTIONS] (no options defaults to update brew)")]
pub struct Args {
	/// List all installed formulae with descriptions
	#[arg(short, long, conflicts_with = "leaves")]
	pub all: bool,

	/// List all manually installed formulae with descriptions
	#[arg(short, long)]
	pub leaves: bool,

	/// Display Style
	#[arg(short, long, value_enum, default_value_t = Style::Blank)]
	pub style: Style,
}

pub fn parse() -> (Option<SpecialMode>, Style) {
	let Args { all, leaves, style } = Args::parse();
	
	let special_mode = if all {
		Some(SpecialMode::All)
	} else if leaves {
		Some(SpecialMode::Leaves)
	} else {
		None
	};
		
	(special_mode, style)
}
