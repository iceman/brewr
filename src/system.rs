pub use std::process::{Command, Output, Stdio, Child};
use std::io;
use std::str::from_utf8;

/// Generic system command
pub fn execute(cmd: &str, args: &[&str]) -> io::Result<Output> {
	Command::new(cmd)
		.args(args)
		.output()
}

/// Piped series of system commands with convenient interface
///
/// # Example
///
/// ```
/// use brewr::system::pipe;
///
/// pipe(
///     &[
///         ("brew",  &["list", "-1", "--casks"]),
///         ("tr",    &["\n", " "]),
///         ("xargs", &["bash", "-c", r#"brew desc "${0}" "${@}" --eval-all --casks"#]),
///     ]
/// );
/// ```
pub fn pipe(cmd_args: &[(&str, &[&str])]) -> io::Result<Output> {
	
	let mut child: Option<Child> = None;
	
	for (i, (cmd, args)) in cmd_args.iter().enumerate() {
		let stdio = match i {
			0 => Stdio::null(),
			_ => Stdio::from(child.unwrap().stdout.unwrap()),
		};
		
		child = Some(
			Command::new(*cmd)
				.args(*args)
				.stdin(stdio)
				.stdout(Stdio::piped())
				.spawn()?
		);
	}
	
	child.unwrap().wait_with_output()
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
	match from_utf8(bytes) {
		Ok(s) => String::from(s),
		Err(e) => {
			println!("[Error: {}]", e);
			String::new()
		},
	}
}

