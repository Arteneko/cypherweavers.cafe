use cache::download;
use std::{
	error::Error,
	fs::{self, copy, create_dir_all},
};

use crate::{config::Ring, page::make_page};

mod cache;
mod config;
mod page;

fn make_output(content: &String) -> Result<(), Box<dyn Error>> {
	copy("style.css", "public/style.css")?;
	fs::write("public/index.html", content)?;

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
	make_output(&web_page.0).expect("somehow failed to create the website on-disk");

	println!("woof!");
}
