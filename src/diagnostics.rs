pub fn error_if_not_in_path(cmds: &[&str]) {
	for cmd in cmds {
		if command_not_in_path(cmd) {
			eprintln!("{} command not found.", cmd);
			std::process::exit(1);
		}
	}
}

fn command_not_in_path(cmd: &str) -> bool {
	if let Ok(path) = std::env::var("PATH") {
		for p in path.split(':') {
			let p_str = format!("{}/{}", p, cmd);
			if std::fs::metadata(p_str).is_ok() {
				return false;
			}
		}
	}
	true
}
