use page::PageData;
use std::fs::{self, copy, create_dir_all};
use url::Url;

use crate::page::make_page;

mod page;

#[derive(knuffel::Decode, Debug)]
struct Node {
	#[knuffel(argument)]
	url: String,
	#[knuffel(child, unwrap(argument))]
	label: Option<String>,
	#[knuffel(child, unwrap(argument))]
	badge: Option<String>,
}

impl<'a> Node {
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

#[derive(knuffel::Decode, Debug)]
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
						node.get_badge().map(|v| v.to_string()),
					)
				})
				.collect(),
		};

		// Alphabetic sorting of labels
		res.nodes.sort_unstable_by(|a, b| a.0.cmp(&b.0));

		res
	}
}

fn make_output(content: &String) -> Result<(), std::io::Error> {
	create_dir_all("public")?;
	copy("style.css", "public/style.css")?;
	fs::write("public/index.html", content)?;

	Ok(())
}

fn main() {
	let filename = "config.kdl";
	let nodefile = fs::read_to_string(filename).expect("config.kdl file not found");
	let ring = knuffel::parse::<Ring>(filename, &nodefile).expect("invalid kdl file");
	let web_page = make_page(&ring.into());

	make_output(&web_page.0).expect("somehow failed to create the website on-disk");

	println!("woof!");
}
