use std::io;
pub use std::process::{Child, Command, Output, Stdio};

// Commands
mod pipe;
pub use pipe::pipe;

/// Generic system command
pub fn execute(cmd: &str, args: &[&str]) -> io::Result<Output> {
	Command::new(cmd).args(args).output()
}

/// Default Output methods to convert Std Streams to String
pub trait StreamsToString {
	fn stdout_string(&self) -> String;
	fn stderr_string(&self) -> String;
}

impl StreamsToString for Output {
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
