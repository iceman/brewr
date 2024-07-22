use crate::brew::cmd_with_items;
use serde::Deserialize;

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

#[derive(Deserialize)]
#[serde(untagged)]
enum Name {
	Formulae(String),
	Casks(Vec<String>),
}

impl Item {
	fn name(&self) -> String {
		match &self.name {
			Name::Formulae(s) => s,
			Name::Casks(v) => &v[0],
		}
		.to_string()
	}
}

pub fn name_desc_homepage(items: &[&str]) -> [Vec<String>; 3] {
	let result = data_model_from_u8(items);
	match result {
		Ok(d) => vectorize_json_data(items.len(), d),
		Err(e) => [
			vec![e.to_string()],
			vec![String::new()],
			vec![String::new()],
		],
	}
}

fn vectorize_json_data(n: usize, d: Data) -> [Vec<String>; 3] {
	let mut names = Vec::with_capacity(n);
	let mut descs = Vec::with_capacity(n);
	let mut pages = Vec::with_capacity(n);

	for item_type in [d.formulae, d.casks] {
		for item in item_type {
			names.push(item.name());
			descs.push(item.desc.unwrap_or_default());
			pages.push(item.homepage.unwrap_or_default());
		}
	}
	[names, descs, pages]
}

fn data_model_from_u8(items: &[&str]) -> serde_json::Result<Data> {
	let bytes = cmd_with_items("info", items, "--json=v2").output.stdout;
	serde_json::from_slice(&bytes)
}
