use std::fmt::format;

use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use base64::{encode, decode};
use image::load_from_memory;


#[wasm_bindgen]
pub fn grayscale(encoded_file:&str) -> String
{
	log_1(&"Grayscale Called!".into());
	// log_1(&encoded_file.into());

	let base64_decoded = decode(encoded_file).unwrap();

	log_1(&"Image decoded!".into());

	let mut img = load_from_memory(&base64_decoded).unwrap();

	log_1(&"Image loaded!".into());

	img = img.grayscale();

	log_1(&"Image grayscaled!".into());

	let mut buffer = vec![];

	img.write_to(&mut buffer, image::ImageOutputFormat::Png).unwrap();

	log_1(&"Image written!".into());

	let encoded_file = encode(&buffer);
	let data_url = format!("data:image/png;base64,{}", encoded_file);

	return data_url;
}