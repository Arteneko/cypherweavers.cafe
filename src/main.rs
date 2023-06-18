use cache::download;
use page::PageData;
use std::{
	collections::hash_map::DefaultHasher,
	error::Error,
	fs::{self, copy, create_dir_all},
	hash::Hasher,
};
use url::Url;

use crate::page::make_page;

mod cache;
mod page;

#[derive(knuffel::Decode, Debug, Clone)]
struct Node {
	extension: String,
	#[knuffel(argument)]
	url: String,
	#[knuffel(child, unwrap(argument))]
	label: Option<String>,
	#[knuffel(child, unwrap(argument))]
	badge: Option<String>,
	#[knuffel(child)]
	social: Option<Social>,
}

#[derive(knuffel::Decode, Debug, Clone)]
pub struct Social {
	#[knuffel(argument)]
	pub url: String,
	#[knuffel(argument)]
	pub id: String,
}

impl<'a> Node {
	// This generates a hash based on the label.
	// it's notably used for associating the in-cache badge image to the node
	fn get_id(&'a self) -> String {
		let mut hasher = DefaultHasher::new();
		hasher.write(self.get_label().as_bytes());
		hasher.finish().to_string()
	}

	fn get_url(&'a self) -> Url {
		Url::parse(&self.url).expect(format!("invalid url: {}", &self.url).as_str())
	}

	fn get_badge(&'a self) -> Option<Url> {
		if let Some(badge) = &self.badge {
			Some(Url::parse(badge).expect(format!("invalid url: {}", badge).as_str()))
		} else {
			None
		}
	}

	fn get_cached_badge(&'a self) -> Option<String> {
		if let Some(_) = &self.get_badge() {
			Some(format!("{}.badge.{}", self.get_id(), self.extension))
		} else {
			None
		}
	}

	fn get_label(&'a self) -> String {
		if let Some(label) = &self.label {
			String::from(label)
		} else {
			String::from(
				self.get_url()
					.host_str()
					.expect("node url should have a domain fragment"),
			)
		}
	}
}

#[derive(knuffel::Decode, Debug, Clone)]
struct Ring {
	#[knuffel(child, unwrap(argument))]
	title: String,
	#[knuffel(children(name = "node"))]
	nodes: Vec<Node>,
}

impl Into<PageData> for Ring {
	fn into(self) -> PageData {
		let mut res = PageData {
			title: self.title,
			nodes: self
				.nodes
				.iter()
				.map(|node| {
					(
						node.get_label(),
						node.get_url().to_string(),
						node.get_cached_badge(),
						node.social.clone(),
					)
				})
				.collect(),
		};

		// Alphabetic sorting of labels
		res.nodes.sort_unstable_by(|a, b| a.0.cmp(&b.0));

		res
	}
}

fn make_output(content: &String) -> Result<(), Box<dyn Error>> {
	copy("style.css", "public/style.css")?;
	fs::write("public/index.html", content)?;

	Ok(())
}

fn main() {
	let filename = "config.kdl";
	let nodefile = fs::read_to_string(filename).expect("config.kdl file not found");
	let mut ring = knuffel::parse::<Ring>(filename, &nodefile).expect("invalid kdl file");

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
