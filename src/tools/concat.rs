use crate::util::color_to_rgba;
use image::{ DynamicImage, GenericImage, GenericImageView };
use color_processing::Color;

const DFT_BACKGROUND: &str = "#00000000";
const DFT_PADDING: &str    = "0";

define_tool! {
	about: "Concatenates multiple images together",
	inputs: any,
	options: [
		ToolOption {
			name: "vertical",
			long: Some("vertical"),
			short: Some("v"),
			help: Some("Concatenate vertically"),
			flags: OPTIONAL,
		},
		ToolOption {
			name: "background",
			long: Some("background"),
			short: Some("b"),
			help: Some("Background between images"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "padding",
			long: Some("padding"),
			short: Some("p"),
			help: Some("Padding between images"),
			flags: OPTIONAL | TAKES_VALUE,
		},
	],
}

pub fn run(inputs: RunInputs, options: RunOptions) -> RunResult {
	let vertical = option!(options, is present "vertical");
	let background = option!(options, value of "background" or DFT_BACKGROUND);
	let padding = option!(options, value of "padding" or DFT_PADDING => u32)?;

	let background = match Color::new_string(background) {
		Ok(color) => color_to_rgba(color),
		Err(error) => return Err(
			format!("Background argument invalid ({})", error.to_string())
		),
	};

	let pri_dim = inputs.iter()
		.map(|i| if vertical { i.height() } else { i.width() })
		.sum::<u32>()
		+ ((inputs.len() - 1) as u32 * padding);
	
	let alt_dim = inputs.iter()
		.map(|i| if vertical { i.width() } else { i.height() })
		.max().unwrap();

	let mut output = DynamicImage::new_rgba8(
		if vertical { alt_dim } else { pri_dim },
		if vertical { pri_dim } else { alt_dim },
	);

	let mut offset = 0u32;

	for (index, image) in inputs.iter().enumerate() {
		let image_pri_dim = if vertical { image.height() }
			else { image.width() };
		let image_alt_dim = if vertical { image.width() }
			else { image.height() };

		for pri in 0..image_pri_dim {
			for alt in 0..image_alt_dim {
				output.put_pixel(
					if vertical { alt }
					else { offset },
					if vertical { offset }
					else { alt },
					image.get_pixel(
						if vertical { alt }
						else { pri },
						if vertical { pri }
						else { alt },
					),
				);
			}

			for alt in image_alt_dim..alt_dim {
				output.put_pixel(
					if vertical { alt }
					else { offset },
					if vertical { offset }
					else { alt },
					background,
				);
			}

			offset += 1;
		}

		if index < (inputs.len() - 1) {
			for _ in 0..padding {
				for alt in 0..alt_dim {
					output.put_pixel(
						if vertical { alt }
						else { offset },
						if vertical { offset }
						else { alt },
						background,
					);
				}

				offset += 1;
			}
		}
	}
	
	Ok(Some(output))
}
