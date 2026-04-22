use rayon::prelude::*;

use super::Category;
use super::system::{self, OutputToString};

pub(super) struct Subcommand {
	pub(super) result: String,
}

impl Subcommand {
	// constructors
	pub(super) fn update() -> Self {
		Self::stderr(
			system::execute(&["update"])
		)
	}
	
	pub(super) fn desc(items: &[&str], category: Option<Category>) -> Self {
		match category {
			None => Self::run_with_items("desc", items, "--eval-all"),
			Some(cat) => Self::run_with_items("desc", items, cat.option()),
		}
	}
	
	/// Sorted list of all outdated formulae and casks w/ parallel iterator
	pub(super) fn outdated() -> Self {
		Self::stdout(
			Category::all().into_par_iter().map(|category| {
				Self::run(&["outdated", "-v", category.option()]).result
			})
			.collect::<String>()
		)
	}
	
	/// Outputs name and description for all items of category
	pub(super) fn list_with_desc(category: Category) -> Self {
		let list = Self::run(&["list", "-1", category.option()]);
		Self::desc(&list.itemize(), Some(category))
	}
	
	/// Outputs name and description for all leaves (formulae only)
	pub(super) fn leaves_with_desc() -> Self {
		let list = Self::run(&["leaves"]);
		Self::desc(&list.itemize(), None)
	}
	
	fn run(args: &[&str]) -> Self {
		Self::stdout(
			system::execute(args)
		)
	}
	
	fn run_with_items(sub_cmd: &str, items: &[&str], args: &str) -> Self {
		Self::stdout(
			system::execute_with_items(sub_cmd, items, args)
		)
	}
	
	fn stdout<T: OutputToString>(output: T) -> Self {
		Self {
			result: output.stdout_string()
		}
	}
	
	fn stderr<T: OutputToString>(output: T) -> Self {
		Self {
			result: output.stderr_string()
		}
	}
}

impl Subcommand {
	// behavior
	/// Isolates item list between two string markers
	pub(super) fn extract_new_items<'a>(&'a self, category: &Category) -> Option<Vec<&'a str>> {
		Some(
			self.result
				.split_once(category.title())?.1
				.split_once("\n==>")?.0
				.lines()
				.map(|line| line.split_once(':').map(|(before, _)| before).unwrap_or(line))
				.collect()
		)
	}
	
	/// Split brew's space/colon separated output into two columns
	pub(super) fn cols(&self) -> (Vec<&str>, Vec<&str>) {
		self.cols_iter().unzip()
	}
	
	pub(super) fn sorted_cols(&self) -> (Vec<&str>, Vec<&str>) {
		let mut pairs: Vec<(&str, &str)> = self.cols_iter().collect();
		pairs.sort_by_key(|(first, _)| *first);
		pairs.into_iter().unzip()
	}
	
	fn cols_iter(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
		self.result
			.lines()
			.map(|l| l.split_once([' ', ':']).unwrap())
	}
	
	pub(super) fn array(&self) -> [Vec<&str>; 2] {
		self.cols().into()
	}
	
	pub(super) fn contains_new_items(&self) -> bool {
		!self.result.contains("Already up-to-date")
	}
	
	pub(super) fn contains_results(&self) -> bool {
		!self.result.is_empty()
	}
	
	fn itemize(&self) -> Vec<&str> {
		self.result.lines().collect()
	}
}

#[cfg(test)]
mod tests {
	use super::{Subcommand, Category};
	
	#[test]
	fn processing_test() {
		let brew_test = Subcommand {
			result: "homebrew/cask).\n==> New Formulae\nform1\nform2\n==> New Casks\ncask1\ncask2\n==> Outdated Formulae\nabcd\n".to_owned()
		};

		let new_items = brew_test.extract_new_items(&Category::Formulae).unwrap();
		assert_eq!(new_items, vec!["form1", "form2"]);

		let new_items = brew_test.extract_new_items(&Category::Casks).unwrap();
		assert_eq!(new_items, vec!["cask1", "cask2"]);
	}
}