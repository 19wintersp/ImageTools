use crate::util::color_to_rgba;
use image::{ DynamicImage, GenericImage, GenericImageView, Rgba };
use color_processing::Color;

define_tool! {
	about: "Adds a border to the image",
	inputs: 1,
	options: [
		ToolOption {
			name: "color",
			long: Some("color"),
			short: Some("c"),
			help: Some("Specify border color"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "width",
			long: Some("width"),
			short: Some("w"),
			help: Some("Specify border width"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "radius",
			long: Some("radius"),
			short: Some("r"),
			help: Some("Specify border radius"),
			flags: OPTIONAL | TAKES_VALUE,
		},
	],
}

pub fn run(inputs: RunInputs, options: RunOptions) -> RunResult {
	let input = inputs[0].clone();

	let color = option!(options, value of "color" or "black");
	let width = option!(options, value of "width" or "0" => u32)?;
	let radius = option!(options, value of "radius" or "0" => u32)?;

	let color = match Color::new_string(color) {
		Ok(color) => color_to_rgba(color),
		Err(error) => return Err(
			format!("Color argument invalid ({})", error.to_string())
		),
	};

	let imw = input.width() + (width * 2);
	let imh = input.height() + (width * 2);
	let mut image = DynamicImage::new_rgba8(imw, imh);

	for x in 0..imw {
		for y in 0..imh {
			if x >= width
				&& x < (width + input.width())
				&& y >= width
				&& y < (width + input.height())
			{
				image.put_pixel(x, y, input.get_pixel(x - width, y - width));
			} else {
				image.put_pixel(x, y, color);
			}
		}
	}

	let transparent = Rgba([ 0, 0, 0, 0 ]);

	for x in 0..radius {
		for y in 0..radius {
			if !point_on_circle(
				(radius as i32, radius as i32),
				radius as i32,
				(x as i32, y as i32),
			) {
				image.put_pixel(x, y, transparent);
				image.put_pixel(imw - x - 1, y, transparent);
				image.put_pixel(x, imh - y - 1, transparent);
				image.put_pixel(imw - x - 1, imh - y - 1, transparent);
			}
		}
	}

	Ok(Some(image))
}

fn point_on_circle(center: (i32, i32), radius: i32, point: (i32, i32)) -> bool {
	(point.0 - center.0).pow(2) + (point.1 - center.1).pow(2) <= radius.pow(2)
}
