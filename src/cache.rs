// cache.rs contains code specific to caching data like badges and such
// we download badges to publish them alongside the website for
// cache / archive / cross-site perfs reasons

use std::{error::Error, fs::File};

use reqwest::header::CONTENT_TYPE;
use url::Url;

pub fn download(url: Url, output_path: &str) -> Result<String, Box<dyn Error>> {
	println!(":: grabbing the badge at {}", url);

	let mut res = reqwest::blocking::get(url)?;

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
		"image/vnd.microsoft.icon" => "ico",
		wat => {
			println!(":: 	got content type {}, wat??", wat);
			panic!("content_type doesn't make sense");
		}
	};

	let mut file = File::create(format!("{}.{}", output_path, extension))?;

	res.copy_to(&mut file)?;

	Ok(String::from(extension))
}
