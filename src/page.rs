use maud::{html, Markup, DOCTYPE};

pub struct PageData {
	pub title: String,
	// (label, url, optional badge)
	pub nodes: Vec<(String, String, Option<String>)>,
}

pub fn make_page(data: &PageData) -> Markup {
	html! {
		(DOCTYPE)
		html {
			head {
				meta charset="utf-8";
				meta name="viewport" content="width=device-width, initial-scale=1";
				title { (data.title) }

				link rel="stylesheet" href="style.css";
			}
			body {
				h1 { (data.title) }

				ul {
					@for node in &data.nodes {
						li {
							a href=(node.1) title=(node.0) {
								@if let Some(badge) = &node.2 {
									img src=(badge) alt=(node.0);
								} @else {
									(node.0)
								}
							}
						}
					}
				}
			}
		}
	}
}
