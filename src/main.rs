use cache::download;
use serde_json::json;
use std::{
	error::Error,
	fs::{self, copy, create_dir_all},
};

use crate::{config::Ring, page::make_page};

mod cache;
mod config;
mod page;

fn make_output(content: &String, config: &String) -> Result<(), Box<dyn Error>> {
	copy("style.css", "public/style.css")?;
	fs::write("public/index.html", content)?;
	fs::write("public/config.json", config)?;

	Ok(())
}

fn main() {
	let filename = "config.kdl";
	let nodefile = fs::read_to_string(filename).expect("config.kdl file not found");
	let mut ring = knuffel::parse::<Ring>(filename, &nodefile).expect("invalid kdl file");
	ring.nodes
		.sort_unstable_by(|a, b| a.get_label().cmp(&b.get_label()));

	println!(":: making a cute lil webring in public/");
	create_dir_all("public").expect("somehow failed to create the public dir");

	println!(":: it has {} sites!! wow!!!", ring.nodes.len());

	for knows in &ring.knows {
		if ring.neighbors.iter().find(|n| n.id == knows.id).is_none() {
			println!(
				"!! the ring says it knows {} but there's no neighbor with this id",
				knows.id
			);
		}
	}
	for node in &ring.nodes {
		for knows in &node.knows {
			if ring.neighbors.iter().find(|n| n.id == knows.id).is_none() {
				println!(
					"!! site {} says it knows {} but there's no neighbor with this id",
					node.get_label(),
					knows.id
				);
			}
		}
	}

	println!(":: grabbing the lil badge thingies");
	for node in &mut ring.nodes {
		if let Some(badge_url) = node.get_badge() {
			let proper_extension = download(badge_url, &format!("public/{}.badge", node.get_id()))
				.expect("somehow failed to grab the lil badge thingies");
			node.extension = proper_extension;
		}
	}

	println!(":: making the hypersoup document");
	let web_page = make_page(&ring.clone().into());

	let json_config = json!(ring);

	make_output(&web_page.0, &json_config.to_string())
		.expect("somehow failed to create the website on-disk");

	println!(":: woof!");
}
