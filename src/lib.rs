mod brew;
pub mod config;
pub mod system;
mod table;
use std::thread;

/// - Prints new formulae, new casks, and outdated with descriptions
pub fn print_output_with_new_item_desc() {
	let update = brew::update();
	let outdated_handle = thread::spawn(print_outdated_with_desc);

	if update.contains_new_items() {
		brew::each_style(move |style| {
			if let Some(new_items) = update.extract_new_items(&style) {
				println!(
					"==> {}{}\n",
					style.title(),
					table::from_columns(brew::name_desc_homepage_array(&new_items))
				);
			}
		});
	};

	outdated_handle.join().unwrap();
}

/// Prints a table of |name|version|desc| for outdated formulae
fn print_outdated_with_desc() {
	let outdated = brew::outdated();
	if outdated.contains_results() {
		let (items, versions) = outdated.cols();
		
		println!(
			"==> Outdated\n{}\n",
			table::from_columns([
				&items,
				&versions,
				&brew::command_with_items("desc", &items, "--eval-all").cols().1,
			])
		);
	};
}

/// Lists all installed items with description
pub fn print_desc_for_all_installed() {
	brew::each_style(|style| {
		println!(
			"==> All {}\n{}\n",
			style.name(),
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
