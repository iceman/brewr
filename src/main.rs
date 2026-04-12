mod brew;
mod command_line;
mod params;
mod table;

use brew::Brew;
use params::{Params, SpecialMode};
use table::Style;

fn main() {
	let brew = Brew { params: command_line::params() };
	brew.run();
}
