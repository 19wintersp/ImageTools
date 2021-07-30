use crate::util::filter_from_string;
use image::GenericImageView;

define_tool! {
	about: "Resizes an image to exact dimensions",
	inputs: 1,
	options: [
		ToolOption {
			name: "width",
			long: Some("width"),
			short: Some("w"),
			help: Some("Specify the new width"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "height",
			long: Some("height"),
			short: Some("h"),
			help: Some("Specify the new height"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "resizefilter",
			long: Some("resize-filter"),
			short: Some("f"),
			help: Some("Resize image with filter"),
			flags: OPTIONAL | TAKES_VALUE,
		},
	],
}

pub fn run(inputs: RunInputs, options: RunOptions) -> RunResult {
	let mut image = inputs[0].clone();

	let width = option!(options, value of "width" or image.width().to_string() => u32)?;
	let height = option!(options, value of "height" or image.height().to_string() => u32)?;
	let resize_filter = option!(options, value of "resizefilter" or "nearest");

	let resize_filter = match filter_from_string(resize_filter) {
		Some(filter) => filter,
		None => return Err(
			format!("Resize filter invalid")
		),
	};

	image = image.resize_exact(
		width,
		height,
		resize_filter,
	);

	Ok(Some(image))
}
