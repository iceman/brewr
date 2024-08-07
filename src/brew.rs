mod json;
mod style;
use crate::system::{self, Output, StreamsToString};
use std::{sync::Arc, thread};
use style::Style;

// Public Module Functions
pub fn command(args: &[&str]) -> Brew {
	Brew::new(system::execute("brew", args).unwrap())
}

pub fn command_with_items(cmd: &str, items: &[&str], args: &str) -> Brew {
	command(&[&[cmd], items, &[args]].concat())
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

/// Iterates through styles yielding the Enum and style name to a passed closure
/// Executes each pass in a separate thread with atomic reference counting
pub fn map<F>(func: F)
where
	F: Fn(Style, &str) + Send + 'static + Sync,
{
	let func = Arc::new(func);
	let handles = Style::iter()
		.map(|style| {
			let func = Arc::clone(&func);
			thread::spawn(move || {
				let name = style.name();
				func(style, name);
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

pub struct Brew {
	pub stdout: String,
	output: Output,
}

// Interface Methods
impl Brew {
	// Split brew's space/colon separated output into two columns
	pub fn cols(&self) -> (Vec<&str>, Vec<&str>) {
		self.stdout
			.lines()
			.map(|l| l.split_once([' ', ':']).unwrap())
			.unzip()
	}
	
	pub fn array(&self) -> [Vec<&str>; 2] {
		self.cols().into()
	}

	pub fn stderr(&self) -> String {
		self.output.stderr_string()
	}
}

// Private Implementation
impl Brew {
	fn new(output: Output) -> Self {
		let stdout = output.stdout_string();
		Self { stdout, output }
	}

	/// Takes brew command args to first generate a list of names, then calls brew again with list to get names and descriptions
	fn with_desc(args: &[&str], style: Style) -> Self {
		let desc_command = format!(r#"brew desc "${{0}}" "${{@}}" --eval-all {}"#, style.option());
		
		Self::new(
			system::pipe(&[
					("brew",  args),
					("tr", 	  &["\n", " "]),
					("xargs", &["bash", "-c", &desc_command]),
			])
			.unwrap()
		)
	}
}
