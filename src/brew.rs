mod category;
mod json;
mod print;
mod subcommand;
mod system;

use category::Category;
use subcommand::Subcommand;
use super::Table;

pub enum SpecialMode {
	All,
	Leaves,
}

pub struct Brew {
	pub(super) special_mode: Option<SpecialMode>,
	pub(super) table: Table,
}

impl Brew {
	pub fn run(&mut self) {		
		match self.special_mode {
			Some(SpecialMode::All) => self.print_desc_for_all_installed(),
			Some(SpecialMode::Leaves) => self.print_desc_for_leaves(),
			None => self.print_outdated_with_new_item_desc(),
		}
	}
}
