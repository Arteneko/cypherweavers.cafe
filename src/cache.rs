// cache.rs contains code specific to caching data like badges and such
// we download badges to publish them alongside the website for
// cache / archive / cross-site perfs reasons

use std::{
	error::Error,
	fs::File,
	hash::{DefaultHasher, Hasher},
};

use reqwest::header::CONTENT_TYPE;

pub struct Downloader {
	client: reqwest::blocking::Client,
}

impl Downloader {
	pub fn init() -> reqwest::Result<Self> {
		let client = reqwest::blocking::Client::builder()
			.use_rustls_tls()
			.user_agent("gh/Arteneko/cypherweavers.cafe")
			.build()?;

		Ok(Downloader { client })
	}

	pub fn download(&self, url: &str, label: &str) -> Result<String, Box<dyn Error>> {
		println!(":: grabbing the badge at {}", url);

		let mut res = self.client.get(url).send()?;

		let headers = res.headers();
		let content_type = headers
			.get(CONTENT_TYPE)
			.map(|v| {
				v.to_str()
					.expect("the content-type header should have a proper value")
			})
			.expect("the server should provide a content-type for this image");

		// being proper abt. the extension because some browsers are bitching
		let extension = match content_type {
			"image/png" => "png",
			"image/jpg" | "image/jpeg" => "jpeg",
			"image/gif" => "gif",
			"image/bmp" => "bmp",
			"image/webp" => "webp",
			"image/vnd.microsoft.icon" => "ico",
			wat => {
				println!(":: 	got content type {}, wat??", wat);
				panic!("content_type doesn't make sense");
			}
		};

		// Generating a proper filename
		let mut hasher = DefaultHasher::new();
		hasher.write(label.as_bytes());
		let output_file = format!("badge.{}.{}", hasher.finish().to_string(), extension);

		let mut file = File::create(format!("public/{}", output_file))?;
		res.copy_to(&mut file)?;

		Ok(output_file)
	}
}
