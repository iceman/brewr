use std::process::{Command, Output};

pub(super) fn execute(args: &[&str]) -> Output {
	Command::new("brew")
		.args(args)
		.output()
		.unwrap()
}

pub(super) fn execute_with_items(sub_cmd: &str, items: &[&str], args: &str) -> Output {
	Command::new("brew")
		.arg(sub_cmd)
		.args(items)
		.arg(args)
		.output()
		.unwrap()
}

/// Default Output methods to convert Std Streams to String
pub(super) trait OutputToString {
	fn stdout_string(&self) -> String;
	fn stderr_string(&self) -> String;
}

impl OutputToString for Output {
	fn stdout_string(&self) -> String {
		bytes_to_string(&self.stdout)
	}

	fn stderr_string(&self) -> String {
		bytes_to_string(&self.stderr)
	}
}

fn bytes_to_string(bytes: &[u8]) -> String {
	String::from_utf8_lossy(bytes).into_owned()
}
