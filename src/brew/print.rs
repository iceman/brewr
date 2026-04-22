use rayon::prelude::*;

use crate::table;
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
				if outdated.contains_results() {
					let (items, versions) = outdated.sorted_cols();
					let desc = Subcommand::desc(&items, None);
					Some((items, versions, desc))
				} else {
					None
				}
			},
		);
		
		if let Some((items, versions, desc)) = outdated_columns {			
			println!(
				"==> Outdated\n{}\n",
				table::from_columns(
				[
					&items,
					&versions,
					&desc.cols().1
				],
				self.params.style
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
						table::from_columns(json::name_desc_homepage_array(&new_items), self.params.style)
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
				table::from_columns(
					Subcommand::list_with_desc(category).array(),
					self.params.style
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
