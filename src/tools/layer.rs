use image::{ DynamicImage, GenericImage, GenericImageView, Pixel };

define_tool! {
	about: "Layers multiple images above each other",
	inputs: any,
	options: [
		ToolOption {
			name: "halign",
			long: Some("h-align"),
			short: Some("h"),
			help: Some("Horizontal alignment (start/center/end)"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "valign",
			long: Some("v-align"),
			short: Some("v"),
			help: Some("Vertical alignment (start/center/end)"),
			flags: OPTIONAL | TAKES_VALUE,
		},
	],
}

pub fn run(inputs: RunInputs, options: RunOptions) -> RunResult {
	let height = inputs.iter().map(|image| image.height()).max().unwrap();
	let width = inputs.iter().map(|image| image.width()).max().unwrap();

	let xoff_mult = off_mult(option!(options, value of "halign" or "start"))?;
	let yoff_mult = off_mult(option!(options, value of "valign" or "start"))?;
	
	let mut out = DynamicImage::new_rgba8(width, height);
	for image in inputs {
		let xoff = xoff_mult(width - image.width());
		let yoff = yoff_mult(height - image.height());

		for x in 0..image.width() {
			for y in 0..image.height() {
				let mut pixel = out.get_pixel(x + xoff, y + yoff);
				pixel.blend(&image.get_pixel(x, y));
				out.put_pixel(x + xoff, y + yoff, pixel);
			}
		}
	}

	Ok(Some(out))
}

fn off_mult(alignment: String) -> Result<Box<dyn Fn(u32) -> u32>, String> {
	let mult: f32 = match alignment.as_str() {
		"start" | "s" => 0f32,
		"center" | "c" => 0.5f32,
		"end" | "e" => 1f32,
		_ => return Err("Invalid alignment".into()),
	};

	Ok(Box::new(move |number: u32| (number as f32 * mult) as u32))
}
