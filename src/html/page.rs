use maud::{html, Markup, DOCTYPE};

use crate::{
	config::Ring,
	html::fragments::{banner, head},
};

pub fn make_page(data: &Ring) -> Markup {
	html! {
		(DOCTYPE)
		html {
			head {
				(head())

				title { (data.title) }
			}
			body {
				(banner())

				section.page {
					h2 { "Home" span.ls-blink; }

					@for node in &data.nodes {
						@let node_badge = node.get_badge();
						article.badged[node_badge.is_some()].node {
							@if let Some(badge) = &node_badge {
								header.badge {
									img src=(badge) alt=(node.get_label());
								}
							}
							main {
								h3.label { a href=(node.url) { (node.get_label()) } }

								@if node.bio.len() != 0 {
									section.bio {
										@for line in &node.bio {
											p { (line.paragraph) }
										}
									}
								}
							}
							@if node.social.len() != 0 {
								aside.social {
									ul {
										@for s in &node.social {
											li {
												a rel="me" href=(s.url) { (s.id) }
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}
}
