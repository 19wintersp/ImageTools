use crate::util::image_type;
use std::hash::{ Hash, Hasher };
use std::collections::hash_map::DefaultHasher;
use image::GenericImageView;

define_tool! {
	about: "Gets information about an image",
	inputs: 1,
	options: [
		ToolOption {
			name: "machine",
			long: Some("machine"),
			short: Some("m"),
			help: Some("Print a more machine-parsable output"),
			flags: OPTIONAL,
		},
	],
}

pub fn run(inputs: RunInputs, options: RunOptions) -> RunResult {
	let image = inputs[0].clone();

	let mut hasher = DefaultHasher::new();
	image.hash(&mut hasher);

	if option!(options, is present "machine") {
		println!("width={}", image.width());
		println!("height={}", image.height());
		println!("type={}", image_type(image));
		println!("hash={:x}", hasher.finish());
	} else {
		println!(" Width: {}px", image.width());
		println!("Height: {}px", image.height());
		println!("  Type: {}", image_type(image));
		println!("  Hash: {:x}", hasher.finish());
	}

	Ok(None)
}
