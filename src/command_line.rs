use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(override_usage = "brewr [OPTIONS] (no options defaults to update brew)")]
pub struct Args {
	/// List all installed formulae with descriptions
	#[arg(short, long)]
	pub all: bool,

	/// List all manually installed formulae with descriptions
	#[arg(short, long)]
	pub leaves: bool,

	/// Display results with grid lines
	#[arg(short, long)]
	pub grid: bool,
}

impl Args {
	pub fn get() -> Self {
		Self::parse()
	}
}