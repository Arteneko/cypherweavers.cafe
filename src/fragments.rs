use maud::{html, Markup};

pub fn banner() -> Markup {
	html! {
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
	}
}

pub fn head() -> Markup {
	html! {
		meta charset="utf-8";
		meta name="viewport" content="width=device-width, initial-scale=1";
		link rel="stylesheet" href="style.css";
	}
}
