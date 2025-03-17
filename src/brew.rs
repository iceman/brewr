mod json;
mod result;
mod style;
use crate::system::{self, StreamsToString};
use result::BrewResult as Brew;
use std::{sync::Arc, thread};
use style::Style;

pub fn command(args: &[&str]) -> Brew {
	let output = system::execute("brew", args).unwrap();
	Brew::new(output)
}

pub fn command_with_items(cmd: &str, items: &[&str], args: &str) -> Brew {
	let args = &[&[cmd], items, &[args]].concat();
	command(args)
}

pub fn update() -> Brew {
	let mut update = command(&["update"]);
	update.stderr = Some(update.output.stderr_string());
	update
}

/// Sorted list of all outdated formulae and casks
pub fn outdated() -> Brew {
	Brew::new(
		system::pipe(&[
				("bash",  &["-c", "cat <(brew outdated -v --formulae) <(brew outdated -v --casks)"]),
				("sort",  &[]),
		])
		.unwrap()
	)
}

/// Outputs name and description for all items of style
pub fn list_with_desc(style: Style) -> Brew {
	Brew::with_desc(&["list", "-1", style.option()], style)
}

/// Outputs name and description for all leaves (formulae only)
pub fn leaves_with_desc() -> Brew {
	Brew::with_desc(&["leaves"], Style::Formulae)
}

/// Iterates through styles, spawning a thread and yielding to a passed closure
pub fn each_style<F>(func: F)
where
	F: Fn(Style) + Send + 'static + Sync,
{
	let func = Arc::new(func);
	let handles = Style::iter()
		.map(|style| {
			let func = Arc::clone(&func);
			thread::spawn(move || func(style))
		})
		.collect::<Vec<_>>();

	handles.into_iter().for_each(|h| h.join().unwrap());
}

/// Name, description, homepage from JSON data parse
pub fn name_desc_homepage_array(items: &[&str]) -> [Vec<String>; 3] {
	let bytes = command_with_items("info", items, "--json=v2").output.stdout;
	json::name_desc_homepage(items.len(), bytes)
}
