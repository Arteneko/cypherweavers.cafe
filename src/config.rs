use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use serde::Serialize;
use url::Url;

// Neighbor is both webring neighbor and peer (individual entity) neighbor
#[derive(knuffel::Decode, Serialize, Debug, Clone)]
pub struct Neighbor {
	#[knuffel(argument)]
	pub label: String,
	#[knuffel(argument)]
	pub link: String,
	#[knuffel(argument)]
	pub badge: String,
}

#[derive(knuffel::Decode, Serialize, Debug, Clone)]
pub struct Bio {
	#[knuffel(argument)]
	pub paragraph: String,
}

#[derive(knuffel::Decode, Serialize, Debug, Clone)]
pub struct Node {
	pub extension: String,
	#[knuffel(argument)]
	pub url: String,
	#[knuffel(property)]
	pub label: Option<String>,
	#[knuffel(property)]
	pub badge: Option<String>,
	#[knuffel(children(name = "social"))]
	#[serde(default)]
	pub social: Vec<Social>,
	#[knuffel(children(name = "bio"))]
	pub bio: Vec<Bio>,
}

#[derive(knuffel::Decode, Serialize, Debug, Clone)]
pub struct Social {
	#[knuffel(argument)]
	pub url: String,
	#[knuffel(argument)]
	pub id: String,
}

impl<'a> Node {
	// This generates a hash based on the label.
	// it's notably used for associating the in-cache badge image to the node
	pub fn get_id(&'a self) -> String {
		let mut hasher = DefaultHasher::new();
		hasher.write(self.get_label().as_bytes());
		hasher.finish().to_string()
	}

	pub fn get_url(&'a self) -> Url {
		Url::parse(&self.url).expect(format!("invalid url: {}", &self.url).as_str())
	}

	pub fn get_badge(&'a self) -> Option<Url> {
		if let Some(badge) = &self.badge {
			Some(Url::parse(badge).expect(format!("invalid url: {}", badge).as_str()))
		} else {
			None
		}
	}

	pub fn get_cached_badge(&'a self) -> Option<String> {
		if let Some(_) = &self.get_badge() {
			Some(format!("{}.badge.{}", self.get_id(), self.extension))
		} else {
			None
		}
	}

	pub fn get_label(&'a self) -> String {
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

#[derive(knuffel::Decode, Serialize, Debug, Clone)]
pub struct Ring {
	#[knuffel(child, unwrap(argument))]
	pub title: String,
	#[knuffel(children(name = "node"))]
	pub nodes: Vec<Node>,
	#[knuffel(children(name = "neighbor"))]
	pub neighbors: Vec<Neighbor>,
	#[knuffel(children(name = "peer"))]
	pub peers: Vec<Neighbor>,
}
