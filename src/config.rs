use serde::Serialize;

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
	#[knuffel(argument)]
	pub url: String,
	#[knuffel(property)]
	pub label: Option<String>,
	#[knuffel(property)]
	badge: Option<String>,
	#[knuffel(children(name = "social"))]
	#[serde(default)]
	pub social: Vec<Social>,
	#[knuffel(children(name = "bio"))]
	pub bio: Vec<Bio>,

	pub cached_badge_url: Option<String>,
}

// TODO: want to rework how the nodes we ref are made cuz its kinda messy
#[derive(knuffel::Decode, Serialize, Debug, Clone)]
pub struct Social {
	#[knuffel(argument)]
	pub url: String,
	#[knuffel(argument)]
	pub id: String,
}

impl<'a> Node {
	pub fn get_badge(&'a self) -> Option<&'a String> {
		self.cached_badge_url.as_ref().or(self.badge.as_ref())
	}

	pub fn get_label(&'a self) -> &'a str {
		self.label.as_ref().unwrap_or(&self.url)
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
