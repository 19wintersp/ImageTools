use crate::util::color_to_rgba;
use image::{ DynamicImage, GenericImage, GenericImageView };
use color_processing::Color;

const DFT_BACKGROUND: &str = "#00000000";

define_tool! {
	about: "Crops an image",
	inputs: 1,
	options: [
		ToolOption {
			name: "width",
			long: Some("width"),
			short: Some("w"),
			help: Some("Cropped width"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "height",
			long: Some("height"),
			short: Some("h"),
			help: Some("Cropped height"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "xoffset",
			long: Some("x-offset"),
			short: Some("x"),
			help: Some("Crop area offset from top"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "yoffset",
			long: Some("y-offset"),
			short: Some("y"),
			help: Some("Crop area offset from left"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "background",
			long: Some("background"),
			short: Some("b"),
			help: Some("Background color"),
			flags: OPTIONAL | TAKES_VALUE,
		},
	],
}

pub fn run(inputs: RunInputs, options: RunOptions) -> RunResult {
	let image = inputs[0].clone();
	let image_width = image.width() as i32;
	let image_height = image.height() as i32;

	let width = option!(options, value of "width" or image_width.to_string() => u32)?;
	let height = option!(options, value of "height" or image_height.to_string() => u32)?;
	let x_offset = option!(options, value of "xoffset" or "0" => i32)?;
	let y_offset = option!(options, value of "yoffset" or "0" => i32)?;
	let background = option!(options, value of "background" or DFT_BACKGROUND);
	
	let background = match Color::new_string(background) {
		Ok(color) => color_to_rgba(color),
		Err(error) => return Err(
			format!("Background argument invalid ({})", error.to_string())
		),
	};

	let mut output = DynamicImage::new_rgba8(width, height);

	for x in 0..width {
		for y in 0..height {
			let ox = x as i32 + x_offset;
			let oy = y as i32 + y_offset;

			output.put_pixel(
				x, y,
				if ox < 0 || ox > image_width || oy < 0 || oy > image_height {
					background
				} else {
					image.get_pixel(ox as u32, oy as u32)
				}
			);
		}
	}

	Ok(Some(output))
}
