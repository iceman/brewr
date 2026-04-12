use rayon::prelude::*;

use crate::table;
use super::{Brew, Category, json, Subcommand};

impl Brew {
	/// Prints new formulae, new casks, and outdated with descriptions
	pub(super) fn print_output_with_new_item_desc(&self) {
		let (update, outdated) = rayon::join(
			Subcommand::update,
			Subcommand::outdated,
		);
		
		if update.contains_new_items() {
			Category::all().into_par_iter().for_each(|category| {
				if let Some(new_items) = update.extract_new_items(&category) {
					println!(
						"==> {}{}\n",
						category.title(),
						table::from_columns(json::name_desc_homepage_array(&new_items), self.params.style)
					);
				}
			});
		};
		
		if outdated.contains_results() {
			let (items, versions) = outdated.sorted_cols();
			
			println!(
				"==> Outdated\n{}\n",
				table::from_columns(
				[
					&items,
					&versions,
					&Subcommand::desc(&items, None).cols().1
				],
				self.params.style
				)
			);
		};
	}
	
	/// Lists all installed items with description
	pub(super) fn print_desc_for_all_installed(&self) {
		let style = self.params.style;
		
		Category::all().into_par_iter().for_each(|category| {
			println!(
				"\n==> All {}\n{}\n",
				category.name(),
				table::from_columns(
					Subcommand::list_with_desc(category).array(),
					style
				)
			);
		});
	}
	
	/// Lists all manually installed formulae with descriptions
	pub(super) fn print_desc_for_leaves(&self) {
		println!(
			"\n==> Leaves\n{}\n",
			table::from_columns(
				Subcommand::leaves_with_desc().array(),
				self.params.style
			)
		);
	}
}
