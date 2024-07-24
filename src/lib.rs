mod brew;
pub mod config;
pub mod system;
mod table;
use std::thread;

const UP_TO_DATE: &str = "Already up-to-date.\n";

/// - Prints new formulae, new casks, and outdated with descriptions
pub fn print_output_with_new_item_desc() {
	let update = brew::command(&["update"]);
	let outdated_handle = thread::spawn(print_outdated_with_desc);

	if update.stdout.contains(UP_TO_DATE) {
		println!("{}", UP_TO_DATE);
	} else {
		let update = update.stderr();
		brew::map(move |_, style_name| {
			print_new_items(&update, style_name); // `brew update` outputs list to stderr
		});
	};

	outdated_handle.join().unwrap();
}

/// Lists all installed items with description
pub fn print_desc_for_all_installed() {
	brew::map(|style, style_name| {
		println!(
			"==> All {}\n{}\n",
			style_name,
			table::from_columns(brew::list_with_desc(style).array())
		);
	});
}

/// Lists all manually installed formulae with descriptions
pub fn print_desc_for_leaves() {
	println!(
		"==> Leaves\n{}\n",
		table::from_columns(brew::leaves_with_desc().array())
	);
}

/// Prints a table of |name|version|desc| for outdated formulae
fn print_outdated_with_desc() {
	let outdated = brew::outdated();
	if outdated.stdout.is_empty() {
		return;
	};

	let (items, versions) = outdated.cols();

	println!(
		"==> Outdated\n{}\n",
		table::from_columns([
			&items,
			&versions,
			&brew::command_with_items("desc", &items, "--eval-all").cols().1,
		])
	);
}

/// Prints new items if found in output
fn print_new_items(output: &str, style_name: &str) {
	let search_str = format!("New {}\n", style_name);

	if let Some(new_items) = extract_new_items(output, &search_str) {
		println!(
			"==> {}{}\n",
			search_str,
			table::from_columns(brew::name_desc_homepage_array(&new_items))
		);
	}
}

/// Isolates item list between two string markers
fn extract_new_items<'a>(text: &'a str, search_str: &str) -> Option<Vec<&'a str>> {
	Some(
		text
			.split_once(search_str)?.1
			.split_once("\n==>")?.0
			.lines()
			.collect()
	)
}

#[cfg(test)]
mod tests {
	use super::extract_new_items;

	#[test]
	fn processing_test() {
		let output = "homebrew/cask).\n==> New Formulae\nform1\nform2\n==> New Casks\ncask1\ncask2\n==> Outdated Formulae\naugeas\nawscli\ncjson\ncmake\ndav1d\n";

		let new_items = extract_new_items(&output, "New Formulae\n").unwrap();
		assert_eq!(new_items, vec!["form1", "form2"]);

		let new_items = extract_new_items(&output, "New Casks\n").unwrap();
		assert_eq!(new_items, vec!["cask1", "cask2"]);
	}
}
