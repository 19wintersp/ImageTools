use crate::util::color_to_rgba;
use image::{ DynamicImage, GenericImage };
use color_processing::Color;

define_tool! {
	about: "Generates an image of a single color",
	inputs: 0,
	options: [
		ToolOption {
			name: "color",
			long: Some("color"),
			short: Some("c"),
			help: Some("Specify image color"),
			flags: REQUIRED | TAKES_VALUE,
		},
		ToolOption {
			name: "width",
			long: Some("width"),
			short: Some("w"),
			help: Some("Specify image width"),
			flags: REQUIRED | TAKES_VALUE,
		},
		ToolOption {
			name: "height",
			long: Some("height"),
			short: Some("h"),
			help: Some("Specify image height"),
			flags: REQUIRED | TAKES_VALUE,
		},
	],
}

pub fn run(_inputs: RunInputs, options: RunOptions) -> RunResult {
	let color = option!(options, value of "color");
	let width = option!(options, value of "width" => u32)?;
	let height = option!(options, value of "height" => u32)?;

	let color = match Color::new_string(color) {
		Ok(color) => color_to_rgba(color),
		Err(error) => return Err(
			format!("Color argument invalid ({})", error.to_string())
		),
	};

	let mut image = DynamicImage::new_rgba8(width, height);

	for x in 0..width {
		for y in 0..height {
			image.put_pixel(x, y, color);
		}
	}

	Ok(Some(image))
}
