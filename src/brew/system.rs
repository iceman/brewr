use std::process::{Command, Output};

pub(super) fn execute(args: &[&str]) -> Output {
	Command::new("brew")
		.args(args)
		.output()
		.unwrap()
}

pub(super) fn execute_with_items(sub_cmd: &str, items: &[&str], args: Option<&str>) -> Output {
	let mut cmd = Command::new("brew");
		cmd.arg(sub_cmd).args(items);
		
		if let Some(a) = args {
			cmd.arg(a);
		}
		
		cmd.output().unwrap()
}

/// Convenience Type to always output String from Std Streams/String
pub(super) trait OutputToString {
	fn stdout_string(self) -> String;
	fn stderr_string(self) -> String;
}

impl OutputToString for String {
	fn stdout_string(self) -> String { self }
	fn stderr_string(self) -> String { self }
}

impl OutputToString for Output {
	fn stdout_string(self) -> String {
		bytes_to_string(&self.stdout)
	}

	fn stderr_string(self) -> String {
		bytes_to_string(&self.stderr)
	}
}

fn bytes_to_string(bytes: &[u8]) -> String {
	String::from_utf8_lossy(bytes).into_owned()
}
