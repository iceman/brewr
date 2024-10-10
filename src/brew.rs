mod json;
mod result;
mod style;
use crate::system::{self, StreamsToString};
use result::BrewResult;
use std::{sync::Arc, thread};
use style::Style;

pub fn command(args: &[&str]) -> BrewResult {
	BrewResult::new(system::execute("brew", args).unwrap())
}

pub fn command_with_items(cmd: &str, items: &[&str], args: &str) -> BrewResult {
	command(&[&[cmd], items, &[args]].concat())
}

pub fn update() -> BrewResult {
	let mut update = command(&["update"]);
	update.stderr = update.output.stderr_string();
	update
}

/// Sorted list of all outdated formulae and casks
pub fn outdated() -> BrewResult {
	BrewResult::new(
		system::pipe(&[
				("bash",  &["-c", "cat <(brew outdated -v --formulae) <(brew outdated -v --casks)"]),
				("sort",  &[]),
		])
		.unwrap()
	)
}

/// Outputs name and description for all items of style
pub fn list_with_desc(style: Style) -> BrewResult {
	BrewResult::with_desc(&["list", "-1", style.option()], style)
}

/// Outputs name and description for all leaves (formulae only)
pub fn leaves_with_desc() -> BrewResult {
	BrewResult::with_desc(&["leaves"], Style::Formulae)
}

/// Iterates through styles yielding to a passed closure
/// Executes each pass in a separate thread with atomic reference counting
pub fn each<F>(func: F)
where
	F: Fn(Style) + Send + 'static + Sync,
{
	let func = Arc::new(func);
	let handles = Style::iter()
		.map(|style| {
			let func = Arc::clone(&func);
			thread::spawn(move || {
				func(style);
			})
		})
		.collect::<Vec<_>>();

	handles.into_iter().for_each(|h| h.join().unwrap());
}

/// Name, description, homepage from JSON data parse
pub fn name_desc_homepage_array(items: &[&str]) -> [Vec<String>; 3] {
	let bytes = command_with_items("info", items, "--json=v2").output.stdout;
	json::name_desc_homepage(items.len(), bytes)
}
