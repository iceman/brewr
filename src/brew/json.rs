use serde::Deserialize;
use super::Brew;

#[derive(Deserialize)]
struct Data {
	formulae: Vec<Item>,
	casks: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
	name: Name,
	desc: String,
	homepage: String,
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

pub fn name_desc_homepage(items: &[&str]) -> [Vec<String>;3] {
	let n = items.len();
	let mut names = Vec::with_capacity(n);
	let mut descs = Vec::with_capacity(n);
	let mut pages = Vec::with_capacity(n);
	
	let d = data_model_from_u8(items);
	
	for item_type in [d.formulae, d.casks] {
		for item in item_type {
			names.push(item.name());
			descs.push(item.desc);
			pages.push(item.homepage);
		}
	}
	[names, descs, pages]
}

fn data_model_from_u8(items: &[&str]) -> Data {
	let bytes = json(items).output.stdout;
	serde_json::from_slice(&bytes).unwrap()
}

fn json(items: &[&str]) -> Brew {
	Brew::cmd_with_items("info", items, "--json=v2")
}
