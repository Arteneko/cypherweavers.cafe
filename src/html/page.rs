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
						article.badged.node {
							header.badge {
								img src=(&node_badge) alt=(node.label);
							}
							main {
								h3.label { a href=(node.url) { (node.label) } }

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

				section.page {
					h2 { "Neighbors" span.nmap-blink; }

					aside {
						p { "Neighboring star systems (a.k.a webrings) we encountered" }
					}

					div.badges {
						@for node in &data.neighbors {
							@let node_badge = node.get_badge();
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
						}
					}
				}

				section.page {
					h2 { "Peers" span.wg-blink; }

					aside {
						p { "Peers are cute badged websites from friendly entities we met" }
					}

					div.badges {
						@for node in &data.peers {
							@let node_badge = node.get_badge();
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
							a href=(node.link) { img src=(node_badge) alt=(node.label); }
						}
					}
				}
			}
		}
	}
}
