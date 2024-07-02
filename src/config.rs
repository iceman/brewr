use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

fn settings() -> &'static Mutex<HashMap<String, bool>> {
	static CONFIG: OnceLock<Mutex<HashMap<String, bool>>> = OnceLock::new();
	CONFIG.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn insert(k: String, v: bool) {
	settings().lock().unwrap().insert(k, v);
}

pub fn get(k: String) -> bool {
	*settings().lock().unwrap().get(&k).unwrap()
}
