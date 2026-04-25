use rayon::prelude::*;

use super::{Brew, Category, json, Subcommand};

impl Brew {
	/// Prints new formulae, new casks, and outdated with descriptions in parallel
	pub(super) fn print_outdated_with_new_item_desc(&self) {
		let (update, outdated) = rayon::join(
			Subcommand::update,
			Subcommand::outdated,
		);
		
		let (_, outdated_columns) = rayon::join(
			|| {
				self.print_new_items(update)
			},
			|| {
				outdated.contains_results().then(
					|| {
						let (items, versions) = outdated.sorted_cols();
						let desc = Subcommand::desc(&items, None);
						(items, versions, desc)
					}
				)
			},
		);
		
		if let Some((items, versions, desc)) = outdated_columns {			
			println!(
				"==> Outdated\n{}\n",
				self.table.from_columns(
				[
					&items,
					&versions,
					&desc.cols().1
				]
				)
			);
		};
	}
	
	fn print_new_items(&self, update: Subcommand) {
		if update.contains_new_items() {
			Category::all().into_par_iter().for_each(|category| {
				if let Some(new_items) = update.extract_new_items(&category) {
					println!(
						"==> {}{}\n",
						category.title(),
						self.table.from_columns(json::name_desc_homepage_array(&new_items))
					);
				}
			});
		};
	}
	
	/// Lists all installed items with description in parallel
	pub(super) fn print_desc_for_all_installed(&self) {
		Category::all().into_par_iter().for_each(|category| {
			println!(
				"\n==> All {}\n{}\n",
				category.name(),
				self.table.from_columns(Subcommand::list_with_desc(category).array())
			);
		});
	}
	
	/// Lists all manually installed formulae with descriptions
	pub(super) fn print_desc_for_leaves(&self) {
		println!(
			"\n==> Leaves\n{}\n",
			self.table.from_columns(Subcommand::leaves_with_desc().array())
		);
	}
}
