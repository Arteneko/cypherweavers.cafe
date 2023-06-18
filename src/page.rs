use maud::{html, Markup, DOCTYPE};

use crate::Social;

pub struct PageData {
	pub title: String,
	// (label, url, optional badge)
	pub nodes: Vec<(String, String, Option<String>, Option<Social>)>,
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
				header.banner {
					h1.title {
						span.banner {
							span.block {
								span { "Cypher" }
								span { "Weavers" }
							}
							span.aside { "Cafe" }
						}
					}
				}


				@for node in &data.nodes {
					article.badged[node.2.is_some()].node {
						@if let Some(badge) = &node.2 {
							header.badge {
								img src=(badge) alt=(node.0);
							}
						}
						main.label {
							a href=(node.1) { (node.0) }

							@if let Some(social) = &node.3 {
								aside.social {
									a rel="me" href=(social.url) { (social.id) }
								}
							}
						}
					}
				}
			}
		}
	}
}
