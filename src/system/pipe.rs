use super::*;

/// Piped series of system commands with convenient interface
///
/// # Example
///
/// ```
/// use brewr::system::pipe;
///
/// pipe(&[
///     ("brew",  &["list", "-1", "--casks"]),
///     ("tr",    &["\n", " "]),
///     ("xargs", &["bash", "-c", r#"brew desc "${0}" "${@}" --eval-all --casks"#]),
/// ]);
/// ```
pub fn pipe(cmd_args: &[(&str, &[&str])]) -> io::Result<Output> {

	let mut child: Option<Child> = None;

	for (cmd, args) in cmd_args.iter() {
		let stdin = match child {
			None => Stdio::null(),
			Some(c) => Stdio::from(c.stdout.unwrap()),
		};

		child = Some(
			Command::new(*cmd)
				.args(*args)
				.stdin(stdin)
				.stdout(Stdio::piped())
				.spawn()?
		);
	}

	child.unwrap().wait_with_output()
}
