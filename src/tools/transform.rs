use crate::util::filter_from_string;
use image::GenericImageView;

define_tool! {
	about: "Applies transformation to an image",
	inputs: 1,
	options: [
		ToolOption {
			name: "rclockwise",
			long: Some("rotate-c"),
			short: Some("r"),
			help: Some("Rotate clockwise by 90deg"),
			flags: OPTIONAL | MULTIPLE,
		},
		ToolOption {
			name: "ranticlockwise",
			long: Some("rotate-a"),
			short: Some("R"),
			help: Some("Rotate anti-clockwise by 90deg"),
			flags: OPTIONAL | MULTIPLE,
		},
		ToolOption {
			name: "fhorizontal",
			long: Some("flip-h"),
			short: Some("h"),
			help: Some("Flip horizontally"),
			flags: OPTIONAL,
		},
		ToolOption {
			name: "fvertical",
			long: Some("flip-v"),
			short: Some("v"),
			help: Some("Flip vertically"),
			flags: OPTIONAL,
		},
		ToolOption {
			name: "scale",
			long: Some("scale"),
			short: Some("s"),
			help: Some("Scale image by specified factor"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "resizefilter",
			long: Some("resize-filter"),
			short: Some("f"),
			help: Some("Scale image with filter"),
			flags: OPTIONAL | TAKES_VALUE,
		},
	],
}

pub fn run(inputs: RunInputs, options: RunOptions) -> RunResult {
	let mut image = inputs[0].clone();

	let scale = option!(options, value of "scale" or "1" => f32)?;
	let resize_filter = option!(options, value of "resizefilter" or "nearest");

	let resize_filter = match filter_from_string(resize_filter) {
		Some(filter) => filter,
		None => return Err(
			format!("Resize filter invalid")
		),
	};

	for _ in option!(options, values of "rclockwise") {
		image = image.rotate90();
	}

	for _ in option!(options, values of "ranticlockwise") {
		image = image.rotate270();
	}

	if option!(options, is present "fhorizontal") {
		image = image.fliph();
	}

	if option!(options, is present "fvertical") {
		image = image.flipv();
	}

	image = image.resize(
		(scale * image.width() as f32) as u32,
		(scale * image.height() as f32) as u32,
		resize_filter,
	);

	Ok(Some(image))
}
