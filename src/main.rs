use serde_json::json;
use std::{
	error::Error,
	fs::{self, copy, create_dir_all},
};

use crate::{
	cache::Downloader,
	config::Ring,
	html::{make_card_page, make_page},
};

mod cache;
mod config;
mod html;

fn make_output(
	content: &String,
	_cards: &Vec<String>,
	config: &String,
) -> Result<(), Box<dyn Error>> {
	copy("style.css", "public/style.css")?;
	fs::write("public/index.html", content)?;
	fs::write("public/config.json", config)?;

	Ok(())
}

macro_rules! download_badge {
	($d:ident, $e:ident) => {
		match $d.download(&$e.get_badge(), &$e.label) {
			Ok(cached_name) => $e.cached_badge_url = Some(cached_name),
			Err(e) => println!(
				"failed to grab the lil badge thingy for {}: {:?}",
				$e.label, e
			),
		};
	};
}

fn main() -> miette::Result<()> {
	let filename = "config.kdl";
	let nodefile = fs::read_to_string(filename).expect("config.kdl file not found");
	let mut ring = knuffel::parse::<Ring>(filename, &nodefile)?;
	ring.nodes.sort_unstable_by(|a, b| a.label.cmp(&b.label));
	let downloader = Downloader::init().expect("unable to init http client");

	println!(":: making a cute lil webring in public/");
	create_dir_all("public").expect("somehow failed to create the public dir");

	println!(":: it has {} sites!! wow!!!", ring.nodes.len());

	println!(":: grabbing the lil badge thingies");
	for node in &mut ring.nodes {
		download_badge!(downloader, node);
	}
	for neighbor in &mut ring.neighbors {
		download_badge!(downloader, neighbor);
	}
	for peer in &mut ring.peers {
		download_badge!(downloader, peer);
	}

	println!(":: making the hypersoup documents");
	let web_page = make_page(&ring.clone().into());
	let card_pages: Vec<String> = ring
		.nodes
		.clone()
		.into_iter()
		.map(|node| make_card_page(&node, &ring.neighbors).into_string())
		.collect();

	let json_config = json!(ring);

	make_output(&web_page.0, &card_pages, &json_config.to_string())
		.expect("somehow failed to create the website on-disk");

	println!(":: woof!");

	Ok(())
}
