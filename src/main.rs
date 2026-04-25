mod brew;
mod command_line;
mod table;

use brew::{Brew, SpecialMode};
use table::{Style, Table};

fn main() {
	let (special_mode, style) = command_line::parse();
	
	Brew {
		special_mode,
		table: Table {style},
	}
	.run();
}
