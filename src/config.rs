use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub const GRID: &str = "grid";

fn settings() -> &'static Mutex<HashMap<String, bool>> {
	static CONFIG: OnceLock<Mutex<HashMap<String, bool>>> = OnceLock::new();
	CONFIG.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn insert(k: &str, v: bool) {
	settings()
		.lock()
		.expect("Failed to lock settings mutex")
		.insert(k.to_owned(), v);
}

pub fn get(k: &str) -> bool {
	*settings()
		.lock()
		.expect("Failed to lock settings mutex")
		.get(k)
		.expect("Key not found in settings")
}
