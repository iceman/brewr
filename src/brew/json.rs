use serde::Deserialize;

use super::system;

#[derive(Deserialize)]
struct Data {
	formulae: Vec<Item>,
	casks: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
	name: Name,
	desc: Option<String>,
	homepage: Option<String>,
}

impl Item {
	fn name(&self) -> String {
		match &self.name {
			Name::Formulae(s) => s,
			Name::Casks(v) => &v[0],
		}
		.to_owned()
	}
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Name {
	Formulae(String),
	Casks(Vec<String>),
}

/// Name, description, homepage from JSON data parse
pub(super) fn name_desc_homepage_array(items: &[&str]) -> [Vec<String>; 3] {
	let output = system::execute_with_items("info", items, Some("--json=v2"));
	match serde_json::from_slice(&output.stdout) {
		Ok(d)  => vectorize_json_data(items.len(), d),
		Err(e) => [
			vec![e.to_string()],
			vec![String::new()],
			vec![String::new()],
		],
	}
}

fn vectorize_json_data(size: usize, d: Data) -> [Vec<String>; 3] {
	let mut names = Vec::with_capacity(size);
	let mut descs = Vec::with_capacity(size);
	let mut pages = Vec::with_capacity(size);

	for item_type in [d.formulae, d.casks] {
		for item in item_type {
			names.push(item.name());
			descs.push(item.desc.unwrap_or_default());
			pages.push(item.homepage.unwrap_or_default());
		}
	}
	[names, descs, pages]
}
