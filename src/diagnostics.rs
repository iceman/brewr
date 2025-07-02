pub fn error_if_not_in_path(cmds: &[&str]) {
	for cmd in cmds {
		if !command_in_path(cmd) {
			eprintln!("{cmd} command not found.");
			std::process::exit(1);
		}
	}
}

fn command_in_path(cmd: &str) -> bool {
	std::env::var_os("PATH")
		.map(|paths| {
			std::env::split_paths(&paths).any(|dir| dir.join(cmd).exists())
		})
		.unwrap_or(false)
}