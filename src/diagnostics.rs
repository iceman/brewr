pub fn command_not_in_path(command: &str) -> bool {
	if let Ok(path) = std::env::var("PATH") {
		for p in path.split(':') {
			let p_str = format!("{}/{}", p, command);
			if std::fs::metadata(p_str).is_ok() {
				return false;
			}
		}
	}
	true
}
