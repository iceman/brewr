use crate::system::{self, Output, StreamsToString};
use super::style::Style;

pub struct BrewResult {
	pub(super) stdout: String,
	pub(super) stderr: String,
	pub(super) output: Output,
}

// Public Methods
impl BrewResult {
	/// Isolates item list between two string markers
	pub fn extract_new_items<'a>(&'a self, style: &Style) -> Option<Vec<&'a str>> {
		Some(
			self.stderr
				.split_once(style.title())?.1
				.split_once("\n==>")?.0
				.lines()
				.collect()
		)
	}
	
	/// Split brew's space/colon separated output into two columns
	pub fn cols(&self) -> (Vec<&str>, Vec<&str>) {
		self.stdout
			.lines()
			.map(|l| l.split_once([' ', ':']).unwrap())
			.unzip()
	}
	
	pub fn array(&self) -> [Vec<&str>; 2] {
		self.cols().into()
	}
	
	pub fn contains_new_items(&self) -> bool {
		!self.stdout.contains("Already up-to-date")
	}
	
	pub fn contains_results(&self) -> bool {
		!self.stdout.is_empty()
	}
}

// Constructors
impl BrewResult {
	pub(super) fn new(output: Output) -> Self {
		let stdout = output.stdout_string();
		let stderr = String::new();
		Self { stdout, stderr, output }
	}

	/// Takes brew command args to first generate a list of names, then calls brew again with list to get names and descriptions
	pub(super) fn with_desc(args: &[&str], style: Style) -> Self {
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

#[cfg(test)]
mod tests {
	use super::{super::command, Style};
	
	#[test]
	fn processing_test() {
		let test_line = "homebrew/cask).\n==> New Formulae\nform1\nform2\n==> New Casks\ncask1\ncask2\n==> Outdated Formulae\naugeas\nawscli\ncjson\ncmake\ndav1d\n".to_string();

		let mut brew_test = command(&["-h"]);
		brew_test.stderr = test_line;

		let new_items = brew_test.extract_new_items(&Style::Formulae).unwrap();
		assert_eq!(new_items, vec!["form1", "form2"]);

		let new_items = brew_test.extract_new_items(&Style::Casks).unwrap();
		assert_eq!(new_items, vec!["cask1", "cask2"]);
	}
}