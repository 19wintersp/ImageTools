use crate::util::{
	ColorFormat, color_to_rgba, color_to_string, pixel_to_color,
};
use std::str::FromStr;
use color_processing::Color;
use image::{ GenericImage, GenericImageView };

define_tool! {
	about: "Single-pixel operations on an image",
	inputs: 1,
	options: [
		ToolOption {
			name: "xpos",
			long: Some("x-pos"),
			short: Some("x"),
			help: Some("Specify X co-ordinate"),
			flags: REQUIRED | TAKES_VALUE,
		},
		ToolOption {
			name: "ypos",
			long: Some("y-pos"),
			short: Some("y"),
			help: Some("Specify Y co-ordinate"),
			flags: REQUIRED | TAKES_VALUE,
		},
		ToolOption {
			name: "color",
			long: Some("set"),
			short: Some("v"),
			help: Some("Set pixel value"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "format",
			long: Some("color-format"),
			short: Some("f"),
			help: Some("Specify color output format"),
			flags: OPTIONAL | TAKES_VALUE,
		},
	],
}

pub fn run(inputs: RunInputs, options: RunOptions) -> RunResult {
	let mut image = inputs[0].clone();

	let x = option!(options, value of "xpos" => u32)?;
	let y = option!(options, value of "ypos" => u32)?;

	if option!(options, is present "color") {
		let color = Color::new_string(option!(options, value of "color"))
			.map_err(|_| "Color invalid".to_string())?;

		image.put_pixel(x, y, color_to_rgba(color));
		Ok(Some(image))
	} else {
		let color = pixel_to_color(image.get_pixel(x, y));
		let format = ColorFormat::from_str(
			option!(options, value of "format" or "hex").as_str()
		)
			.map_err(|_| "Color format invalid".to_string())?;
		
		println!("{}", color_to_string(color, format));
		Ok(None)
	}
}
