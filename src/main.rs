use std::fs;

use url::Url;

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
    #[knuffel(children(name = "node"))]
    nodes: Vec<Node>,
}

fn main() {
    let filename = "nodes.kdl";
    let nodefile = fs::read_to_string(filename).expect("nodes.kdl file not found");

    let ring = knuffel::parse::<Ring>(filename, &nodefile).expect("invalid kdl file");

    println!("woof: {:?}", ring.nodes[1].get_label());
}
