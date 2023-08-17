use maud::{html, Markup, DOCTYPE};

use crate::{
	config::{Neighbor, Node},
	fragments::head,
};

pub fn make_card_page(
	_node: &Node,
	// dependencies
	_neighbors: &Vec<Neighbor>,
) -> Markup {
	html! {
		(DOCTYPE)
		head {
			(head())

			// TODO: Social card stuff!!!
			title { "WOOF TODO" }
		}

		body {
			// TODO: make the header pretty??
			// maybe not the full logo, or maybe a "back to" link

			// TODO: re-make the card here, with extra stuff?? maybe
		}
	}
}
