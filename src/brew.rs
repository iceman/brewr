mod json;
mod style;
use crate::system::{self, Output, StreamsToString};
use std::{sync::Arc, thread};
use style::Style;

pub struct Brew {
	pub stdout: String,
	output: Output,
}

impl Brew {
	fn new(output: Output) -> Self {
		let stdout = output.stdout_string();
		Self { stdout, output }
	}

	pub fn cmd(args: &[&str]) -> Self {
		Self::new(system::execute("brew", args).unwrap())
	}

	pub fn cmd_with_items(cmd: &str, items: &[&str], args: &str) -> Self {
		Self::cmd(&[&[cmd], items, &[args]].concat())
	}

	/// Iterates through styles yielding the Enum and Name to a passed closure
	/// Executes each pass in a separate thread and joins handles
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

	/// Sorted list of all outdated formulae and casks
	pub fn outdated() -> Self {
		Self::new(
			system::pipe(
				&[
					("bash",  &["-c", "cat <(brew outdated -v --formulae) <(brew outdated -v --casks)"]),
					("sort",  &[]),
				]
			)
			.unwrap()
		)
	}

	/// Takes brew command args to generate a list of names, outputs same w/ desc, specify --formulae or --casks
	fn output_with_desc(args: &[&str], item_type: &str) -> Self {
		let desc_cmd = format!(r#"brew desc "${{0}}" "${{@}}" --eval-all {}"#, item_type);
		
		Self::new(
			system::pipe(
				&[
					("brew",  args),
					("tr", 	  &["\n", " "]),
					("xargs", &["bash", "-c", &desc_cmd]),
				]
			)
			.unwrap()
		)
	}

	/// Outputs name and description for all items of style
	pub fn list_with_desc(style: Style) -> Self {
		Self::output_with_desc(&["list", "-1", style.option()], style.option())
	}

	/// Outputs name and description for all leaves (formulae only)
	pub fn leaves_with_desc() -> Self {
		Self::output_with_desc(&["leaves"], Style::Formulae.option())
	}

	/// JSON Parser yielding name, description, homepage
	pub fn name_desc_homepage_array(items: &[&str]) -> [Vec<String>; 3] {
		json::name_desc_homepage(items)
	}

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
