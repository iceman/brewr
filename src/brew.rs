mod category;
mod json;
mod print;
mod subcommand;
mod system;

use category::Category;
use subcommand::Subcommand;
use super::{Params, SpecialMode};

pub struct Brew {
	pub(super) params: Params,
}

impl Brew {
	pub fn run(&self) {
		if let Some(special_mode) = self.params.special_mode {
			match special_mode {
				SpecialMode::All => self.print_desc_for_all_installed(),
				SpecialMode::Leaves => self.print_desc_for_leaves(),
			}
		} else {
			self.print_output_with_new_item_desc()
		}
	}
}
